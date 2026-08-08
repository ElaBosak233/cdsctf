import {
  EyeIcon,
  FilterIcon,
  HashIcon,
  LibraryIcon,
  PlusCircleIcon,
  TypeIcon,
} from "lucide-react";
import { parseAsInteger, useQueryState } from "nuqs";
import { useState } from "react";
import { useTranslation } from "react-i18next";
import { Outlet, useLocation } from "react-router";
import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";
import { Field, FieldIcon } from "@/components/ui/field";
import { ScrollableNav } from "@/components/ui/scrollable-nav";
import { Select } from "@/components/ui/select";
import { TextField } from "@/components/ui/text-field";
import { cn } from "@/utils";
import { categories } from "@/utils/category";
import type { ColumnFiltersState } from "@/utils/data-table";
import { ChallengeListContext } from "./context";

function setFilter(
  prev: ColumnFiltersState,
  id: string,
  value: unknown
): ColumnFiltersState {
  const rest = prev.filter((f) => f.id !== id);
  return value === undefined || value === "" ? rest : [...rest, { id, value }];
}

type FilterFieldsProps = {
  columnFilters: ColumnFiltersState;
  compact?: boolean;
  setColumnFilters: (
    updater:
      | ColumnFiltersState
      | ((prev: ColumnFiltersState) => ColumnFiltersState)
  ) => void;
};

function FilterFields({
  columnFilters,
  compact = false,
  setColumnFilters,
}: FilterFieldsProps) {
  const { t } = useTranslation();
  const idValue =
    (columnFilters.find((c) => c.id === "id")?.value as string) ?? "";
  const titleValue =
    (columnFilters.find((c) => c.id === "title")?.value as string) ?? "";
  const categoryValue =
    (columnFilters.find((c) => c.id === "category")?.value as string) ?? "all";
  const publicValue =
    (columnFilters.find((c) => c.id === "public")?.value as string) ?? "all";

  return (
    <>
      <Field size="sm" className={cn(compact && ["w-28", "shrink-0"])}>
        <FieldIcon>
          <HashIcon className="size-4" />
        </FieldIcon>
        <TextField
          placeholder={t("challenge:form.id._")}
          value={idValue}
          onChange={(event) =>
            setColumnFilters((current) =>
              setFilter(current, "id", event.target.value || undefined)
            )
          }
        />
      </Field>
      <Field size="sm" className={cn(compact && ["w-56", "shrink-0"])}>
        <FieldIcon>
          <TypeIcon className="size-4" />
        </FieldIcon>
        <TextField
          placeholder={t("challenge:title")}
          value={titleValue}
          onChange={(event) =>
            setColumnFilters((current) =>
              setFilter(current, "title", event.target.value || undefined)
            )
          }
        />
      </Field>
      <Field size="sm" className={cn(compact && ["w-44", "shrink-0"])}>
        <FieldIcon>
          <LibraryIcon className="size-4" />
        </FieldIcon>
        <Select
          options={[
            { value: "all", content: t("common:all") },
            ...categories.map((category) => {
              const Icon = category.icon!;
              return {
                value: String(category.id),
                content: (
                  <div className={cn(["flex", "items-center", "gap-2"])}>
                    <Icon className="size-4" />
                    {category.name?.toUpperCase()}
                  </div>
                ),
              };
            }),
          ]}
          onValueChange={(value) =>
            setColumnFilters((current) => setFilter(current, "category", value))
          }
          value={categoryValue}
        />
      </Field>
      <Field size="sm" className={cn(compact && ["w-36", "shrink-0"])}>
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
            setColumnFilters((current) => setFilter(current, "public", value))
          }
          value={publicValue}
        />
      </Field>
    </>
  );
}

export default function Layout() {
  const { t } = useTranslation();
  const { pathname } = useLocation();
  const isListPage =
    pathname === "/admin/challenges" || pathname === "/admin/challenges/";

  const [createDialogOpen, setCreateDialogOpen] = useState(false);
  const [columnFilters, setColumnFiltersState] = useState<ColumnFiltersState>([
    { id: "category", value: "all" },
    { id: "public", value: "all" },
  ]);
  const [, setPage] = useQueryState("page", parseAsInteger.withDefault(1));

  const setColumnFilters = (
    updater:
      | ColumnFiltersState
      | ((prev: ColumnFiltersState) => ColumnFiltersState)
  ) => {
    setColumnFiltersState((current) =>
      typeof updater === "function" ? updater(current) : updater
    );
    void setPage(1);
  };

  if (!isListPage) {
    return <Outlet />;
  }

  return (
    <ChallengeListContext.Provider
      value={{
        createDialogOpen,
        setCreateDialogOpen,
        columnFilters,
        setColumnFilters,
      }}
    >
      <div
        className={cn([
          "flex",
          "flex-col",
          "xl:flex-row",
          "xl:min-h-(--app-content-height)",
          "flex-1",
          "min-h-0",
          "min-w-0",
          "xl:pl-64",
        ])}
      >
        <ScrollableNav className={cn(["xl:hidden"])}>
          <Button
            icon={<PlusCircleIcon className="size-4" />}
            variant="solid"
            size="sm"
            className={cn(["shrink-0"])}
            onClick={() => setCreateDialogOpen(true)}
          >
            {t("common:actions.add")}
          </Button>
          <div
            className={cn(["mx-1", "h-6", "w-px", "shrink-0", "bg-border"])}
          />
          <FilterFields
            compact
            columnFilters={columnFilters}
            setColumnFilters={setColumnFilters}
          />
        </ScrollableNav>
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
            <LibraryIcon className="size-4" />
            {t("challenge:_")}
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
            <FilterFields
              columnFilters={columnFilters}
              setColumnFilters={setColumnFilters}
            />
          </div>
        </aside>
        <Card
          className={cn([
            "flex-1",
            "min-h-0",
            "min-w-0",
            "border-y-0",
            "rounded-none",
            "flex",
            "flex-col",
            "xl:h-(--app-content-height)",
            "xl:rounded-l-none",
          ])}
        >
          <Outlet />
        </Card>
      </div>
    </ChallengeListContext.Provider>
  );
}
