import type { ColumnDef, Row } from "@tanstack/react-table";
import { StatusCodes } from "http-status-codes";
import {
  ClipboardCheckIcon,
  ClipboardCopyIcon,
  EditIcon,
  SettingsIcon,
  TrashIcon,
} from "lucide-react";
import { useMemo, useState } from "react";
import { Trans, useTranslation } from "react-i18next";
import { Link } from "react-router";
import { toast } from "sonner";
import {
  deleteGameChallenge,
  updateGameChallenge,
} from "@/api/admin/games/game_id/challenges/challenge_id";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";
import { Dialog, DialogContent } from "@/components/ui/dialog";
import { Switch } from "@/components/ui/switch";
import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from "@/components/ui/tooltip";
import { useClipboard } from "@/hooks/use-clipboard";
import type { GameChallenge } from "@/models/game_challenge";
import { useSharedStore } from "@/storages/shared";
import { cn } from "@/utils";
import { getCategory } from "@/utils/category";
import { EditDialog } from "./edit-dialog";

function IsEnabledCell({ row }: { row: Row<GameChallenge> }) {
  const isEnabled = row.original.is_enabled;
  const title = row.original.challenge_title;
  const challenge_id = row.original.challenge_id;
  const game_id = row.original.game_id;
  const [checked, setChecked] = useState(isEnabled);

  function handlePublicnessChange() {
    const newValue = !checked;
    setChecked(newValue);

    updateGameChallenge({
      game_id,
      challenge_id,
      is_enabled: newValue,
    }).then((res) => {
      if (res.code === StatusCodes.OK) {
        toast.success(`${newValue ? "启用" : "禁用"} 赛题 ${title}`, {
          id: "publicness_change",
        });
      }
    });
  }

  return (
    <Switch
      checked={checked}
      onCheckedChange={handlePublicnessChange}
      aria-label="公开性开关"
    />
  );
}

function ChallengeIdCell({ row }: { row: Row<GameChallenge> }) {
  const id = row.original.challenge_id!;
  const { t } = useTranslation();
  const { isCopied, copyToClipboard } = useClipboard();
  return (
    <div className={cn(["flex", "items-center", "gap-2"])}>
      <Badge className={cn(["font-mono"])}>{id}</Badge>
      <Tooltip>
        <TooltipTrigger asChild>
          <Button
            icon={isCopied ? <ClipboardCheckIcon /> : <ClipboardCopyIcon />}
            square
            size={"sm"}
            onClick={() => copyToClipboard(String(id))}
          />
        </TooltipTrigger>
        <TooltipContent>{t("common.tooltip.copy")}</TooltipContent>
      </Tooltip>
    </div>
  );
}

function ActionsCell({ row }: { row: Row<GameChallenge> }) {
  const { t } = useTranslation();

  const challenge_id = row.original.challenge_id;
  const game_id = row.original.game_id;
  const title = row.original.challenge_title;

  const sharedStore = useSharedStore();

  const [editDialogOpen, setEditDialogOpen] = useState<boolean>(false);
  const [deleteDialogOpen, setDeleteDialogOpen] = useState<boolean>(false);

  function handleDelete() {
    deleteGameChallenge({
      game_id,
      challenge_id,
    })
      .then((res) => {
        if (res.code === StatusCodes.OK) {
          toast.success(
            t("game.actions.delete.success", {
              title,
            })
          );
          setDeleteDialogOpen(false);
        }
      })
      .finally(() => {
        sharedStore?.setRefresh();
      });
  }

  return (
    <div className={cn(["flex", "items-center", "justify-center", "gap-2"])}>
      <Button
        variant={"ghost"}
        size={"sm"}
        square
        icon={<SettingsIcon />}
        onClick={() => setEditDialogOpen(true)}
      />
      <Dialog open={editDialogOpen} onOpenChange={setEditDialogOpen}>
        <DialogContent>
          <EditDialog
            gameChallenge={row.original}
            onClose={() => setEditDialogOpen(false)}
          />
        </DialogContent>
      </Dialog>
      <Button icon={<EditIcon />} square size={"sm"} asChild>
        <Link to={`/admin/challenges/${row.original.challenge_id}`} />
      </Button>
      <Button
        level={"error"}
        variant={"ghost"}
        size={"sm"}
        square
        icon={<TrashIcon />}
        onClick={() => setDeleteDialogOpen(true)}
      />
      <Dialog open={deleteDialogOpen} onOpenChange={setDeleteDialogOpen}>
        <DialogContent>
          <Card
            className={cn([
              "flex",
              "flex-col",
              "p-5",
              "min-h-32",
              "w-lg",
              "gap-5",
            ])}
          >
            <div className={cn(["flex", "gap-2", "items-center", "text-sm"])}>
              <TrashIcon className={cn(["size-4", "text-error"])} />
              {t("game.challenge.actions.delete._")}
            </div>
            <p className={cn(["text-sm"])}>
              <Trans
                i18nKey="game.challenge.actions.delete.message"
                values={{ title }}
                components={{
                  muted: <span className={cn(["text-muted-foreground"])} />,
                }}
              />
            </p>
            <div className={cn(["flex", "justify-end"])}>
              <Button
                level={"error"}
                variant={"solid"}
                size={"sm"}
                onClick={handleDelete}
              >
                {t("common.actions.confirm")}
              </Button>
            </div>
          </Card>
        </DialogContent>
      </Dialog>
    </div>
  );
}

function useColumns() {
  const { t } = useTranslation();

  const columns: Array<ColumnDef<GameChallenge>> = useMemo(() => {
    return [
      {
        accessorKey: "game_id",
        enableHiding: false,
      },
      {
        accessorKey: "is_enabled",
        header: t("game.challenge.is_enabled._"),
        cell: IsEnabledCell,
      },
      {
        id: "challenge_id",
        header: "ID",
        cell: ChallengeIdCell,
      },
      {
        id: "challenge_title",
        header: t("challenge.title"),
        cell: ({ row }) => row.original.challenge_title || "-",
      },
      {
        id: "challenge_category",
        header: t("challenge.category"),
        cell: ({ row }) => {
          const categoryId = row.original.challenge_category;
          const category = getCategory(categoryId!);

          const Icon = category.icon!;
          return (
            <div className={cn(["flex", "gap-2", "items-center"])}>
              <Icon className={cn(["size-4"])} />
              {category.name?.toUpperCase()}
            </div>
          );
        },
      },
      {
        accessorKey: "pts",
        id: "pts",
        header: t("game.challenge.pts"),
        cell: ({ row }) => (
          <span>
            {row.original.pts}{" "}
            <span className={cn(["text-muted-foreground"])}>pts</span>
          </span>
        ),
      },
      {
        id: "actions",
        header: () => (
          <div className={cn(["justify-self-center"])}>
            {t("game.challenge.actions._")}
          </div>
        ),
        cell: ActionsCell,
      },
    ];
  }, [t]);

  return columns;
}

export { useColumns };
