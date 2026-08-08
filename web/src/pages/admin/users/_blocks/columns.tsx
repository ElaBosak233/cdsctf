import {
  AlertCircleIcon,
  ArrowDownIcon,
  ArrowUpDownIcon,
  ArrowUpIcon,
  CircleCheckIcon,
  ClipboardCheckIcon,
  ClipboardCopyIcon,
  EditIcon,
  EllipsisIcon,
  ShieldIcon,
  TrashIcon,
  UserRoundCheckIcon,
  UserRoundIcon,
  UserRoundXIcon,
} from "lucide-react";
import { useMemo, useState } from "react";
import { Trans, useTranslation } from "react-i18next";
import { Link } from "react-router";
import { toast } from "sonner";
import { deleteUser } from "@/api/admin/users/user_id";
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
import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from "@/components/ui/tooltip";
import { useClipboard } from "@/hooks/use-clipboard";
import { Group, type UserAccountView } from "@/models/user";
import { useSharedStore } from "@/storages/shared";
import { cn } from "@/utils";
import type { Column, ColumnDef, Row } from "@/utils/data-table";

function UserCell({ row }: { row: Row<UserAccountView> }) {
  const user = row.original;
  const { t } = useTranslation();
  const { isCopied, copyToClipboard } = useClipboard();

  return (
    <div className={cn(["flex", "min-w-0", "items-center", "gap-3"])}>
      <Avatar
        src={user.avatar_hash && `/api/media?hash=${user.avatar_hash}`}
        fallback={user.username?.charAt(0)}
        className="size-9 shrink-0"
      />
      <div className={cn(["min-w-0", "flex-1"])}>
        <div className="truncate text-sm font-semibold">{user.username}</div>
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
          <span className="shrink-0 font-mono">#{user.id}</span>
          <Tooltip>
            <TooltipTrigger asChild>
              <Button
                icon={isCopied ? <ClipboardCheckIcon /> : <ClipboardCopyIcon />}
                square
                size="sm"
                variant="ghost"
                className={cn(["size-6", "shrink-0", "text-muted-foreground"])}
                aria-label={t("common:tooltip.copy")}
                onClick={() => copyToClipboard(String(user.id))}
              />
            </TooltipTrigger>
            <TooltipContent>{t("common:tooltip.copy")}</TooltipContent>
          </Tooltip>
          {user.name && (
            <>
              <span className="shrink-0">·</span>
              <span className="truncate">{user.name}</span>
            </>
          )}
        </div>
      </div>
    </div>
  );
}

function AccountStatusCell({ row }: { row: Row<UserAccountView> }) {
  const { t } = useTranslation();
  const groupConfig = {
    [Group.Guest]: {
      name: t("user:group.guest"),
      icon: UserRoundIcon,
      className: "text-muted-foreground",
    },
    [Group.Banned]: {
      name: t("user:group.banned"),
      icon: UserRoundXIcon,
      className: "border-error/20 bg-error/10 text-error",
    },
    [Group.User]: {
      name: t("user:group.user"),
      icon: UserRoundCheckIcon,
      className: "border-success/20 bg-success/10 text-success",
    },
    [Group.Admin]: {
      name: t("user:group.admin"),
      icon: ShieldIcon,
      className: "border-info/20 bg-info/10 text-info",
    },
  };
  const config =
    groupConfig[row.original.group as Group] ?? groupConfig[Group.Guest];
  const Icon = config.icon;

  return (
    <div className={cn(["flex", "flex-wrap", "items-center", "gap-1.5"])}>
      <Badge variant="outline" className={config.className}>
        <Icon />
        {config.name}
      </Badge>
      <Badge
        variant="outline"
        className={cn(
          row.original.verified
            ? ["border-success/20", "bg-success/10", "text-success"]
            : ["text-warning"]
        )}
      >
        {row.original.verified ? <CircleCheckIcon /> : <AlertCircleIcon />}
        {t(
          row.original.verified
            ? "user:list.verified.true"
            : "user:list.verified.false"
        )}
      </Badge>
    </div>
  );
}

function CreatedAtHeader({ column }: { column: Column<UserAccountView> }) {
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
      {t("user:list.columns.created_at")}
    </Button>
  );
}

function CreatedAtCell({
  row,
  formatter,
}: {
  row: Row<UserAccountView>;
  formatter: Intl.DateTimeFormat;
}) {
  return (
    <span className="whitespace-nowrap text-sm text-secondary-foreground">
      {formatter.format(new Date(row.original.created_at * 1000))}
    </span>
  );
}

function ActionsCell({ row }: { row: Row<UserAccountView> }) {
  const { t } = useTranslation();
  const id = row.original.id;
  const username = row.original.username;
  const sharedStore = useSharedStore();
  const [deleteDialogOpen, setDeleteDialogOpen] = useState(false);

  async function handleDelete() {
    await deleteUser({ id: id! });
    toast.success(t("user:actions.delete.success", { username }));
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
            aria-label={t("user:actions.update._")}
            asChild
          >
            <Link to={`/admin/users/${id}`} />
          </Button>
        </TooltipTrigger>
        <TooltipContent>{t("user:actions.update._")}</TooltipContent>
      </Tooltip>
      <DropdownMenu>
        <DropdownMenuTrigger asChild>
          <Button
            square
            size="sm"
            variant="ghost"
            icon={<EllipsisIcon />}
            aria-label={t("user:actions._")}
          />
        </DropdownMenuTrigger>
        <DropdownMenuContent>
          <DropdownMenuItem
            onClick={() => setDeleteDialogOpen(true)}
            className="text-error"
            disabled={row.original.group === Group.Admin}
          >
            <TrashIcon />
            {t("user:actions.delete._")}
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
                  {t("user:actions.delete._")}
                </h3>
              </div>
              <p className="text-sm">
                <Trans
                  i18nKey="user:actions.delete.message"
                  values={{ username }}
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

  return useMemo<Array<ColumnDef<UserAccountView>>>(
    () => [
      {
        id: "user",
        accessorFn: (user) => user.username,
        header: () => t("user:list.columns.user"),
        cell: UserCell,
      },
      {
        id: "status",
        header: () => t("user:list.columns.status"),
        cell: AccountStatusCell,
      },
      {
        accessorKey: "created_at",
        id: "created_at",
        header: CreatedAtHeader,
        cell: ({ row }) => <CreatedAtCell row={row} formatter={formatter} />,
      },
      {
        id: "actions",
        header: () => (
          <div className="justify-self-center">{t("user:actions._")}</div>
        ),
        cell: ActionsCell,
      },
    ],
    [formatter, t]
  );
}

export { useColumns };
