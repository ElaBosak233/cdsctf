import {
  EyeIcon,
  FilterIcon,
  LibraryIcon,
  PlusCircleIcon,
  SearchIcon,
} from "lucide-react";
import {
  parseAsBoolean,
  parseAsInteger,
  parseAsNumberLiteral,
  parseAsString,
  useQueryStates,
} from "nuqs";
import { useState } from "react";
import { useTranslation } from "react-i18next";
import { Outlet, useLocation } from "react-router";
import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";
import { Field, FieldIcon } from "@/components/ui/field";
import { ScrollableNav } from "@/components/ui/scrollable-nav";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { Sidebar } from "@/components/ui/sidebar";
import { TextField } from "@/components/ui/text-field";
import { cn } from "@/utils";
import { categories } from "@/utils/category";
import { type ChallengeFilters, ChallengeListContext } from "./context";

const listSearchParams = {
  q: parseAsString.withDefault(""),
  category: parseAsNumberLiteral(categories.map(({ id }) => id)),
  public: parseAsBoolean,
  page: parseAsInteger.withDefault(1),
};

function FilterFields({
  filters,
  compact = false,
  onChange,
}: {
  filters: ChallengeFilters;
  compact?: boolean;
  onChange: (next: Partial<ChallengeFilters>) => void;
}) {
  const { t } = useTranslation();
  const searchLabel = t("common:search_title_or_id");

  return (
    <>
      <Field size="sm" className={compact ? "w-56 shrink-0" : undefined}>
        <FieldIcon>
          <SearchIcon />
        </FieldIcon>
        <TextField
          aria-label={searchLabel}
          placeholder={searchLabel}
          value={filters.q}
          onChange={(event) => onChange({ q: event.target.value })}
        />
      </Field>
      <Field size="sm" className={cn(compact && ["w-44", "shrink-0"])}>
        <FieldIcon>
          <LibraryIcon className="size-4" />
        </FieldIcon>
        <Select
          value={filters.category?.toString() ?? "all"}
          onValueChange={(value) =>
            onChange({ category: value === "all" ? null : Number(value) })
          }
        >
          <SelectTrigger aria-label={t("challenge:category")}>
            <SelectValue />
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="all">
              {t("challenge:search.all_categories")}
            </SelectItem>
            {categories.map((category) => {
              const Icon = category.icon!;
              return (
                <SelectItem key={category.id} value={String(category.id)}>
                  <Icon className="size-4" />
                  {category.name?.toUpperCase()}
                </SelectItem>
              );
            })}
          </SelectContent>
        </Select>
      </Field>
      <Field size="sm" className={cn(compact && ["w-36", "shrink-0"])}>
        <FieldIcon>
          <EyeIcon className="size-4" />
        </FieldIcon>
        <Select
          value={filters.public?.toString() ?? "all"}
          onValueChange={(value) =>
            onChange({ public: value === "all" ? null : value === "true" })
          }
        >
          <SelectTrigger aria-label={t("challenge:search.public._")}>
            <SelectValue />
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="all">
              {t("challenge:search.all_visibility")}
            </SelectItem>
            <SelectItem value="true">
              {t("challenge:search.public.true")}
            </SelectItem>
            <SelectItem value="false">
              {t("challenge:search.public.false")}
            </SelectItem>
          </SelectContent>
        </Select>
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
  const [filters, setFilters] = useQueryStates(listSearchParams);

  function updateFilters(next: Partial<ChallengeFilters>) {
    void setFilters({ ...next, page: 1 });
  }

  if (!isListPage) {
    return <Outlet />;
  }

  return (
    <ChallengeListContext.Provider
      value={{
        createDialogOpen,
        setCreateDialogOpen,
        filters,
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
          "xl:items-stretch",
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
          <FilterFields compact filters={filters} onChange={updateFilters} />
        </ScrollableNav>
        <Sidebar
          collapsible="none"
          className={cn([
            "hidden",
            "xl:flex",
            "xl:sticky",
            "xl:top-16",
            "xl:left-auto",
            "xl:z-auto",
            "xl:h-(--app-content-height)",
            "xl:w-64",
            "xl:shrink-0",
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
            <FilterFields filters={filters} onChange={updateFilters} />
          </div>
        </Sidebar>
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
