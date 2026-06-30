import { ApiClient } from "../client";
import type { Order, Bill, Payment, Shift, VoidRequest, Table, TableZone, Reservation } from "../../types";

// Stub endpoints — return empty arrays. Real implementation in future issues.

export function createPosEndpoints(client: ApiClient) {
  return {
    createOrder: (data: Record<string, unknown>) =>
      client.post<Order>("/orders", data),

    getOrder: (id: string) =>
      client.get<Order>(`/orders/${id}`),

    getActiveOrders: () =>
      client.get<Order[]>("/orders/active"),

    addItem: (orderId: string, data: Record<string, unknown>) =>
      client.post<Order>(`/orders/${orderId}/items`, data),

    removeItem: (orderId: string, itemId: string) =>
      client.delete<void>(`/orders/${orderId}/items/${itemId}`),

    sendToKitchen: (orderId: string) =>
      client.post<void>(`/orders/${orderId}/send-to-kitchen`),

    requestVoid: (orderId: string, data: Record<string, unknown>) =>
      client.post<VoidRequest>(`/orders/${orderId}/void`, data),

    getBills: (orderId: string) =>
      client.get<Bill[]>(`/orders/${orderId}/bills`),

    splitBill: (orderId: string, data: Record<string, unknown>) =>
      client.post<Bill[]>(`/orders/${orderId}/bills/split`, data),

    combineBills: (data: Record<string, unknown>) =>
      client.post<Bill>("/bills/combine", data),

    transferItems: (billId: string, data: Record<string, unknown>) =>
      client.post<Bill>(`/bills/${billId}/transfer-items`, data),

    pay: (billId: string, data: Record<string, unknown>) =>
      client.post<Payment>(`/bills/${billId}/pay`, data),

    openShift: (startingFloat: number) =>
      client.post<Shift>("/shifts/open", { starting_float: startingFloat }),

    closeShift: (shiftId: string, endingCash: number) =>
      client.post<Shift>(`/shifts/${shiftId}/close`, { ending_cash: endingCash }),

    getCurrentShift: () =>
      client.get<Shift | null>("/shifts/current"),

    // Table/Floor Plan endpoints
    getTables: (params?: { zone_id?: string; status?: string }) =>
      client.get<{ zones: Array<TableZone & { tables: Table[] }> }>("/tables", params as Record<string, string>),

    getTable: (id: string) =>
      client.get<Table>(`/tables/${id}`),

    updateTableStatus: (id: string, status: string) =>
      client.put<Table>(`/tables/${id}/status`, { status }),

    reserveTable: (id: string, data: { customer_name: string; start_time: string; end_time: string }) =>
      client.post<Reservation>(`/tables/${id}/reserve`, data),
  };
}

export type PosEndpoints = ReturnType<typeof createPosEndpoints>;
