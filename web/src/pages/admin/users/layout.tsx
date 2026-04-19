import type { ColumnFiltersState } from "@tanstack/react-table";
import {
  FilterIcon,
  HashIcon,
  PlusCircleIcon,
  TypeIcon,
  UserRoundIcon,
} from "lucide-react";
import { useState } from "react";
import { useTranslation } from "react-i18next";
import { Outlet, useLocation } from "react-router";
import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";
import { Field, FieldIcon } from "@/components/ui/field";
import { Select } from "@/components/ui/select";
import { TextField } from "@/components/ui/text-field";
import { Group } from "@/models/user";
import { cn } from "@/utils";
import { UserListContext } from "./context";

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
  const { pathname } = useLocation();
  const isListPage =
    pathname === "/admin/users" || pathname === "/admin/users/";

  const [createDialogOpen, setCreateDialogOpen] = useState(false);
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

  if (!isListPage) {
    return <Outlet />;
  }

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

  return (
    <UserListContext.Provider
      value={{ createDialogOpen, setCreateDialogOpen, columnFilters, setColumnFilters }}
    >
      <div
        className={cn([
          "flex",
          "flex-col",
          "xl:flex-row",
          "xl:min-h-(--app-content-height)",
          "flex-1",
          "min-h-0",
          "xl:pl-64",
        ])}
      >
        <aside
          className={cn([
            "hidden",
            "xl:flex",
            "xl:fixed",
            "xl:left-16",
            "xl:top-16",
            "xl:z-10",
            "xl:h-(--app-content-height)",
            "xl:w-64",
            "xl:flex-col",
            "xl:border-r",
            "xl:bg-card/30",
            "xl:backdrop-blur-sm",
            "py-5",
            "px-4",
            "gap-4",
            "overflow-y-auto",
          ])}
        >
          <div
            className={cn([
              "flex",
              "items-center",
              "gap-2",
              "px-2",
              "text-sm",
              "font-medium",
              "text-muted-foreground",
              "shrink-0",
            ])}
          >
            <UserRoundIcon className="size-4" />
            {t("user:_")}
          </div>
          <Button
            icon={<PlusCircleIcon className="size-4" />}
            variant="solid"
            className={cn(["justify-start", "w-full", "shrink-0"])}
            onClick={() => setCreateDialogOpen(true)}
          >
            {t("common:actions.add")}
          </Button>
          <div className={cn(["flex", "flex-col", "gap-3", "shrink-0"])}>
            <div
              className={cn([
                "flex",
                "items-center",
                "gap-2",
                "text-muted-foreground",
                "text-xs",
                "font-medium",
              ])}
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
        </aside>
        <Card
          className={cn([
            "h-(--app-content-height)",
            "flex-1",
            "min-h-0",
            "min-w-0",
            "border-y-0",
            "rounded-none",
            "flex",
            "flex-col",
            "xl:rounded-l-none",
          ])}
        >
          <Outlet />
        </Card>
      </div>
    </UserListContext.Provider>
  );
}
