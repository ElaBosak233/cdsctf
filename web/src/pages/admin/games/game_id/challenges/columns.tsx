import { Button } from "@/components/ui/button";
import {
    ClipboardCheck,
    ClipboardCopy,
    EditIcon,
    TrashIcon,
} from "lucide-react";
import { useState } from "react";
import { ColumnDef } from "@tanstack/react-table";
import { cn } from "@/utils";
import { useCategoryStore } from "@/storages/category";
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

const columns: ColumnDef<GameChallenge>[] = [
    {
        accessorKey: "game_id",
        enableHiding: false,
    },
    {
        accessorKey: "is_enabled",
        header: "启用",
        cell: ({ row }) => {
            const isEnabled = row.getValue<boolean>("is_enabled");
            const title = row.getValue<string>("challenge.title");
            const challenge_id = row.getValue<string>("challenge_id");
            const game_id = row.getValue<number>("game_id");
            const [checked, setChecked] = useState(isEnabled);

            function handlePublicnessChange() {
                const newValue = !checked;
                setChecked(newValue);

                updateGameChallenge({
                    game_id,
                    challenge_id,
                    is_enabled: newValue,
                }).then((res) => {
                    if (res.code === 200) {
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
        accessorKey: "challenge_id",
        id: "challenge_id",
        header: "ID",
        cell: ({ row }) => {
            const id = row.getValue<string>("challenge_id");
            const { isCopied, copyToClipboard } = useClipboard();
            return (
                <div className={cn(["flex", "items-center", "gap-1"])}>
                    <Badge>{id?.split("-")?.[0]}</Badge>
                    <Tooltip>
                        <TooltipTrigger asChild>
                            <Button
                                icon={isCopied ? ClipboardCheck : ClipboardCopy}
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
        accessorKey: "challenge.title",
        id: "challenge.title",
        header: "标题",
        cell: ({ row }) => {
            const title = row.getValue("challenge.title") as string;
            return title || "-";
        },
    },
    {
        accessorKey: "challenge.category",
        id: "challenge.category",
        header: "分类",
        cell: ({ row }) => {
            const categoryId = row.getValue("challenge.category") as number;
            const category = useCategoryStore
                .getState()
                .getCategory(categoryId);

            const Icon = category.icon!;
            return (
                <div className={cn(["flex", "gap-2", "items-center"])}>
                    <Icon className={cn(["size-4"])} />
                    {useCategoryStore
                        .getState()
                        .getCategory(categoryId)
                        ?.name?.toUpperCase()}
                </div>
            );
        },
    },
    {
        accessorKey: "pts",
        id: "pts",
        header: "当前分值",
        cell: ({ row }) => {
            const pts = row.getValue<number>("pts");

            return pts;
        },
    },
    {
        id: "actions",
        header: () => <div className={cn(["justify-self-center"])}>操作</div>,
        cell: ({ row }) => {
            const challenge_id = row.getValue<string>("challenge_id");
            const game_id = row.getValue<number>("game_id");
            const title = row.getValue<string>("challenge.title");

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
                        if (res.code === 200) {
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
                    <Button
                        variant={"ghost"}
                        size={"sm"}
                        square
                        icon={EditIcon}
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
                        icon={TrashIcon}
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
