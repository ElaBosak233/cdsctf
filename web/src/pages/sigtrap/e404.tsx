import { CircleOff } from "lucide-react";
import { useTranslation } from "react-i18next";

import { Separator } from "@/components/ui/separator";
import { useConfigStore } from "@/storages/config";
import { cn } from "@/utils";

export default function E404() {
  const configStore = useConfigStore();
  const { t } = useTranslation("sigtrap");

  return (
    <>
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
        <title>{`404 - ${configStore?.config?.meta?.title}`}</title>
        <CircleOff className={cn(["size-32"])} strokeWidth={1.2} />
        <div className={cn(["flex", "gap-2", "text-xl"])}>
          <span>404</span>
          <Separator orientation={"vertical"} />
          <span>{t("404.title")}</span>
        </div>
        <span className={cn(["text-secondary-foreground"])}>
          {t("404.description")}
        </span>
      </div>
    </>
  );
}
