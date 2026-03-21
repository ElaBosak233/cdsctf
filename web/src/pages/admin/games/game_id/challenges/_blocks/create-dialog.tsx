import { HashIcon, LibraryIcon, TypeIcon } from "lucide-react";
import { useCallback, useContext, useEffect, useState } from "react";
import { useTranslation } from "react-i18next";
import { useParams } from "react-router";
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
import { parseRouteNumericId } from "@/utils/query";
import { Context } from "../../context";

interface CreateDialogProps {
  onClose: () => void;
}

function CreateDialog(props: CreateDialogProps) {
  const { onClose } = props;
  const { t } = useTranslation();

  const { game_id } = useParams<{ game_id: string }>();
  const routeGameId = parseRouteNumericId(game_id);
  const { game } = useContext(Context);
  const sharedStore = useSharedStore();

  const [id, setId] = useState<string>("");
  const debouncedId = useDebounce(id, 100);
  const [title, setTitle] = useState<string>("");
  const debounceTitle = useDebounce(title, 100);
  const [challenges, setChallenges] = useState<Array<Challenge>>();

  const fetchChallenges = useCallback(() => {
    getChallenges({
      id: debouncedId ? Number(debouncedId) : undefined,
      title: debounceTitle,
      public: false,
      size: 10,
      page: 1,
      sorts: "-created_at",
    }).then((res) => {
      setChallenges(res.challenges);
    });
  }, [debouncedId, debounceTitle]);

  useEffect(() => {
    void debounceTitle;
    void debouncedId;

    fetchChallenges();
  }, [fetchChallenges, debounceTitle, debouncedId]);

  function handleCreateGameChallenge(challenge: Challenge) {
    const gid = routeGameId ?? game?.id;
    if (gid == null || challenge.id == null) return;

    createGameChallenge({
      game_id: gid,
      challenge_id: challenge.id,
      enabled: false,
      max_pts: 2000,
      min_pts: 500,
      difficulty: 5,
      bonus_ratios: [],
    }).then(() => {
      toast.success(
        t("game:challenge.actions.add.success", { title: challenge?.title })
      );
      sharedStore?.setRefresh();
      onClose();
    });
  }

  return (
    <Card
      className={cn(["p-5", "w-156", "min-h-64", "flex", "flex-col", "gap-5"])}
    >
      <h3 className={cn(["flex", "gap-3", "items-center", "text-md"])}>
        <LibraryIcon className={cn(["size-4"])} />
        {t("game:challenge.actions.add._")}
      </h3>
      <span className={cn(["text-secondary-foreground", "text-sm"])}>
        {t("game:challenge.actions.add.message")}
      </span>
      <div className={cn(["flex", "gap-3"])}>
        <Field size={"sm"} className={cn(["w-full"])}>
          <FieldIcon>
            <HashIcon />
          </FieldIcon>
          <TextField
            value={id}
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
            placeholder={t("challenge:title")}
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
              <Badge className={cn(["font-mono"])}>{challenge?.id}</Badge>
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
