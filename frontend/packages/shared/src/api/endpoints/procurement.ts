import { ApiClient } from "../client";
import type { Supplier, PurchaseOrder } from "../../types";

export function createProcurementEndpoints(client: ApiClient) {
  return {
    getSuppliers: () =>
      client.get<Supplier[]>("/suppliers"),

    createPurchaseOrder: (data: Record<string, unknown>) =>
      client.post<PurchaseOrder>("/purchase-orders", data),

    getPurchaseOrders: () =>
      client.get<PurchaseOrder[]>("/purchase-orders"),

    receiveStock: (poId: string, data: Record<string, unknown>) =>
      client.post<PurchaseOrder>(`/purchase-orders/${poId}/receive`, data),
  };
}

export type ProcurementEndpoints = ReturnType<typeof createProcurementEndpoints>;
