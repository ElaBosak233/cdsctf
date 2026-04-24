import { StatusCodes } from "http-status-codes";
import {
  CheckCircleIcon,
  ClockIcon,
  DownloadIcon,
  FilePenIcon,
} from "lucide-react";
import { useTranslation } from "react-i18next";
import { toast } from "sonner";
import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";
import {
  DropZoneArea,
  Dropzone,
  DropzoneTrigger,
  useDropzone,
} from "@/components/ui/dropzone";
import { Separator } from "@/components/ui/separator";
import { useGameStore } from "@/storages/game";
import { useSharedStore } from "@/storages/shared";
import { cn } from "@/utils";

export default function Index() {
  const { t } = useTranslation();

  const sharedStore = useSharedStore();
  const { currentGame, selfTeam, setSelfTeam } = useGameStore();
  const hasWriteup = selfTeam?.has_writeup === true;
  const writeupUrl = `/api/games/${currentGame?.id}/teams/us/writeup`;

  const dropzone = useDropzone({
    onDropFile: async (file) => {
      const formData = new FormData();
      formData.append("file", file);
      const xhr = new XMLHttpRequest();
      xhr.open("POST", `/api/games/${currentGame?.id}/teams/us/writeup`, true);
      xhr.upload.onprogress = (event) => {
        if (event.lengthComputable) {
          const percentComplete = (event.loaded / event.total) * 100;
          toast.loading(
            t("team:write_up.actions.upload.progress", {
              percent: Math.round(percentComplete),
            }),
            {
              id: "writeup-upload",
            }
          );
        }
      };
      xhr.onload = () => {
        if (xhr.status === StatusCodes.OK) {
          try {
            const body = JSON.parse(xhr.responseText || "{}") as {
              team?: typeof selfTeam;
            };
            if (body.team) {
              setSelfTeam(body.team);
            }
          } catch {
            sharedStore.setRefresh();
          }
          toast.success(t("team:write_up.actions.upload.success"), {
            id: "writeup-upload",
          });
          sharedStore.setRefresh();
        } else {
          toast.error(t("team:write_up.actions.upload.error"), {
            id: "writeup-upload",
            description: xhr.responseText,
          });
        }
      };
      xhr.onerror = () => {
        toast.error(t("team:write_up.actions.upload.error"), {
          id: "writeup-upload",
        });
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
          "min-h-0",
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
          {t("team:write_up._")}
        </h1>
        <Separator />

        <Dropzone {...dropzone}>
          <DropZoneArea>
            <DropzoneTrigger className="h-fit flex flex-col items-center gap-4 bg-transparent p-10 text-center text-sm">
              <p className="font-semibold">
                {t("team:write_up.actions.upload._")}
              </p>
              <p className="text-sm text-muted-foreground">
                {t("team:write_up.actions.upload.hint")}
              </p>
            </DropzoneTrigger>
          </DropZoneArea>
        </Dropzone>

        <Card
          className={cn([
            "p-5",
            "rounded-xl",
            "flex",
            "flex-col",
            "gap-4",
            "sm:flex-row",
            "sm:items-center",
            "sm:justify-between",
          ])}
        >
          <div className={cn(["flex", "items-center", "gap-3", "min-w-0"])}>
            {hasWriteup ? (
              <CheckCircleIcon className={cn(["size-4", "text-success"])} />
            ) : (
              <ClockIcon className={cn(["size-4", "text-muted-foreground"])} />
            )}
            <div className={cn(["min-w-0"])}>
              <p className={cn(["font-medium"])}>
                {hasWriteup
                  ? t("team:write_up.actions.submit.done")
                  : t("team:write_up.actions.submit._")}
              </p>
            </div>
          </div>
          <Button
            variant={"tonal"}
            icon={<DownloadIcon />}
            disabled={!hasWriteup}
            asChild={hasWriteup}
          >
            <a href={writeupUrl} download>
              {t("team:write_up.actions.download._")}
            </a>
          </Button>
        </Card>
      </div>
    </>
  );
}
