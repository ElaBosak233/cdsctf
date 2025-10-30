import { useQuery } from "@tanstack/react-query";
import { KeyIcon, RefreshCcwIcon, UsersRoundIcon } from "lucide-react";
import { Link } from "react-router";
import { createToken, getToken } from "@/api/games/game_id/teams/profile/token";
import { Avatar } from "@/components/ui/avatar";
import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";
import { Field, FieldIcon } from "@/components/ui/field";
import { Separator } from "@/components/ui/separator";
import { TextField } from "@/components/ui/text-field";
import { useRefresh } from "@/hooks/use-refresh";
import { State } from "@/models/team";
import { useGameStore } from "@/storages/game";
import { cn } from "@/utils";

export default function Index() {
  const { currentGame, selfTeam, members } = useGameStore();
  const { tick, bump } = useRefresh();

  const disabled =
    Date.now() > Number(currentGame?.ended_at) * 1000 ||
    selfTeam?.state !== State.Preparing;

  const { data: token } = useQuery({
    queryKey: ["game_token", currentGame?.id, selfTeam?.id, tick],
    queryFn: () =>
      getToken({
        game_id: currentGame?.id,
        team_id: selfTeam?.id,
      }),
    enabled: !!currentGame?.id && !!selfTeam?.id,
    select: (res) => res.data,
  });

  async function handleCreateToken() {
    if (!currentGame || !selfTeam) return;
    await createToken({
      game_id: currentGame.id!,
      team_id: selfTeam.id!,
    });
    bump();
  }

  return (
    <>
      <title>{`团队成员 - ${currentGame?.title}`}</title>
      <div
        className={cn([
          "flex",
          "flex-col",
          "flex-1",
          "p-10",
          "xl:mx-50",
          "lg:mx-30",
          "gap-5",
        ])}
      >
        <h1
          className={cn([
            "text-2xl",
            "font-bold",
            "flex",
            "gap-2",
            "items-center",
          ])}
        >
          <UsersRoundIcon />
          团队成员
        </h1>
        <Separator />
        {!disabled && (
          <div className={cn(["flex", "gap-5", "items-center"])}>
            <Field className={cn(["flex-1"])}>
              <FieldIcon>
                <KeyIcon />
              </FieldIcon>
              <TextField
                readOnly
                disabled={disabled}
                value={
                  token ? `${selfTeam?.id ?? ""}:${token || ""}` : "暂无邀请码"
                }
                onChange={() => {}}
              />
            </Field>

            <Button
              icon={<RefreshCcwIcon />}
              variant={"solid"}
              onClick={handleCreateToken}
              size={"lg"}
            >
              生成新邀请码
            </Button>
          </div>
        )}
        <div className={cn(["grid", "grid-cols-2", "gap-5"])}>
          {members?.map((user) => (
            <Link key={user?.id} to={`/users/${user?.id}`}>
              <Card className={cn(["p-3", "flex", "gap-3", "items-center"])}>
                <Avatar
                  src={user?.has_avatar && `/api/users/${user?.id}/avatar`}
                  fallback={user?.name?.charAt(0)}
                />
                <div>
                  <p className={cn(["text-md"])}>{user?.name}</p>
                  <p
                    className={cn(["text-sm", "text-secondary-foreground"])}
                  >{`# ${user?.username}`}</p>
                </div>
              </Card>
            </Link>
          ))}
        </div>
      </div>
    </>
  );
}
