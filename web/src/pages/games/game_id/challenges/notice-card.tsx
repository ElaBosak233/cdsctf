import { useQuery } from "@tanstack/react-query";
import { RssIcon } from "lucide-react";
import { getGameNotice } from "@/api/games/game_id/notices";
import { Card } from "@/components/ui/card";
import { Dialog, DialogContent, DialogTrigger } from "@/components/ui/dialog";
import { MarkdownRender } from "@/components/ui/markdown-render";
import { ScrollArea } from "@/components/ui/scroll-area";
import { Separator } from "@/components/ui/separator";
import { Typography } from "@/components/ui/typography";
import { useGameStore } from "@/storages/game";
import { cn } from "@/utils";

function NoticeCard() {
  const { currentGame } = useGameStore();
  const { data: gameNotices } = useQuery({
    queryKey: ["game_notices", currentGame?.id],
    queryFn: () =>
      getGameNotice({
        game_id: currentGame?.id,
      }),
    select: (response) => response.data,
    refetchInterval: 15000,
  });

  return (
    <Card
      className={cn([
        "p-5",
        "flex",
        "flex-col",
        "flex-1",
        "gap-5",
        "overflow-auto",
      ])}
    >
      <div className={cn(["flex", "gap-3", "items-center", "select-none"])}>
        <RssIcon className={cn(["size-4"])} />
        <h3 className={cn(["text-sm"])}>公告栏</h3>
      </div>
      <ScrollArea>
        <div
          className={cn([
            "overflow-auto",
            "flex-1",
            "flex",
            "flex-col",
            "gap-3",
          ])}
        >
          {gameNotices?.map((gameNotice) => (
            <Dialog key={gameNotice?.id}>
              <DialogTrigger>
                <Card
                  className={cn([
                    "flex",
                    "flex-col",
                    "gap-1",
                    "p-3",
                    "cursor-pointer",
                    "hover:bg-foreground/10",
                    "select-none",
                  ])}
                >
                  <h4 className={cn(["text-sm"])}>{gameNotice?.title}</h4>
                  <span
                    className={cn(["text-secondary-foreground", "text-xs"])}
                  >
                    {new Date(
                      Number(gameNotice?.created_at) * 1000
                    ).toLocaleString()}
                  </span>
                </Card>
              </DialogTrigger>
              <DialogContent>
                <Card
                  className={cn([
                    "p-6",
                    "min-h-81",
                    "w-screen",
                    "md:w-xl",
                    "flex",
                    "flex-col",
                    "gap-5",
                  ])}
                >
                  <div className={cn("flex", "flex-col", "gap-3")}>
                    <div
                      className={cn([
                        "flex",
                        "justify-between",
                        "items-baseline",
                      ])}
                    >
                      <div className={cn(["flex", "gap-3", "items-center"])}>
                        <RssIcon className={cn(["size-5"])} />
                        <h3>{gameNotice?.title}</h3>
                      </div>
                      <span
                        className={cn(["text-secondary-foreground", "text-xs"])}
                      >
                        {new Date(
                          Number(gameNotice?.created_at) * 1000
                        ).toLocaleString()}
                      </span>
                    </div>
                    <Separator />
                  </div>
                  <div
                    className={cn([
                      "flex",
                      "flex-1",
                      "flex-col",
                      "max-h-144",
                      "overflow-auto",
                    ])}
                  >
                    <Typography>
                      <MarkdownRender src={gameNotice?.content} />
                    </Typography>
                  </div>
                </Card>
              </DialogContent>
            </Dialog>
          ))}
        </div>
      </ScrollArea>
    </Card>
  );
}

export { NoticeCard };
