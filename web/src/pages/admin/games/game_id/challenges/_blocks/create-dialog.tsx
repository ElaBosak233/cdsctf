import { keepPreviousData, useQuery } from "@tanstack/react-query";
import { CheckIcon, LibraryIcon } from "lucide-react";
import { useContext, useState } from "react";
import { useTranslation } from "react-i18next";
import { useParams } from "react-router";
import { toast } from "sonner";
import { getChallenges } from "@/api/admin/challenges";
import { createGameChallenge } from "@/api/admin/games/game_id/challenges";
import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";
import {
  Combobox,
  ComboboxContent,
  ComboboxEmpty,
  ComboboxInput,
  ComboboxItem,
  ComboboxList,
} from "@/components/ui/combobox";
import { Field, FieldIcon } from "@/components/ui/field";
import { useDebounce } from "@/hooks/use-debounce";
import type { ChallengeDetail } from "@/models/challenge";
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

  const [query, setQuery] = useState("");
  const debouncedQuery = useDebounce(query.trim(), 150);
  const [selectedChallenge, setSelectedChallenge] =
    useState<ChallengeDetail | null>(null);

  const challengeQuery = useQuery({
    queryKey: ["admin", "game-challenge-options", debouncedQuery],
    queryFn: async () => {
      const numericId = /^\d+$/.test(debouncedQuery)
        ? Number(debouncedQuery)
        : undefined;
      const [exactIdResult, titleResult] = await Promise.all([
        numericId != null
          ? getChallenges({
              id: numericId,
              public: false,
              size: 1,
              page: 1,
            })
          : Promise.resolve({ challenges: [], total: 0 }),
        getChallenges({
          title: debouncedQuery || undefined,
          public: false,
          size: 10,
          page: 1,
          sorts: "-created_at",
        }),
      ]);

      const seen = new Set<number>();
      return [...exactIdResult.challenges, ...titleResult.challenges]
        .filter((challenge) => {
          if (seen.has(challenge.id)) return false;
          seen.add(challenge.id);
          return true;
        })
        .slice(0, 10);
    },
    enabled: true,
    placeholderData: keepPreviousData,
  });
  const challenges = challengeQuery.data ?? [];

  function handleCreateGameChallenge(challenge: ChallengeDetail) {
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
      className={cn([
        "w-full",
        "max-w-2xl",
        "min-h-64",
        "rounded-elevated",
        "shadow-lg",
        "overflow-hidden",
        "flex",
        "flex-col",
      ])}
    >
      <div className={cn(["p-5", "flex", "flex-col", "gap-5"])}>
        <div className={cn(["flex", "items-center", "gap-3"])}>
          <div
            className={cn([
              "flex items-center justify-center",
              "size-10 rounded-badge",
              "bg-primary/10",
              "shrink-0",
            ])}
          >
            <LibraryIcon className={cn(["size-5"])} />
          </div>
          <h3 className={cn(["text-base", "font-semibold"])}>
            {t("game:challenge.actions.add._")}
          </h3>
        </div>
        <span className={cn(["text-secondary-foreground", "text-sm"])}>
          {t("game:challenge.actions.add.message")}
        </span>
        <Field size="sm" className="w-full">
          <FieldIcon>
            <LibraryIcon />
          </FieldIcon>
          <Combobox<ChallengeDetail>
            options={challenges.map((challenge) => ({
              value: challenge,
              content: challenge.title,
            }))}
            itemToStringLabel={(challenge) => challenge?.title ?? ""}
            isItemEqualToValue={(item, value) => item.id === value.id}
            filter={null}
            onInputValueChange={(value, details) => {
              if (details.reason === "input-change" || value === "") {
                setQuery(value);
              }
            }}
            value={selectedChallenge}
            onValueChange={setSelectedChallenge}
            placeholder={t("common:search")}
            emptyText={t("challenge:empty")}
          >
            <ComboboxInput showClear placeholder={t("common:search")} />
            <ComboboxContent>
              <ComboboxEmpty>{t("challenge:empty")}</ComboboxEmpty>
              <ComboboxList>
                {challenges.map((challenge) => {
                  const Icon = getCategory(challenge.category!).icon!;
                  return (
                    <ComboboxItem key={challenge.id} value={challenge}>
                      <span className="shrink-0 font-mono text-xs text-muted-foreground">
                        #{challenge.id}
                      </span>
                      <Icon className="size-4 shrink-0" />
                      <span className="min-w-0 truncate">
                        {challenge.title}
                      </span>
                    </ComboboxItem>
                  );
                })}
              </ComboboxList>
            </ComboboxContent>
          </Combobox>
        </Field>
        <Button
          variant={"solid"}
          icon={<CheckIcon />}
          level={"success"}
          // loading={loading}
          disabled={selectedChallenge == null}
          onClick={() => {
            if (selectedChallenge) {
              handleCreateGameChallenge(selectedChallenge);
            }
          }}
          // type={"submit"}
        >
          {t("common:actions.confirm")}
        </Button>
      </div>
    </Card>
  );
}

export { CreateDialog };
