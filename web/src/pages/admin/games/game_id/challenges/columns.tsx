import { Button } from "@/components/ui/button";
import {
    ClipboardCheckIcon,
    ClipboardCopyIcon,
    EditIcon,
    SettingsIcon,
    TrashIcon,
} from "lucide-react";
import { useState } from "react";
import { ColumnDef } from "@tanstack/react-table";
import { cn } from "@/utils";
import { Switch } from "@/components/ui/switch";
import { Badge } from "@/components/ui/badge";
import {
    Tooltip,
    TooltipContent,
    TooltipTrigger,
} from "@/components/ui/tooltip";
import { useClipboard } from "@/hooks/use-clipboard";
import { toast } from "sonner";
import { Dialog, DialogContent } from "@/components/ui/dialog";
import { Card } from "@/components/ui/card";
import { useSharedStore } from "@/storages/shared";
import { GameChallenge } from "@/models/game_challenge";
import {
    deleteGameChallenge,
    updateGameChallenge,
} from "@/api/admin/games/game_id/challenges/challenge_id";
import { EditDialog } from "./edit-dialog";
import { StatusCodes } from "http-status-codes";
import { getCategory } from "@/utils/category";
import { Link } from "react-router";

const columns: ColumnDef<GameChallenge>[] = [
    {
        accessorKey: "game_id",
        enableHiding: false,
    },
    {
        accessorKey: "is_enabled",
        header: "启用",
        cell: ({ row }) => {
            const isEnabled = row.original.is_enabled;
            const title = row.original.challenge_title;
            const challenge_id = row.original.challenge_id;
            const game_id = row.original.game_id;
            const [checked, setChecked] = useState(isEnabled);

            function handlePublicnessChange() {
                const newValue = !checked;
                setChecked(newValue);

                updateGameChallenge({
                    game_id,
                    challenge_id,
                    is_enabled: newValue,
                }).then((res) => {
                    if (res.code === StatusCodes.OK) {
                        toast.success(
                            `${newValue ? "启用" : "禁用"} 赛题 ${title}`,
                            {
                                id: "publicness_change",
                            }
                        );
                    }
                });
            }

            return (
                <Switch
                    checked={checked}
                    onCheckedChange={handlePublicnessChange}
                    aria-label="公开性开关"
                />
            );
        },
    },
    {
        id: "challenge_id",
        header: "ID",
        cell: ({ row }) => {
            const id = row.original.challenge_id!;
            const { isCopied, copyToClipboard } = useClipboard();
            return (
                <div className={cn(["flex", "items-center", "gap-2"])}>
                    <Badge className={cn(["font-mono"])}>
                        {id?.split("-")?.[0]}
                    </Badge>
                    <Tooltip>
                        <TooltipTrigger asChild>
                            <Button
                                icon={
                                    isCopied ? (
                                        <ClipboardCheckIcon />
                                    ) : (
                                        <ClipboardCopyIcon />
                                    )
                                }
                                square
                                size={"sm"}
                                onClick={() => copyToClipboard(id)}
                            />
                        </TooltipTrigger>
                        <TooltipContent>复制到剪贴板</TooltipContent>
                    </Tooltip>
                </div>
            );
        },
    },
    {
        id: "challenge_title",
        header: "标题",
        cell: ({ row }) => {
            return row.original.challenge_title || "-";
        },
    },
    {
        id: "challenge_category",
        header: "分类",
        cell: ({ row }) => {
            const categoryId = row.original.challenge_category;
            const category = getCategory(categoryId!);

            const Icon = category.icon!;
            return (
                <div className={cn(["flex", "gap-2", "items-center"])}>
                    <Icon className={cn(["size-4"])} />
                    {category.name?.toUpperCase()}
                </div>
            );
        },
    },
    {
        accessorKey: "pts",
        id: "pts",
        header: "当前分值",
        cell: ({ row }) => row.original.pts,
    },
    {
        id: "actions",
        header: () => <div className={cn(["justify-self-center"])}>操作</div>,
        cell: ({ row }) => {
            const challenge_id = row.original.challenge_id;
            const game_id = row.original.game_id;
            const title = row.original.challenge_title;

            const sharedStore = useSharedStore();

            const [editDialogOpen, setEditDialogOpen] =
                useState<boolean>(false);
            const [deleteDialogOpen, setDeleteDialogOpen] =
                useState<boolean>(false);

            function handleDelete() {
                deleteGameChallenge({
                    game_id,
                    challenge_id,
                })
                    .then((res) => {
                        if (res.code === StatusCodes.OK) {
                            toast.success(`赛题 ${title} 删除成功`);
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
                    <Button icon={<EditIcon />} square size={"sm"} asChild>
                        <Link
                            to={`/admin/challenges/${row.original.challenge_id}`}
                        />
                    </Button>
                    <Button
                        variant={"ghost"}
                        size={"sm"}
                        square
                        icon={<SettingsIcon />}
                        onClick={() => setEditDialogOpen(true)}
                    />
                    <Dialog
                        open={editDialogOpen}
                        onOpenChange={setEditDialogOpen}
                    >
                        <DialogContent>
                            <EditDialog
                                gameChallenge={row.original}
                                onClose={() => setEditDialogOpen(false)}
                            />
                        </DialogContent>
                    </Dialog>
                    <Button
                        level={"error"}
                        variant={"ghost"}
                        size={"sm"}
                        square
                        icon={<TrashIcon />}
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
                                    删除赛题
                                </div>
                                <p className={cn(["text-sm"])}>
                                    你确定要删除赛题 {title} 吗？
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
