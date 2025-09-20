import type { ColumnDef } from "@tanstack/react-table";
import { StatusCodes } from "http-status-codes";
import {
  ArrowDownIcon,
  ArrowUpDownIcon,
  ArrowUpIcon,
  BoxIcon,
  CheckIcon,
  ClipboardCheckIcon,
  ClipboardCopyIcon,
  EditIcon,
  EyeClosedIcon,
  EyeIcon,
  ShipWheelIcon,
  TrashIcon,
  XIcon,
} from "lucide-react";
import { useMemo, useState } from "react";
import { Link } from "react-router";
import { toast } from "sonner";

import {
  deleteChallenge,
  updateChallenge,
} from "@/api/admin/challenges/challenge_id";
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
import type { Challenge } from "@/models/challenge";
import { useSharedStore } from "@/storages/shared";
import { cn } from "@/utils";
import { getCategory } from "@/utils/category";

const columns: Array<ColumnDef<Challenge>> = [
  {
    accessorKey: "id",
    id: "id",
    header: "ID",
    cell: function IdCell({ row }) {
      const id = row.original.id!;
      const { isCopied, copyToClipboard } = useClipboard();
      return (
        <div className={cn(["flex", "items-center", "gap-2"])}>
          <Badge className={cn(["font-mono"])}>{id}</Badge>
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
    },
  },
  {
    accessorKey: "title",
    id: "title",
    header: "标题",
    cell: ({ row }) => (
      <div
        className={cn([
          "w-42",
          "overflow-hidden",
          "text-ellipsis",
          "whitespace-nowrap",
        ])}
      >
        {row.original.title || "-"}
      </div>
    ),
  },
  {
    accessorKey: "tags",
    id: "tags",
    header: "标签",
    cell: ({ row }) => {
      const tags = row.original.tags;

      return (
        <div className={cn(["flex", "flex-wrap", "gap-1", "w-36"])}>
          {tags?.map((tag, index) => (
            <Badge key={index}>{tag}</Badge>
          ))}
        </div>
      );
    },
  },
  {
    accessorKey: "category",
    header: "分类",
    cell: ({ row }) => {
      const categoryId = row.original.category!;
      const category = getCategory(categoryId);

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
    accessorKey: "has_attachment",
    header: "附件",
    cell: ({ row }) => {
      const hasAttachment = row.original.has_attachment;

      const options = [
        {
          className: ["bg-warning", "text-warning-foreground"],
          icon: <XIcon />,
        },
        {
          className: ["bg-info", "text-info-foreground"],
          icon: <CheckIcon />,
        },
      ];

      return (
        <Badge className={cn([options[Number(hasAttachment)]?.className])}>
          {options[Number(hasAttachment)]?.icon}
        </Badge>
      );
    },
  },
  {
    accessorKey: "is_dynamic",
    header: "动态性",
    cell: ({ row }) => {
      const isDynamic = row.original.is_dynamic;

      return (
        <Badge
          className={cn([
            isDynamic
              ? ["bg-info", "text-info-foreground"]
              : ["bg-success", "text-success-foreground"],
          ])}
        >
          {isDynamic ? <ShipWheelIcon /> : <BoxIcon />}
          {isDynamic ? "动态" : "静态"}
        </Badge>
      );
    },
  },
  {
    accessorKey: "updated_at",
    id: "updated_at",
    header: function UpdatedAtHeader({ column }) {
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
          更新于
          <Button
            icon={icon}
            square
            size={"sm"}
            onClick={() => column.toggleSorting()}
          />
        </div>
      );
    },
    cell: ({ row }) => (
      <span className={cn(["text-secondary-foreground", "text-sm"])}>
        {new Date(row.getValue<number>("updated_at") * 1000).toLocaleString()}
      </span>
    ),
  },
  {
    accessorKey: "created_at",
    id: "created_at",
    header: function CreatedAtHeader({ column }) {
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
          创建于
          <Button
            icon={icon}
            square
            size={"sm"}
            onClick={() => column.toggleSorting()}
          />
        </div>
      );
    },
    cell: ({ row }) => (
      <span className={cn(["text-secondary-foreground", "text-sm"])}>
        {new Date(row.getValue<number>("created_at") * 1000).toLocaleString()}
      </span>
    ),
  },
  {
    id: "actions",
    header: () => <div className={cn(["justify-self-center"])}>操作</div>,
    cell: function ActionsCell({ row }) {
      const id = row.original.id;
      const title = row.original.title;
      const isPublic = row.original.is_public;

      const sharedStore = useSharedStore();

      const [deleteDialogOpen, setDeleteDialogOpen] = useState<boolean>(false);

      const [checked, setChecked] = useState(isPublic);

      async function handlePublicnessChange() {
        const newValue = !checked;
        setChecked(newValue);

        const res = await updateChallenge({
          id,
          is_public: newValue,
        });

        if (res.code === StatusCodes.OK) {
          toast.success(
            `更新题目 ${title} 的公开性: ${newValue ? "公开" : "私有"}`,
            {
              id: "publicness_change",
            }
          );
        }
      }

      async function handleDelete() {
        try {
          const res = await deleteChallenge({
            id,
          });

          if (res.code === StatusCodes.OK) {
            toast.success(`题目 ${title} 删除成功`);
            setDeleteDialogOpen(false);
          }
        } finally {
          sharedStore?.setRefresh();
        }
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
            <Link to={`/admin/challenges/${id}`} />
          </Button>

          <Tooltip>
            <TooltipTrigger asChild>
              <Button
                level={checked ? "warning" : "success"}
                variant={"ghost"}
                size={"sm"}
                square
                icon={checked ? <EyeClosedIcon /> : <EyeIcon />}
                onClick={handlePublicnessChange}
              />
            </TooltipTrigger>
            <TooltipContent>{checked ? "隐藏" : "公开"}</TooltipContent>
          </Tooltip>

          <Button
            level={"error"}
            variant={"ghost"}
            size={"sm"}
            square
            icon={<TrashIcon />}
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
                  删除题目
                </div>
                <p className={cn(["text-sm"])}>你确定要删除题目 {title} 吗？</p>
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
