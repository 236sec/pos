import type { Money } from "./auth";

export interface MenuItem {
  id: string;
  name: string;
  name_en: string;
  category_id: string;
  category_name: string;
  base_price: Money;
  channel_prices: ChannelPrice[];
  modifier_groups: ModifierGroup[];
  image_url: string | null;
  is_available: boolean;
}

export interface ChannelPrice {
  channel: "DineIn" | "Takeaway" | "Delivery";
  price: Money;
}

export interface MenuCategory {
  id: string;
  name: string;
  name_en: string;
  sort_order: number;
}

export interface ModifierGroup {
  id: string;
  name: string;
  name_en: string;
  selection_type: "single" | "multiple";
  is_required: boolean;
  options: ModifierOption[];
}

export interface ModifierOption {
  id: string;
  name: string;
  name_en: string;
  price: Money;
}

export interface Table {
  id: string;
  name: string;
  zone_id: string;
  zone_name: string;
  x: number;
  y: number;
  seats: number;
  status: "available" | "occupied" | "dirty" | "reserved";
  current_order_id: string | null;
}

export interface TableZone {
  id: string;
  name: string;
  floor: number;
}

export interface Promotion {
  id: string;
  name: string;
  name_en: string;
  trigger_type: string;
  condition: Record<string, unknown>;
  reward_type: string;
  reward: Record<string, unknown>;
  start_date: string;
  end_date: string;
  is_active: boolean;
}
