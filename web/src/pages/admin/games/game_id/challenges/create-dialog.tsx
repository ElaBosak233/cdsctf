import { StatusCodes } from "http-status-codes";
import { HashIcon, LibraryIcon, TypeIcon } from "lucide-react";
import { useContext, useEffect, useState } from "react";
import { toast } from "sonner";
import { getChallenges } from "@/api/admin/challenges";
import { createGameChallenge } from "@/api/admin/games/game_id/challenges";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";
import { Field, FieldIcon } from "@/components/ui/field";
import { TextField } from "@/components/ui/text-field";
import { useDebounce } from "@/hooks/use-debounce";
import type { Challenge } from "@/models/challenge";
import { useSharedStore } from "@/storages/shared";
import { cn } from "@/utils";
import { getCategory } from "@/utils/category";
import { Context } from "../context";

interface CreateDialogProps {
  onClose: () => void;
}

function CreateDialog(props: CreateDialogProps) {
  const { onClose } = props;

  const { game } = useContext(Context);
  const sharedStore = useSharedStore();

  const [id, setId] = useState<string>("");
  const debouncedId = useDebounce(id, 100);
  const [title, setTitle] = useState<string>("");
  const debounceTitle = useDebounce(title, 100);
  const [challenges, setChallenges] = useState<Array<Challenge>>();

  useEffect(() => {
    getChallenges({
      id: id || undefined,
      title,
      is_public: false,
      size: 10,
      page: 1,
      sorts: "-created_at",
    }).then((res) => {
      if (res.code === StatusCodes.OK) {
        setChallenges(res.data);
      }
    });
  }, [debounceTitle, debouncedId]);

  function handleCreateGameChallenge(challenge: Challenge) {
    if (!game) return;

    createGameChallenge({
      game_id: game.id!,
      challenge_id: challenge.id!,
      is_enabled: false,
      max_pts: 2000,
      min_pts: 500,
      difficulty: 5,
      bonus_ratios: [],
    }).then((res) => {
      if (res.code === StatusCodes.OK) {
        toast.success(`成功添加赛题 ${challenge?.title}`);
        sharedStore?.setRefresh();
        onClose();
      }

      if (res.code === 409) {
        toast.error(`题目在该比赛中已存在`);
      }
    });
  }

  return (
    <Card
      className={cn(["p-5", "w-156", "min-h-64", "flex", "flex-col", "gap-5"])}
    >
      <h3 className={cn(["flex", "gap-3", "items-center", "text-md"])}>
        <LibraryIcon className={cn(["size-4"])} />
        添加赛题
      </h3>
      <span className={cn(["text-secondary-foreground", "text-sm"])}>
        请使用 ID 或者题目名检索题目，列表仅展示前 10
        条结果。点击题目即可添加赛题。
      </span>
      <div className={cn(["flex", "gap-3"])}>
        <Field size={"sm"} className={cn(["w-full"])}>
          <FieldIcon>
            <HashIcon />
          </FieldIcon>
          <TextField
            value={id || ""}
            onChange={(e) => setId(e.target.value)}
            placeholder={"ID"}
          />
        </Field>
        <Field size={"sm"} className={cn(["w-full"])}>
          <FieldIcon>
            <TypeIcon />
          </FieldIcon>
          <TextField
            value={title}
            onChange={(e) => setTitle(e.target.value)}
            placeholder={"题目名"}
          />
        </Field>
      </div>
      <div className={cn(["grid", "grid-cols-2", "gap-3"])}>
        {challenges?.map((challenge) => {
          const Icon = getCategory(challenge.category!).icon!;
          return (
            <Button
              key={challenge?.id}
              className={cn(["justify-start"])}
              variant={"ghost"}
              onClick={() => handleCreateGameChallenge(challenge)}
            >
              <Badge className={cn(["font-mono"])}>
                {challenge?.id?.split("-")?.[0]}
              </Badge>
              <Icon className={cn(["size-4"])} />
              <span>{challenge?.title}</span>
            </Button>
          );
        })}
      </div>
    </Card>
  );
}

export { CreateDialog };
