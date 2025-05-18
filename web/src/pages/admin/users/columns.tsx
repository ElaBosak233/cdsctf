import { ColumnDef } from "@tanstack/react-table";
import { StatusCodes } from "http-status-codes";
import {
  ArrowDownIcon,
  ArrowUpDownIcon,
  ArrowUpIcon,
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
import { Group, User } from "@/models/user";
import { useSharedStore } from "@/storages/shared";
import { cn } from "@/utils";

const columns: Array<ColumnDef<User>> = [
  {
    accessorKey: "id",
    id: "id",
    header: "ID",
    cell: function IdCell({ row }) {
      const id = row.original.id;
      const idString = String(id);
      const { isCopied, copyToClipboard } = useClipboard();

      const displayId = idString.includes("-")
        ? idString.split("-")[0]
        : idString;

      return (
        <div className={cn(["flex", "items-center", "gap-1"])}>
          <Badge className={cn(["font-mono"])}>{displayId}</Badge>
          <Tooltip>
            <TooltipTrigger asChild>
              <Button
                icon={isCopied ? <ClipboardCheckIcon /> : <ClipboardCopyIcon />}
                square
                size={"sm"}
                onClick={() => copyToClipboard(idString)}
              />
            </TooltipTrigger>
            <TooltipContent>复制到剪贴板</TooltipContent>
          </Tooltip>
        </div>
      );
    },
  },
  {
    accessorKey: "username",
    id: "username",
    header: "用户名",
    cell: ({ row }) => (
      <div className={cn(["flex", "items-center", "gap-3"])}>
        <Avatar
          src={`/api/users/${row.original.id}/avatar`}
          fallback={row.original.username?.charAt(0)}
        />
        {row.original.username}
      </div>
    ),
  },
  {
    accessorKey: "name",
    id: "name",
    header: "昵称",
    cell: ({ row }) => row.original.name || "-",
  },
  {
    accessorKey: "email",
    id: "email",
    header: "邮箱",
    cell: ({ row }) => row.original.email || "-",
  },
  {
    accessorKey: "group",
    header: "用户组",
    cell: ({ row }) => {
      const groupId = row.original.group;

      const groupConfig = {
        [Group.Guest]: {
          name: "GUEST",
          icon: UserRoundIcon,
          className: "bg-secondary text-secondary-foreground",
        },
        [Group.Banned]: {
          name: "BANNED",
          icon: UserRoundXIcon,
          className: "bg-destructive text-destructive-foreground",
        },
        [Group.User]: {
          name: "USER",
          icon: UserRoundCheckIcon,
          className: "bg-primary text-primary-foreground",
        },
        [Group.Admin]: {
          name: "ADMIN",
          icon: ShieldIcon,
          className: "bg-info text-info-foreground",
        },
      };

      const config = groupConfig[groupId as Group] || groupConfig[Group.Guest];
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
    header: function CreatedAtHeader({ column }) {
      const icon = useMemo(() => {
        switch (column.getIsSorted()) {
          case "asc":
            return <ArrowUpIcon />;
          case "desc":
            return <ArrowDownIcon />;
          case false:
          default:
            return <ArrowUpDownIcon />;
        }
      }, [column.getIsSorted()]);

      return (
        <div className={cn(["flex", "gap-1", "items-center"])}>
          注册时间
          <Button
            icon={icon}
            square
            size={"sm"}
            onClick={() => column.toggleSorting()}
          />
        </div>
      );
    },
    cell: ({ row }) => {
      return new Date(Number(row.original.created_at) * 1000).toLocaleString();
    },
  },
  {
    id: "actions",
    header: () => <div className={cn(["justify-self-center"])}>操作</div>,
    cell: function ActionsCell({ row }) {
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
              toast.success(`用户 ${username} 删除成功`);
              setDeleteDialogOpen(false);
            }
          })
          .finally(() => {
            sharedStore?.setRefresh();
          });
      }

      return (
        <div
          className={cn(["flex", "items-center", "justify-center", "gap-2"])}
        >
          <Button
            variant={"ghost"}
            size={"sm"}
            square
            icon={<EditIcon />}
            asChild
          >
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
                  "w-72",
                  "gap-5",
                ])}
              >
                <div
                  className={cn(["flex", "gap-2", "items-center", "text-sm"])}
                >
                  <TrashIcon className={cn(["size-4"])} />
                  删除用户
                </div>
                <p className={cn(["text-sm"])}>
                  你确定要删除用户 {username} 吗？
                </p>
                <div className={cn(["flex", "justify-end"])}>
                  <Button
                    level={"error"}
                    variant={"tonal"}
                    size={"sm"}
                    onClick={handleDelete}
                  >
                    确定
                  </Button>
                </div>
              </Card>
            </DialogContent>
          </Dialog>
        </div>
      );
    },
  },
];

export { columns };
