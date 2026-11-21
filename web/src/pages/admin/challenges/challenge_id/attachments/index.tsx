import { useQuery } from "@tanstack/react-query";
import {
  CircleAlertIcon,
  FileIcon,
  FolderOpenIcon,
  RotateCwIcon,
  UploadIcon,
} from "lucide-react";
import prettyBytes from "pretty-bytes";
import { useContext, useId } from "react";
import { useTranslation } from "react-i18next";
import { toast } from "sonner";
import { getChallengeAttachments } from "@/api/management/challenges/challenge_id/attachments";
import { Button } from "@/components/ui/button";
import { DropZoneArea, Dropzone, useDropzone } from "@/components/ui/dropzone";
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
import { useSharedStore } from "@/storages/shared";
import { cn } from "@/utils";
import { uploadFile } from "@/utils/file";
import { notifyApiError } from "@/utils/query";
import { Context } from "../context";
import { AttachmentActions } from "./attachment-actions";

const MAX_ATTACHMENT_SIZE = 200 * 1024 * 1024;
const ATTACHMENT_EXTENSION =
  /^.+?\.(tar\.(?:gz|bz2|xz|zst|lz4|lzma|lz|z)|[a-z0-9]+)$/i;

export default function Index() {
  const { t } = useTranslation();

  const { challenge } = useContext(Context);
  const challengeId = challenge?.id;
  const sharedStore = useSharedStore();
  const headingId = useId();
  const uploadHintId = useId();

  const {
    data: attachments,
    isPending,
    isError,
    isFetching,
    refetch,
  } = useQuery({
    queryKey: [
      "admin",
      "challenge-attachments",
      challengeId,
      sharedStore.refresh,
    ],
    queryFn: () => getChallengeAttachments(challengeId!),
    select: (response) => response.attachments ?? [],
    enabled: challengeId != null,
  });

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
          `/api/management/challenges/${challengeId}/attachments`,
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
      } catch (error) {
        await notifyApiError(error, {
          id: "attachment-upload",
          title: t("challenge:attachment.upload.error"),
        });
        return {
          status: "error",
          error: t("challenge:attachment.upload.error"),
        };
      }
    },
    validation: {
      maxSize: MAX_ATTACHMENT_SIZE,
    },
  });

  const isEmpty = attachments?.length === 0 && !isError;
  const sizeLimit = prettyBytes(MAX_ATTACHMENT_SIZE, {
    binary: true,
    nonBreakingSpace: true,
  });
  const uploadHint = t("challenge:attachment.upload.hint", {
    limit: sizeLimit,
  });

  return (
    <section
      aria-labelledby={headingId}
      className="flex min-h-0 min-w-0 flex-1 flex-col gap-4"
    >
      <div className="flex shrink-0 flex-wrap items-baseline gap-x-3 gap-y-1">
        <h1 id={headingId} className="text-xl font-semibold">
          {t("challenge:edit.attachment")}
        </h1>
        {attachments && (
          <span className="text-sm text-muted-foreground tabular-nums">
            {t("challenge:attachment.count", { count: attachments.length })}
          </span>
        )}
      </div>

      {challengeId != null && (
        <Dropzone {...dropzone}>
          <DropZoneArea
            data-drag-active={dropzone.isDragActive}
            className={cn(
              "animate-none flex-col gap-3 rounded-lg border-dashed bg-muted/30 transition-colors motion-reduce:transition-none data-[drag-active=true]:border-primary data-[drag-active=true]:bg-primary/10",
              !dropzone.isInvalid && "border-border",
              isEmpty
                ? "flex-1 px-4 py-8 text-center sm:py-12"
                : "shrink-0 items-start px-3 py-3 sm:flex-row sm:items-center sm:justify-between sm:px-4"
            )}
          >
            <div
              className={cn(
                "flex min-w-0 gap-3",
                isEmpty ? "flex-col items-center" : "items-center"
              )}
            >
              <span className="flex size-10 shrink-0 items-center justify-center rounded-badge bg-primary/10 text-primary">
                {isEmpty ? (
                  <FolderOpenIcon className="size-5" aria-hidden="true" />
                ) : (
                  <UploadIcon className="size-4" aria-hidden="true" />
                )}
              </span>
              <div className="min-w-0">
                {isEmpty && (
                  <p className="text-sm font-semibold">
                    {t("challenge:attachment.empty")}
                  </p>
                )}
                <p id={uploadHintId} className="text-sm text-muted-foreground">
                  {uploadHint}
                </p>
              </div>
            </div>
            <input {...dropzone.getInputProps()} />
            <Button
              size="sm"
              variant="solid"
              aria-describedby={uploadHintId}
              onClick={dropzone.open}
            >
              {t("challenge:attachment.upload._")}
            </Button>
          </DropZoneArea>
          {dropzone.rootError && (
            <p
              id={dropzone.rootMessageId}
              role="alert"
              className="shrink-0 text-sm text-error"
            >
              {t("challenge:attachment.upload.too_large", { limit: sizeLimit })}
            </p>
          )}
        </Dropzone>
      )}

      {isPending && (
        <div
          aria-busy="true"
          className="relative min-h-0 flex-1 rounded-lg border"
        >
          <LoadingOverlay loading />
        </div>
      )}
      {isError && (
        <div className="flex shrink-0 flex-wrap items-center justify-between gap-3 rounded-lg border border-error/20 bg-error/5 p-3">
          <p
            role="alert"
            className="flex items-center gap-2 text-sm text-error"
          >
            <CircleAlertIcon className="size-4 shrink-0" aria-hidden="true" />
            {t("challenge:attachment.load_error")}
          </p>
          <Button
            size="sm"
            variant="outline"
            icon={<RotateCwIcon />}
            loading={isFetching}
            onClick={() => void refetch()}
          >
            {t("challenge:attachment.retry")}
          </Button>
        </div>
      )}
      {!!attachments?.length && (
        <ScrollArea className="min-h-0 flex-1 rounded-lg border bg-card">
          <Table aria-labelledby={headingId} className="table-fixed">
            <TableHeader className="sticky top-0 z-2 bg-muted/95 backdrop-blur-sm">
              <TableRow className="hover:bg-transparent">
                <TableHead scope="col" className="h-10 px-3 sm:px-4">
                  {t("challenge:attachment.filename")}
                </TableHead>
                <TableHead
                  scope="col"
                  className="hidden h-10 w-28 px-3 sm:table-cell"
                >
                  {t("challenge:attachment.file_type")}
                </TableHead>
                <TableHead
                  scope="col"
                  className="hidden h-10 w-28 px-3 text-right sm:table-cell"
                >
                  {t("challenge:attachment.size")}
                </TableHead>
                <TableHead
                  scope="col"
                  className="h-10 w-16 px-2 text-center sm:w-24"
                >
                  {t("challenge:attachment.actions._")}
                </TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              {attachments.map(({ filename, size }) => {
                const sizeLabel = prettyBytes(size);
                const extension = ATTACHMENT_EXTENSION.exec(filename)?.[1];
                const fileType =
                  extension?.toUpperCase() ??
                  t("challenge:attachment.unknown_type");

                return (
                  <TableRow key={filename}>
                    <TableCell className="px-3 py-2.5 sm:px-4">
                      <div className="flex items-center gap-3">
                        <span className="flex size-8 shrink-0 items-center justify-center rounded-md bg-muted text-muted-foreground">
                          <FileIcon className="size-4" aria-hidden="true" />
                        </span>
                        <div className="min-w-0">
                          <p className="wrap-anywhere font-medium">
                            {filename}
                          </p>
                          <dl className="mt-0.5 flex flex-wrap gap-x-3 gap-y-0.5 font-mono text-xs text-muted-foreground sm:hidden">
                            <div className="min-w-0">
                              <dt className="sr-only">
                                {t("challenge:attachment.file_type")}
                              </dt>
                              <dd className="wrap-anywhere">{fileType}</dd>
                            </div>
                            <div>
                              <dt className="sr-only">
                                {t("challenge:attachment.size")}
                              </dt>
                              <dd>{sizeLabel}</dd>
                            </div>
                          </dl>
                        </div>
                      </div>
                    </TableCell>
                    <TableCell className="hidden wrap-anywhere px-3 py-2.5 font-mono text-xs text-muted-foreground sm:table-cell">
                      {fileType}
                    </TableCell>
                    <TableCell className="hidden px-3 py-2.5 text-right font-mono text-xs text-muted-foreground sm:table-cell">
                      {sizeLabel}
                    </TableCell>
                    <TableCell className="px-2 py-2 text-center">
                      <AttachmentActions filename={filename} />
                    </TableCell>
                  </TableRow>
                );
              })}
            </TableBody>
          </Table>
        </ScrollArea>
      )}
    </section>
  );
}
