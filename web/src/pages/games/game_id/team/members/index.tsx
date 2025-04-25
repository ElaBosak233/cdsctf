import { createToken, getToken } from "@/api/games/game_id/teams/profile/token";
import { Avatar } from "@/components/ui/avatar";
import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";
import { Field, FieldIcon } from "@/components/ui/field";
import { TextField } from "@/components/ui/text-field";
import { useGameStore } from "@/storages/game";
import { cn } from "@/utils";
import { KeyIcon, RefreshCcwIcon } from "lucide-react";
import { useEffect, useState } from "react";

export default function Index() {
    const { currentGame, selfTeam } = useGameStore();
    const [token, setToken] = useState<string>();
    const disabled = Date.now() / 1000 > currentGame?.ended_at!;

    useEffect(() => {
        if (currentGame?.id && selfTeam?.id) {
            getToken({
                game_id: currentGame?.id!,
                team_id: selfTeam?.id!,
            }).then((res) => {
                setToken(res.data);
            });
        }
    }, [currentGame?.id, selfTeam?.id]);

    function handleCreateToken() {
        createToken({
            game_id: currentGame?.id!,
            team_id: selfTeam?.id!,
        }).then((res) => {
            setToken(res.data);
        });
    }

    return (
        <div
            className={cn([
                "flex",
                "flex-col",
                "flex-1",
                "p-10",
                "xl:mx-50",
                "lg:mx-30",
                "gap-8",
            ])}
        >
            <div className={cn(["flex", "gap-5", "items-center"])}>
                <Field className={cn(["flex-1"])}>
                    <FieldIcon>
                        <KeyIcon />
                    </FieldIcon>
                    <TextField
                        readOnly
                        disabled={disabled}
                        value={
                            token
                                ? `${selfTeam?.id ?? ""}:${token || ""}`
                                : "暂无邀请码"
                        }
                        onChange={() => {}}
                    />
                </Field>

                <Button
                    icon={RefreshCcwIcon}
                    variant={"solid"}
                    onClick={handleCreateToken}
                    size={"lg"}
                    disabled={disabled}
                >
                    生成新邀请码
                </Button>
            </div>
            <div className={cn(["grid", "grid-cols-2", "gap-5"])}>
                {selfTeam?.users?.map((user) => (
                    <Card
                        key={user?.id}
                        className={cn(["p-3", "flex", "gap-3", "items-center"])}
                    >
                        <Avatar
                            src={`/api/users/${user?.id}/avatar`}
                            fallback={user?.username?.charAt(0)}
                        />
                        <div>
                            <p className={cn(["text-md"])}>{user?.nickname}</p>
                            <p className={cn(["text-sm"])}>{user?.username}</p>
                        </div>
                    </Card>
                ))}
            </div>
        </div>
    );
}
