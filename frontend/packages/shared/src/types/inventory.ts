import type { Money, Quantity } from "./auth";

export interface Ingredient {
  id: string;
  name: string;
  name_en: string;
  unit: string;
  current_stock: Quantity;
  reorder_threshold: Quantity;
  unit_cost: Money;
}

export interface StockMovement {
  id: string;
  ingredient_id: string;
  quantity_change: Quantity;
  reason: "order_deduction" | "void_restock" | "adjustment" | "receiving";
  reference_id: string | null;
  created_at: string;
}

export interface Recipe {
  id: string;
  menu_item_id: string;
  ingredients: RecipeIngredient[];
}

export interface RecipeIngredient {
  ingredient_id: string;
  ingredient_name: string;
  quantity: Quantity;
}

export interface StockAdjustment {
  ingredient_id: string;
  new_quantity: Quantity;
  reason: string;
}
