import { Challenge } from "@/models/challenge";
import { Button } from "@/components/ui/button";
import {
    ArrowDown,
    ArrowUp,
    ArrowUpDown,
    Box,
    Check,
    ClipboardCheck,
    ClipboardCopy,
    EditIcon,
    ShipWheel,
    TrashIcon,
    X,
} from "lucide-react";
import { useMemo, useState } from "react";
import {
    deleteChallenge,
    updateChallenge,
} from "@/api/admin/challenges/challenge_id";
import { ColumnDef } from "@tanstack/react-table";
import { cn } from "@/utils";
import { useCategoryStore } from "@/storages/category";
import { ContentDialog } from "@/components/widgets/content-dialog";
import { Switch } from "@/components/ui/switch";
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

const columns: ColumnDef<Challenge>[] = [
    {
        accessorKey: "is_public",
        header: "练习",
        cell: ({ row }) => {
            const isPublic = row.getValue<boolean>("is_public");
            const title = row.getValue<string>("title");
            const id = row.getValue<string>("id");
            const [checked, setChecked] = useState(isPublic);

            function handlePublicnessChange() {
                const newValue = !checked;
                setChecked(newValue);

                updateChallenge({
                    id,
                    is_public: newValue,
                }).then((res) => {
                    if (res.code === 200) {
                        toast.success(
                            `更新题目 ${title} 的公开性: ${newValue ? "公开" : "私有"}`,
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
        accessorKey: "id",
        id: "id",
        header: "ID",
        cell: ({ row }) => {
            const id = row.getValue<string>("id");
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
        accessorKey: "title",
        id: "title",
        header: "标题",
        cell: ({ row }) => {
            const title = row.getValue("title") as string;
            return title || "-";
        },
    },
    {
        accessorKey: "tags",
        id: "tags",
        header: "标签",
        cell: ({ row }) => {
            const tags = row.getValue("tags") as string[] | undefined;

            if (!tags || !Array.isArray(tags) || tags.length === 0) {
                return "-";
            }

            return (
                <div className="flex flex-wrap gap-1">
                    {tags.slice(0, 3).map((tag, index) => (
                        <Badge key={index}>{tag}</Badge>
                    ))}
                    {tags.length > 3 && (
                        <ContentDialog
                            title="所有标签"
                            content={tags.join(", ")}
                            triggerText={`+${tags.length - 3}`}
                        />
                    )}
                </div>
            );
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
        accessorKey: "category",
        header: "分类",
        cell: ({ row }) => {
            const categoryId = row.getValue("category") as number;
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
        accessorKey: "has_attachment",
        header: "附件",
        cell: ({ row }) => {
            const hasAttachment = row.getValue<boolean>("has_attachment");

            const options = [
                {
                    className: ["bg-warning", "text-warning-foreground"],
                    icon: <X />,
                },
                {
                    className: ["bg-info", "text-info-foreground"],
                    icon: <Check />,
                },
            ];

            return (
                <Badge
                    className={cn([options[Number(hasAttachment)]?.className])}
                >
                    {options[Number(hasAttachment)]?.icon}
                </Badge>
            );
        },
    },
    {
        accessorKey: "is_dynamic",
        header: "动态性",
        cell: ({ row }) => {
            const isDynamic = row.getValue<boolean>("is_dynamic");

            return (
                <Badge
                    className={cn([
                        isDynamic
                            ? ["bg-info", "text-info-foreground"]
                            : ["bg-success", "text-success-foreground"],
                    ])}
                >
                    {isDynamic ? <ShipWheel /> : <Box />}
                    {isDynamic ? "动态" : "静态"}
                </Badge>
            );
        },
    },
    {
        accessorKey: "updated_at",
        id: "updated_at",
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
                    更新时间
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
                row.getValue<number>("updated_at") * 1000
            ).toLocaleString();
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
                    创建时间
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
            const id = row.getValue<string>("id");
            const title = row.getValue<string>("title");

            const sharedStore = useSharedStore();

            const [deleteDialogOpen, setDeleteDialogOpen] =
                useState<boolean>(false);

            function handleDelete() {
                deleteChallenge({
                    id,
                })
                    .then((res) => {
                        if (res.code === 200) {
                            toast.success(`题目 ${title} 删除成功`);
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
                        <Link to={`/admin/challenges/${id}`} />
                    </Button>
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
                                    删除题目
                                </div>
                                <p className={cn(["text-sm"])}>
                                    你确定要删除题目 {title} 吗？
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
