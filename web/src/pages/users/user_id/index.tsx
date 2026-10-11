import { ThumbsDownIcon, UserRoundPenIcon } from "lucide-react";
import { useContext } from "react";
import { useTranslation } from "react-i18next";
import { LoadingOverlay } from "@/components/ui/loading-overlay";
import { MarkdownRender } from "@/components/ui/markdown-render";
import { Separator } from "@/components/ui/separator";
import { Typography } from "@/components/ui/typography";
import { useConfigStore } from "@/storages/config";
import { cn } from "@/utils";
import { Context } from "./context";

export default function Index() {
  const { t } = useTranslation();

  const configStore = useConfigStore();
  const { user } = useContext(Context);
  const description = user?.description?.trim();

  return (
    <>
      <title>{`${user?.name} - ${configStore?.config?.meta?.title}`}</title>
      <div
        className={cn([
          "p-4",
          "sm:p-8",
          "lg:p-12",
          "flex",
          "flex-col",
          "gap-5",
          "min-w-0",
        ])}
      >
        <div className={cn(["flex", "flex-col", "gap-3"])}>
          <div className={cn(["flex", "gap-2", "items-center"])}>
            <UserRoundPenIcon />
            <span>{t("user:description")}</span>
          </div>
          <Separator />
        </div>
        <div className={cn(["relative", "min-h-48", "flex-1"])}>
          <LoadingOverlay loading={!user} />
          {user && description ? (
            <Typography>
              <MarkdownRender src={user.description} />
            </Typography>
          ) : user ? (
            <div
              className={cn([
                "absolute",
                "inset-0",
                "flex",
                "flex-col",
                "items-center",
                "justify-center",
                "gap-5",
                "select-none",
                "text-center",
                "text-muted-foreground",
              ])}
            >
              <ThumbsDownIcon className={cn(["size-12"])} />
              <span>{t("user:description_empty")}</span>
            </div>
          ) : null}
        </div>
      </div>
    </>
  );
}
