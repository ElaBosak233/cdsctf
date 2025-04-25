import { Button } from "@/components/ui/button";
import {
    BanIcon,
    CheckCheckIcon,
    ClipboardCheck,
    ClipboardCopy,
    Undo2,
} from "lucide-react";
import { ColumnDef } from "@tanstack/react-table";
import { cn } from "@/utils";
import { Badge } from "@/components/ui/badge";
import {
    Tooltip,
    TooltipContent,
    TooltipTrigger,
} from "@/components/ui/tooltip";
import { useClipboard } from "@/hooks/use-clipboard";
import { toast } from "sonner";
import { useSharedStore } from "@/storages/shared";
import { State, Team } from "@/models/team";
import { updateTeam } from "@/api/admin/games/game_id/teams/team_id";
import { Avatar } from "@/components/ui/avatar";

const columns: ColumnDef<Team>[] = [
    {
        accessorKey: "id",
        id: "id",
        header: "ID",
        cell: ({ row }) => {
            const id = row.getValue<string>("id");
            const { isCopied, copyToClipboard } = useClipboard();
            return (
                <div className={cn(["flex", "items-center", "gap-1"])}>
                    <Badge>{id}</Badge>
                    <Tooltip>
                        <TooltipTrigger asChild>
                            <Button
                                icon={isCopied ? ClipboardCheck : ClipboardCopy}
                                square
                                size={"sm"}
                                onClick={() => copyToClipboard(id)}
                            />
                        </TooltipTrigger>
                        <TooltipContent>复制到剪贴板</TooltipContent>
                    </Tooltip>
                </div>
            );
        },
    },
    {
        accessorKey: "name",
        id: "name",
        header: "团队名",
        cell: ({ row }) => {
            const name = row.getValue<string>("name");
            return (
                <div className={cn(["flex", "gap-2", "items-center"])}>
                    <Avatar
                        src={`/api/games/${row.original.game_id}/teams/${row.original?.id}/avatar`}
                        fallback={name.charAt(0)}
                    />
                    <span>{name}</span>
                </div>
            );
        },
    },
    {
        accessorKey: "slogan",
        id: "slogan",
        header: "口号",
        cell: ({ row }) => {
            const slogan = row.getValue<string>("slogan");
            return slogan || "-";
        },
    },
    {
        accessorKey: "rank",
        id: "rank",
        header: "排名",
        cell: ({ row }) => {
            const rank = row.getValue<number>("rank");

            return rank;
        },
    },
    {
        accessorKey: "pts",
        id: "pts",
        header: "当前分值",
        cell: ({ row }) => {
            const pts = row.getValue<number>("pts");

            return pts;
        },
    },
    {
        accessorKey: "state",
        id: "state",
        header: "当前状态",
        cell: ({ row }) => {
            const state = row.getValue<State>("state");

            switch (state) {
                case State.Banned:
                    return (
                        <Badge
                            className={cn([
                                "bg-error",
                                "text-error-foreground",
                            ])}
                        >
                            禁赛中
                        </Badge>
                    );
                case State.Preparing:
                    return (
                        <Badge
                            className={cn(["bg-info", "text-info-foreground"])}
                        >
                            准备中
                        </Badge>
                    );
                case State.Pending:
                    return (
                        <Badge
                            className={cn([
                                "bg-warning",
                                "text-warning-foreground",
                            ])}
                        >
                            待审核
                        </Badge>
                    );
                case State.Passed:
                    return (
                        <Badge
                            className={cn([
                                "bg-success",
                                "text-success-foreground",
                            ])}
                        >
                            正常参赛
                        </Badge>
                    );
            }
        },
    },
    {
        id: "actions",
        header: () => <div className={cn(["justify-self-center"])}>操作</div>,
        cell: ({ row }) => {
            const id = row.original.id;
            const game_id = row.original.game_id;
            const state = row.original.state;

            const sharedStore = useSharedStore();

            function handleStateChange(state: State) {
                updateTeam({
                    team_id: id!,
                    game_id: game_id!,
                    state,
                })
                    .then((res) => {
                        if (res.code === 200) {
                            toast.success(
                                `团队 ${row.original.name} 状态更新成功`
                            );
                        }
                    })
                    .finally(() => {
                        sharedStore?.setRefresh();
                    });
            }

            return (
                <div
                    className={cn([
                        "flex",
                        "items-center",
                        "justify-center",
                        "gap-2",
                    ])}
                >
                    <Tooltip>
                        <TooltipTrigger asChild>
                            <Button
                                disabled={state === State.Preparing}
                                variant={"ghost"}
                                size={"sm"}
                                level={"info"}
                                square
                                icon={Undo2}
                                onClick={() =>
                                    handleStateChange(State.Preparing)
                                }
                            />
                        </TooltipTrigger>
                        <TooltipContent>打回</TooltipContent>
                    </Tooltip>
                    <Tooltip>
                        <TooltipTrigger asChild>
                            <Button
                                disabled={state === State.Banned}
                                variant={"ghost"}
                                size={"sm"}
                                level={"error"}
                                square
                                icon={BanIcon}
                                onClick={() => handleStateChange(State.Banned)}
                            />
                        </TooltipTrigger>
                        <TooltipContent>禁赛</TooltipContent>
                    </Tooltip>
                    <Tooltip>
                        <TooltipTrigger asChild>
                            <Button
                                disabled={state === State.Passed}
                                variant={"ghost"}
                                size={"sm"}
                                level={"success"}
                                square
                                icon={CheckCheckIcon}
                                onClick={() => handleStateChange(State.Passed)}
                            />
                        </TooltipTrigger>
                        <TooltipContent>通过审核</TooltipContent>
                    </Tooltip>
                </div>
            );
        },
    },
];

export { columns };
