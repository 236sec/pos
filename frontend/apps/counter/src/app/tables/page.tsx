"use client";

import { useEffect, useState, useCallback } from "react";
import { useRouter } from "next/navigation";
import { useTranslations } from "next-intl";
import { ApiClient } from "@pos/shared/api";
import { useAuth } from "@pos/shared/hooks";
import { Button, Badge, Card, CardContent } from "@pos/shared/ui";
import { env } from "@/env";
import type { Table, TableZone } from "@pos/shared/types";

const STATUS_COLORS: Record<Table["status"], "success" | "destructive" | "warning" | "default"> = {
  available: "success",
  occupied: "destructive",
  dirty: "warning",
  reserved: "default",
};

const STATUS_DOT_COLORS: Record<Table["status"], string> = {
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

export default function TablesPage() {
  const t = useTranslations("floorPlan");
  const tc = useTranslations("common");
  const router = useRouter();
  const { user, loading: authLoading } = useAuth(env.NEXT_PUBLIC_API_URL);

  const [tables, setTables] = useState<Table[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  // Auth redirect
  useEffect(() => {
    if (!authLoading && !user) {
      router.push("/login");
    }
  }, [authLoading, user, router]);

  const fetchTables = useCallback(async () => {
    setLoading(true);
    setError(null);
    try {
      const client = getApiClient();
      const res = await client.get<{ zones: ZoneWithTables[] }>("/tables");
      // Flatten all tables from all zones
      const allTables = res.zones.flatMap((z) => z.tables);
      setTables(allTables);
    } catch (e: unknown) {
      const msg = e instanceof Error ? e.message : "Failed to load tables";
      setError(msg);
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    if (!authLoading && user) {
      fetchTables();
    }
  }, [authLoading, user, fetchTables]);

  const handleSeatCustomer = async (table: Table) => {
    try {
      const client = getApiClient();
      await client.put(`/tables/${table.id}/status`, { status: "occupied" });
      // Navigate to order creation or refresh
      router.push(`/orders/new?tableId=${table.id}`);
    } catch (e: unknown) {
      const msg = e instanceof Error ? e.message : "Failed to seat customer";
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
          <CardContent className="p-6">
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
      <div className="flex items-center justify-between">
        <h1 className="text-2xl font-bold">{t("title")}</h1>
        <Badge variant="outline">
          {t("available")}: {tables.filter((t) => t.status === "available").length}
        </Badge>
      </div>

      {loading ? (
        <div className="flex items-center justify-center h-64">
          <p className="text-muted-foreground">{tc("loading")}</p>
        </div>
      ) : tables.length === 0 ? (
        <div className="flex items-center justify-center h-64">
          <p className="text-muted-foreground">{t("noTables")}</p>
        </div>
      ) : (
        <div className="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 gap-4">
          {tables.map((table) => (
            <Card key={table.id} className="overflow-hidden">
              <CardContent className="p-4 space-y-3">
                {/* Table header with status dot */}
                <div className="flex items-center justify-between">
                  <span className="font-semibold text-base">{table.name}</span>
                  <span
                    className="inline-block w-3 h-3 rounded-full flex-shrink-0"
                    style={{ backgroundColor: STATUS_DOT_COLORS[table.status] }}
                    title={t(table.status)}
                  />
                </div>

                {/* Zone and seats info */}
                <div className="text-sm text-muted-foreground space-y-1">
                  <p>{table.zone_name}</p>
                  <p>{t("seats", { count: table.seats })}</p>
                </div>

                {/* Status badge + action */}
                <div className="flex items-center justify-between pt-1">
                  <Badge variant={STATUS_COLORS[table.status]}>
                    {t(table.status)}
                  </Badge>
                  {table.status === "available" && (
                    <Button size="sm" onClick={() => handleSeatCustomer(table)}>
                      Seat
                    </Button>
                  )}
                </div>
              </CardContent>
            </Card>
          ))}
        </div>
      )}
    </main>
  );
}
