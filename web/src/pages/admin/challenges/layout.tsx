import type { ColumnFiltersState } from "@tanstack/react-table";
import {
  EyeIcon,
  FilterIcon,
  HashIcon,
  LibraryIcon,
  PlusCircleIcon,
  TypeIcon,
} from "lucide-react";
import { useState } from "react";
import { useTranslation } from "react-i18next";
import { Button } from "@/components/ui/button";
import { Field, FieldIcon } from "@/components/ui/field";
import { Select } from "@/components/ui/select";
import { TextField } from "@/components/ui/text-field";
import { cn } from "@/utils";
import { categories } from "@/utils/category";
import { type AdminListContextValue, AdminListLayout } from "../_list";

function setFilter(
  prev: ColumnFiltersState,
  id: string,
  value: unknown
): ColumnFiltersState {
  const rest = prev.filter((f) => f.id !== id);
  return value === undefined || value === "" ? rest : [...rest, { id, value }];
}

export default function Layout() {
  const { t } = useTranslation();

  const [createDialogOpen, setCreateDialogOpen] = useState(false);
  const [columnFilters, setColumnFiltersState] = useState<ColumnFiltersState>([
    { id: "category", value: "all" },
    { id: "public", value: "all" },
  ]);

  const setColumnFilters = (
    updater:
      | ColumnFiltersState
      | ((prev: ColumnFiltersState) => ColumnFiltersState)
  ) => {
    setColumnFiltersState(
      typeof updater === "function" ? updater(columnFilters) : updater
    );
  };

  const listContextValue: AdminListContextValue = {
    createDialogOpen,
    setCreateDialogOpen,
    columnFilters,
    setColumnFilters,
  };

  const idValue =
    (columnFilters.find((c) => c.id === "id")?.value as string) ?? "";
  const titleValue =
    (columnFilters.find((c) => c.id === "title")?.value as string) ?? "";
  const categoryValue =
    (columnFilters.find((c) => c.id === "category")?.value as string) ?? "all";
  const publicValue =
    (columnFilters.find((c) => c.id === "public")?.value as string) ?? "all";

  const sidebar = (
    <>
      <div
        className={cn(
          "flex items-center gap-2 px-2 text-sm font-medium text-muted-foreground shrink-0"
        )}
      >
        <LibraryIcon className="size-4" />
        {t("challenge:_")}
      </div>
      <Button
        icon={<PlusCircleIcon className="size-4" />}
        variant="solid"
        className={cn("justify-start w-full shrink-0")}
        onClick={() => setCreateDialogOpen(true)}
      >
        {t("common:actions.add")}
      </Button>
      <div className={cn("flex flex-col gap-3 shrink-0")}>
        <div
          className={cn(
            "flex items-center gap-2 text-muted-foreground text-xs font-medium"
          )}
        >
          <FilterIcon className="size-3.5" />
          {t("common:filter")}
        </div>
        <Field size="sm">
          <FieldIcon>
            <HashIcon className="size-4" />
          </FieldIcon>
          <TextField
            placeholder="ID"
            value={idValue}
            onChange={(e) =>
              setColumnFilters((prev) =>
                setFilter(prev, "id", e.target.value || undefined)
              )
            }
          />
        </Field>
        <Field size="sm">
          <FieldIcon>
            <TypeIcon className="size-4" />
          </FieldIcon>
          <TextField
            placeholder={t("challenge:title")}
            value={titleValue}
            onChange={(e) =>
              setColumnFilters((prev) =>
                setFilter(prev, "title", e.target.value || undefined)
              )
            }
          />
        </Field>
        <Field size="sm">
          <FieldIcon>
            <LibraryIcon className="size-4" />
          </FieldIcon>
          <Select
            options={[
              {
                value: "all",
                content: (
                  <div className={cn("flex gap-2 items-center")}>
                    {t("common:all")}
                  </div>
                ),
              },
              ...(categories || []).map((cat) => {
                const Icon = cat.icon!;
                return {
                  value: String(cat?.id),
                  content: (
                    <div className={cn("flex gap-2 items-center")}>
                      <Icon className="size-4" />
                      {cat?.name?.toUpperCase()}
                    </div>
                  ),
                };
              }),
            ]}
            onValueChange={(value) =>
              setColumnFilters((prev) => setFilter(prev, "category", value))
            }
            value={categoryValue}
          />
        </Field>
        <Field size="sm">
          <FieldIcon>
            <EyeIcon className="size-4" />
          </FieldIcon>
          <Select
            options={[
              { value: "all", content: t("common:all") },
              { value: "true", content: t("challenge:search.public.true") },
              { value: "false", content: t("challenge:search.public.false") },
            ]}
            onValueChange={(value) =>
              setColumnFilters((prev) => setFilter(prev, "public", value))
            }
            value={publicValue}
          />
        </Field>
      </div>
    </>
  );

  return <AdminListLayout value={listContextValue} sidebar={sidebar} />;
}
