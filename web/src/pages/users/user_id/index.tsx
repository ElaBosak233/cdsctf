import { LoadingOverlay } from "@/components/ui/loading-overlay";
import { Separator } from "@/components/ui/separator";
import { Typography } from "@/components/ui/typography";
import { MarkdownRender } from "@/components/utils/markdown-render";
import { useConfigStore } from "@/storages/config";
import { cn } from "@/utils";
import { UserRoundPenIcon } from "lucide-react";
import { useContext } from "react";
import { Context } from "./context";

export default function Index() {
  const configStore = useConfigStore();
  const { user } = useContext(Context);

  return (
    <>
      <title>{`${user?.name} - ${configStore?.config?.meta?.title}`}</title>
      <div className={cn(["p-12", "flex", "flex-col", "gap-5"])}>
        <div className={cn(["flex", "flex-col", "gap-3"])}>
          <div className={cn(["flex", "gap-2", "items-center"])}>
            <UserRoundPenIcon />
            <span>个人简介</span>
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
