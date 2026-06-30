import { ApiClient } from "../client";
import type {
  DailySummary,
  ShiftReport,
  MenuPerformanceItem,
  InventoryValuation,
  CogsReport,
} from "../../types";

export function createReportEndpoints(client: ApiClient) {
  return {
    getDailySummary: (date?: string) =>
      client.get<DailySummary>(
        "/reports/daily-summary",
        date ? { date } : undefined,
      ),

    getShiftReport: (shiftId: string) =>
      client.get<ShiftReport>(`/reports/shift?shift_id=${shiftId}`),

    getMenuPerformance: (from?: string, to?: string) =>
      client.get<MenuPerformanceItem[]>("/reports/menu-performance", {
        ...(from ? { from } : {}),
        ...(to ? { to } : {}),
      }),

    getInventoryValuation: () =>
      client.get<InventoryValuation[]>("/reports/inventory-valuation"),

    getCogs: (from?: string, to?: string) =>
      client.get<CogsReport[]>("/reports/cogs", {
        ...(from ? { from } : {}),
        ...(to ? { to } : {}),
      }),
  };
}

export type ReportEndpoints = ReturnType<typeof createReportEndpoints>;
