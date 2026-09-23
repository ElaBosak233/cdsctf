import { SnowflakeIcon } from "lucide-react";
import { useEffect, useState } from "react";
import { useTranslation } from "react-i18next";
import { Badge } from "@/components/ui/badge";
import { useTickerTime } from "@/hooks/use-ticker-time";
import type { Timestamp } from "@/types";
import { cn } from "@/utils";
import { secondsUntil } from "@/utils/time";

function FrozenBadge({ frozenAt }: { frozenAt: Timestamp }) {
  const [remaining, setRemaining] = useState(() => secondsUntil(frozenAt) ?? 0);
  const now = useTickerTime();
  const { t } = useTranslation();

  useEffect(() => {
    setRemaining(secondsUntil(frozenAt, now) ?? 0);
  }, [frozenAt, now]);

  const formatRemaining = (ms: number) => {
    if (ms <= 0) return t("challenge:frozen.already");
    const totalSeconds = ms;
    const hours = Math.floor(totalSeconds / 3600);
    const minutes = Math.floor((totalSeconds % 3600) / 60);
    const seconds = totalSeconds % 60;
    return t("challenge:frozen.countdown", { hours, minutes, seconds });
  };

  return (
    <Badge className={cn(["flex", "items-center", "gap-1"])}>
      <SnowflakeIcon className="size-4" />
      <span>{formatRemaining(remaining)}</span>
    </Badge>
  );
}

export { FrozenBadge };
