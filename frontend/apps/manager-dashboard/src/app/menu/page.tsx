"use client";

import { useRouter } from "next/navigation";
import { useState, useEffect, useCallback } from "react";
import { ApiClient, ApiError } from "@pos/shared/api";
import {
  createMenuEndpoints,
  type CreateMenuItemRequest,
  type UpdateMenuItemRequest,
  type CreateCategoryRequest,
} from "@pos/shared/api/endpoints";
import type {
  MenuItem,
  MenuCategory,
  ChannelPrice,
  Money,
} from "@pos/shared/types";
import { Button, Card, Input, Badge } from "@pos/shared/ui";
import { useAuth } from "@pos/shared/hooks";
import { env } from "@/env";

// ── Money helpers ──────────────────────────────────────────────

function bahtToMoney(baht: number): Money {
  return { amount: Math.round(baht * 100), currency: "THB" };
}

function moneyToBaht(m: Money): number {
  return m.amount / 100;
}

function formatBaht(amount: number): string {
  return new Intl.NumberFormat("th-TH", {
    minimumFractionDigits: 2,
    maximumFractionDigits: 2,
  }).format(amount);
}

// ── Form state types ───────────────────────────────────────────

interface ModifierOptionForm {
  id?: string;
  name: string;
  name_en: string;
  price: number;
  sort_order: number;
}

interface ModifierGroupForm {
  id?: string;
  name: string;
  name_en: string;
  selection_type: "single" | "multiple";
  is_required: boolean;
  sort_order: number;
  options: ModifierOptionForm[];
}

interface ChannelPricesForm {
  DineIn: number | null;
  Takeaway: number | null;
  Delivery: number | null;
}

interface ItemFormState {
  name: string;
  name_en: string;
  category_id: string;
  base_price: number;
  image_url: string;
  is_available: boolean;
  modifier_groups: ModifierGroupForm[];
  channel_prices: ChannelPricesForm;
}

// ── Helpers ────────────────────────────────────────────────────

function emptyFormState(firstCategoryId: string): ItemFormState {
  return {
    name: "",
    name_en: "",
    category_id: firstCategoryId,
    base_price: 0,
    image_url: "",
    is_available: true,
    modifier_groups: [],
    channel_prices: { DineIn: null, Takeaway: null, Delivery: null },
  };
}

function itemToFormState(item: MenuItem): ItemFormState {
  const channelPrices: ChannelPricesForm = {
    DineIn: null,
    Takeaway: null,
    Delivery: null,
  };
  for (const cp of item.channel_prices) {
    channelPrices[cp.channel] = moneyToBaht(cp.price);
  }
  return {
    name: item.name,
    name_en: item.name_en,
    category_id: item.category_id,
    base_price: moneyToBaht(item.base_price),
    image_url: item.image_url ?? "",
    is_available: item.is_available,
    modifier_groups: item.modifier_groups.map((mg, gi) => ({
      id: mg.id,
      name: mg.name,
      name_en: mg.name_en,
      selection_type: mg.selection_type,
      is_required: mg.is_required,
      sort_order: gi,
      options: mg.options.map((opt, oi) => ({
        id: opt.id,
        name: opt.name,
        name_en: opt.name_en,
        price: moneyToBaht(opt.price),
        sort_order: oi,
      })),
    })),
    channel_prices: channelPrices,
  };
}

function formStateToCreateRequest(f: ItemFormState): CreateMenuItemRequest {
  const basePriceMoney = bahtToMoney(f.base_price);
  const channelPrices: ChannelPrice[] = [
    {
      channel: "DineIn",
      price:
        f.channel_prices.DineIn != null
          ? bahtToMoney(f.channel_prices.DineIn)
          : basePriceMoney,
    },
    {
      channel: "Takeaway",
      price:
        f.channel_prices.Takeaway != null
          ? bahtToMoney(f.channel_prices.Takeaway)
          : basePriceMoney,
    },
    {
      channel: "Delivery",
      price:
        f.channel_prices.Delivery != null
          ? bahtToMoney(f.channel_prices.Delivery)
          : basePriceMoney,
    },
  ];
  return {
    name: f.name,
    name_en: f.name_en,
    category_id: f.category_id,
    base_price: basePriceMoney,
    image_url: f.image_url || null,
    modifier_groups: f.modifier_groups.map((mg, gi) => ({
      name: mg.name,
      name_en: mg.name_en,
      selection_type: mg.selection_type,
      is_required: mg.is_required,
      sort_order: gi,
      options: mg.options.map((opt, oi) => ({
        name: opt.name,
        name_en: opt.name_en,
        price: bahtToMoney(opt.price),
        sort_order: oi,
      })),
    })),
    channel_prices: channelPrices,
  };
}

// ── Page component ─────────────────────────────────────────────

export default function MenuPage() {
  const router = useRouter();
  const { user, loading: authLoading } = useAuth(env.NEXT_PUBLIC_API_URL);
  const getToken = useCallback(
    () => localStorage.getItem("pos_auth_token"),
    [],
  );

  // ── API ──────────────────────────────────────────────────────
  const client = new ApiClient(env.NEXT_PUBLIC_API_URL, getToken);
  const api = createMenuEndpoints(client);

  // ── Data state ───────────────────────────────────────────────
  const [items, setItems] = useState<MenuItem[]>([]);
  const [categories, setCategories] = useState<MenuCategory[]>([]);
  const [selectedCategoryId, setSelectedCategoryId] = useState<string | null>(
    null,
  );
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  // ── Drawer state ─────────────────────────────────────────────
  const [drawerOpen, setDrawerOpen] = useState(false);
  const [editingItem, setEditingItem] = useState<MenuItem | null>(null);
  const [formState, setFormState] = useState<ItemFormState>(emptyFormState(""));
  const [saving, setSaving] = useState(false);
  const [saveError, setSaveError] = useState<string | null>(null);

  // ── Category dialog state ────────────────────────────────────
  const [categoryDialogOpen, setCategoryDialogOpen] = useState(false);
  const [newCategory, setNewCategory] = useState<CreateCategoryRequest>({
    name: "",
    name_en: "",
    sort_order: 0,
  });
  const [creatingCategory, setCreatingCategory] = useState(false);

  // ── Delete confirm state ─────────────────────────────────────
  const [deleteTarget, setDeleteTarget] = useState<MenuItem | null>(null);
  const [deleting, setDeleting] = useState(false);

  // ── Redirect if not authenticated ────────────────────────────
  useEffect(() => {
    if (!authLoading && !user) {
      router.push("/login");
    }
  }, [user, authLoading, router]);

  // ── Fetch data ───────────────────────────────────────────────
  const fetchData = useCallback(async () => {
    setLoading(true);
    setError(null);
    try {
      const [itemsData, categoriesData] = await Promise.all([
        api.getItems(),
        api.getCategories(),
      ]);
      setItems(itemsData);
      setCategories(categoriesData);
    } catch (e: unknown) {
      if (e instanceof ApiError) {
        setError(e.body?.message || "Failed to load menu data");
      } else if (e instanceof Error) {
        setError(e.message);
      } else {
        setError("Failed to load menu data");
      }
    } finally {
      setLoading(false);
    }
  }, [api]);

  useEffect(() => {
    if (user) fetchData();
  }, [user, fetchData]);

  // ── Derived data ─────────────────────────────────────────────
  const filteredItems =
    selectedCategoryId === null
      ? items
      : items.filter((item) => item.category_id === selectedCategoryId);

  const itemCountByCategory = useCallback(
    (catId: string) => items.filter((i) => i.category_id === catId).length,
    [items],
  );

  // ── Drawer handlers ──────────────────────────────────────────

  const openCreateDrawer = useCallback(() => {
    setEditingItem(null);
    setFormState(
      emptyFormState(categories.length > 0 ? categories[0]!.id : ""),
    );
    setSaveError(null);
    setDrawerOpen(true);
  }, [categories]);

  const openEditDrawer = useCallback((item: MenuItem) => {
    setEditingItem(item);
    setFormState(itemToFormState(item));
    setSaveError(null);
    setDrawerOpen(true);
  }, []);

  const closeDrawer = useCallback(() => {
    setDrawerOpen(false);
    setEditingItem(null);
    setSaveError(null);
  }, []);

  // ── Save handler ─────────────────────────────────────────────

  const handleSave = useCallback(async () => {
    setSaving(true);
    setSaveError(null);
    try {
      if (editingItem) {
        const updateData: UpdateMenuItemRequest =
          formStateToCreateRequest(formState);
        await api.updateItem(editingItem.id, updateData);
      } else {
        const createData = formStateToCreateRequest(formState);
        await api.createItem(createData);
      }
      closeDrawer();
      await fetchData();
    } catch (e: unknown) {
      if (e instanceof ApiError) {
        setSaveError(e.body?.message || "Failed to save item");
      } else if (e instanceof Error) {
        setSaveError(e.message);
      } else {
        setSaveError("Failed to save item");
      }
    } finally {
      setSaving(false);
    }
  }, [editingItem, formState, api, closeDrawer, fetchData]);

  // ── Delete handler ───────────────────────────────────────────

  const handleDeleteConfirm = useCallback(async () => {
    if (!deleteTarget) return;
    setDeleting(true);
    try {
      await api.deleteItem(deleteTarget.id);
      setDeleteTarget(null);
      await fetchData();
    } catch (e: unknown) {
      if (e instanceof ApiError) {
        setError(e.body?.message || "Failed to delete item");
      } else if (e instanceof Error) {
        setError(e.message);
      } else {
        setError("Failed to delete item");
      }
    } finally {
      setDeleting(false);
    }
  }, [deleteTarget, api, fetchData]);

  // ── Category create handler ──────────────────────────────────

  const handleCreateCategory = useCallback(async () => {
    if (!newCategory.name.trim()) return;
    setCreatingCategory(true);
    try {
      await api.createCategory(newCategory);
      setCategoryDialogOpen(false);
      setNewCategory({ name: "", name_en: "", sort_order: 0 });
      await fetchData();
    } catch (e: unknown) {
      if (e instanceof ApiError) {
        setError(e.body?.message || "Failed to create category");
      } else if (e instanceof Error) {
        setError(e.message);
      } else {
        setError("Failed to create category");
      }
    } finally {
      setCreatingCategory(false);
    }
  }, [newCategory, api, fetchData]);

  // ── Form state updaters ──────────────────────────────────────

  const updateFormField = useCallback(
    <K extends keyof ItemFormState>(key: K, value: ItemFormState[K]) => {
      setFormState((prev) => ({ ...prev, [key]: value }));
    },
    [],
  );

  const updateChannelPrice = useCallback(
    (channel: keyof ChannelPricesForm, value: string) => {
      const num = value === "" ? null : parseFloat(value);
      setFormState((prev) => ({
        ...prev,
        channel_prices: { ...prev.channel_prices, [channel]: num },
      }));
    },
    [],
  );

  const addModifierGroup = useCallback(() => {
    setFormState((prev) => ({
      ...prev,
      modifier_groups: [
        ...prev.modifier_groups,
        {
          name: "",
          name_en: "",
          selection_type: "single",
          is_required: false,
          sort_order: prev.modifier_groups.length,
          options: [],
        },
      ],
    }));
  }, []);

  const removeModifierGroup = useCallback((gi: number) => {
    setFormState((prev) => ({
      ...prev,
      modifier_groups: prev.modifier_groups.filter((_, i) => i !== gi),
    }));
  }, []);

  const updateModifierGroup = useCallback(
    (gi: number, field: keyof ModifierGroupForm, value: unknown) => {
      setFormState((prev) => {
        const groups = [...prev.modifier_groups];
        const group = groups[gi];
        if (!group) return prev;
        groups[gi] = { ...group, [field]: value } as ModifierGroupForm;
        return { ...prev, modifier_groups: groups };
      });
    },
    [],
  );

  const addModifierOption = useCallback((gi: number) => {
    setFormState((prev) => {
      const group = prev.modifier_groups[gi];
      if (!group) return prev;
      const options = [...group.options];
      options.push({
        name: "",
        name_en: "",
        price: 0,
        sort_order: options.length,
      });
      const groups = [...prev.modifier_groups];
      groups[gi] = { ...group, options };
      return { ...prev, modifier_groups: groups };
    });
  }, []);

  const removeModifierOption = useCallback((gi: number, oi: number) => {
    setFormState((prev) => {
      const group = prev.modifier_groups[gi];
      if (!group) return prev;
      const options = group.options.filter((_, i) => i !== oi);
      const groups = [...prev.modifier_groups];
      groups[gi] = { ...group, options };
      return { ...prev, modifier_groups: groups };
    });
  }, []);

  const updateModifierOption = useCallback(
    (
      gi: number,
      oi: number,
      field: keyof ModifierOptionForm,
      value: unknown,
    ) => {
      setFormState((prev) => {
        const group = prev.modifier_groups[gi];
        if (!group) return prev;
        const options = [...group.options];
        const opt = options[oi];
        if (!opt) return prev;
        options[oi] = { ...opt, [field]: value } as ModifierOptionForm;
        const groups = [...prev.modifier_groups];
        groups[gi] = { ...group, options };
        return { ...prev, modifier_groups: groups };
      });
    },
    [],
  );

  // ── Loading / auth guard ─────────────────────────────────────

  if (authLoading) {
    return (
      <main className="flex min-h-screen items-center justify-center p-8">
        <p className="text-muted-foreground">Loading...</p>
      </main>
    );
  }

  if (!user) {
    return null;
  }

  // ── Render ───────────────────────────────────────────────────

  return (
    <main className="min-h-screen p-6">
      {/* ── Header ─────────────────────────────────────────── */}
      <div className="mb-6 flex items-center justify-between">
        <h1 className="text-2xl font-bold">Menu Management</h1>
        <Button onClick={openCreateDrawer}>Add Item</Button>
      </div>

      {error && (
        <div className="mb-4 rounded-md bg-destructive/10 p-3 text-sm text-destructive">
          {error}
        </div>
      )}

      {/* ── Two-column layout ──────────────────────────────── */}
      <div className="flex gap-6">
        {/* ── Category sidebar ─────────────────────────────── */}
        <aside className="w-64 shrink-0">
          <Card>
            <div className="p-4">
              <h2 className="mb-3 text-sm font-semibold text-muted-foreground uppercase tracking-wider">
                Categories
              </h2>
              <nav className="space-y-1">
                <button
                  onClick={() => setSelectedCategoryId(null)}
                  className={`w-full rounded-md px-3 py-2 text-left text-sm transition-colors ${
                    selectedCategoryId === null
                      ? "bg-primary text-primary-foreground"
                      : "hover:bg-muted"
                  }`}
                >
                  <span>All</span>
                  <span className="ml-2 text-xs opacity-60">
                    ({items.length})
                  </span>
                </button>
                {categories.map((cat) => (
                  <button
                    key={cat.id}
                    onClick={() => setSelectedCategoryId(cat.id)}
                    className={`w-full rounded-md px-3 py-2 text-left text-sm transition-colors ${
                      selectedCategoryId === cat.id
                        ? "bg-primary text-primary-foreground"
                        : "hover:bg-muted"
                    }`}
                  >
                    <span>{cat.name}</span>
                    <span className="ml-2 text-xs opacity-60">
                      ({itemCountByCategory(cat.id)})
                    </span>
                  </button>
                ))}
              </nav>
              <div className="mt-4 pt-3 border-t">
                <Button
                  variant="outline"
                  size="sm"
                  className="w-full"
                  onClick={() => setCategoryDialogOpen(true)}
                >
                  + Add Category
                </Button>
              </div>
            </div>
          </Card>
        </aside>

        {/* ── Item list ────────────────────────────────────── */}
        <div className="flex-1 min-w-0">
          {loading ? (
            <div className="flex items-center justify-center py-16">
              <p className="text-muted-foreground">Loading items...</p>
            </div>
          ) : filteredItems.length === 0 ? (
            <div className="flex items-center justify-center py-16">
              <p className="text-muted-foreground">
                {selectedCategoryId
                  ? "No items in this category"
                  : "No menu items yet. Click 'Add Item' to create one."}
              </p>
            </div>
          ) : (
            <div className="overflow-x-auto">
              <table className="w-full text-sm">
                <thead>
                  <tr className="border-b text-left text-muted-foreground">
                    <th className="pb-3 pr-4 font-medium">Name (TH)</th>
                    <th className="pb-3 pr-4 font-medium">Name (EN)</th>
                    <th className="pb-3 pr-4 font-medium">Category</th>
                    <th className="pb-3 pr-4 font-medium">Base Price</th>
                    <th className="pb-3 pr-4 font-medium">Channel Prices</th>
                    <th className="pb-3 pr-4 font-medium">Status</th>
                    <th className="pb-3 font-medium">Actions</th>
                  </tr>
                </thead>
                <tbody>
                  {filteredItems.map((item) => (
                    <tr
                      key={item.id}
                      className="border-b last:border-0 hover:bg-muted/50 transition-colors"
                    >
                      <td className="py-3 pr-4">{item.name}</td>
                      <td className="py-3 pr-4 text-muted-foreground">
                        {item.name_en}
                      </td>
                      <td className="py-3 pr-4">{item.category_name}</td>
                      <td className="py-3 pr-4">
                        {formatBaht(moneyToBaht(item.base_price))} ฿
                      </td>
                      <td className="py-3 pr-4">
                        <div className="flex flex-wrap gap-1">
                          {item.channel_prices.map((cp) => (
                            <Badge key={cp.channel} variant="outline">
                              {cp.channel === "DineIn"
                                ? "Dine-in"
                                : cp.channel === "Takeaway"
                                  ? "Takeaway"
                                  : "Delivery"}{" "}
                              {formatBaht(moneyToBaht(cp.price))} ฿
                            </Badge>
                          ))}
                        </div>
                      </td>
                      <td className="py-3 pr-4">
                        <Badge
                          variant={
                            item.is_available ? "success" : "destructive"
                          }
                        >
                          {item.is_available ? "Active" : "Inactive"}
                        </Badge>
                      </td>
                      <td className="py-3">
                        <div className="flex gap-2">
                          <Button
                            variant="ghost"
                            size="sm"
                            onClick={() => openEditDrawer(item)}
                          >
                            Edit
                          </Button>
                          <Button
                            variant="ghost"
                            size="sm"
                            className="text-destructive hover:text-destructive"
                            onClick={() => setDeleteTarget(item)}
                          >
                            Delete
                          </Button>
                        </div>
                      </td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          )}
        </div>
      </div>

      {/* ── Create / Edit Drawer ───────────────────────────── */}
      {drawerOpen && (
        <div className="fixed inset-0 z-50 flex">
          {/* Backdrop */}
          <div className="fixed inset-0 bg-black/50" onClick={closeDrawer} />
          {/* Panel */}
          <div className="relative ml-auto h-full w-full max-w-2xl overflow-y-auto bg-background shadow-xl">
            {/* Drawer header */}
            <div className="sticky top-0 z-10 flex items-center justify-between border-b bg-background px-6 py-4">
              <h2 className="text-lg font-semibold">
                {editingItem ? "Edit Menu Item" : "Add Menu Item"}
              </h2>
              <Button variant="ghost" size="sm" onClick={closeDrawer}>
                ✕
              </Button>
            </div>

            {/* Drawer body */}
            <div className="space-y-6 p-6">
              {/* ── Basic info ──────────────────────────────── */}
              <Card>
                <div className="p-4 space-y-4">
                  <h3 className="text-sm font-semibold text-muted-foreground uppercase tracking-wider">
                    Basic Information
                  </h3>

                  <div className="grid grid-cols-2 gap-4">
                    <div className="space-y-1.5">
                      <label className="text-sm font-medium">Name (Thai)</label>
                      <Input
                        value={formState.name}
                        onChange={(e) =>
                          updateFormField("name", e.target.value)
                        }
                        placeholder="ชื่อเมนู"
                      />
                    </div>
                    <div className="space-y-1.5">
                      <label className="text-sm font-medium">Name (EN)</label>
                      <Input
                        value={formState.name_en}
                        onChange={(e) =>
                          updateFormField("name_en", e.target.value)
                        }
                        placeholder="Menu name"
                      />
                    </div>
                  </div>

                  <div className="grid grid-cols-2 gap-4">
                    <div className="space-y-1.5">
                      <label className="text-sm font-medium">Category</label>
                      <select
                        className="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2"
                        value={formState.category_id}
                        onChange={(e) =>
                          updateFormField("category_id", e.target.value)
                        }
                      >
                        {categories.map((cat) => (
                          <option key={cat.id} value={cat.id}>
                            {cat.name}
                          </option>
                        ))}
                      </select>
                    </div>
                    <div className="space-y-1.5">
                      <label className="text-sm font-medium">
                        Base Price (฿)
                      </label>
                      <Input
                        type="number"
                        step="0.01"
                        min="0"
                        value={formState.base_price || ""}
                        onChange={(e) =>
                          updateFormField(
                            "base_price",
                            e.target.value === ""
                              ? 0
                              : parseFloat(e.target.value),
                          )
                        }
                        placeholder="0.00"
                      />
                    </div>
                  </div>

                  <div className="grid grid-cols-2 gap-4">
                    <div className="space-y-1.5">
                      <label className="text-sm font-medium">
                        Image URL
                        <span className="ml-1 text-xs text-muted-foreground">
                          (optional)
                        </span>
                      </label>
                      <Input
                        value={formState.image_url}
                        onChange={(e) =>
                          updateFormField("image_url", e.target.value)
                        }
                        placeholder="https://..."
                      />
                    </div>
                    <div className="flex items-end pb-2">
                      <label className="flex items-center gap-2 cursor-pointer">
                        <input
                          type="checkbox"
                          checked={formState.is_available}
                          onChange={(e) =>
                            updateFormField("is_available", e.target.checked)
                          }
                          className="h-4 w-4 rounded border-gray-300 text-primary focus:ring-primary"
                        />
                        <span className="text-sm font-medium">Active</span>
                      </label>
                    </div>
                  </div>
                </div>
              </Card>

              {/* ── Modifier Groups ─────────────────────────── */}
              <Card>
                <div className="p-4 space-y-4">
                  <div className="flex items-center justify-between">
                    <h3 className="text-sm font-semibold text-muted-foreground uppercase tracking-wider">
                      Modifier Groups
                    </h3>
                    <Button
                      variant="outline"
                      size="sm"
                      onClick={addModifierGroup}
                    >
                      + Add Group
                    </Button>
                  </div>

                  {formState.modifier_groups.length === 0 && (
                    <p className="text-sm text-muted-foreground">
                      No modifier groups yet. Click &ldquo;Add Group&rdquo; to
                      create one.
                    </p>
                  )}

                  {formState.modifier_groups.map((group, gi) => (
                    <div key={gi} className="rounded-lg border p-4 space-y-3">
                      <div className="flex items-center justify-between">
                        <span className="text-sm font-medium">
                          Group #{gi + 1}
                        </span>
                        <Button
                          variant="ghost"
                          size="sm"
                          className="text-destructive h-auto px-2 py-1 text-xs"
                          onClick={() => removeModifierGroup(gi)}
                        >
                          Remove
                        </Button>
                      </div>

                      <div className="grid grid-cols-2 gap-3">
                        <div className="space-y-1">
                          <label className="text-xs font-medium">
                            Name (Thai)
                          </label>
                          <Input
                            value={group.name}
                            onChange={(e) =>
                              updateModifierGroup(gi, "name", e.target.value)
                            }
                            placeholder="ชื่อกลุ่ม"
                          />
                        </div>
                        <div className="space-y-1">
                          <label className="text-xs font-medium">
                            Name (EN)
                          </label>
                          <Input
                            value={group.name_en}
                            onChange={(e) =>
                              updateModifierGroup(gi, "name_en", e.target.value)
                            }
                            placeholder="Group name"
                          />
                        </div>
                      </div>

                      <div className="flex items-center gap-6">
                        <div className="flex items-center gap-3">
                          <label className="text-xs font-medium">
                            Selection:
                          </label>
                          <label className="flex items-center gap-1 text-xs">
                            <input
                              type="radio"
                              name={`sel-type-${gi}`}
                              checked={group.selection_type === "single"}
                              onChange={() =>
                                updateModifierGroup(
                                  gi,
                                  "selection_type",
                                  "single",
                                )
                              }
                              className="h-3.5 w-3.5"
                            />
                            Single
                          </label>
                          <label className="flex items-center gap-1 text-xs">
                            <input
                              type="radio"
                              name={`sel-type-${gi}`}
                              checked={group.selection_type === "multiple"}
                              onChange={() =>
                                updateModifierGroup(
                                  gi,
                                  "selection_type",
                                  "multiple",
                                )
                              }
                              className="h-3.5 w-3.5"
                            />
                            Multiple
                          </label>
                        </div>
                        <label className="flex items-center gap-1.5 text-xs">
                          <input
                            type="checkbox"
                            checked={group.is_required}
                            onChange={(e) =>
                              updateModifierGroup(
                                gi,
                                "is_required",
                                e.target.checked,
                              )
                            }
                            className="h-3.5 w-3.5 rounded"
                          />
                          Required
                        </label>
                      </div>

                      {/* Options list */}
                      <div className="space-y-2 pl-2 border-l-2 border-muted">
                        <div className="flex items-center justify-between">
                          <span className="text-xs font-medium text-muted-foreground">
                            Options
                          </span>
                          <Button
                            variant="ghost"
                            size="sm"
                            className="h-auto px-2 py-1 text-xs"
                            onClick={() => addModifierOption(gi)}
                          >
                            + Add Option
                          </Button>
                        </div>

                        {group.options.length === 0 && (
                          <p className="text-xs text-muted-foreground">
                            No options yet.
                          </p>
                        )}

                        {group.options.map((opt, oi) => (
                          <div
                            key={oi}
                            className="flex items-start gap-2 rounded border p-2"
                          >
                            <div className="flex-1 grid grid-cols-3 gap-2">
                              <Input
                                value={opt.name}
                                onChange={(e) =>
                                  updateModifierOption(
                                    gi,
                                    oi,
                                    "name",
                                    e.target.value,
                                  )
                                }
                                placeholder="ชื่อ"
                                className="h-8 text-xs"
                              />
                              <Input
                                value={opt.name_en}
                                onChange={(e) =>
                                  updateModifierOption(
                                    gi,
                                    oi,
                                    "name_en",
                                    e.target.value,
                                  )
                                }
                                placeholder="Name"
                                className="h-8 text-xs"
                              />
                              <div className="relative">
                                <Input
                                  type="number"
                                  step="0.01"
                                  min="0"
                                  value={opt.price || ""}
                                  onChange={(e) =>
                                    updateModifierOption(
                                      gi,
                                      oi,
                                      "price",
                                      e.target.value === ""
                                        ? 0
                                        : parseFloat(e.target.value),
                                    )
                                  }
                                  placeholder="Price"
                                  className="h-8 text-xs pr-5"
                                />
                                <span className="absolute right-2 top-1/2 -translate-y-1/2 text-xs text-muted-foreground">
                                  ฿
                                </span>
                              </div>
                            </div>
                            <Button
                              variant="ghost"
                              size="sm"
                              className="h-8 w-8 shrink-0 p-0 text-destructive"
                              onClick={() => removeModifierOption(gi, oi)}
                            >
                              ✕
                            </Button>
                          </div>
                        ))}
                      </div>
                    </div>
                  ))}
                </div>
              </Card>

              {/* ── Channel Pricing ─────────────────────────── */}
              <Card>
                <div className="p-4 space-y-4">
                  <h3 className="text-sm font-semibold text-muted-foreground uppercase tracking-wider">
                    Channel Pricing
                  </h3>
                  <p className="text-xs text-muted-foreground">
                    Leave empty to use the base price for that channel.
                  </p>

                  <div className="grid grid-cols-3 gap-4">
                    <div className="space-y-1.5">
                      <label className="text-sm font-medium">Dine-in (฿)</label>
                      <Input
                        type="number"
                        step="0.01"
                        min="0"
                        value={
                          formState.channel_prices.DineIn !== null
                            ? formState.channel_prices.DineIn
                            : ""
                        }
                        onChange={(e) =>
                          updateChannelPrice("DineIn", e.target.value)
                        }
                        placeholder={
                          formState.base_price
                            ? formatBaht(formState.base_price)
                            : "0.00"
                        }
                      />
                    </div>
                    <div className="space-y-1.5">
                      <label className="text-sm font-medium">
                        Takeaway (฿)
                      </label>
                      <Input
                        type="number"
                        step="0.01"
                        min="0"
                        value={
                          formState.channel_prices.Takeaway !== null
                            ? formState.channel_prices.Takeaway
                            : ""
                        }
                        onChange={(e) =>
                          updateChannelPrice("Takeaway", e.target.value)
                        }
                        placeholder={
                          formState.base_price
                            ? formatBaht(formState.base_price)
                            : "0.00"
                        }
                      />
                    </div>
                    <div className="space-y-1.5">
                      <label className="text-sm font-medium">
                        Delivery (฿)
                      </label>
                      <Input
                        type="number"
                        step="0.01"
                        min="0"
                        value={
                          formState.channel_prices.Delivery !== null
                            ? formState.channel_prices.Delivery
                            : ""
                        }
                        onChange={(e) =>
                          updateChannelPrice("Delivery", e.target.value)
                        }
                        placeholder={
                          formState.base_price
                            ? formatBaht(formState.base_price)
                            : "0.00"
                        }
                      />
                    </div>
                  </div>
                </div>
              </Card>

              {/* Save error */}
              {saveError && (
                <div className="rounded-md bg-destructive/10 p-3 text-sm text-destructive">
                  {saveError}
                </div>
              )}
            </div>

            {/* Drawer footer */}
            <div className="sticky bottom-0 flex items-center justify-end gap-3 border-t bg-background px-6 py-4">
              <Button variant="outline" onClick={closeDrawer}>
                Cancel
              </Button>
              <Button onClick={handleSave} disabled={saving}>
                {saving
                  ? "Saving..."
                  : editingItem
                    ? "Save Changes"
                    : "Create Item"}
              </Button>
            </div>
          </div>
        </div>
      )}

      {/* ── Delete Confirmation Dialog ─────────────────────── */}
      {deleteTarget && (
        <div className="fixed inset-0 z-50 flex items-center justify-center">
          <div
            className="fixed inset-0 bg-black/50"
            onClick={() => !deleting && setDeleteTarget(null)}
          />
          <div className="relative w-full max-w-sm rounded-lg bg-background p-6 shadow-xl">
            <h3 className="text-lg font-semibold">Delete Item</h3>
            <p className="mt-2 text-sm text-muted-foreground">
              Are you sure you want to delete &ldquo;{deleteTarget.name}
              &rdquo;? This action cannot be undone.
            </p>
            <div className="mt-6 flex justify-end gap-3">
              <Button
                variant="outline"
                onClick={() => setDeleteTarget(null)}
                disabled={deleting}
              >
                Cancel
              </Button>
              <Button
                variant="destructive"
                onClick={handleDeleteConfirm}
                disabled={deleting}
              >
                {deleting ? "Deleting..." : "Delete"}
              </Button>
            </div>
          </div>
        </div>
      )}

      {/* ── Add Category Dialog ────────────────────────────── */}
      {categoryDialogOpen && (
        <div className="fixed inset-0 z-50 flex items-center justify-center">
          <div
            className="fixed inset-0 bg-black/50"
            onClick={() => !creatingCategory && setCategoryDialogOpen(false)}
          />
          <div className="relative w-full max-w-sm rounded-lg bg-background p-6 shadow-xl">
            <h3 className="text-lg font-semibold">Add Category</h3>
            <div className="mt-4 space-y-3">
              <div className="space-y-1.5">
                <label className="text-sm font-medium">Name (Thai)</label>
                <Input
                  value={newCategory.name}
                  onChange={(e) =>
                    setNewCategory((prev) => ({
                      ...prev,
                      name: e.target.value,
                    }))
                  }
                  placeholder="ชื่อหมวดหมู่"
                />
              </div>
              <div className="space-y-1.5">
                <label className="text-sm font-medium">Name (EN)</label>
                <Input
                  value={newCategory.name_en}
                  onChange={(e) =>
                    setNewCategory((prev) => ({
                      ...prev,
                      name_en: e.target.value,
                    }))
                  }
                  placeholder="Category name"
                />
              </div>
              <div className="space-y-1.5">
                <label className="text-sm font-medium">Sort Order</label>
                <Input
                  type="number"
                  min="0"
                  value={newCategory.sort_order}
                  onChange={(e) =>
                    setNewCategory((prev) => ({
                      ...prev,
                      sort_order: parseInt(e.target.value) || 0,
                    }))
                  }
                />
              </div>
            </div>
            <div className="mt-6 flex justify-end gap-3">
              <Button
                variant="outline"
                onClick={() => setCategoryDialogOpen(false)}
                disabled={creatingCategory}
              >
                Cancel
              </Button>
              <Button
                onClick={handleCreateCategory}
                disabled={creatingCategory || !newCategory.name.trim()}
              >
                {creatingCategory ? "Creating..." : "Create"}
              </Button>
            </div>
          </div>
        </div>
      )}
    </main>
  );
}
