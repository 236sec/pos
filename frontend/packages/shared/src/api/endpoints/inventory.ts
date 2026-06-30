import { ApiClient } from "../client";
import type {
  Ingredient,
  StockMovement,
  Recipe,
  StockAdjustment,
} from "../../types";

export function createInventoryEndpoints(client: ApiClient) {
  return {
    getStock: () => client.get<Ingredient[]>("/inventory"),

    getMovements: (ingredientId: string) =>
      client.get<StockMovement[]>(`/inventory/${ingredientId}/movements`),

    adjustStock: (data: StockAdjustment) =>
      client.post<void>("/inventory/adjust", data),

    getRecipe: (menuItemId: string) =>
      client.get<Recipe>(`/recipes/${menuItemId}`),
  };
}

export type InventoryEndpoints = ReturnType<typeof createInventoryEndpoints>;
