CREATE TABLE cached_users (
    id UUID PRIMARY KEY,
    username VARCHAR(50) UNIQUE NOT NULL,
    email VARCHAR(100) NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE cached_roles (
    id UUID PRIMARY KEY,
    name VARCHAR(50) UNIQUE NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE cached_permissions (
    id UUID PRIMARY KEY,
    resource VARCHAR(100) NOT NULL,
    action VARCHAR(20) NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(resource, action)
);

CREATE TABLE cached_user_roles (
    user_id UUID REFERENCES cached_users(id) ON DELETE CASCADE,
    role_id UUID REFERENCES cached_roles(id) ON DELETE CASCADE,
    PRIMARY KEY (user_id, role_id)
);

CREATE TABLE cached_role_permissions (
    role_id UUID REFERENCES cached_roles(id) ON DELETE CASCADE,
    permission_id UUID REFERENCES cached_permissions(id) ON DELETE CASCADE,
    PRIMARY KEY (role_id, permission_id)
);
