import {
  ArrowDownIcon,
  ArrowUpDownIcon,
  ArrowUpIcon,
  EditIcon,
  EllipsisIcon,
  Globe2Icon,
  TrashIcon,
  UserRoundPlusIcon,
} from "lucide-react";
import {
  createContext,
  type ReactNode,
  useCallback,
  useContext,
  useMemo,
  useOptimistic,
  useState,
  useTransition,
} from "react";
import { Trans, useTranslation } from "react-i18next";
import { Link } from "react-router";
import { toast } from "sonner";
import { deleteAdminIdp, updateAdminIdp } from "@/api/admin/idps";
import { Avatar } from "@/components/ui/avatar";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";
import { Dialog, DialogContent } from "@/components/ui/dialog";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { Switch } from "@/components/ui/switch";
import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from "@/components/ui/tooltip";
import type { IdpView } from "@/models/idp";
import { useSharedStore } from "@/storages/shared";
import { cn } from "@/utils";
import type { Column, ColumnDef, Row } from "@/hooks/use-data-table";

const RowContext = createContext<{
  optimisticEnabled: boolean;
  optimisticRegistrationEnabled: boolean;
  pending: boolean;
  toggleEnabled: () => void;
  toggleRegistrationEnabled: () => void;
} | null>(null);

function useRowContext() {
  return useContext(RowContext);
}

function RowProvider({ idp, children }: { idp: IdpView; children: ReactNode }) {
  const { t } = useTranslation();
  const [status, setStatus] = useState({
    enabled: idp.enabled,
    registrationEnabled: idp.registration_enabled,
  });
  const [pending, startTransition] = useTransition();
  const [optimisticStatus, setOptimisticStatus] = useOptimistic(status);

  const updateStatus = useCallback(
    (next: typeof status, action: "enabled" | "registration") => {
      startTransition(async () => {
        setOptimisticStatus(next);
        try {
          await updateAdminIdp(idp.id, {
            name: idp.name,
            enabled: next.enabled,
            registration_enabled: next.registrationEnabled,
            portal: idp.portal,
            script: idp.script,
          });
          setStatus(next);
          toast.success(
            t(
              action === "enabled"
                ? next.enabled
                  ? "admin:idp.actions.enable.success"
                  : "admin:idp.actions.disable.success"
                : next.registrationEnabled
                  ? "admin:idp.actions.registration_enable.success"
                  : "admin:idp.actions.registration_disable.success",
              { name: idp.name }
            )
          );
        } catch {
          setOptimisticStatus(status);
          toast.error(t("common:errors.default"));
        }
      });
    },
    [idp.id, idp.name, idp.portal, idp.script, setOptimisticStatus, status, t]
  );

  const toggleEnabled = useCallback(() => {
    updateStatus(
      { ...optimisticStatus, enabled: !optimisticStatus.enabled },
      "enabled"
    );
  }, [optimisticStatus, updateStatus]);

  const toggleRegistrationEnabled = useCallback(() => {
    updateStatus(
      {
        ...optimisticStatus,
        registrationEnabled: !optimisticStatus.registrationEnabled,
      },
      "registration"
    );
  }, [optimisticStatus, updateStatus]);

  return (
    <RowContext.Provider
      value={{
        optimisticEnabled: optimisticStatus.enabled,
        optimisticRegistrationEnabled: optimisticStatus.registrationEnabled,
        pending,
        toggleEnabled,
        toggleRegistrationEnabled,
      }}
    >
      {children}
    </RowContext.Provider>
  );
}

function StatusToggle({
  checked,
  disabled,
  icon,
  label,
  onCheckedChange,
}: {
  checked: boolean;
  disabled: boolean;
  icon: ReactNode;
  label: string;
  onCheckedChange: () => void;
}) {
  return (
    <div className={cn(["flex", "items-center", "justify-between", "gap-3"])}>
      <Badge
        variant="outline"
        className={cn(
          checked
            ? ["border-success/20", "bg-success/10", "text-success"]
            : ["text-muted-foreground"]
        )}
      >
        {icon}
        {label}
      </Badge>
      <Switch
        checked={checked}
        disabled={disabled}
        onCheckedChange={onCheckedChange}
        aria-label={label}
      />
    </div>
  );
}

function StatusCell() {
  const { t } = useTranslation();
  const {
    optimisticEnabled,
    optimisticRegistrationEnabled,
    pending,
    toggleEnabled,
    toggleRegistrationEnabled,
  } = useRowContext()!;

  return (
    <div className={cn(["grid", "min-w-44", "gap-2"])}>
      <StatusToggle
        checked={optimisticEnabled}
        disabled={pending}
        icon={<Globe2Icon />}
        label={t("admin:idp.status.service")}
        onCheckedChange={toggleEnabled}
      />
      <StatusToggle
        checked={optimisticRegistrationEnabled}
        disabled={pending}
        icon={<UserRoundPlusIcon />}
        label={t("admin:idp.status.registration")}
        onCheckedChange={toggleRegistrationEnabled}
      />
    </div>
  );
}

function IdpCell({ row }: { row: Row<IdpView> }) {
  const idp = row.original;

  return (
    <div className={cn(["flex", "min-w-0", "items-center", "gap-3"])}>
      <Avatar
        square
        className={cn([
          "size-9",
          "shrink-0",
          "bg-transparent",
          !idp.avatar_hash && "border",
        ])}
        src={idp.avatar_hash && `/api/media?hash=${idp.avatar_hash}`}
        fallback={idp.name?.charAt(0)}
      />
      <div className={cn(["min-w-0", "flex-1"])}>
        <div className="truncate text-sm font-semibold">{idp.name}</div>
        <div
          className={cn([
            "mt-0.5",
            "flex",
            "min-w-0",
            "items-center",
            "gap-1.5",
            "text-xs",
            "text-muted-foreground",
          ])}
        >
          <span className="shrink-0 font-mono">#{idp.id}</span>
          <span className="shrink-0">·</span>
          <span className="truncate">{idp.portal || "-"}</span>
        </div>
      </div>
    </div>
  );
}

function UpdatedAtHeader({ column }: { column: Column<IdpView> }) {
  const { t } = useTranslation();
  const sort = column.getIsSorted();
  const icon = useMemo(() => {
    switch (sort) {
      case "asc":
        return <ArrowUpIcon />;
      case "desc":
        return <ArrowDownIcon />;
      default:
        return <ArrowUpDownIcon />;
    }
  }, [sort]);

  return (
    <Button
      icon={icon}
      variant="ghost"
      size="sm"
      className={cn(["-ml-3", "px-3", "text-muted-foreground"])}
      onClick={() => column.toggleSorting()}
    >
      {t("admin:idp.list.columns.updated_at")}
    </Button>
  );
}

function UpdatedAtCell({
  row,
  formatter,
}: {
  row: Row<IdpView>;
  formatter: Intl.DateTimeFormat;
}) {
  return (
    <span className="whitespace-nowrap text-sm text-secondary-foreground">
      {formatter.format(new Date(row.original.updated_at * 1000))}
    </span>
  );
}

function ActionsCell({ row }: { row: Row<IdpView> }) {
  const { t } = useTranslation();
  const idp = row.original;
  const sharedStore = useSharedStore();
  const [deleteDialogOpen, setDeleteDialogOpen] = useState(false);

  async function handleDelete() {
    await deleteAdminIdp(idp.id);
    toast.success(t("admin:idp.actions.delete.success", { name: idp.name }));
    setDeleteDialogOpen(false);
    sharedStore?.setRefresh();
  }

  return (
    <div className={cn(["flex", "items-center", "justify-center", "gap-2"])}>
      <Tooltip>
        <TooltipTrigger asChild>
          <Button
            variant="ghost"
            size="sm"
            square
            icon={<EditIcon />}
            aria-label={t("admin:idp.actions.update._")}
            asChild
          >
            <Link to={`/admin/idps/${idp.id}`} />
          </Button>
        </TooltipTrigger>
        <TooltipContent>{t("admin:idp.actions.update._")}</TooltipContent>
      </Tooltip>
      <DropdownMenu>
        <DropdownMenuTrigger asChild>
          <Button
            square
            size="sm"
            variant="ghost"
            icon={<EllipsisIcon />}
            aria-label={t("admin:idp.actions._")}
          />
        </DropdownMenuTrigger>
        <DropdownMenuContent>
          <DropdownMenuItem
            onClick={() => setDeleteDialogOpen(true)}
            className="text-error"
          >
            <TrashIcon />
            {t("admin:idp.actions.delete._")}
          </DropdownMenuItem>
        </DropdownMenuContent>
      </DropdownMenu>
      <Dialog open={deleteDialogOpen} onOpenChange={setDeleteDialogOpen}>
        <DialogContent>
          <Card
            className={cn([
              "flex",
              "w-full",
              "max-w-xl",
              "flex-col",
              "overflow-hidden",
              "rounded-elevated",
              "shadow-lg",
            ])}
          >
            <div className={cn(["flex", "flex-col", "gap-5", "p-5"])}>
              <div className={cn(["flex", "items-center", "gap-3"])}>
                <div
                  className={cn([
                    "flex",
                    "size-10",
                    "shrink-0",
                    "items-center",
                    "justify-center",
                    "rounded-badge",
                    "bg-error/10",
                    "text-error",
                  ])}
                >
                  <TrashIcon className="size-5" />
                </div>
                <h3 className="text-base font-semibold">
                  {t("admin:idp.actions.delete._")}
                </h3>
              </div>
              <p className="text-sm">
                <Trans
                  i18nKey="admin:idp.actions.delete.message"
                  values={{ name: idp.name }}
                  components={{
                    muted: <span className="text-muted-foreground" />,
                  }}
                />
              </p>
              <div className="flex justify-end">
                <Button
                  level="error"
                  variant="solid"
                  size="sm"
                  onClick={handleDelete}
                >
                  {t("common:actions.confirm")}
                </Button>
              </div>
            </div>
          </Card>
        </DialogContent>
      </Dialog>
    </div>
  );
}

function useColumns() {
  const { t, i18n } = useTranslation();
  const language = i18n.resolvedLanguage ?? i18n.language;
  const formatter = useMemo(
    () =>
      new Intl.DateTimeFormat(language, {
        year: "numeric",
        month: "2-digit",
        day: "2-digit",
        hour: "2-digit",
        minute: "2-digit",
      }),
    [language]
  );

  return useMemo<Array<ColumnDef<IdpView>>>(
    () => [
      {
        id: "idp",
        accessorFn: (idp) => idp.name,
        header: () => t("admin:idp.list.columns.idp"),
        cell: IdpCell,
      },
      {
        id: "status",
        header: () => t("admin:idp.list.columns.status"),
        cell: StatusCell,
      },
      {
        accessorKey: "updated_at",
        id: "updated_at",
        header: UpdatedAtHeader,
        cell: ({ row }) => <UpdatedAtCell row={row} formatter={formatter} />,
      },
      {
        id: "actions",
        header: () => (
          <div className="justify-self-center">{t("admin:idp.actions._")}</div>
        ),
        cell: ActionsCell,
      },
    ],
    [formatter, t]
  );
}

export { RowProvider, useColumns };
