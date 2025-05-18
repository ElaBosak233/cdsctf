import { ColumnDef } from "@tanstack/react-table";
import { StatusCodes } from "http-status-codes";
import {
  ArrowDownIcon,
  ArrowUpDownIcon,
  ArrowUpIcon,
  CheckIcon,
  ClipboardCheckIcon,
  ClipboardCopyIcon,
  EditIcon,
  TrashIcon,
  XIcon,
} from "lucide-react";
import { useMemo, useState } from "react";
import { Link } from "react-router";
import { toast } from "sonner";

import { deleteGame, updateGame } from "@/api/admin/games/game_id";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";
import { Dialog, DialogContent } from "@/components/ui/dialog";
import { Switch } from "@/components/ui/switch";
import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from "@/components/ui/tooltip";
import { ContentDialog } from "@/components/widgets/content-dialog";
import { useClipboard } from "@/hooks/use-clipboard";
import { Game } from "@/models/game";
import { useSharedStore } from "@/storages/shared";
import { cn } from "@/utils";

const columns: Array<ColumnDef<Game>> = [
  {
    accessorKey: "is_enabled",
    id: "is_enabled",
    header: "启用",
    cell: function IsEnabled({ row }) {
      const isEnabled = row.original.is_enabled;
      const title = row.original.title;
      const id = row.original.id;
      const [checked, setChecked] = useState(isEnabled);

      function handlePublicnessChange() {
        const newValue = !checked;
        setChecked(newValue);

        updateGame({
          id,
          is_enabled: newValue,
        }).then((res) => {
          if (res.code === StatusCodes.OK) {
            toast.success(
              `更新比赛 ${title} 的可见性: ${newValue ? "可见" : "不可见"}`,
              {
                id: "enablement_change",
              }
            );
          }
        });
      }

      return (
        <Switch
          checked={checked}
          onCheckedChange={handlePublicnessChange}
          aria-label="启用性开关"
        />
      );
    },
  },
  {
    accessorKey: "id",
    id: "id",
    header: "ID",
    cell: function IdCell({ row }) {
      const id = row.original.id;
      const { isCopied, copyToClipboard } = useClipboard();
      return (
        <div className={cn(["flex", "items-center", "gap-1"])}>
          <Badge className={cn(["font-mono"])}>{id}</Badge>
          <Tooltip>
            <TooltipTrigger asChild>
              <Button
                icon={isCopied ? <ClipboardCheckIcon /> : <ClipboardCopyIcon />}
                square
                size={"sm"}
                onClick={() => copyToClipboard(`${id}`)}
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
    cell: ({ row }) => row.original.title || "-",
  },
  {
    accessorKey: "is_public",
    id: "is_public",
    header: "公开赛",
    cell: ({ row }) => {
      const isPublic = row.original.is_public;

      return (
        <Badge
          className={cn([
            isPublic
              ? ["bg-info", "text-info-foreground"]
              : ["bg-success", "text-success-foreground"],
          ])}
        >
          {isPublic ? <CheckIcon /> : <XIcon />}
        </Badge>
      );
    },
  },
  {
    accessorKey: "sketch",
    header: "简述",
    cell: ({ row }) => {
      const sketch = row.original.sketch;

      if (!sketch) return "-";

      return sketch.length > 10 ? (
        <ContentDialog title="详细描述" content={sketch} />
      ) : (
        sketch
      );
    },
  },
  {
    accessorKey: "started_at",
    id: "started_at",
    header: function StartedAtHeader({ column }) {
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
          开始于
          <Button
            icon={icon}
            square
            size={"sm"}
            onClick={() => column.toggleSorting()}
          />
        </div>
      );
    },
    cell: ({ row }) =>
      new Date(row.getValue<number>("started_at") * 1000).toLocaleString(),
  },
  {
    accessorKey: "ended_at",
    id: "ended_at",
    header: function EndedAtHeader({ column }) {
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
          结束于
          <Button
            icon={icon}
            square
            size={"sm"}
            onClick={() => column.toggleSorting()}
          />
        </div>
      );
    },
    cell: ({ row }) =>
      new Date(row.getValue<number>("ended_at") * 1000).toLocaleString(),
  },
  {
    id: "actions",
    header: () => <div className={cn(["justify-self-center"])}>操作</div>,
    cell: function ActionsCell({ row }) {
      const id = row.original.id;
      const title = row.original.title;

      const sharedStore = useSharedStore();

      const [deleteDialogOpen, setDeleteDialogOpen] = useState<boolean>(false);

      function handleDelete() {
        deleteGame({
          id,
        })
          .then((res) => {
            if (res.code === StatusCodes.OK) {
              toast.success(`比赛 ${title} 删除成功`);
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
            <Link to={`/admin/games/${id}`} />
          </Button>
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
                  删除比赛
                </div>
                <p className={cn(["text-sm"])}>你确定要删除比赛 {title} 吗？</p>
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
