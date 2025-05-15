import { cn } from "@/utils";
import { LoaderCircleIcon } from "lucide-react";
import { useTranslation } from "react-i18next";

function HydrateFallback() {
  const { t } = useTranslation();

  return (
    <div
      className={cn([
        "h-screen",
        "flex",
        "flex-col",
        "justify-center",
        "items-center",
        "gap-3",
      ])}
    >
      <LoaderCircleIcon className={cn(["animate-spin", "size-10"])} />
      <span>{t("loading")}</span>
    </div>
  );
}

export { HydrateFallback };
