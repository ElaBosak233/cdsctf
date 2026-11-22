import {
  CircleUserRoundIcon,
  FilterIcon,
  PlusCircleIcon,
  SearchIcon,
  ShieldIcon,
  UserRoundIcon,
} from "lucide-react";
import { useTranslation } from "react-i18next";
import type { ManagementAccess } from "@/api/management/users";
import { Button } from "@/components/ui/button";
import { Field, FieldIcon } from "@/components/ui/field";
import { ScrollableNav } from "@/components/ui/scrollable-nav";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectSeparator,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { Sidebar } from "@/components/ui/sidebar";
import { TextField } from "@/components/ui/text-field";
import { MANAGEMENT_PERMISSIONS, PERMISSION_OPTIONS } from "@/models/user";
import { cn } from "@/utils";

export const userStatuses = ["active", "banned"] as const;
export const userAccessOptions = [
  "management",
  "no_management",
  ...MANAGEMENT_PERMISSIONS,
] as const;

export type UserFilters = {
  q: string;
  status: (typeof userStatuses)[number] | null;
  access: ManagementAccess | (typeof MANAGEMENT_PERMISSIONS)[number] | null;
};

type UserFilterProps = {
  filters: UserFilters;
  onChange: (filters: Partial<UserFilters>) => void;
};

function FilterFields({
  compact = false,
  filters,
  onChange,
}: UserFilterProps & { compact?: boolean }) {
  const { t } = useTranslation();
  const { q, status, access } = filters;

  return (
    <div
      role="group"
      aria-label={t("common:filter")}
      className={cn(
        "flex",
        compact ? "shrink-0 items-center gap-2" : "flex-col gap-3"
      )}
    >
      <Field size="sm" className={compact ? "w-56 shrink-0" : undefined}>
        <FieldIcon>
          <SearchIcon />
        </FieldIcon>
        <TextField
          aria-label={t("user:search.query")}
          placeholder={t("user:search.query_placeholder")}
          value={q}
          onChange={(event) => onChange({ q: event.target.value })}
        />
      </Field>
      <Field size="sm" className={compact ? "w-40" : undefined}>
        <FieldIcon>
          <CircleUserRoundIcon />
        </FieldIcon>
        <Select
          value={status ?? "all"}
          onValueChange={(value) =>
            onChange({
              status: userStatuses.find((option) => option === value) ?? null,
            })
          }
        >
          <SelectTrigger aria-label={t("user:search.banned")}>
            <SelectValue />
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="all">{t("user:search.all_statuses")}</SelectItem>
            <SelectItem value="active">{t("user:banned.false")}</SelectItem>
            <SelectItem value="banned">{t("user:banned.true")}</SelectItem>
          </SelectContent>
        </Select>
      </Field>
      <Field size="sm" className={compact ? "w-60" : undefined}>
        <FieldIcon>
          <ShieldIcon />
        </FieldIcon>
        <Select
          value={access ?? "all"}
          onValueChange={(value) =>
            onChange({
              access:
                userAccessOptions.find((option) => option === value) ?? null,
            })
          }
        >
          <SelectTrigger aria-label={t("user:permissions._")}>
            <SelectValue />
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="all">
              {t("user:search.all_permissions")}
            </SelectItem>
            <SelectItem value="management">
              {t("user:search.has_management")}
            </SelectItem>
            <SelectItem value="no_management">
              {t("user:search.no_management")}
            </SelectItem>
            <SelectSeparator />
            {PERMISSION_OPTIONS.filter(({ value }) =>
              userAccessOptions.some((option) => option === value)
            ).map(({ value, key }) => (
              <SelectItem key={value} value={value}>
                {t(`user:permissions.options.${key}._`)}
              </SelectItem>
            ))}
          </SelectContent>
        </Select>
      </Field>
    </div>
  );
}

export function UserListFilters({
  onCreate,
  ...filterProps
}: UserFilterProps & { onCreate: () => void }) {
  const { t } = useTranslation();

  return (
    <>
      <ScrollableNav className="xl:hidden">
        <Button
          icon={<PlusCircleIcon />}
          variant="solid"
          size="sm"
          className="shrink-0"
          onClick={onCreate}
        >
          {t("common:actions.add")}
        </Button>
        <div aria-hidden="true" className="mx-1 h-6 w-px shrink-0 bg-border" />
        <FilterFields compact {...filterProps} />
      </ScrollableNav>
      <Sidebar
        collapsible="none"
        className="hidden gap-4 overflow-y-auto px-4 py-5 xl:sticky xl:top-16 xl:flex xl:h-(--app-content-height) xl:w-64 xl:shrink-0 xl:flex-col xl:border-r xl:bg-card/30 xl:backdrop-blur-sm"
      >
        <h1 className="flex shrink-0 items-center gap-2 px-2 text-sm font-medium text-muted-foreground">
          <UserRoundIcon className="size-4" />
          {t("user:_")}
        </h1>
        <Button
          icon={<PlusCircleIcon />}
          variant="solid"
          className="w-full shrink-0 justify-start"
          onClick={onCreate}
        >
          {t("common:actions.add")}
        </Button>
        <div className="flex shrink-0 flex-col gap-3">
          <div className="flex items-center gap-2 text-xs font-medium text-muted-foreground">
            <FilterIcon className="size-3.5" />
            {t("common:filter")}
          </div>
          <FilterFields {...filterProps} />
        </div>
      </Sidebar>
    </>
  );
}
