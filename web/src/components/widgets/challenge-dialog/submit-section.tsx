import { HTTPError } from "ky";
import { BugIcon, FlagIcon, LockIcon, SendIcon } from "lucide-react";
import { useContext, useMemo, useState } from "react";
import { useTranslation } from "react-i18next";
import { toast } from "sonner";
import { debugCreateSubmission } from "@/api/admin/submissions";
import { createSubmission } from "@/api/submissions";
import { Button } from "@/components/ui/button";
import { Field, FieldIcon } from "@/components/ui/field";
import { TextField } from "@/components/ui/text-field";
import { Typography } from "@/components/ui/typography";
import { useInterval } from "@/hooks/use-interval";
import { Status } from "@/models/submission";
import { useCheckerStore } from "@/storages/checker";
import { cn } from "@/utils";
import { formatApiMsg } from "@/utils/query";
import { Context } from "./context";

function SubmitSection() {
  const { t } = useTranslation();

  const { challenge, team, debug, cheated } = useContext(Context);
  const [placeholder, setPlaceholder] = useState<string>("flag");
  const { add } = useCheckerStore();

  const mode = useMemo(() => {
    if (team) {
      return "game";
    }

    return "default";
  }, [team]);

  useInterval(
    () => {
      if (placeholder === "flag_") {
        setPlaceholder("flag");
      } else {
        setPlaceholder("flag_");
      }
    },
    1000,
    [placeholder]
  );

  const [flag, setFlag] = useState<string>();

  function handleFlagSubmit() {
    const challengeId = challenge?.id;
    const trimmed = flag?.trim();
    if (challengeId == null || !Number.isFinite(challengeId) || !trimmed) {
      return;
    }
    let gameIdParam: number | undefined;
    let teamIdParam: number | undefined;
    if (mode === "game") {
      if (team?.id == null || team.game_id == null) return;
      gameIdParam = Number(team.game_id);
      teamIdParam = Number(team.id);
    }

    createSubmission({
      challenge_id: challengeId,
      content: trimmed,
      game_id: gameIdParam,
      team_id: teamIdParam,
    })
      .then((submission) => {
        if (!submission) return;
        setFlag("");
        toast.loading(
          t("submission:submitted", {
            id: submission.id,
            title: submission.challenge_title,
          }),
          {
            id: `submission-${submission.id}`,
            description: t("submission:pending_review"),
          }
        );
        add(submission);
      })
      .catch(async (err) => {
        if (err instanceof HTTPError) {
          try {
            const body = (await err.response.json()) as { msg?: unknown };
            toast.error(t("common:errors.default"), {
              description: formatApiMsg(body.msg) || err.message,
            });
          } catch {
            toast.error(t("common:errors.default"));
          }
        } else {
          toast.error(t("common:errors.default"));
        }
      });
  }

  async function handleDebugSubmit() {
    const challengeId = challenge?.id;
    const trimmed = flag?.trim();
    if (challengeId == null || !Number.isFinite(challengeId) || !trimmed) {
      return;
    }

    try {
      const result = await debugCreateSubmission({
        challenge_id: challengeId,
        content: trimmed,
      });

      if (result.status === Status.Correct) {
        toast.success(t("submission:status.correct"), {
          description: t("submission:result.correct"),
        });
      } else if (result.status === Status.Incorrect) {
        toast.error(t("submission:status.incorrect"), {
          description: t("submission:result.incorrect"),
        });
      } else if (result.status === Status.Cheat) {
        toast.error(t("submission:status.cheat"), {
          description: t("submission:result.cheat"),
        });
      }
    } catch (err) {
      if (!(err instanceof HTTPError)) {
        return;
      }

      try {
        const body = (await err.response.json()) as { msg?: unknown };
        toast.error(t("common:errors.default"), {
          description: formatApiMsg(body.msg) || err.message,
        });
      } catch {
        toast.error(t("common:errors.default"));
      }
    }
  }

  return (
    <div className={cn(["flex", "flex-col", "gap-2"])}>
      {cheated && (
        <Typography
          className={cn([
            "text-xs",
            "text-error",
            "flex",
            "items-center",
            "gap-1.5",
          ])}
        >
          <LockIcon className={cn(["size-3.5"])} />
          {t("submission:cheated")}
        </Typography>
      )}
      <div className={cn(["flex", "gap-3", "items-center"])}>
        <Field size={"sm"} className={cn(["flex-1"])}>
          <FieldIcon>{cheated ? <LockIcon /> : <FlagIcon />}</FieldIcon>
          <TextField
            placeholder={placeholder}
            value={flag}
            disabled={cheated}
            onChange={(e) => setFlag(e.target.value)}
            onKeyDown={(e) => {
              if (e.key === "Enter") {
                e.preventDefault();
                handleFlagSubmit();
              }
            }}
          />
        </Field>
        {debug ? (
          <Button
            variant={"solid"}
            size={"sm"}
            onClick={handleDebugSubmit}
            disabled={cheated}
            square
            icon={<BugIcon />}
          ></Button>
        ) : (
          <Button
            variant={"solid"}
            size={"sm"}
            onClick={handleFlagSubmit}
            disabled={cheated}
            square
            icon={cheated ? <LockIcon /> : <SendIcon />}
          />
        )}
      </div>
    </div>
  );
}

export { SubmitSection };
