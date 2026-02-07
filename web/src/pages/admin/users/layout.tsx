import type { ColumnFiltersState } from "@tanstack/react-table";
import {
  FilterIcon,
  HashIcon,
  ListOrderedIcon,
  PlusCircleIcon,
  TypeIcon,
  UserRoundIcon,
} from "lucide-react";
import { parseAsInteger, useQueryState } from "nuqs";
import { useState } from "react";
import { useTranslation } from "react-i18next";
import { useLocation } from "react-router";
import { Button } from "@/components/ui/button";
import { Field, FieldIcon } from "@/components/ui/field";
import { Pagination } from "@/components/ui/pagination";
import { Select } from "@/components/ui/select";
import { TextField } from "@/components/ui/text-field";
import { cn } from "@/utils";
import { Group } from "@/models/user";
import { AdminListLayout, type AdminListContextValue } from "../_list";

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
  const location = useLocation();
  const pathname = location.pathname;
  const isListPage =
    pathname === "/admin/users" || pathname === "/admin/users/";

  const [createDialogOpen, setCreateDialogOpen] = useState(false);
  const [page, setPage] = useQueryState("page", parseAsInteger.withDefault(1));
  const [size, setSize] = useQueryState("size", parseAsInteger.withDefault(10));
  const [total, setTotal] = useState(0);
  const [columnFilters, setColumnFiltersState] = useState<ColumnFiltersState>([
    { id: "group", value: "all" },
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
    page,
    setPage,
    size,
    setSize,
    total,
    setTotal,
  };

  const idValue =
    (columnFilters.find((c) => c.id === "id")?.value as string) ?? "";
  const usernameValue =
    (columnFilters.find((c) => c.id === "username")?.value as string) ?? "";
  const groupValue =
    (columnFilters.find((c) => c.id === "group")?.value as string) ?? "all";

  const groupSelectOptions = [
    { value: "all", content: t("common:all") },
    { value: Group.Banned.toString(), content: t("user:group.banned") },
    { value: Group.User.toString(), content: t("user:group.user") },
    { value: Group.Admin.toString(), content: t("user:group.admin") },
  ];

  const sidebar = (
    <>
      <div
        className={cn(
          "flex items-center gap-2 px-2 text-sm font-medium text-muted-foreground shrink-0"
        )}
      >
        <UserRoundIcon className="size-4" />
        {t("user:_")}
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
            placeholder={t("user:search.username")}
            value={usernameValue}
            onChange={(e) =>
              setColumnFilters((prev) =>
                setFilter(prev, "username", e.target.value || undefined)
              )
            }
          />
        </Field>
        <Field size="sm">
          <FieldIcon>
            <UserRoundIcon className="size-4" />
          </FieldIcon>
          <Select
            options={groupSelectOptions}
            onValueChange={(value) =>
              setColumnFilters((prev) => setFilter(prev, "group", value))
            }
            value={groupValue}
          />
        </Field>
      </div>
      <div className={cn("flex-1 min-h-4")} />
      <div className={cn("flex flex-col gap-2 shrink-0 border-t pt-4")}>
        <div
          className={cn(
            "flex items-center gap-2 text-muted-foreground text-xs font-medium"
          )}
        >
          <ListOrderedIcon className="size-3.5" />
          {t("common:pagination._")}
        </div>
        <Field size="sm">
          <FieldIcon>
            <ListOrderedIcon className="size-4" />
          </FieldIcon>
          <Select
            options={[
              { value: "10" },
              { value: "20" },
              { value: "40" },
              { value: "60" },
            ]}
            value={String(size)}
            onValueChange={(v) => setSize(Number(v))}
          />
        </Field>
        <p className={cn("text-xs text-muted-foreground")}>
          {total} {t("common:pagination.items")}
        </p>
        <Pagination
          size="sm"
          value={page}
          total={Math.ceil(total / size) || 1}
          onChange={setPage}
        />
      </div>
    </>
  );

  return (
    <AdminListLayout
      isListPage={isListPage}
      value={isListPage ? listContextValue : null}
      sidebar={sidebar}
    />
  );
}
