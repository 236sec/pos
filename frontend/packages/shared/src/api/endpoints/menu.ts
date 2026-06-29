import { ApiClient } from "../client";
import type { MenuItem, MenuCategory, Table, Promotion } from "../../types";

export function createMenuEndpoints(client: ApiClient) {
  return {
    getItems: () =>
      client.get<MenuItem[]>("/menu/items"),

    getCategories: () =>
      client.get<MenuCategory[]>("/menu/categories"),

    getTables: () =>
      client.get<Table[]>("/tables"),

    getTable: (id: string) =>
      client.get<Table>(`/tables/${id}`),

    updateTableStatus: (id: string, status: string) =>
      client.put<Table>(`/tables/${id}/status`, { status }),

    reserveTable: (id: string, data: Record<string, unknown>) =>
      client.post<Table>(`/tables/${id}/reserve`, data),

    getActivePromotions: () =>
      client.get<Promotion[]>("/promotions/active"),
  };
}

export type MenuEndpoints = ReturnType<typeof createMenuEndpoints>;
