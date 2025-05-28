import { ColumnDef } from "@tanstack/react-table";
import { StatusCodes } from "http-status-codes";
import { TrashIcon } from "lucide-react";
import { useContext, useState } from "react";
import { toast } from "sonner";

import { deleteChallengeAttachment } from "@/api/admin/challenges/challenge_id/attachments/filename";
import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";
import { Dialog, DialogContent } from "@/components/ui/dialog";
import { Metadata } from "@/models/media";
import { useSharedStore } from "@/storages/shared";
import { cn } from "@/utils";
import { Context } from "../context";

const columns: Array<ColumnDef<Metadata>> = [
  {
    accessorKey: "filename",
    id: "filename",
    header: () => "文件名",
    cell: ({ row }) => row.original.filename,
  },
  {
    accessorKey: "size",
    header: () => "占用空间（Byte）",
    cell: ({ row }) => row.original.size,
  },
  {
    id: "actions",
    header: () => <div className={cn(["justify-self-center"])}>操作</div>,
    cell: function ActionsCell({ row }) {
      const sharedStore = useSharedStore();
      const { challenge } = useContext(Context);

      const [deleteDialogOpen, setDeleteDialogOpen] = useState<boolean>(false);

      function handleDelete() {
        deleteChallengeAttachment(challenge?.id, row.original.filename)
          .then((res) => {
            if (res.code === StatusCodes.OK) {
              toast.success(`附件 ${row.original.filename} 删除成功`);
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
                  删除附件
                </div>
                <p className={cn(["text-sm"])}>
                  你确定要删除附件 {row.original.filename} 吗？
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
