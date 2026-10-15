import { InfoIcon, XIcon } from "lucide-react";
import { useState } from "react";
import { useTranslation } from "react-i18next";
import { Button } from "@/components/ui/button";
import { useConfigStore } from "@/storages/config";
import { cn } from "@/utils";

function HighlightBanner() {
  const { t } = useTranslation();
  const { config } = useConfigStore();
  const message = config?.meta?.highlight_banner?.trim() ?? "";
  const [dismissedMessage, setDismissedMessage] = useState<string | null>(null);
  const visible = message.length > 0 && dismissedMessage !== message;

  if (!visible) return null;

  function dismiss() {
    setDismissedMessage(message);
  }

  return (
    <div
      role="status"
      className={cn([
        "fixed",
        "inset-x-0",
        "top-16",
        "z-1",
        "flex",
        "min-h-12",
        "w-full",
        "items-center",
        "gap-3",
        "border-b",
        "border-info/25",
        "bg-info/15",
        "px-4",
        "py-2",
        "text-sm",
        "font-medium",
        "text-foreground",
        "backdrop-blur-xs",
        "sm:px-6",
      ])}
    >
      <InfoIcon className={cn(["size-4", "shrink-0", "text-info"])} />
      <p
        className={cn([
          "min-w-0",
          "flex-1",
          "whitespace-pre-wrap",
          "break-words",
        ])}
      >
        {message}
      </p>
      <Button
        type="button"
        variant="ghost"
        size="sm"
        square
        level="info"
        icon={<XIcon />}
        aria-label={t("common:actions.close")}
        onClick={dismiss}
        className={cn(["shrink-0"])}
      />
    </div>
  );
}

export { HighlightBanner };
