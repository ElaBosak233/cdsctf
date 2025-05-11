import { useContext, useMemo, useState } from "react";
import { Context } from "./context";
import { Field, FieldIcon } from "@/components/ui/field";
import { TextField } from "@/components/ui/text-field";
import { Button } from "@/components/ui/button";
import { FlagIcon, Send, SendIcon } from "lucide-react";
import { cn } from "@/utils";
import { useInterval } from "@/hooks/use-interval";
import { createSubmission } from "@/api/submissions";
import { toast } from "sonner";
import { StatusCodes } from "http-status-codes";
import { useCheckerStore } from "@/storages/checker";

function SubmitSection() {
    const { challenge, team } = useContext(Context);
    const [placeholder, setPlaceholder] = useState<string>("flag");
    const { submissions, add } = useCheckerStore();

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
        }).then((res) => {
            if (res.code === StatusCodes.OK) {
                setFlag("");
                toast.loading(
                    `#${res?.data?.id} 已提交题目 ${res?.data?.challenge_title} 的解答`,
                    {
                        id: `submission-${res?.data?.id}`,
                        description: "请等待审核，这不会太久...",
                    }
                );
                add(res.data!);
            }

            if (res.code === 500) {
                toast.error("发生了错误", {
                    description: res.msg,
                });
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
                />
            </Field>
            <Button
                variant={"solid"}
                icon={<SendIcon />}
                onClick={handleFlagSubmit}
                loading={submissions.length > 0}
                disabled={!flag?.trim()}
            >
                提交
            </Button>
        </div>
    );
}

export { SubmitSection };
