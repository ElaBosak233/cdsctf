import type { ColumnDef } from "@tanstack/react-table";

import { Avatar } from "@/components/ui/avatar";
import { Badge } from "@/components/ui/badge";
import type { ScoreRecord } from "@/models/game";
import { useGameStore } from "@/storages/game";
import { cn } from "@/utils";

const columns: Array<ColumnDef<ScoreRecord>> = [
  {
    accessorKey: "team.rank",
    id: "team.rank",
    header: "排名",
    cell: ({ row }) => <Badge>{row.original.team?.rank}</Badge>,
  },
  {
    accessorKey: "team.name",
    id: "team.name",
    header: "团队名",
    cell: function TeamNameCell({ row }) {
      const id = row.original?.team?.id;
      const name = row.original?.team?.name;
      const { currentGame } = useGameStore();

      return (
        <div className={cn(["flex", "items-center", "gap-3"])}>
          <Avatar
            src={
              row.original.team?.has_avatar &&
              `/api/games/${currentGame?.id}/teams/${id}/avatar`
            }
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
    cell: ({ row }) => <span>{row.original.team?.pts}</span>,
  },
  {
    accessorKey: "team.slogan",
    id: "team.slogan",
    header: "口号",
    cell: ({ row }) => <span>{row.original.team?.slogan}</span>,
  },
];

export { columns };
