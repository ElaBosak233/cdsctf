import { SnowflakeIcon } from "lucide-react";
import { useEffect, useState } from "react";
import { useTranslation } from "react-i18next";
import { Badge } from "@/components/ui/badge";
import { useTickerTime } from "@/hooks/use-ticker-time";
import { cn } from "@/utils";

function FrozenBadge({ frozenAt }: { frozenAt: number }) {
  const [remaining, setRemaining] = useState(frozenAt * 1000 - Date.now());
  const now = useTickerTime();
  const { t } = useTranslation();

  useEffect(() => {
    setRemaining(frozenAt * 1000 - now.getTime());
  }, [frozenAt, now]);

  const formatRemaining = (ms: number) => {
    if (ms <= 0) return t("challenge.frozen.already");
    const totalSeconds = Math.floor(ms / 1000);
    const hours = Math.floor(totalSeconds / 3600);
    const minutes = Math.floor((totalSeconds % 3600) / 60);
    const seconds = totalSeconds % 60;
    return t("challenge.frozen.countdown", { hours, minutes, seconds });
  };

  return (
    <Badge className={cn(["flex", "items-center", "gap-1"])}>
      <SnowflakeIcon className="size-4" />
      <span>{formatRemaining(remaining)}</span>
    </Badge>
  );
}

export { FrozenBadge };
