import type { ColumnDef } from "@tanstack/react-table";
import { useMemo } from "react";
import { useTranslation } from "react-i18next";
import { Avatar } from "@/components/ui/avatar";
import { Badge } from "@/components/ui/badge";
import type { ScoreRecord } from "@/models/game";
import { useGameStore } from "@/storages/game";
import { cn } from "@/utils";

function useColumns() {
  const { t } = useTranslation();
  const { currentGame } = useGameStore();

  const columns: Array<ColumnDef<ScoreRecord>> = useMemo(() => {
    return [
      {
        accessorKey: "team.rank",
        id: "team.rank",
        header: t("team.rank"),
        cell: ({ row }) => <Badge>{row.original.team?.rank}</Badge>,
      },
      {
        accessorKey: "team.name",
        id: "team.name",
        header: t("team.name"),
        cell: function TeamNameCell({ row }) {
          const id = row.original?.team?.id;
          const name = row.original?.team?.name;

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
        header: t("team.pts"),
        cell: ({ row }) => <span>{row.original.team?.pts}</span>,
      },
      {
        accessorKey: "team.slogan",
        id: "team.slogan",
        header: t("team.slogan"),
        cell: ({ row }) => <span>{row.original.team?.slogan}</span>,
      },
    ];
  }, [currentGame, t]);

  return columns;
}

export { useColumns };
