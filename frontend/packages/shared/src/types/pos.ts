import type { Money, OrderChannel, OrderStatus } from "./auth";

export interface OrderItem {
  id: string;
  menu_item_id: string;
  name: string;
  name_en: string;
  quantity: number;
  unit_price: Money;
  modifiers: OrderModifier[];
  special_instructions: string;
  status: "waiting" | "in_progress" | "done" | "out_of_stock";
}

export interface OrderModifier {
  modifier_group_id: string;
  modifier_option_id: string;
  name: string;
  price_override: Money;
}

export interface Order {
  id: string;
  branch_id: string;
  channel: OrderChannel;
  table_id: string | null;
  table_name: string | null;
  status: OrderStatus;
  items: OrderItem[];
  bills: Bill[];
  created_by: string;
  created_at: string;
  updated_at: string;
}

export interface Bill {
  id: string;
  order_id: string;
  item_ids: string[];
  subtotal: Money;
  discount: Money;
  total: Money;
  status: "open" | "paid" | "voided";
}

export interface Payment {
  id: string;
  bill_id: string;
  method: "Cash" | "PromptPay";
  amount: Money;
  reference: string | null;
  paid_at: string;
}

export interface Shift {
  id: string;
  user_id: string;
  branch_id: string;
  starting_float: Money;
  ending_cash: Money | null;
  expected_cash: Money | null;
  over_short: Money | null;
  opened_at: string;
  closed_at: string | null;
  status: "open" | "closed";
}

export interface VoidRequest {
  id: string;
  order_id: string;
  item_id: string | null;
  reason_code: string;
  reason_text: string;
  requested_by: string;
  approved_by: string | null;
  status: "pending" | "approved" | "rejected";
  created_at: string;
  resolved_at: string | null;
}
