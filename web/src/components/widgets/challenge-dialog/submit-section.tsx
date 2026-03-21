import { HTTPError } from "ky";
import { FlagIcon, SendIcon } from "lucide-react";
import { useContext, useMemo, useState } from "react";
import { useTranslation } from "react-i18next";
import { toast } from "sonner";
import { createSubmission } from "@/api/submissions";
import { Button } from "@/components/ui/button";
import { Field, FieldIcon } from "@/components/ui/field";
import { TextField } from "@/components/ui/text-field";
import { useInterval } from "@/hooks/use-interval";
import { useCheckerStore } from "@/storages/checker";
import { cn } from "@/utils";
import { Context } from "./context";

function SubmitSection() {
  const { t } = useTranslation();

  const { challenge, team } = useContext(Context);
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
    createSubmission({
      challenge_id: challenge?.id,
      content: flag?.trim(),
      game_id: mode === "game" ? Number(team?.game_id) : undefined,
      team_id: mode === "game" ? Number(team?.id) : undefined,
    })
      .then((submission) => {
        if (!submission) return;
        setFlag("");
        toast.loading(
          `#${submission.id} 已提交题目 ${submission.challenge_title} 的解答`,
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
              description: String(body.msg ?? err.message),
            });
          } catch {
            toast.error(t("common:errors.default"));
          }
        } else {
          toast.error(t("common:errors.default"));
        }
      });
  }

  return (
    <div className={cn(["flex", "gap-3", "items-center"])}>
      <Field size={"sm"} className={cn(["flex-1"])}>
        <FieldIcon>
          <FlagIcon />
        </FieldIcon>
        <TextField
          placeholder={placeholder}
          value={flag}
          onChange={(e) => setFlag(e.target.value)}
          onKeyDown={(e) => {
            if (e.key === "Enter") {
              e.preventDefault();
              handleFlagSubmit();
            }
          }}
        />
      </Field>
      <Button size={"sm"} onClick={handleFlagSubmit}>
        <SendIcon />
      </Button>
    </div>
  );
}

export { SubmitSection };
