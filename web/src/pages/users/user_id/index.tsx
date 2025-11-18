import { UserRoundPenIcon } from "lucide-react";
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

  return (
    <>
      <title>{`${user?.name} - ${configStore?.config?.meta?.title}`}</title>
      <div className={cn(["p-12", "flex", "flex-col", "gap-5"])}>
        <div className={cn(["flex", "flex-col", "gap-3"])}>
          <div className={cn(["flex", "gap-2", "items-center"])}>
            <UserRoundPenIcon />
            <span>{t("user.description")}</span>
          </div>
          <Separator />
        </div>
        <Typography>
          <LoadingOverlay loading={!user} />
          <MarkdownRender src={user?.description} />
        </Typography>
      </div>
    </>
  );
}
