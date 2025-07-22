import {
  flexRender,
  getCoreRowModel,
  getFilteredRowModel,
  useReactTable,
} from "@tanstack/react-table";
import { StatusCodes } from "http-status-codes";
import { CloudUploadIcon } from "lucide-react";
import { useContext, useEffect, useState } from "react";
import { toast } from "sonner";

import { getChallengeAttachments } from "@/api/admin/challenges/challenge_id/attachments";
import {
  DropZoneArea,
  Dropzone,
  DropzoneTrigger,
  useDropzone,
} from "@/components/ui/dropzone";
import { LoadingOverlay } from "@/components/ui/loading-overlay";
import { ScrollArea } from "@/components/ui/scroll-area";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import type { Metadata } from "@/models/media";
import { useSharedStore } from "@/storages/shared";
import { cn } from "@/utils";
import { Context } from "../context";
import { columns } from "./columns";

export default function Index() {
  const { challenge } = useContext(Context);
  const sharedStore = useSharedStore();

  const [metadata, setMetadata] = useState<Array<Metadata>>([]);
  const [loading, setLoading] = useState<boolean>(false);

  useEffect(() => {
    if (!challenge?.id) return;
    setLoading(true);
    getChallengeAttachments(challenge.id!)
      .then((res) => {
        setMetadata(res.data || []);
      })
      .finally(() => {
        setLoading(false);
      });
  }, [challenge?.id, sharedStore.refresh]);

  const dropzone = useDropzone({
    onDropFile: async (file) => {
      const formData = new FormData();
      formData.append("file", file);
      const xhr = new XMLHttpRequest();
      xhr.open(
        "POST",
        `/api/admin/challenges/${challenge?.id}/attachments`,
        true
      );
      xhr.upload.onprogress = (event) => {
        if (event.lengthComputable) {
          const percentComplete = (event.loaded / event.total) * 100;
          toast.loading(`上传进度 ${percentComplete}%`, {
            id: "attachment-upload",
          });
        }
      };
      xhr.onload = () => {
        if (xhr.status === StatusCodes.OK) {
          toast.success("文件上传成功", {
            id: "attachment-upload",
          });
          sharedStore.setRefresh();
        } else {
          toast.error("文件上传失败", {
            id: "attachment-upload",
            description: xhr.responseText,
          });
        }
      };
      xhr.onerror = () => {
        return {
          status: "error",
        };
      };

      xhr.send(formData);

      return {
        status: "success",
        result: "",
      };
    },
  });

  const table = useReactTable<Metadata>({
    data: metadata,
    columns,
    getCoreRowModel: getCoreRowModel(),
    manualPagination: true,
    manualFiltering: true,
    getFilteredRowModel: getFilteredRowModel(),
    manualSorting: true,
  });

  return (
    <div className={cn(["flex", "flex-col", "gap-5"])}>
      <Dropzone {...dropzone}>
        <DropZoneArea>
          <DropzoneTrigger className="h-fit flex flex-col items-center gap-4 bg-transparent p-10 text-center text-sm">
            <CloudUploadIcon className="size-16" />
            <p className="font-semibold">上传附件</p>
            <p className="text-sm text-muted-foreground">
              附件将直接由服务器托管，建议充分考虑存储空间、流量等因素。
            </p>
          </DropzoneTrigger>
        </DropZoneArea>
      </Dropzone>
      <ScrollArea
        className={cn([
          "rounded-md",
          "border",
          "bg-card",
          "min-h-100",
          "h-[calc(100vh-22rem)]",
        ])}
      >
        <LoadingOverlay loading={loading} />
        <Table className={cn(["text-foreground"])}>
          <TableHeader
            className={cn([
              "sticky",
              "top-0",
              "z-2",
              "bg-muted/70",
              "backdrop-blur-md",
            ])}
          >
            {table.getHeaderGroups().map((headerGroup) => (
              <TableRow key={headerGroup.id}>
                {headerGroup.headers.map((header) => {
                  return (
                    <TableHead key={header.id}>
                      {!header.isPlaceholder &&
                        flexRender(
                          header.column.columnDef.header,
                          header.getContext()
                        )}
                    </TableHead>
                  );
                })}
              </TableRow>
            ))}
          </TableHeader>
          <TableBody>
            {table.getRowModel().rows?.length
              ? table.getRowModel().rows.map((row) => (
                  <TableRow
                    key={row.original.filename}
                    data-state={row.getIsSelected() && "selected"}
                  >
                    {row.getVisibleCells().map((cell) => (
                      <TableCell key={cell.id}>
                        {flexRender(
                          cell.column.columnDef.cell,
                          cell.getContext()
                        )}
                      </TableCell>
                    ))}
                  </TableRow>
                ))
              : !loading && (
                  <TableRow>
                    <TableCell
                      colSpan={columns.length}
                      className={cn(["h-24", "text-center"])}
                    >
                      哎呀，好像还没有附件呢。
                    </TableCell>
                  </TableRow>
                )}
          </TableBody>
        </Table>
      </ScrollArea>
    </div>
  );
}
