import { ApiClient } from "../client";
import type {
  MenuItem,
  MenuCategory,
  Table,
  Promotion,
  ChannelPrice,
  Money,
} from "../../types";

// ── Request types ────────────────────────────────────────────

export interface CreateModifierOptionRequest {
  name: string;
  name_en: string;
  price: Money;
  sort_order: number;
}

export interface CreateModifierGroupRequest {
  name: string;
  name_en: string;
  selection_type: "single" | "multiple";
  is_required: boolean;
  sort_order: number;
  options: CreateModifierOptionRequest[];
}

export interface CreateMenuItemRequest {
  name: string;
  name_en: string;
  category_id: string;
  base_price: Money;
  image_url?: string | null;
  modifier_groups: CreateModifierGroupRequest[];
  channel_prices: ChannelPrice[];
}

export interface UpdateMenuItemRequest
  extends Partial<CreateMenuItemRequest> {}

export interface CreateCategoryRequest {
  name: string;
  name_en: string;
  sort_order: number;
}

export interface UpdateCategoryRequest
  extends Partial<CreateCategoryRequest> {}

// ── Endpoints factory ────────────────────────────────────────

export function createMenuEndpoints(client: ApiClient) {
  return {
    // Items
    getItems: () => client.get<MenuItem[]>("/menu/items"),
    createItem: (data: CreateMenuItemRequest) =>
      client.post<MenuItem>("/menu/items", data),
    updateItem: (id: string, data: UpdateMenuItemRequest) =>
      client.put<MenuItem>(`/menu/items/${id}`, data),
    deleteItem: (id: string) =>
      client.delete<void>(`/menu/items/${id}`),

    // Categories
    getCategories: () =>
      client.get<MenuCategory[]>("/menu/categories"),
    createCategory: (data: CreateCategoryRequest) =>
      client.post<MenuCategory>("/menu/categories", data),
    updateCategory: (id: string, data: UpdateCategoryRequest) =>
      client.put<MenuCategory>(`/menu/categories/${id}`, data),
    deleteCategory: (id: string) =>
      client.delete<void>(`/menu/categories/${id}`),

    // Tables
    getTables: () => client.get<Table[]>("/tables"),
    getTable: (id: string) => client.get<Table>(`/tables/${id}`),
    updateTableStatus: (id: string, status: string) =>
      client.put<Table>(`/tables/${id}/status`, { status }),
    reserveTable: (id: string, data: Record<string, unknown>) =>
      client.post<Table>(`/tables/${id}/reserve`, data),

    // Promotions
    getActivePromotions: () =>
      client.get<Promotion[]>("/promotions/active"),
  };
}

export type MenuEndpoints = ReturnType<typeof createMenuEndpoints>;
