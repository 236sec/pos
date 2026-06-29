-- Create menu management tables for head office
-- Order: menu_categories → menu_items → modifier_groups → modifier_options, channel_prices

CREATE TABLE IF NOT EXISTS menu_categories (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    name_en VARCHAR(255) NOT NULL,
    sort_order INTEGER NOT NULL DEFAULT 0,
    deleted_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS menu_items (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    name_en VARCHAR(255) NOT NULL,
    category_id UUID NOT NULL REFERENCES menu_categories(id),
    base_price BIGINT NOT NULL,
    image_url TEXT,
    is_available BOOLEAN NOT NULL DEFAULT true,
    deleted_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_menu_items_category_id ON menu_items(category_id);
CREATE INDEX idx_menu_items_deleted_at ON menu_items(deleted_at);

CREATE TABLE IF NOT EXISTS modifier_groups (
    id UUID PRIMARY KEY,
    menu_item_id UUID NOT NULL REFERENCES menu_items(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    name_en VARCHAR(255) NOT NULL,
    selection_type VARCHAR(20) NOT NULL CHECK (selection_type IN ('single', 'multiple')),
    is_required BOOLEAN NOT NULL DEFAULT false,
    sort_order INTEGER NOT NULL DEFAULT 0,
    deleted_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(menu_item_id, name)
);

CREATE INDEX idx_modifier_groups_menu_item_id ON modifier_groups(menu_item_id);

CREATE TABLE IF NOT EXISTS modifier_options (
    id UUID PRIMARY KEY,
    modifier_group_id UUID NOT NULL REFERENCES modifier_groups(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    name_en VARCHAR(255) NOT NULL,
    price BIGINT NOT NULL DEFAULT 0,
    sort_order INTEGER NOT NULL DEFAULT 0,
    deleted_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_modifier_options_modifier_group_id ON modifier_options(modifier_group_id);

CREATE TABLE IF NOT EXISTS channel_prices (
    id UUID PRIMARY KEY,
    menu_item_id UUID NOT NULL REFERENCES menu_items(id) ON DELETE CASCADE,
    channel VARCHAR(20) NOT NULL CHECK (channel IN ('DineIn', 'Takeaway', 'Delivery')),
    price BIGINT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(menu_item_id, channel)
);

CREATE INDEX idx_channel_prices_menu_item_id ON channel_prices(menu_item_id);
