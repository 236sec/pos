use std::sync::Arc;

use argon2::{Argon2, PasswordHash, PasswordVerifier};
use async_trait::async_trait;
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize};
use tracing::{info, instrument, warn};
use uuid::Uuid;

use crate::application::app_error::AppError;
use crate::application::app_error::AppResult;
use crate::domain::entities::auth::{Permission, Role, User};

// ── Response / Request types ──────────────────────────────────────────────

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub user: UserResponse,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub roles: Vec<Role>,
    pub permissions: Vec<Permission>,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct SyncUser {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub roles: Vec<SyncRole>,
}

#[derive(Debug, Deserialize)]
pub struct SyncRole {
    pub id: Uuid,
    pub name: String,
    pub permissions: Vec<SyncPermission>,
}

#[derive(Debug, Deserialize)]
pub struct SyncPermission {
    pub id: Uuid,
    pub resource: String,
    pub action: String,
}

// ── JWT claims ────────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub username: String,
    pub branch_id: String,
    pub roles: Vec<String>,
    pub exp: usize,
    pub iat: usize,
}

// ── Persistence trait ─────────────────────────────────────────────────────

#[async_trait]
pub trait AuthPersistence: Send + Sync {
    async fn create_user(&self, username: &str, email: &str, password_hash: &str) -> AppResult<()>;
    async fn find_user_by_username(&self, username: &str) -> AppResult<Option<User>>;
    async fn find_user_by_id(&self, id: Uuid) -> AppResult<Option<User>>;
    async fn find_roles_for_user(&self, user_id: Uuid) -> AppResult<Vec<Role>>;
    async fn find_permissions_for_roles(&self, role_ids: &[Uuid]) -> AppResult<Vec<Permission>>;
    async fn upsert_user(&self, user: &User) -> AppResult<()>;
    async fn upsert_role(&self, role: &Role) -> AppResult<()>;
    async fn upsert_permission(&self, perm: &Permission) -> AppResult<()>;
    async fn upsert_user_role(&self, user_id: Uuid, role_id: Uuid) -> AppResult<()>;
    async fn upsert_role_permission(&self, role_id: Uuid, perm_id: Uuid) -> AppResult<()>;
    async fn get_last_sync_timestamp(&self) -> AppResult<Option<chrono::DateTime<chrono::Utc>>>;
    async fn update_sync_timestamp(&self, ts: chrono::DateTime<chrono::Utc>) -> AppResult<()>;
}

// ── Use cases ─────────────────────────────────────────────────────────────

#[derive(Clone)]
pub struct AuthUseCases {
    persistence: Arc<dyn AuthPersistence>,
    jwt_secret: String,
    branch_id: Uuid,
    head_office_url: String,
}

impl AuthUseCases {
    pub fn new(
        persistence: Arc<dyn AuthPersistence>,
        jwt_secret: String,
        branch_id: Uuid,
        head_office_url: String,
    ) -> Self {
        Self {
            persistence,
            jwt_secret,
            branch_id,
            head_office_url,
        }
    }

    /// Expose the last sync timestamp (used by the sync worker).
    #[instrument(skip(self))]
    pub async fn get_last_sync_timestamp(
        &self,
    ) -> AppResult<Option<chrono::DateTime<chrono::Utc>>> {
        self.persistence.get_last_sync_timestamp().await
    }

    #[instrument(skip(self))]
    pub async fn add(&self, username: &str, email: &str, password: &SecretString) -> AppResult<()> {
        info!("Adding user...");

        self.persistence
            .create_user(username, email, password.expose_secret())
            .await?;

        info!("Adding user finished.");

        Ok(())
    }

    /// Authenticate a user by username and password.
    /// Falls back to cached credentials if the head office is unreachable.
    #[instrument(skip(self, password))]
    pub async fn login(&self, username: &str, password: &SecretString) -> AppResult<LoginResponse> {
        info!("Attempting login for user: {}", username);

        // 1. Try to find user in local cache first
        let user = self
            .persistence
            .find_user_by_username(username)
            .await?
            .ok_or_else(|| AppError::AuthenticationError("User not found".into()))?;

        // 2. Verify password
        let parsed_hash = PasswordHash::new(&user.password_hash)
            .map_err(|e| AppError::AuthenticationError(format!("Invalid password hash: {}", e)))?;

        Argon2::default()
            .verify_password(password.expose_secret().as_bytes(), &parsed_hash)
            .map_err(|_| AppError::AuthenticationError("Invalid password".into()))?;

        // 3. Fetch roles and permissions
        let roles = self.persistence.find_roles_for_user(user.id).await?;
        let role_ids: Vec<Uuid> = roles.iter().map(|r| r.id).collect();
        let permissions = if role_ids.is_empty() {
            vec![]
        } else {
            self.persistence
                .find_permissions_for_roles(&role_ids)
                .await?
        };

        // 4. Build JWT
        let now = Utc::now();
        let claims = Claims {
            sub: user.id.to_string(),
            username: user.username.clone(),
            branch_id: self.branch_id.to_string(),
            roles: roles.iter().map(|r| r.name.clone()).collect(),
            exp: (now + Duration::hours(24)).timestamp() as usize,
            iat: now.timestamp() as usize,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_bytes()),
        )
        .map_err(|e| AppError::Internal(format!("JWT encoding failed: {}", e)))?;

        Ok(LoginResponse {
            token,
            user: UserResponse {
                id: user.id,
                username: user.username,
                email: user.email,
                roles,
                permissions,
            },
        })
    }

    /// Decode a JWT and return the user's profile.
    #[instrument(skip(self, token))]
    pub async fn me(&self, token: &str) -> AppResult<UserResponse> {
        info!("Fetching current user from token");

        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.jwt_secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|e| AppError::AuthenticationError(format!("Invalid token: {}", e)))?;

        let claims = token_data.claims;
        let user_id = Uuid::parse_str(&claims.sub).map_err(|e| {
            AppError::AuthenticationError(format!("Invalid user id in token: {}", e))
        })?;

        let user = self
            .persistence
            .find_user_by_id(user_id)
            .await?
            .ok_or_else(|| AppError::AuthenticationError("User not found".into()))?;

        let roles = self.persistence.find_roles_for_user(user.id).await?;
        let role_ids: Vec<Uuid> = roles.iter().map(|r| r.id).collect();
        let permissions = if role_ids.is_empty() {
            vec![]
        } else {
            self.persistence
                .find_permissions_for_roles(&role_ids)
                .await?
        };

        Ok(UserResponse {
            id: user.id,
            username: user.username,
            email: user.email,
            roles,
            permissions,
        })
    }

    /// Sync users from head office, upserting into local cache.
    #[instrument(skip(self))]
    pub async fn sync_users(&self, since: chrono::DateTime<chrono::Utc>) -> AppResult<()> {
        info!("Syncing users from head office since {}", since);

        let url = format!(
            "{}/sync/users?since={}",
            self.head_office_url,
            since.format("%Y-%m-%dT%H:%M:%S%.3fZ")
        );

        let client = reqwest::Client::new();
        let response = match client.get(&url).send().await {
            Ok(resp) => resp,
            Err(e) => {
                warn!("Head office unreachable: {}. Using cached credentials.", e);
                return Ok(());
            }
        };

        let sync_users: Vec<SyncUser> = match response.json().await {
            Ok(users) => users,
            Err(e) => {
                warn!("Failed to parse sync response: {}", e);
                return Ok(());
            }
        };

        for sync_user in &sync_users {
            let user = User {
                id: sync_user.id,
                username: sync_user.username.clone(),
                email: sync_user.email.clone(),
                password_hash: sync_user.password_hash.clone(),
                created_at: sync_user.updated_at.naive_utc(),
            };
            self.persistence.upsert_user(&user).await?;

            for role in &sync_user.roles {
                let r = Role {
                    id: role.id,
                    name: role.name.clone(),
                };
                self.persistence.upsert_role(&r).await?;
                self.persistence.upsert_user_role(user.id, role.id).await?;

                for perm in &role.permissions {
                    let p = Permission {
                        id: perm.id,
                        resource: perm.resource.clone(),
                        action: perm.action.clone(),
                    };
                    self.persistence.upsert_permission(&p).await?;
                    self.persistence
                        .upsert_role_permission(role.id, perm.id)
                        .await?;
                }
            }
        }

        self.persistence.update_sync_timestamp(Utc::now()).await?;

        info!("Sync completed: {} users processed", sync_users.len());

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use async_trait::async_trait;

    use super::*;

    struct MockAuthPersistence;

    #[async_trait]
    impl AuthPersistence for MockAuthPersistence {
        async fn create_user(
            &self,
            _username: &str,
            _email: &str,
            _password_hash: &str,
        ) -> AppResult<()> {
            Ok(())
        }

        async fn find_user_by_username(&self, _username: &str) -> AppResult<Option<User>> {
            Ok(None)
        }

        async fn find_user_by_id(&self, _id: Uuid) -> AppResult<Option<User>> {
            Ok(None)
        }

        async fn find_roles_for_user(&self, _user_id: Uuid) -> AppResult<Vec<Role>> {
            Ok(vec![])
        }

        async fn find_permissions_for_roles(
            &self,
            _role_ids: &[Uuid],
        ) -> AppResult<Vec<Permission>> {
            Ok(vec![])
        }

        async fn upsert_user(&self, _user: &User) -> AppResult<()> {
            Ok(())
        }

        async fn upsert_role(&self, _role: &Role) -> AppResult<()> {
            Ok(())
        }

        async fn upsert_permission(&self, _perm: &Permission) -> AppResult<()> {
            Ok(())
        }

        async fn upsert_user_role(&self, _user_id: Uuid, _role_id: Uuid) -> AppResult<()> {
            Ok(())
        }

        async fn upsert_role_permission(&self, _role_id: Uuid, _perm_id: Uuid) -> AppResult<()> {
            Ok(())
        }

        async fn get_last_sync_timestamp(
            &self,
        ) -> AppResult<Option<chrono::DateTime<chrono::Utc>>> {
            Ok(None)
        }

        async fn update_sync_timestamp(&self, _ts: chrono::DateTime<chrono::Utc>) -> AppResult<()> {
            Ok(())
        }
    }

    #[tokio::test]
    async fn add_user_works() {
        let use_cases = AuthUseCases::new(
            Arc::new(MockAuthPersistence),
            "test_secret".into(),
            Uuid::new_v4(),
            "http://localhost:9999".into(),
        );

        let result = use_cases
            .add("testuser", "testuser@example.com", &"test_pw".into())
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn login_fails_for_unknown_user() {
        let use_cases = AuthUseCases::new(
            Arc::new(MockAuthPersistence),
            "test_secret".into(),
            Uuid::new_v4(),
            "http://localhost:9999".into(),
        );

        let result = use_cases.login("unknown", &"password".into()).await;

        assert!(result.is_err());
    }
}
