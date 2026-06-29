/// Pure function — no I/O, no async. Returns true if any permission authorizes the action on the resource.
/// Matches exact resource OR wildcard "*". Matches exact action OR wildcard "*".
pub fn can(permissions: &[crate::domain::entities::auth::Permission], action: &str, resource: &str) -> bool {
    permissions.iter().any(|p| {
        (p.resource == "*" || p.resource == resource) &&
        (p.action == "*" || p.action == action)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::entities::auth::Permission;
    use uuid::Uuid;

    #[test]
    fn exact_match() {
        let perms = vec![Permission { id: Uuid::new_v4(), resource: "orders".into(), action: "read".into() }];
        assert!(can(&perms, "read", "orders"));
    }

    #[test]
    fn no_match_wrong_action() {
        let perms = vec![Permission { id: Uuid::new_v4(), resource: "orders".into(), action: "read".into() }];
        assert!(!can(&perms, "write", "orders"));
    }

    #[test]
    fn wildcard_resource() {
        let perms = vec![Permission { id: Uuid::new_v4(), resource: "*".into(), action: "read".into() }];
        assert!(can(&perms, "read", "orders"));
    }

    #[test]
    fn wildcard_action() {
        let perms = vec![Permission { id: Uuid::new_v4(), resource: "orders".into(), action: "*".into() }];
        assert!(can(&perms, "write", "orders"));
    }

    #[test]
    fn empty_permissions_deny() {
        assert!(!can(&[], "read", "orders"));
    }

    #[test]
    fn multiple_permissions_union() {
        let perms = vec![
            Permission { id: Uuid::new_v4(), resource: "orders".into(), action: "read".into() },
            Permission { id: Uuid::new_v4(), resource: "menu".into(), action: "*".into() },
        ];
        assert!(can(&perms, "write", "menu"));
        assert!(can(&perms, "read", "orders"));
        assert!(!can(&perms, "write", "orders"));
    }

    #[test]
    fn wrong_resource() {
        let perms = vec![Permission { id: Uuid::new_v4(), resource: "orders".into(), action: "read".into() }];
        assert!(!can(&perms, "read", "reports"));
    }
}
