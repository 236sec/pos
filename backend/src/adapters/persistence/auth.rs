use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::QueryBuilder;
use uuid::Uuid;

use crate::{
    adapters::persistence::PostgresPersistence,
    application::{app_error::AppResult, use_cases::auth::AuthPersistence},
    domain::entities::auth::{Permission, Role, User},
};

#[async_trait]
impl AuthPersistence for PostgresPersistence {
    async fn create_user(&self, username: &str, email: &str, password_hash: &str) -> AppResult<()> {
        let uuid = Uuid::new_v4();
        sqlx::query(
            "INSERT INTO users (id, username, email, password_hash) VALUES ($1, $2, $3, $4)",
        )
        .bind(uuid)
        .bind(username)
        .bind(email)
        .bind(password_hash)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn find_user_by_username(&self, username: &str) -> AppResult<Option<User>> {
        let row = sqlx::query_as::<_, (Uuid, String, String, String, DateTime<Utc>)>(
            "SELECT id, username, email, password_hash, updated_at FROM cached_users WHERE username = $1",
        )
        .bind(username)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|(id, username, email, password_hash, updated_at)| User {
            id,
            username,
            email,
            password_hash,
            created_at: updated_at.naive_utc(),
        }))
    }

    async fn find_user_by_id(&self, id: Uuid) -> AppResult<Option<User>> {
        let row = sqlx::query_as::<_, (Uuid, String, String, String, DateTime<Utc>)>(
            "SELECT id, username, email, password_hash, updated_at FROM cached_users WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|(id, username, email, password_hash, updated_at)| User {
            id,
            username,
            email,
            password_hash,
            created_at: updated_at.naive_utc(),
        }))
    }

    async fn find_roles_for_user(&self, user_id: Uuid) -> AppResult<Vec<Role>> {
        let rows = sqlx::query_as::<_, (Uuid, String)>(
            r#"
            SELECT r.id, r.name
            FROM cached_roles r
            JOIN cached_user_roles ur ON ur.role_id = r.id
            WHERE ur.user_id = $1
            "#,
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|(id, name)| Role { id, name })
            .collect())
    }

    async fn find_permissions_for_roles(&self, role_ids: &[Uuid]) -> AppResult<Vec<Permission>> {
        if role_ids.is_empty() {
            return Ok(vec![]);
        }

        let mut builder = QueryBuilder::new(
            "SELECT DISTINCT p.id, p.resource, p.action FROM cached_permissions p JOIN cached_role_permissions rp ON rp.permission_id = p.id WHERE rp.role_id IN ("
        );

        let mut first = true;
        for role_id in role_ids {
            if first {
                first = false;
            } else {
                builder.push(", ");
            }
            builder.push_bind(role_id);
        }
        builder.push(")");

        let rows = builder
            .build_query_as::<(Uuid, String, String)>()
            .fetch_all(&self.pool)
            .await?;

        Ok(rows
            .into_iter()
            .map(|(id, resource, action)| Permission { id, resource, action })
            .collect())
    }

    async fn upsert_user(&self, user: &User) -> AppResult<()> {
        let updated_at: DateTime<Utc> = DateTime::from_naive_utc_and_offset(user.created_at, Utc);
        sqlx::query(
            r#"
            INSERT INTO cached_users (id, username, email, password_hash, updated_at)
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (id) DO UPDATE SET
                username = EXCLUDED.username,
                email = EXCLUDED.email,
                password_hash = EXCLUDED.password_hash,
                updated_at = EXCLUDED.updated_at
            "#,
        )
        .bind(user.id)
        .bind(&user.username)
        .bind(&user.email)
        .bind(&user.password_hash)
        .bind(updated_at)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn upsert_role(&self, role: &Role) -> AppResult<()> {
        sqlx::query(
            r#"
            INSERT INTO cached_roles (id, name, updated_at)
            VALUES ($1, $2, NOW())
            ON CONFLICT (id) DO UPDATE SET
                name = EXCLUDED.name,
                updated_at = NOW()
            "#,
        )
        .bind(role.id)
        .bind(&role.name)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn upsert_permission(&self, perm: &Permission) -> AppResult<()> {
        sqlx::query(
            r#"
            INSERT INTO cached_permissions (id, resource, action, updated_at)
            VALUES ($1, $2, $3, NOW())
            ON CONFLICT (resource, action) DO UPDATE SET
                id = EXCLUDED.id,
                updated_at = NOW()
            "#,
        )
        .bind(perm.id)
        .bind(&perm.resource)
        .bind(&perm.action)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn upsert_user_role(&self, user_id: Uuid, role_id: Uuid) -> AppResult<()> {
        sqlx::query(
            r#"
            INSERT INTO cached_user_roles (user_id, role_id)
            VALUES ($1, $2)
            ON CONFLICT DO NOTHING
            "#,
        )
        .bind(user_id)
        .bind(role_id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn upsert_role_permission(&self, role_id: Uuid, perm_id: Uuid) -> AppResult<()> {
        sqlx::query(
            r#"
            INSERT INTO cached_role_permissions (role_id, permission_id)
            VALUES ($1, $2)
            ON CONFLICT DO NOTHING
            "#,
        )
        .bind(role_id)
        .bind(perm_id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn get_last_sync_timestamp(&self) -> AppResult<Option<DateTime<Utc>>> {
        let row = sqlx::query_scalar::<_, Option<DateTime<Utc>>>(
            "SELECT MAX(updated_at) FROM cached_users",
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(row)
    }

    async fn update_sync_timestamp(&self, _ts: DateTime<Utc>) -> AppResult<()> {
        // No-op: we derive the last sync timestamp from MAX(updated_at) on cached_users.
        // This avoids needing an extra table.
        Ok(())
    }
}
