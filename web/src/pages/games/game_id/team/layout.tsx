import { deleteTeam, setTeamReady } from "@/api/games/game_id/teams/profile";
import { leaveTeam } from "@/api/games/game_id/teams/profile/users";
import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";
import { Dialog, DialogContent } from "@/components/ui/dialog";
import { Separator } from "@/components/ui/separator";
import { State } from "@/models/team";
import { useGameStore } from "@/storages/game";
import { useSharedStore } from "@/storages/shared";
import { cn } from "@/utils";
import {
    CheckCheckIcon,
    CheckIcon,
    InfoIcon,
    LockIcon,
    TriangleAlertIcon,
    UserRoundMinusIcon,
    UserRoundXIcon,
    UsersRound,
} from "lucide-react";
import { useState } from "react";
import { Link, Outlet, useLocation, useNavigate } from "react-router";
import { toast } from "sonner";

export default function Layout() {
    const sharedStore = useSharedStore();
    const { currentGame, selfTeam } = useGameStore();
    const navigate = useNavigate();
    const location = useLocation();
    const pathname = location.pathname;
    const disabled = Date.now() / 1000 > currentGame?.ended_at!;

    const options = [
        {
            link: `/games/${currentGame?.id}/team`,
            name: "基本信息",
            icon: InfoIcon,
        },
        {
            link: `/games/${currentGame?.id}/team/members`,
            name: "团队成员",
            icon: UsersRound,
        },
    ];

    const [confirmDialogOpen, setConfirmDialogOpen] = useState<boolean>(false);

    function handleSetReady() {
        setTeamReady({
            game_id: currentGame?.id,
            id: selfTeam?.id,
        })
            .then((res) => {
                if (res.code === 200) {
                    toast.success("准备成功", {
                        description: `${selfTeam?.name} 即将登场！`,
                    });
                    setConfirmDialogOpen(false);
                }

                if (res.code === 400) {
                    toast.error("发生了错误", {
                        description: res.msg,
                    });
                }
            })
            .finally(() => {
                sharedStore.setRefresh();
            });
    }

    const [disbandDialogOpen, setDisbandDialogOpen] = useState<boolean>(false);

    function handleDisband() {
        deleteTeam({
            team_id: selfTeam?.id!,
            game_id: currentGame?.id!,
        })
            .then((res) => {
                if (res.code === 200) {
                    toast.success("解散成功", {
                        description: `已解散团队 ${selfTeam?.name}`,
                    });
                    setDisbandDialogOpen(false);
                    navigate(`/games/${currentGame?.id}`);
                }
            })
            .finally(() => {
                sharedStore.setRefresh();
            });
    }

    const [leaveDialogOpen, setLeaveDialogOpen] = useState<boolean>(false);

    function handleLeave() {
        leaveTeam({
            team_id: selfTeam?.id!,
            game_id: currentGame?.id!,
        })
            .then((res) => {
                if (res.code === 200) {
                    toast.success("离队成功", {
                        description: `已离开团队 ${selfTeam?.name}`,
                    });
                    setDisbandDialogOpen(false);
                    navigate(`/games/${currentGame?.id}`);
                }

                if (res.code === 400) {
                    toast.success("离队失败", {
                        description: res.msg,
                    });
                }
            })
            .finally(() => {
                sharedStore.setRefresh();
            });
    }

    return (
        <div className={cn(["flex", "flex-1"])}>
            <div
                className={cn([
                    "hidden",
                    "lg:w-1/5",
                    "bg-card/30",
                    "backdrop-blur-sm",
                    "lg:flex",
                    "flex-col",
                    "gap-3",
                    "p-5",
                    "border-r-1",
                    "lg:sticky",
                    "top-16",
                    "h-[calc(100vh-64px)]",
                ])}
            >
                {options?.map((option, index) => (
                    <Button
                        key={index}
                        size={"lg"}
                        className={cn(["justify-start"])}
                        icon={option.icon}
                        variant={pathname === option.link ? "tonal" : "ghost"}
                        asChild
                    >
                        <Link to={option.link}>{option.name}</Link>
                    </Button>
                ))}
                <Separator />
                <div className={cn(["flex", "gap-5", "justify-center"])}>
                    <Button
                        size={"md"}
                        icon={UserRoundXIcon}
                        level={"error"}
                        className={cn(["w-1/2"])}
                        disabled={
                            selfTeam?.state !== State.Preparing || disabled
                        }
                        onClick={() => setDisbandDialogOpen(true)}
                    >
                        解散团队
                    </Button>
                    <Dialog
                        onOpenChange={setDisbandDialogOpen}
                        open={disbandDialogOpen}
                    >
                        <DialogContent>
                            <Card
                                className={cn([
                                    "flex",
                                    "flex-col",
                                    "w-128",
                                    "p-5",
                                    "gap-5",
                                ])}
                            >
                                <h3
                                    className={cn([
                                        "flex",
                                        "gap-3",
                                        "text-md",
                                        "items-center",
                                    ])}
                                >
                                    <UserRoundXIcon
                                        className={cn(["size-4"])}
                                    />
                                    解散团队
                                </h3>
                                <p className={cn(["text-sm"])}>
                                    团队将被直接删除，所有成员都可创建或加入其他赛队。
                                </p>
                                <Button
                                    icon={CheckCheckIcon}
                                    level={"error"}
                                    variant={"solid"}
                                    onClick={handleDisband}
                                >
                                    确定
                                </Button>
                            </Card>
                        </DialogContent>
                    </Dialog>
                    <Button
                        size={"md"}
                        icon={UserRoundMinusIcon}
                        level={"warning"}
                        className={cn(["w-1/2"])}
                        disabled={
                            selfTeam?.state !== State.Preparing ||
                            selfTeam?.users?.length === 1 ||
                            disabled
                        }
                        onClick={() => setLeaveDialogOpen(true)}
                    >
                        离队
                    </Button>
                    <Dialog
                        onOpenChange={setLeaveDialogOpen}
                        open={leaveDialogOpen}
                    >
                        <DialogContent>
                            <Card
                                className={cn([
                                    "flex",
                                    "flex-col",
                                    "w-128",
                                    "p-5",
                                    "gap-5",
                                ])}
                            >
                                <h3
                                    className={cn([
                                        "flex",
                                        "gap-3",
                                        "text-md",
                                        "items-center",
                                    ])}
                                >
                                    <UserRoundMinusIcon
                                        className={cn(["size-4"])}
                                    />
                                    离开团队
                                </h3>
                                <p className={cn(["text-sm"])}>
                                    你即将离开这个团队，届时你将可以创建或加入其他的团队。
                                </p>
                                <Button
                                    icon={CheckCheckIcon}
                                    level={"error"}
                                    variant={"solid"}
                                    onClick={handleLeave}
                                >
                                    确定
                                </Button>
                            </Card>
                        </DialogContent>
                    </Dialog>
                </div>
                <Button
                    size={"lg"}
                    className={cn(["justify-start"])}
                    icon={
                        selfTeam?.state === State.Preparing
                            ? CheckIcon
                            : LockIcon
                    }
                    level={
                        selfTeam?.state === State.Preparing
                            ? "success"
                            : "error"
                    }
                    variant={"solid"}
                    disabled={selfTeam?.state !== State.Preparing || disabled}
                    onClick={() => setConfirmDialogOpen(true)}
                >
                    {selfTeam?.state === State.Preparing
                        ? "准备好了！"
                        : "团队已锁定"}
                </Button>
                <Dialog
                    onOpenChange={setConfirmDialogOpen}
                    open={confirmDialogOpen}
                >
                    <DialogContent>
                        <Card
                            className={cn([
                                "flex",
                                "flex-col",
                                "w-128",
                                "p-5",
                                "gap-5",
                            ])}
                        >
                            <h3
                                className={cn([
                                    "flex",
                                    "gap-3",
                                    "text-md",
                                    "items-center",
                                ])}
                            >
                                <TriangleAlertIcon className={cn(["size-4"])} />
                                最终提醒
                            </h3>
                            <p className={cn(["text-sm"])}>
                                你真的确定你准备好了吗？
                            </p>
                            <p className={cn(["text-sm"])}>
                                你接下来的操作会将团队状态设置为待审核，届时将不能进行包括
                                <span
                                    className={cn([
                                        "font-semibold",
                                        "underline",
                                    ])}
                                >
                                    成员新增、成员退出、团队退赛
                                </span>
                                等任何影响团队状态的操作。
                            </p>
                            <Button
                                icon={CheckCheckIcon}
                                level={"warning"}
                                variant={"solid"}
                                onClick={handleSetReady}
                            >
                                真的准备好了！
                            </Button>
                        </Card>
                    </DialogContent>
                </Dialog>
            </div>
            <div className={cn(["flex-1", "flex", "flex-col"])}>
                <Outlet />
            </div>
        </div>
    );
}
