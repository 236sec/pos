import type { Money } from "./auth";

export interface DailySummary {
  date: string;
  total_revenue: Money;
  order_count: number;
  average_ticket: Money;
  payment_breakdown: PaymentBreakdown;
  channel_breakdown: ChannelBreakdown;
}

export interface PaymentBreakdown {
  cash: Money;
  promptpay: Money;
}

export interface ChannelBreakdown {
  dine_in: ChannelRevenue;
  takeaway: ChannelRevenue;
  delivery: ChannelRevenue;
}

export interface ChannelRevenue {
  revenue: Money;
  order_count: number;
}

export interface ShiftReport {
  shift_id: string;
  user_name: string;
  starting_float: Money;
  expected_cash: Money;
  actual_cash: Money;
  over_short: Money;
  transaction_count: number;
  opened_at: string;
  closed_at: string;
}

export interface MenuPerformanceItem {
  menu_item_id: string;
  name: string;
  quantity_sold: number;
  revenue: Money;
  profit: Money;
}

export interface InventoryValuation {
  ingredient_id: string;
  ingredient_name: string;
  current_stock: number;
  unit: string;
  unit_cost: Money;
  total_value: Money;
}

export interface CogsReport {
  menu_item_id: string;
  menu_item_name: string;
  quantity_sold: number;
  total_revenue: Money;
  total_cost: Money;
  gross_profit: Money;
  margin_percent: number;
}
