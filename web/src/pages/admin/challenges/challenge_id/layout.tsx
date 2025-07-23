import {
  ChartAreaIcon,
  ContainerIcon,
  FolderIcon,
  InfoIcon,
  LibraryIcon,
  PlayIcon,
  ScrollTextIcon,
} from "lucide-react";
import { useEffect, useMemo, useState } from "react";
import { Link, Outlet, useLocation, useParams } from "react-router";
import { getChallenge } from "@/api/admin/challenges/challenge_id";
import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";
import { Dialog, DialogContent, DialogTrigger } from "@/components/ui/dialog";
import { ChallengeCard } from "@/components/widgets/challenge-card";
import { ChallengeDialog } from "@/components/widgets/challenge-dialog";
import type { Challenge } from "@/models/challenge";
import { useSharedStore } from "@/storages/shared";
import { cn } from "@/utils";
import { Context } from "./context";

export default function Layout() {
  const location = useLocation();
  const pathname = location.pathname;
  const sharedStore = useSharedStore();
  const { challenge_id } = useParams<{ challenge_id: string }>();
  const [challenge, setChallenge] = useState<Challenge>();

  useEffect(() => {
    void sharedStore.refresh;

    getChallenge({
      id: challenge_id,
    }).then((res) => {
      setChallenge(res.data);
    });
  }, [challenge_id, sharedStore?.refresh]);

  const options = useMemo(() => {
    return [
      {
        link: `/admin/challenges/${challenge_id}`,
        name: "基本信息",
        icon: <InfoIcon />,
      },
      {
        link: `/admin/challenges/${challenge_id}/checker`,
        name: "检查器",
        icon: <ScrollTextIcon />,
      },
      {
        link: `/admin/challenges/${challenge_id}/attachments`,
        name: "附件",
        icon: <FolderIcon />,
        disabled: !challenge?.has_attachment,
      },
      {
        link: `/admin/challenges/${challenge_id}/env`,
        name: "动态环境",
        icon: <ContainerIcon />,
        disabled: !challenge?.is_dynamic,
      },
      {
        link: `/admin/challenges/${challenge_id}/statistics`,
        name: "统计数据",
        icon: <ChartAreaIcon />,
        disabled: true,
      },
    ];
  }, [challenge_id, challenge]);

  return (
    <Context.Provider value={{ challenge }}>
      <div
        className={cn([
          "flex",
          "flex-col",
          "xl:flex-row",
          "flex-1",
          "gap-10",
          "xl:mx-30",
        ])}
      >
        <div
          className={cn([
            "space-y-6",
            "h-fit",
            "my-10",
            "mx-10",
            "xl:mx-0",
            "xl:my-0",
            "xl:w-64",
            "xl:sticky",
            "xl:top-24",
          ])}
        >
          <div
            className={cn([
              "flex",
              "flex-wrap",
              "justify-center",
              "gap-3",
              "select-none",
            ])}
          >
            <LibraryIcon />
            题目编辑
          </div>
          <Card className={cn(["flex", "flex-col", "p-5", "gap-3"])}>
            {options?.map((option, index) => {
              const Comp = option?.disabled ? Button : Link;
              return (
                <Button
                  key={index}
                  icon={option?.icon}
                  variant={pathname === option?.link ? "tonal" : "ghost"}
                  className={cn(["justify-start"])}
                  asChild
                  disabled={option?.disabled}
                >
                  <Comp to={option?.link}>{option?.name}</Comp>
                </Button>
              );
            })}
          </Card>
          <div
            className={cn([
              "flex",
              "flex-wrap",
              "justify-center",
              "gap-3",
              "select-none",
            ])}
          >
            <PlayIcon />
            题目预览
          </div>

          <Dialog>
            <DialogTrigger>
              <ChallengeCard digest={challenge} debug />
            </DialogTrigger>
            <DialogContent>
              <ChallengeDialog digest={challenge} debug />
            </DialogContent>
          </Dialog>
        </div>
        <Card
          className={cn([
            "flex-1",
            "p-10",
            "border-y-0",
            "rounded-none",
            "flex",
            "flex-col",
          ])}
        >
          <Outlet />
        </Card>
      </div>
    </Context.Provider>
  );
}
