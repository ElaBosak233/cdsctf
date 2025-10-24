import { SnowflakeIcon } from "lucide-react";
import { useEffect, useState } from "react";
import { Badge } from "@/components/ui/badge";
import { cn } from "@/utils";

function FrozenBadge({ frozenAt }: { frozenAt: number }) {
  const [remaining, setRemaining] = useState(frozenAt * 1000 - Date.now());

  useEffect(() => {
    const timer = setInterval(() => {
      setRemaining(frozenAt * 1000 - Date.now());
    }, 1000);
    return () => clearInterval(timer);
  }, [frozenAt]);

  const formatRemaining = (ms: number) => {
    if (ms <= 0) return "已冻结";
    const totalSeconds = Math.floor(ms / 1000);
    const hours = Math.floor(totalSeconds / 3600);
    const minutes = Math.floor((totalSeconds % 3600) / 60);
    const seconds = totalSeconds % 60;
    return `距冻结 ${hours} 时 ${minutes} 分 ${seconds} 秒`;
  };

  return (
    <Badge className={cn(["flex", "items-center", "gap-1"])}>
      <SnowflakeIcon className="size-4" />
      <span>{formatRemaining(remaining)}</span>
    </Badge>
  );
}

export { FrozenBadge };
