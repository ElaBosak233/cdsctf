import type { Column, ColumnDef, Row } from "@tanstack/react-table";
import { StatusCodes } from "http-status-codes";
import {
  AlertCircleIcon,
  ArrowDownIcon,
  ArrowUpDownIcon,
  ArrowUpIcon,
  CircleCheckIcon,
  ClipboardCheckIcon,
  ClipboardCopyIcon,
  EditIcon,
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
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from "@/components/ui/tooltip";
import { useClipboard } from "@/hooks/use-clipboard";
import { Group, type User } from "@/models/user";
import { useSharedStore } from "@/storages/shared";
import { cn } from "@/utils";

function IdCell({ row }: { row: Row<User> }) {
  const id = row.original.id;
  const { isCopied, copyToClipboard } = useClipboard();

  return (
    <div className={cn(["flex", "items-center", "gap-2"])}>
      <Badge className={cn(["font-mono"])}># {id}</Badge>
      <Tooltip>
        <TooltipTrigger asChild>
          <Button
            icon={isCopied ? <ClipboardCheckIcon /> : <ClipboardCopyIcon />}
            square
            size={"sm"}
            onClick={() => copyToClipboard(String(id))}
          />
        </TooltipTrigger>
        <TooltipContent>复制到剪贴板</TooltipContent>
      </Tooltip>
    </div>
  );
}

function CreatedAtHeader({ column }: { column: Column<User> }) {
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
    <div className={cn(["flex", "gap-1", "items-center"])}>
      {t("user.created_at")}
      <Button
        icon={icon}
        square
        size={"sm"}
        onClick={() => column.toggleSorting()}
      />
    </div>
  );
}

function ActionsCell({ row }: { row: Row<User> }) {
  const { t } = useTranslation();

  const id = row.original.id;
  const username = row.original.username;

  const sharedStore = useSharedStore();

  const [deleteDialogOpen, setDeleteDialogOpen] = useState<boolean>(false);

  function handleDelete() {
    deleteUser({
      id: id!,
    })
      .then((res) => {
        if (res.code === StatusCodes.OK) {
          toast.success(t("user.actions.delete.success", { username }));
          setDeleteDialogOpen(false);
        }
      })
      .finally(() => {
        sharedStore?.setRefresh();
      });
  }

  return (
    <div className={cn(["flex", "items-center", "justify-center", "gap-2"])}>
      <Button variant={"ghost"} size={"sm"} square icon={<EditIcon />} asChild>
        <Link to={`/admin/users/${id}`} />
      </Button>
      <Button
        level={"error"}
        variant={"ghost"}
        size={"sm"}
        square
        icon={<TrashIcon />}
        disabled={row.original.group === Group.Admin}
        onClick={() => setDeleteDialogOpen(true)}
      />
      <Dialog open={deleteDialogOpen} onOpenChange={setDeleteDialogOpen}>
        <DialogContent>
          <Card
            className={cn([
              "flex",
              "flex-col",
              "p-5",
              "min-h-32",
              "w-lg",
              "gap-5",
            ])}
          >
            <div className={cn(["flex", "gap-2", "items-center", "text-sm"])}>
              <TrashIcon className={cn(["size-4", "text-error"])} />
              {t("user.actions.delete._")}
            </div>
            <p className={cn(["text-sm"])}>
              <Trans
                i18nKey={"user.actions.delete.message"}
                values={{ username: row.original.username }}
                components={{
                  muted: <span className={cn(["text-muted-foreground"])} />,
                }}
              />
            </p>
            <div className={cn(["flex", "justify-end"])}>
              <Button
                level={"error"}
                variant={"solid"}
                size={"sm"}
                onClick={handleDelete}
              >
                {t("common.actions.confirm")}
              </Button>
            </div>
          </Card>
        </DialogContent>
      </Dialog>
    </div>
  );
}

function useColumns() {
  const { t } = useTranslation();

  const columns: Array<ColumnDef<User>> = useMemo(() => {
    return [
      {
        accessorKey: "id",
        id: "id",
        header: "ID",
        cell: IdCell,
      },
      {
        accessorKey: "username",
        id: "username",
        header: t("user.username"),
        cell: ({ row }) => (
          <div className={cn(["flex", "items-center", "gap-3"])}>
            <Avatar
              src={
                row.original.has_avatar &&
                `/api/users/${row.original.id}/avatar`
              }
              fallback={row.original.username?.charAt(0)}
            />
            {row.original.username}
            {row.original.is_verified ? (
              <CircleCheckIcon className={cn(["text-success", "size-3.5"])} />
            ) : (
              <AlertCircleIcon className={cn(["text-warning", "size-3.5"])} />
            )}
          </div>
        ),
      },
      {
        accessorKey: "name",
        id: "name",
        header: t("user.name"),
        cell: ({ row }) => row.original.name || "-",
      },
      {
        accessorKey: "group",
        header: t("user.group._"),
        cell: ({ row }) => {
          const groupId = row.original.group;

          const groupConfig = {
            [Group.Guest]: {
              name: t("user.group.guest"),
              icon: UserRoundIcon,
              className: "bg-secondary text-secondary-foreground",
            },
            [Group.Banned]: {
              name: t("user.group.banned"),
              icon: UserRoundXIcon,
              className: "bg-destructive text-destructive-foreground",
            },
            [Group.User]: {
              name: t("user.group.user"),
              icon: UserRoundCheckIcon,
              className: "bg-primary text-primary-foreground",
            },
            [Group.Admin]: {
              name: t("user.group.admin"),
              icon: ShieldIcon,
              className: "bg-info text-info-foreground",
            },
          };

          const config =
            groupConfig[groupId as Group] || groupConfig[Group.Guest];
          const Icon = config.icon;

          return (
            <div className={cn(["flex", "gap-2", "items-center"])}>
              <Badge className={config.className}>
                <div className="flex items-center gap-1">
                  <Icon className="size-3.5" />
                  <span>{config.name}</span>
                </div>
              </Badge>
            </div>
          );
        },
      },
      {
        accessorKey: "created_at",
        id: "created_at",
        header: CreatedAtHeader,
        cell: ({ row }) => (
          <span className={cn(["text-sm", "text-secondary-foreground"])}>
            {new Date(Number(row.original.created_at) * 1000).toLocaleString()}
          </span>
        ),
      },
      {
        id: "actions",
        header: () => (
          <div className={cn(["justify-self-center"])}>
            {t("user.actions._")}
          </div>
        ),
        cell: ActionsCell,
      },
    ];
  }, [t]);

  return columns;
}

export { useColumns };
