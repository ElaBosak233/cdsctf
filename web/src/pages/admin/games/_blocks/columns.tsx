import type { Column, ColumnDef, Row } from "@tanstack/react-table";
import {
  ArrowDownIcon,
  ArrowRightIcon,
  ArrowUpDownIcon,
  ArrowUpIcon,
  ClipboardCheckIcon,
  ClipboardCopyIcon,
  ClockFadingIcon,
  EditIcon,
  EllipsisIcon,
  EyeClosedIcon,
  EyeIcon,
  FlagIcon,
  Globe2Icon,
  LockIcon,
  LockKeyholeIcon,
  MoonIcon,
  TrashIcon,
} from "lucide-react";
import {
  createContext,
  type ReactNode,
  useCallback,
  useContext,
  useMemo,
  useOptimistic,
  useState,
  useTransition,
} from "react";
import { Trans, useTranslation } from "react-i18next";
import { Link } from "react-router";
import { toast } from "sonner";
import { deleteGame, updateGame } from "@/api/admin/games/game_id";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";
import { Dialog, DialogContent } from "@/components/ui/dialog";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { Image } from "@/components/ui/image";
import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from "@/components/ui/tooltip";
import { useClipboard } from "@/hooks/use-clipboard";
import type { GameDetail } from "@/models/game";
import { useSharedStore } from "@/storages/shared";
import { cn } from "@/utils";

const RowContext = createContext<{
  optimisticEnabled: boolean;
  toggleEnabled: (title: string) => void;
} | null>(null);

function useRowContext() {
  return useContext(RowContext);
}

function RowProvider({
  game,
  children,
}: {
  game: GameDetail;
  children: ReactNode;
}) {
  const { t } = useTranslation();
  const [isEnabled, setIsEnabled] = useState(game.enabled ?? false);
  const [, startTransition] = useTransition();
  const [optimisticEnabled, setOptimisticEnabled] = useOptimistic(isEnabled);
  const gameId = game.id;

  const toggleEnabled = useCallback(
    (title: string) => {
      if (gameId == null) return;
      const newValue = !optimisticEnabled;
      startTransition(async () => {
        setOptimisticEnabled(newValue);
        await updateGame({ id: gameId, enabled: newValue });
        setIsEnabled(newValue);
        toast.success(t("game:enabled.actions.success", { title }), {
          id: "enablement_change",
        });
      });
    },
    [optimisticEnabled, gameId, setOptimisticEnabled, t]
  );

  return (
    <RowContext.Provider value={{ optimisticEnabled, toggleEnabled }}>
      {children}
    </RowContext.Provider>
  );
}

type GameStatus =
  | "disabled"
  | "paused"
  | "upcoming"
  | "ongoing"
  | "frozen"
  | "ended";

function getGameStatus(game: GameDetail): GameStatus {
  const now = Date.now() / 1000;

  if (!game.enabled) return "disabled";
  if (now > game.ended_at) return "ended";
  if (game.paused) return "paused";
  if (now < game.started_at) return "upcoming";
  if (now > game.frozen_at) return "frozen";
  return "ongoing";
}

const statusClasses: Record<GameStatus, string[]> = {
  disabled: ["border-transparent", "bg-muted", "text-muted-foreground"],
  paused: ["border-warning/20", "bg-warning/10", "text-warning"],
  upcoming: ["border-info/20", "bg-info/10", "text-info"],
  ongoing: ["border-success/20", "bg-success/10", "text-success"],
  frozen: ["border-warning/20", "bg-warning/10", "text-warning"],
  ended: ["border-transparent", "bg-muted", "text-muted-foreground"],
};

function GameCell({ row }: { row: Row<GameDetail> }) {
  const id = row.original.id;
  const { t } = useTranslation();
  const { isCopied, copyToClipboard } = useClipboard();

  return (
    <div className={cn(["flex", "min-w-0", "items-center", "gap-3"])}>
      {row.original.icon_hash ? (
        <Image
          src={`/api/media?hash=${row.original.icon_hash}`}
          alt=""
          delay={0}
          glass={false}
          fallback={<FlagIcon className="size-4" />}
          className={cn(["size-9", "shrink-0", "rounded-md", "bg-muted"])}
        />
      ) : (
        <div
          className={cn([
            "flex",
            "size-9",
            "shrink-0",
            "items-center",
            "justify-center",
            "rounded-md",
            "bg-muted",
            "text-muted-foreground",
          ])}
        >
          <FlagIcon className="size-4" />
        </div>
      )}
      <div className={cn(["min-w-0", "flex-1"])}>
        <Link
          to={`/admin/games/${id}`}
          className={cn([
            "block",
            "truncate",
            "text-sm",
            "font-semibold",
            "hover:underline",
            "underline-offset-4",
          ])}
        >
          {row.original.title || "-"}
        </Link>
        <div
          className={cn([
            "mt-0.5",
            "flex",
            "min-w-0",
            "items-center",
            "gap-1.5",
            "text-xs",
            "text-muted-foreground",
          ])}
        >
          <span className={cn(["shrink-0", "font-mono"])}>#{id}</span>
          <Tooltip>
            <TooltipTrigger asChild>
              <Button
                icon={isCopied ? <ClipboardCheckIcon /> : <ClipboardCopyIcon />}
                square
                size="sm"
                variant="ghost"
                className={cn(["size-6", "shrink-0", "text-muted-foreground"])}
                aria-label={t("common:tooltip.copy")}
                onClick={() => copyToClipboard(`${id}`)}
              />
            </TooltipTrigger>
            <TooltipContent>{t("common:tooltip.copy")}</TooltipContent>
          </Tooltip>
          {row.original.sketch && (
            <>
              <span className={cn(["shrink-0"])}>·</span>
              <span className={cn(["truncate"])}>{row.original.sketch}</span>
            </>
          )}
        </div>
      </div>
    </div>
  );
}

function StatusCell({ row }: { row: Row<GameDetail> }) {
  const { t } = useTranslation();
  const { optimisticEnabled } = useRowContext()!;
  const status = getGameStatus({
    ...row.original,
    enabled: optimisticEnabled,
  });
  const statusLabel =
    status === "disabled"
      ? t("game:enabled.false")
      : status === "paused"
        ? t("game:form.paused.true")
        : t(`game:status.${status}._`);

  return (
    <div className={cn(["flex", "flex-wrap", "items-center", "gap-1.5"])}>
      <Badge variant="outline" className={cn(statusClasses[status])}>
        {status === "disabled" && <LockIcon />}
        {statusLabel}
      </Badge>
      <Badge variant="outline" className={cn(["text-muted-foreground"])}>
        {row.original.public ? <Globe2Icon /> : <LockKeyholeIcon />}
        {t(
          row.original.public
            ? "game:list.visibility.public"
            : "game:list.visibility.private"
        )}
      </Badge>
      {row.original.blacked_out && (
        <Badge
          variant="outline"
          className={cn(["border-info/20", "bg-info/10", "text-info"])}
        >
          <MoonIcon />
          {t("game:form.blacked_out.true")}
        </Badge>
      )}
    </div>
  );
}

function ScheduleCell({
  row,
  formatter,
}: {
  row: Row<GameDetail>;
  formatter: Intl.DateTimeFormat;
}) {
  const { t } = useTranslation();
  const format = (timestamp: number) =>
    formatter.format(new Date(timestamp * 1000));

  return (
    <div className={cn(["flex", "min-w-0", "flex-col", "gap-1"])}>
      <div
        className={cn([
          "flex",
          "items-center",
          "gap-1.5",
          "whitespace-nowrap",
          "text-sm",
        ])}
      >
        <span>{format(row.original.started_at)}</span>
        <ArrowRightIcon className="size-3.5 shrink-0 text-muted-foreground" />
        <span>{format(row.original.ended_at)}</span>
      </div>
      <div
        className={cn([
          "flex",
          "items-center",
          "gap-1.5",
          "text-xs",
          "text-muted-foreground",
        ])}
      >
        <ClockFadingIcon className="size-3.5" />
        {t("game:list.freeze_at", {
          time: format(row.original.frozen_at),
        })}
      </div>
    </div>
  );
}

function ActionsCell({ row }: { row: Row<GameDetail> }) {
  const { t } = useTranslation();

  const id = row.original.id;
  const title = row.original.title;

  const sharedStore = useSharedStore();
  const [deleteDialogOpen, setDeleteDialogOpen] = useState<boolean>(false);

  const { optimisticEnabled, toggleEnabled } = useRowContext()!;

  function handleEnabledChange() {
    toggleEnabled(title!);
  }

  async function handleDelete() {
    try {
      await deleteGame({ id });
      toast.success(t("game:actions.delete.success", { title }));
      setDeleteDialogOpen(false);
    } finally {
      sharedStore?.setRefresh();
    }
  }

  return (
    <div className={cn(["flex", "items-center", "justify-center", "gap-2"])}>
      <Tooltip>
        <TooltipTrigger asChild>
          <Button
            variant="ghost"
            size="sm"
            square
            icon={<EditIcon />}
            aria-label={t("game:actions.update._")}
            asChild
          >
            <Link to={`/admin/games/${id}`} />
          </Button>
        </TooltipTrigger>
        <TooltipContent>{t("game:actions.update._")}</TooltipContent>
      </Tooltip>

      <DropdownMenu>
        <DropdownMenuTrigger asChild>
          <Button
            square
            size={"sm"}
            variant={"ghost"}
            icon={<EllipsisIcon />}
            aria-label={t("game:actions._")}
          />
        </DropdownMenuTrigger>
        <DropdownMenuContent>
          <DropdownMenuItem onClick={handleEnabledChange}>
            {optimisticEnabled ? <EyeClosedIcon /> : <EyeIcon />}
            {optimisticEnabled
              ? t("game:enabled.actions.false")
              : t("game:enabled.actions.true")}
          </DropdownMenuItem>
          <DropdownMenuItem
            onClick={() => setDeleteDialogOpen(true)}
            className={cn(["text-error"])}
          >
            <TrashIcon />
            {t("game:actions.delete._")}
          </DropdownMenuItem>
        </DropdownMenuContent>
      </DropdownMenu>

      <Dialog open={deleteDialogOpen} onOpenChange={setDeleteDialogOpen}>
        <DialogContent>
          <Card
            className={cn([
              "w-full",
              "max-w-xl",
              "rounded-elevated",
              "shadow-lg",
              "overflow-hidden",
              "flex",
              "flex-col",
            ])}
          >
            <div className={cn(["p-5", "flex", "flex-col", "gap-5"])}>
              <div className={cn(["flex", "items-center", "gap-3"])}>
                <div
                  className={cn([
                    "flex items-center justify-center",
                    "size-10 rounded-badge",
                    "bg-error/10 text-error",
                    "shrink-0",
                  ])}
                >
                  <TrashIcon className={cn(["size-5"])} />
                </div>
                <h3 className={cn(["text-base", "font-semibold"])}>
                  {t("game:actions.delete._")}
                </h3>
              </div>
              <p className={cn(["text-sm"])}>
                <Trans
                  i18nKey="game:actions.delete.message"
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
                  {t("common:actions.confirm")}
                </Button>
              </div>
            </div>
          </Card>
        </DialogContent>
      </Dialog>
    </div>
  );
}

function ScheduleHeader({ column }: { column: Column<GameDetail> }) {
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
    <Button
      icon={icon}
      variant="ghost"
      size="sm"
      className={cn(["-ml-3", "px-3", "text-muted-foreground"])}
      onClick={() => column.toggleSorting()}
    >
      {t("game:list.columns.schedule")}
    </Button>
  );
}

function useColumns() {
  const { t, i18n } = useTranslation();
  const language = i18n.resolvedLanguage ?? i18n.language;
  const formatter = useMemo(
    () =>
      new Intl.DateTimeFormat(language, {
        month: "2-digit",
        day: "2-digit",
        hour: "2-digit",
        minute: "2-digit",
      }),
    [language]
  );

  const columns: Array<ColumnDef<GameDetail>> = useMemo(() => {
    return [
      {
        id: "game",
        accessorFn: (game) => game.title,
        header: () => t("game:list.columns.game"),
        cell: GameCell,
      },
      {
        id: "status",
        header: () => t("game:list.columns.status"),
        cell: StatusCell,
      },
      {
        accessorKey: "started_at",
        id: "started_at",
        header: ScheduleHeader,
        cell: ({ row }) => <ScheduleCell row={row} formatter={formatter} />,
      },
      {
        id: "actions",
        header: () => (
          <div className={cn(["justify-self-center"])}>
            {t("game:actions._")}
          </div>
        ),
        cell: ActionsCell,
      },
    ];
  }, [formatter, t]);

  return columns;
}

export { RowProvider, useColumns };
