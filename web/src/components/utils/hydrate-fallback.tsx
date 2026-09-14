import { LoaderCircleIcon } from "lucide-react";
import { useTranslation } from "react-i18next";

import { cn } from "@/utils";

function HydrateFallback() {
  const { t } = useTranslation();

  return (
    <div
      className={cn([
        "absolute",
        "inset-0",
        "z-50",
        "flex",
        "h-full",
        "w-full",
        "flex-col",
        "items-center",
        "justify-center",
        "gap-3",
        "bg-background",
      ])}
    >
      <LoaderCircleIcon className={cn(["animate-spin", "size-10"])} />
      <span>{t("common:loading")}</span>
    </div>
  );
}

export { HydrateFallback };
