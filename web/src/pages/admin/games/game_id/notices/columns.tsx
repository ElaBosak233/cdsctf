import { ColumnDef } from "@tanstack/react-table";
import { StatusCodes } from "http-status-codes";
import { ClipboardCheckIcon, ClipboardCopyIcon, TrashIcon } from "lucide-react";
import { useState } from "react";
import { toast } from "sonner";

import { deleteGameNotice } from "@/api/admin/games/game_id/notices";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";
import { Dialog, DialogContent } from "@/components/ui/dialog";
import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from "@/components/ui/tooltip";
import { ContentDialog } from "@/components/widgets/content-dialog";
import { useClipboard } from "@/hooks/use-clipboard";
import { GameNotice } from "@/models/game_notice";
import { useSharedStore } from "@/storages/shared";
import { cn } from "@/utils";

const columns: Array<ColumnDef<GameNotice>> = [
  {
    accessorKey: "id",
    id: "id",
    header: "ID",
    cell: function IdCell({ row }) {
      const id = row.original.id;
      const { isCopied, copyToClipboard } = useClipboard();
      return (
        <div className={cn(["flex", "items-center", "gap-1"])}>
          <Badge>{id}</Badge>
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
    cell: ({ row }) => row.original.title,
  },
  {
    accessorKey: "content",
    header: "详情",
    cell: ({ row }) => {
      const content = row.original.content;

      if (!content) return "-";

      return content.length > 10 ? (
        <ContentDialog title="详细描述" content={content} />
      ) : (
        content
      );
    },
  },
  {
    accessorKey: "created_at",
    id: "created_at",
    header: "发布时间",
    cell: ({ row }) => {
      return new Date(
        row.getValue<number>("created_at") * 1000
      ).toLocaleString();
    },
  },
  {
    id: "actions",
    header: () => <div className={cn(["justify-self-center"])}>操作</div>,
    cell: function ActionsCell({ row }) {
      const sharedStore = useSharedStore();

      const [deleteDialogOpen, setDeleteDialogOpen] = useState<boolean>(false);

      function handleDelete() {
        deleteGameNotice({
          game_id: row.original.game_id,
          id: row.original.id,
        })
          .then((res) => {
            if (res.code === StatusCodes.OK) {
              toast.success(`赛题 ${row.original.title} 删除成功`);
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
                  删除通知
                </div>
                <p className={cn(["text-sm"])}>
                  你确定要删除通知 {row.original.title} 吗？
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
