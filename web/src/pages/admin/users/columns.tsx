import { Group, User } from "@/models/user";
import { Button } from "@/components/ui/button";
import {
    ArrowDown,
    ArrowUp,
    ArrowUpDown,
    ClipboardCheck,
    ClipboardCopy,
    EditIcon,
    TrashIcon,
} from "lucide-react";
import {
    UserRoundIcon,
    ShieldIcon,
    UserRoundXIcon,
    UserRoundCheckIcon,
} from "lucide-react";
import { useMemo, useState } from "react";
import { deleteUser } from "@/api/admin/users/user_id";
import { ColumnDef } from "@tanstack/react-table";
import { cn } from "@/utils";
import { ContentDialog } from "@/components/widgets/content-dialog";
import { Badge } from "@/components/ui/badge";
import {
    Tooltip,
    TooltipContent,
    TooltipTrigger,
} from "@/components/ui/tooltip";
import { useClipboard } from "@/hooks/use-clipboard";
import { Link } from "react-router";
import { toast } from "sonner";
import { Dialog, DialogContent } from "@/components/ui/dialog";
import { Card } from "@/components/ui/card";
import { useSharedStore } from "@/storages/shared";
import { Avatar } from "@/components/ui/avatar";

const columns: ColumnDef<User>[] = [
    {
        accessorKey: "id",
        id: "id",
        header: "ID",
        cell: ({ row }) => {
            const id = row.getValue("id");
            const idString = String(id);
            const { isCopied, copyToClipboard } = useClipboard();

            const displayId = idString.includes("-")
                ? idString.split("-")[0]
                : idString;

            return (
                <div className={cn(["flex", "items-center", "gap-1"])}>
                    <Badge>{displayId}</Badge>
                    <Tooltip>
                        <TooltipTrigger asChild>
                            <Button
                                icon={isCopied ? ClipboardCheck : ClipboardCopy}
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
        cell: ({ row }) => {
            const id = row.getValue("id") as number;
            const username = row.getValue("username") as string;

            return (
                <div className={cn(["flex", "items-center", "gap-3"])}>
                    <Avatar
                        src={`/api/users/${id}/avatar`}
                        fallback={username?.charAt(0)}
                    />
                    {username}
                </div>
            );
        },
    },
    {
        accessorKey: "nickname",
        id: "nickname",
        header: "昵称",
        cell: ({ row }) => {
            const nickname = row.getValue("nickname") as string;
            return nickname || "-";
        },
    },
    {
        accessorKey: "email",
        id: "email",
        header: "邮箱",
        cell: ({ row }) => {
            const email = row.getValue("email") as string;
            return email || "-";
        },
    },
    {
        accessorKey: "description",
        header: "描述",
        cell: ({ row }) => {
            const description = row.getValue("description") as string;

            if (!description) return "-";

            return description.length > 10 ? (
                <ContentDialog title="详细描述" content={description} />
            ) : (
                description
            );
        },
    },
    {
        accessorKey: "group",
        header: "用户组",
        cell: ({ row }) => {
            const groupId = row.getValue("group") as number;

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
        header: ({ column }) => {
            const Icon = useMemo(() => {
                switch (column.getIsSorted()) {
                    case "asc":
                        return ArrowUp;
                    case "desc":
                        return ArrowDown;
                    case false:
                    default:
                        return ArrowUpDown;
                }
            }, [column.getIsSorted()]);

            return (
                <div className={cn(["flex", "gap-1", "items-center"])}>
                    注册时间
                    <Button
                        icon={Icon}
                        square
                        size={"sm"}
                        onClick={() => column.toggleSorting()}
                    />
                </div>
            );
        },
        cell: ({ row }) => {
            return new Date(
                row.getValue<number>("created_at") * 1000
            ).toLocaleString();
        },
    },
    {
        id: "actions",
        header: () => <div className={cn(["justify-self-center"])}>操作</div>,
        cell: ({ row }) => {
            const id = row.getValue<number>("id");
            const username = row.getValue<string>("username");

            const sharedStore = useSharedStore();

            const [deleteDialogOpen, setDeleteDialogOpen] =
                useState<boolean>(false);

            function handleDelete() {
                deleteUser({
                    id,
                })
                    .then((res) => {
                        if (res.code === 200) {
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
                    className={cn([
                        "flex",
                        "items-center",
                        "justify-center",
                        "gap-2",
                    ])}
                >
                    <Button
                        variant={"ghost"}
                        size={"sm"}
                        square
                        icon={EditIcon}
                        asChild
                    >
                        <Link to={`/admin/users/${id}`} />
                    </Button>
                    <Button
                        level={"error"}
                        variant={"ghost"}
                        size={"sm"}
                        square
                        icon={TrashIcon}
                        disabled={row.original.group === Group.Admin}
                        onClick={() => setDeleteDialogOpen(true)}
                    />
                    <Dialog
                        open={deleteDialogOpen}
                        onOpenChange={setDeleteDialogOpen}
                    >
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
                                    className={cn([
                                        "flex",
                                        "gap-2",
                                        "items-center",
                                        "text-sm",
                                    ])}
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
