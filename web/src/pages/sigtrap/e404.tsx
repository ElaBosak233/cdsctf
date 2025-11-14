import { MessageCircleDashedIcon } from "lucide-react";
import { useTranslation } from "react-i18next";

import { Separator } from "@/components/ui/separator";
import { useConfigStore } from "@/storages/config";
import { cn } from "@/utils";

export default function E404() {
  const { config } = useConfigStore();
  const { t } = useTranslation();

  return (
    <>
      <title>{`404 - ${config?.meta?.title}`}</title>
      <div
        className={cn([
          "flex-1",
          "flex",
          "flex-col",
          "items-center",
          "justify-center",
          "gap-7",
          "text-foreground",
        ])}
      >
        <title>{`404 - ${config?.meta?.title}`}</title>
        <MessageCircleDashedIcon
          className={cn(["size-32"])}
          strokeWidth={1.2}
        />
        <div className={cn(["flex", "gap-2", "text-xl"])}>
          <span>404</span>
          <Separator orientation={"vertical"} />
          <span>{t("sigtrap.404.title")}</span>
        </div>
        <span className={cn(["text-secondary-foreground"])}>
          {t("sigtrap.404.description")}
        </span>
      </div>
    </>
  );
}
