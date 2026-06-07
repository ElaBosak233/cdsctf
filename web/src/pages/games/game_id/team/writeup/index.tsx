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
import { uploadFile } from "@/utils/file";

export default function Index() {
  const { t } = useTranslation();

  const sharedStore = useSharedStore();
  const { currentGame, selfTeam, setSelfTeam } = useGameStore();
  const hasWriteup = selfTeam?.has_writeup === true;
  const writeupUrl = `/api/games/${currentGame?.id}/teams/us/writeup`;

  const dropzone = useDropzone({
    onDropFile: async (file) => {
      toast.loading(
        t("team:write_up.actions.upload.progress", {
          percent: "0",
        }),
        { id: "writeup-upload" }
      );

      try {
        const res = await uploadFile(
          `/api/games/${currentGame?.id}/teams/us/writeup`,
          [file],
          ({ percent }) => {
            toast.loading(
              t("team:write_up.actions.upload.progress", {
                percent: Math.round(percent).toString(),
              }),
              { id: "writeup-upload" }
            );
          }
        );
        const body = res as { team?: typeof selfTeam } | undefined;
        if (body?.team) {
          setSelfTeam(body.team);
        }
        toast.success(t("team:write_up.actions.upload.success"), {
          id: "writeup-upload",
        });
        sharedStore.setRefresh();
        return {
          status: "success",
          result: "",
        };
      } catch {
        toast.error(t("team:write_up.actions.upload.error"), {
          id: "writeup-upload",
        });
        return {
          status: "error",
        };
      }
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
      <title>{`${t("team:write_up._")} - ${currentGame?.title}`}</title>
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
        <div className={cn(["flex", "items-start", "gap-3.5"])}>
          <div
            className={cn([
              "flex items-center justify-center",
              "size-10 rounded-badge",
              "bg-primary/10",
              "shrink-0",
            ])}
          >
            <FilePenIcon className={cn(["size-5"])} />
          </div>
          <div className={cn(["flex flex-col gap-1", "pt-0.5"])}>
            <h2 className={cn(["text-sm", "font-semibold", "text-foreground"])}>
              {t("team:write_up._")}
            </h2>
            <p
              className={cn([
                "text-xs",
                "text-muted-foreground/80",
                "leading-relaxed",
              ])}
            >
              {t("team:write_up.actions.upload.hint")}
            </p>
          </div>
        </div>
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
            "rounded-elevated",
            "shadow-lg",
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
