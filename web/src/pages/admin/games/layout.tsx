import type { ColumnFiltersState } from "@tanstack/react-table";
import {
  EyeIcon,
  FilterIcon,
  FlagIcon,
  HashIcon,
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
    { id: "enabled", value: "all" },
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
  const enabledValue =
    (columnFilters.find((c) => c.id === "enabled")?.value as string) ?? "all";

  const sidebar = (
    <>
      <div
        className={cn(
          "flex items-center gap-2 px-2 text-sm font-medium text-muted-foreground shrink-0"
        )}
      >
        <FlagIcon className="size-4" />
        {t("game:_")}
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
            placeholder={t("game:title")}
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
            <EyeIcon className="size-4" />
          </FieldIcon>
          <Select
            options={[
              { value: "all", content: t("common:all") },
              { value: "true", content: t("game:enabled.true") },
              { value: "false", content: t("game:enabled.false") },
            ]}
            onValueChange={(value) =>
              setColumnFilters((prev) => setFilter(prev, "enabled", value))
            }
            value={enabledValue}
          />
        </Field>
      </div>
    </>
  );

  return <AdminListLayout value={listContextValue} sidebar={sidebar} />;
}
