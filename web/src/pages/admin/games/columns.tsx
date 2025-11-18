import type { Column, ColumnDef, Row } from "@tanstack/react-table";
import { StatusCodes } from "http-status-codes";
import {
  ArrowDownIcon,
  ArrowUpDownIcon,
  ArrowUpIcon,
  CheckIcon,
  ClipboardCheckIcon,
  ClipboardCopyIcon,
  EditIcon,
  EyeClosedIcon,
  EyeIcon,
  TrashIcon,
  XIcon,
} from "lucide-react";
import { useMemo, useState } from "react";
import { Trans, useTranslation } from "react-i18next";
import { Link } from "react-router";
import { toast } from "sonner";
import { deleteGame, updateGame } from "@/api/admin/games/game_id";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";
import { Dialog, DialogContent } from "@/components/ui/dialog";
import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from "@/components/ui/tooltip";
import { useClipboard } from "@/hooks/use-clipboard";
import type { Game } from "@/models/game";
import { useSharedStore } from "@/storages/shared";
import { cn } from "@/utils";

function IdCell({ row }: { row: Row<Game> }) {
  const id = row.original.id;
  const { t } = useTranslation();
  const { isCopied, copyToClipboard } = useClipboard();
  return (
    <div className={cn(["flex", "items-center", "gap-2"])}>
      <Badge className={cn(["font-mono"])}># {id}</Badge>
      <Tooltip>
        <TooltipTrigger asChild>
          <Button
            icon={isCopied ? <ClipboardCheckIcon /> : <ClipboardCopyIcon />}
            square
            size={"sm"}
            onClick={() => copyToClipboard(`${id}`)}
          />
        </TooltipTrigger>
        <TooltipContent>{t("common.tooltip.copy")}</TooltipContent>
      </Tooltip>
    </div>
  );
}

function ActionsCell({ row }: { row: Row<Game> }) {
  const { t } = useTranslation();

  const id = row.original.id;
  const title = row.original.title;

  const sharedStore = useSharedStore();

  const [deleteDialogOpen, setDeleteDialogOpen] = useState<boolean>(false);

  const isEnabled = row.original.is_enabled;
  const [checked, setChecked] = useState(isEnabled);

  async function handlePublicnessChange() {
    const newValue = !checked;
    setChecked(newValue);

    const res = await updateGame({
      id,
      is_enabled: newValue,
    });

    if (res.code === StatusCodes.OK) {
      toast.success(t("game.is_enabled.actions.success", { title }), {
        id: "enablement_change",
      });
      sharedStore?.setRefresh();
    }
  }

  async function handleDelete() {
    try {
      const res = await deleteGame({
        id,
      });

      if (res.code === StatusCodes.OK) {
        toast.success(t("game.actions.delete.success", { title }));
        setDeleteDialogOpen(false);
      }
    } finally {
      sharedStore?.setRefresh();
    }
  }

  return (
    <div className={cn(["flex", "items-center", "justify-center", "gap-2"])}>
      <Button variant={"ghost"} size={"sm"} square icon={<EditIcon />} asChild>
        <Link to={`/admin/games/${id}`} />
      </Button>

      <Tooltip>
        <TooltipTrigger asChild>
          <Button
            level={checked ? "warning" : "success"}
            variant={"ghost"}
            size={"sm"}
            square
            icon={checked ? <EyeClosedIcon /> : <EyeIcon />}
            onClick={handlePublicnessChange}
          />
        </TooltipTrigger>
        <TooltipContent>
          {checked
            ? t("game.is_enabled.actions.false")
            : t("game.is_enabled.actions.true")}
        </TooltipContent>
      </Tooltip>

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
              {t("game.actions.delete._")}
            </div>
            <p className={cn(["text-sm"])}>
              <Trans
                i18nKey="game.actions.delete.message"
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

function StartedAtHeader({ column }: { column: Column<Game> }) {
  const { t } = useTranslation();

  const sort = column.getIsSorted();

  const icon = useMemo(() => {
    switch (sort) {
      case "asc":
        return <ArrowUpIcon />;
      case "desc":
        return <ArrowDownIcon />;
      default:
        return <ArrowUpDownIcon />;
    }
  }, [sort]);

  return (
    <div className={cn(["flex", "gap-1", "items-center"])}>
      {t("game.started_at")}
      <Button
        icon={icon}
        square
        size={"sm"}
        onClick={() => column.toggleSorting()}
      />
    </div>
  );
}

function EndedAtHeader({ column }: { column: Column<Game> }) {
  const { t } = useTranslation();

  const sort = column.getIsSorted();

  const icon = useMemo(() => {
    switch (sort) {
      case "asc":
        return <ArrowUpIcon />;
      case "desc":
        return <ArrowDownIcon />;
      default:
        return <ArrowUpDownIcon />;
    }
  }, [sort]);

  return (
    <div className={cn(["flex", "gap-1", "items-center"])}>
      {t("game.ended_at")}
      <Button
        icon={icon}
        square
        size={"sm"}
        onClick={() => column.toggleSorting()}
      />
    </div>
  );
}

function useColumns() {
  const { t } = useTranslation();

  const columns: Array<ColumnDef<Game>> = useMemo(() => {
    return [
      {
        accessorKey: "id",
        id: "id",
        header: "ID",
        cell: IdCell,
      },
      {
        accessorKey: "title",
        id: "title",
        header: () => t("game.title"),
        cell: ({ row }) => (
          <div
            className={cn([
              "w-64",
              "overflow-hidden",
              "text-ellipsis",
              "whitespace-nowrap",
            ])}
          >
            {row.original.title || "-"}
          </div>
        ),
      },
      {
        accessorKey: "is_public",
        id: "is_public",
        header: () => t("game.is_public"),
        cell: ({ row }) => {
          const isPublic = row.original.is_public;

          return (
            <Badge
              className={cn([
                isPublic
                  ? ["bg-info", "text-info-foreground"]
                  : ["bg-success", "text-success-foreground"],
              ])}
            >
              {isPublic ? <CheckIcon /> : <XIcon />}
            </Badge>
          );
        },
      },
      {
        accessorKey: "sketch",
        header: () => t("game.sketch"),
        cell: ({ row }) => (
          <div className={cn(["w-42", "text-wrap"])}>{row.original.sketch}</div>
        ),
      },
      {
        accessorKey: "started_at",
        id: "started_at",
        header: StartedAtHeader,
        cell: ({ row }) => (
          <span className={cn(["text-sm", "text-secondary-foreground"])}>
            {new Date(Number(row.original.started_at) * 1000).toLocaleString()}
          </span>
        ),
      },
      {
        accessorKey: "ended_at",
        id: "ended_at",
        header: EndedAtHeader,
        cell: ({ row }) => (
          <span className={cn(["text-sm", "text-secondary-foreground"])}>
            {new Date(Number(row.original.ended_at) * 1000).toLocaleString()}
          </span>
        ),
      },
      {
        id: "actions",
        header: () => (
          <div className={cn(["justify-self-center"])}>
            {t("game.actions._")}
          </div>
        ),
        cell: ActionsCell,
      },
    ];
  }, [t]);

  return columns;
}

export { useColumns };
