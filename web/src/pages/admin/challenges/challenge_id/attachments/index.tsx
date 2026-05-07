import {
  flexRender,
  getCoreRowModel,
  getFilteredRowModel,
  useReactTable,
} from "@tanstack/react-table";
import { CloudUploadIcon } from "lucide-react";
import { useContext, useEffect, useState } from "react";
import { useTranslation } from "react-i18next";
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
import { uploadFile } from "@/utils/file";
import { Context } from "../context";
import { useColumns } from "./columns";

export default function Index() {
  const { t } = useTranslation();

  const { challenge } = useContext(Context);
  const sharedStore = useSharedStore();

  const [metadata, setMetadata] = useState<Array<Metadata>>([]);
  const [loading, setLoading] = useState<boolean>(false);

  useEffect(() => {
    void sharedStore.refresh;

    if (!challenge?.id) return;
    setLoading(true);
    getChallengeAttachments(challenge.id!)
      .then((res) => {
        setMetadata(res.attachments || []);
      })
      .finally(() => {
        setLoading(false);
      });
  }, [challenge?.id, sharedStore.refresh]);

  const dropzone = useDropzone({
    onDropFile: async (file) => {
      toast.loading(
        t("challenge:attachment.upload.progress", {
          percent: "0",
        }),
        { id: "attachment-upload" }
      );

      try {
        await uploadFile(
          `/api/admin/challenges/${challenge?.id}/attachments`,
          [file],
          ({ percent }) => {
            toast.loading(
              t("challenge:attachment.upload.progress", {
                percent: Math.round(percent).toString(),
              }),
              { id: "attachment-upload" }
            );
          }
        );
        toast.success(t("challenge:attachment.upload.success"), {
          id: "attachment-upload",
        });
        sharedStore.setRefresh();
        return {
          status: "success",
          result: "",
        };
      } catch {
        toast.error(t("challenge:attachment.upload.error"), {
          id: "attachment-upload",
        });
        return {
          status: "error",
        };
      }
    },
  });

  const columns = useColumns();
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
    <div className={cn(["flex", "flex-1", "min-h-0", "flex-col", "gap-5"])}>
      <Dropzone {...dropzone}>
        <DropZoneArea>
          <DropzoneTrigger className="h-fit flex flex-col items-center gap-4 bg-transparent p-10 text-center text-sm">
            <CloudUploadIcon className="size-16" />
            <p className="font-semibold">
              {t("challenge:attachment.upload._")}
            </p>
            <p className="text-sm text-muted-foreground">
              {t("challenge:attachment.upload.hint")}
            </p>
          </DropzoneTrigger>
        </DropZoneArea>
      </Dropzone>
      <div className={cn(["flex-1", "min-h-0", "flex", "flex-col"])}>
        <ScrollArea
          className={cn([
            "rounded-md",
            "border",
            "bg-card",
            "h-full",
            "min-h-0",
            "overflow-hidden",
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
                        {t("challenge:attachment.empty")}
                      </TableCell>
                    </TableRow>
                  )}
            </TableBody>
          </Table>
        </ScrollArea>
      </div>
    </div>
  );
}
