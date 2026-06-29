"use client";

import { useEffect, useState, useCallback } from "react";
import { useRouter } from "next/navigation";
import { useTranslations } from "next-intl";
import { ApiClient } from "@pos/shared/api";
import { useAuth } from "@pos/shared/hooks";
import { Button, Badge, Card, CardHeader, CardTitle, CardContent } from "@pos/shared/ui";
import { env } from "@/env";
import type { Table, TableZone } from "@pos/shared/types";

type StatusColor = "success" | "destructive" | "warning" | "default";

const STATUS_COLORS: Record<Table["status"], StatusColor> = {
  available: "success",
  occupied: "destructive",
  dirty: "warning",
  reserved: "default",
};

const STATUS_FILLS: Record<Table["status"], string> = {
  available: "#22c55e",
  occupied: "#ef4444",
  dirty: "#f59e0b",
  reserved: "#3b82f6",
};

interface ZoneWithTables extends TableZone {
  tables: Table[];
}

function getApiClient() {
  const getToken = () =>
    typeof window !== "undefined" ? localStorage.getItem("pos_auth_token") : null;
  return new ApiClient(env.NEXT_PUBLIC_API_URL, getToken);
}

export default function FloorPlanPage() {
  const t = useTranslations("floorPlan");
  const tc = useTranslations("common");
  const router = useRouter();
  const { user, loading: authLoading } = useAuth(env.NEXT_PUBLIC_API_URL);

  const [zones, setZones] = useState<ZoneWithTables[]>([]);
  const [selectedZone, setSelectedZone] = useState<string | null>(null);
  const [selectedTable, setSelectedTable] = useState<Table | null>(null);
  const [loadingTables, setLoadingTables] = useState(true);
  const [error, setError] = useState<string | null>(null);

  // Auth redirect
  useEffect(() => {
    if (!authLoading && !user) {
      router.push("/login");
    }
  }, [authLoading, user, router]);

  const fetchTables = useCallback(async () => {
    setLoadingTables(true);
    setError(null);
    try {
      const client = getApiClient();
      const res = await client.get<{ zones: ZoneWithTables[] }>("/tables");
      setZones(res.zones);
      if (res.zones.length > 0 && !selectedZone) {
        const first = res.zones[0];
        if (first) setSelectedZone(first.id);
      }
    } catch (e: unknown) {
      const msg = e instanceof Error ? e.message : "Failed to load tables";
      setError(msg);
    } finally {
      setLoadingTables(false);
    }
  }, [selectedZone]);

  useEffect(() => {
    if (!authLoading && user) {
      fetchTables();
    }
  }, [authLoading, user, fetchTables]);

  const currentZone = zones.find((z) => z.id === selectedZone);
  const currentTables = currentZone?.tables ?? [];

  const handleTableClick = (table: Table) => {
    setSelectedTable(table);
  };

  const handleCloseDetail = () => {
    setSelectedTable(null);
  };

  const handleStatusAction = async (
    tableId: string,
    newStatus: string,
    reservationData?: { customer_name: string; start_time: string; end_time: string }
  ) => {
    try {
      const client = getApiClient();
      if (newStatus === "reserved" && reservationData) {
        await client.post(`/tables/${tableId}/reserve`, reservationData);
      } else {
        await client.put(`/tables/${tableId}/status`, { status: newStatus });
      }
      // Refresh tables
      await fetchTables();
      setSelectedTable(null);
    } catch (e: unknown) {
      const msg = e instanceof Error ? e.message : "Action failed";
      setError(msg);
    }
  };

  if (authLoading) {
    return (
      <main className="flex min-h-screen items-center justify-center p-8">
        <p className="text-muted-foreground">{tc("loading")}</p>
      </main>
    );
  }

  if (!user) {
    return null;
  }

  if (error) {
    return (
      <main className="flex min-h-screen items-center justify-center p-8">
        <Card className="w-full max-w-md">
          <CardHeader>
            <CardTitle>{tc("error")}</CardTitle>
          </CardHeader>
          <CardContent>
            <p className="text-muted-foreground">{error}</p>
            <Button className="mt-4" onClick={fetchTables}>
              {tc("retry") ?? "Retry"}
            </Button>
          </CardContent>
        </Card>
      </main>
    );
  }

  return (
    <main className="min-h-screen p-4 space-y-4">
      {/* Header */}
      <div className="flex items-center justify-between">
        <h1 className="text-2xl font-bold">{t("title")}</h1>
      </div>

      {/* Zone tabs */}
      <div className="flex gap-2 overflow-x-auto pb-2">
        {zones.map((zone) => (
          <Button
            key={zone.id}
            variant={selectedZone === zone.id ? "default" : "outline"}
            size="sm"
            onClick={() => setSelectedZone(zone.id)}
          >
            {zone.name}
          </Button>
        ))}
      </div>

      {/* Legend */}
      <div className="flex gap-4 text-sm text-muted-foreground">
        <span className="flex items-center gap-1">
          <span className="inline-block w-3 h-3 rounded-full" style={{ backgroundColor: STATUS_FILLS.available }} />
          {t("available")}
        </span>
        <span className="flex items-center gap-1">
          <span className="inline-block w-3 h-3 rounded-full" style={{ backgroundColor: STATUS_FILLS.occupied }} />
          {t("occupied")}
        </span>
        <span className="flex items-center gap-1">
          <span className="inline-block w-3 h-3 rounded-full" style={{ backgroundColor: STATUS_FILLS.dirty }} />
          {t("dirty")}
        </span>
        <span className="flex items-center gap-1">
          <span className="inline-block w-3 h-3 rounded-full" style={{ backgroundColor: STATUS_FILLS.reserved }} />
          {t("reserved")}
        </span>
      </div>

      {/* Floor plan area */}
      {loadingTables ? (
        <div className="flex items-center justify-center h-96">
          <p className="text-muted-foreground">{tc("loading")}</p>
        </div>
      ) : currentTables.length === 0 ? (
        <div className="flex items-center justify-center h-96">
          <p className="text-muted-foreground">{t("noTables")}</p>
        </div>
      ) : (
        <div className="relative border rounded-lg bg-card overflow-hidden">
          <svg
            viewBox="0 0 1000 800"
            className="w-full h-auto"
            style={{ minHeight: "400px" }}
          >
            {currentTables.map((table) => {
              const w = Math.max(table.seats * 30, 60);
              const h = 50;
              return (
                <g
                  key={table.id}
                  onClick={() => handleTableClick(table)}
                  className="cursor-pointer"
                >
                  <rect
                    x={table.x}
                    y={table.y}
                    width={w}
                    height={h}
                    rx={6}
                    fill={STATUS_FILLS[table.status]}
                    opacity={0.85}
                    stroke="white"
                    strokeWidth={1.5}
                  />
                  <text
                    x={table.x + w / 2}
                    y={table.y + h / 2}
                    textAnchor="middle"
                    dominantBaseline="central"
                    fill="white"
                    fontSize={13}
                    fontWeight={600}
                    className="pointer-events-none select-none"
                  >
                    {table.name}
                  </text>
                </g>
              );
            })}
          </svg>
        </div>
      )}

      {/* Table detail panel (overlay) */}
      {selectedTable && (
        <div className="fixed inset-0 z-50 bg-black/50 flex items-end sm:items-center justify-center p-4">
          <Card className="w-full max-w-md">
            <CardHeader>
              <div className="flex items-center justify-between">
                <CardTitle>{selectedTable.name}</CardTitle>
                <Badge variant={STATUS_COLORS[selectedTable.status]}>
                  {t(selectedTable.status)}
                </Badge>
              </div>
            </CardHeader>
            <CardContent className="space-y-4">
              <div className="text-sm space-y-1">
                <p>
                  <span className="text-muted-foreground">{t("zones")}: </span>
                  {selectedTable.zone_name}
                </p>
                <p>
                  <span className="text-muted-foreground">{t("seats", { count: selectedTable.seats })}</span>
                </p>
              </div>

              {/* Current order link */}
              {selectedTable.current_order_id && (
                <div className="flex items-center justify-between p-3 bg-muted rounded-md">
                  <span className="text-sm font-medium">{t("currentOrder")}</span>
                  <Button
                    variant="outline"
                    size="sm"
                    onClick={() => router.push(`/orders/${selectedTable.current_order_id}`)}
                  >
                    {t("viewOrder")}
                  </Button>
                </div>
              )}

              {/* Action buttons */}
              <div className="flex flex-wrap gap-2">
                {(selectedTable.status === "available" || selectedTable.status === "reserved") && (
                  <Button
                    variant="default"
                    onClick={() => handleStatusAction(selectedTable.id, "occupied")}
                  >
                    {t("startOrder")}
                  </Button>
                )}
                {selectedTable.status === "occupied" && (
                  <Button
                    variant="destructive"
                    onClick={() => handleStatusAction(selectedTable.id, "dirty")}
                  >
                    {t("markDirty")}
                  </Button>
                )}
                {selectedTable.status === "dirty" && (
                  <Button
                    variant="default"
                    onClick={() => handleStatusAction(selectedTable.id, "available")}
                  >
                    {t("markClean")}
                  </Button>
                )}
                <Button variant="outline" onClick={handleCloseDetail}>
                  {tc("close")}
                </Button>
              </div>
            </CardContent>
          </Card>
        </div>
      )}
    </main>
  );
}
