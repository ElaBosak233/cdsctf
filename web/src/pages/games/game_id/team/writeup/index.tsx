import { StatusCodes } from "http-status-codes";
import { FilePenIcon } from "lucide-react";
import { toast } from "sonner";
import { Card } from "@/components/ui/card";
import {
  DropZoneArea,
  Dropzone,
  DropzoneTrigger,
  useDropzone,
} from "@/components/ui/dropzone";
import { PDFViewer } from "@/components/ui/pdf-viewer";
import { Separator } from "@/components/ui/separator";
import { useGameStore } from "@/storages/game";
import { useSharedStore } from "@/storages/shared";
import { cn } from "@/utils";

export default function Index() {
  const sharedStore = useSharedStore();
  const { currentGame, selfTeam } = useGameStore();

  const dropzone = useDropzone({
    onDropFile: async (file) => {
      const formData = new FormData();
      formData.append("file", file);
      const xhr = new XMLHttpRequest();
      xhr.open(
        "POST",
        `/api/games/${currentGame?.id}/teams/profile/writeup`,
        true
      );
      xhr.upload.onprogress = (event) => {
        if (event.lengthComputable) {
          const percentComplete = (event.loaded / event.total) * 100;
          toast.loading(`上传进度 ${percentComplete}%`, {
            id: "writeup-upload",
          });
        }
      };
      xhr.onload = () => {
        if (xhr.status === StatusCodes.OK) {
          toast.success("题解上传成功", {
            id: "writeup-upload",
          });
          sharedStore.setRefresh();
        } else {
          toast.error("题解上传失败", {
            id: "writeup-upload",
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
    validation: {
      maxFiles: 1,
      maxSize: 50 * 1024 * 1024,
      accept: {
        "application/pdf": [".pdf"],
      },
    },
  });

  return (
    <>
      <title>{`Write-up - ${currentGame?.title}`}</title>
      <div
        className={cn([
          "flex",
          "flex-col",
          "flex-1",
          "p-10",
          "xl:mx-50",
          "lg:mx-30",
          "gap-5",
        ])}
      >
        <h1
          className={cn([
            "text-2xl",
            "font-bold",
            "flex",
            "gap-2",
            "items-center",
          ])}
        >
          <FilePenIcon />
          Write-up
        </h1>
        <Separator />

        <Dropzone {...dropzone}>
          <DropZoneArea>
            <DropzoneTrigger className="h-fit flex flex-col items-center gap-4 bg-transparent p-10 text-center text-sm">
              <p className="font-semibold">上传 Write-up</p>
              <p className="text-sm text-muted-foreground">
                请将 Write-up 导出为 PDF 格式后上传，文件大小不超过 50 MB。
              </p>
            </DropzoneTrigger>
          </DropZoneArea>
        </Dropzone>

        {selfTeam?.has_write_up && (
          <Card className={cn(["p-5", "rounded-xl"])}>
            <PDFViewer
              url={`/api/games/${currentGame?.id}/teams/profile/writeup`}
            />
          </Card>
        )}
      </div>
    </>
  );
}
