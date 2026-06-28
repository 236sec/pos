-- Branch operational data
CREATE TABLE orders (
    id UUID PRIMARY KEY,
    branch_id UUID NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'pending',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
