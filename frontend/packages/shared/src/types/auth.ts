export interface User {
  id: string;
  username: string;
  display_name: string;
  role: Role;
  branch_id: string | null;
  branches: string[];
}

export interface Role {
  id: string;
  name: string;
  permissions: Permission[];
}

export interface Permission {
  resource: string;
  action: "create" | "read" | "update" | "delete" | "approve";
}

export interface LoginRequest {
  username: string;
  password: string;
  branch_id?: string;
}

export interface LoginResponse {
  token: string;
  user: User;
}

export type PaymentMethod = "Cash" | "PromptPay";

export type OrderChannel = "DineIn" | "Takeaway" | "Delivery";

export type OrderStatus =
  "draft" | "placed" | "in_kitchen" | "served" | "completed" | "voided";

export type KitchenItemStatus =
  "waiting" | "in_progress" | "done" | "out_of_stock";

export type TableStatus = "available" | "occupied" | "dirty" | "reserved";

export interface Money {
  amount: number;
  currency: string;
}

export interface Quantity {
  value: number;
  unit: string;
}
