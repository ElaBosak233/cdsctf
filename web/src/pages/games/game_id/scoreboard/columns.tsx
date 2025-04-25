import { ColumnDef } from "@tanstack/react-table";
import { cn } from "@/utils";
import { Badge } from "@/components/ui/badge";
import { Avatar } from "@/components/ui/avatar";
import { ScoreRecord } from "@/models/game";
import { useGameStore } from "@/storages/game";

const columns: ColumnDef<ScoreRecord>[] = [
    {
        accessorKey: "team.rank",
        id: "team.rank",
        header: "排名",
        cell: ({ row }) => {
            const rank = row.getValue<number>("team.rank");

            return <Badge>{rank}</Badge>;
        },
    },
    {
        accessorKey: "team.name",
        id: "team.name",
        header: "团队名",
        cell: ({ row }) => {
            const id = row.original?.team?.id;
            const name = row.original?.team?.name;
            const { currentGame } = useGameStore();

            return (
                <div className={cn(["flex", "items-center", "gap-3"])}>
                    <Avatar
                        src={`/api/games/${currentGame?.id}/teams/${id}/avatar`}
                        fallback={name?.charAt(0)}
                    />
                    {name}
                </div>
            );
        },
    },
    {
        accessorKey: "team.pts",
        id: "team.pts",
        header: "分数",
        cell: ({ row }) => {
            const pts = row.getValue<number>("team.pts");

            return <span>{pts}</span>;
        },
    },
    {
        accessorKey: "team.slogan",
        id: "team.slogan",
        header: "口号",
        cell: ({ row }) => {
            const slogan = row.getValue<string>("team.slogan");

            return <span>{slogan}</span>;
        },
    },
];

export { columns };
