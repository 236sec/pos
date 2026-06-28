import type { Money, Quantity } from "./auth";

export interface Supplier {
  id: string;
  name: string;
  contact_name: string;
  phone: string;
  email: string;
  products: SupplierProduct[];
}

export interface SupplierProduct {
  id: string;
  ingredient_id: string;
  ingredient_name: string;
  unit_price: Money;
}

export interface PurchaseOrder {
  id: string;
  supplier_id: string;
  supplier_name: string;
  branch_id: string;
  items: PurchaseOrderItem[];
  status: "draft" | "ordered" | "partially_received" | "received" | "cancelled";
  created_at: string;
  updated_at: string;
}

export interface PurchaseOrderItem {
  id: string;
  ingredient_id: string;
  ingredient_name: string;
  quantity_ordered: Quantity;
  quantity_received: Quantity;
  unit_price: Money;
}
