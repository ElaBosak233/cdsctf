import {
  ClipboardCheckIcon,
  ClipboardCopyIcon,
  EditIcon,
  SettingsIcon,
  TrashIcon,
} from "lucide-react";
import {
  useContext,
  useMemo,
  useOptimistic,
  useState,
  useTransition,
} from "react";
import { Trans, useTranslation } from "react-i18next";
import { Link, useParams } from "react-router";
import { toast } from "sonner";
import {
  deleteGameChallenge,
  updateGameChallenge,
} from "@/api/admin/games/game_id/challenges/challenge_id";
import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";
import {
  Dialog,
  DialogBody,
  DialogContent,
  DialogFooter,
  DialogHeader,
} from "@/components/ui/dialog";
import { Switch } from "@/components/ui/switch";
import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from "@/components/ui/tooltip";
import { useClipboard } from "@/hooks/use-clipboard";
import type { ColumnDef, Row } from "@/hooks/use-data-table";
import type { GameChallengeView } from "@/models/game_challenge";
import { useSharedStore } from "@/storages/shared";
import { cn } from "@/utils";
import { getCategory } from "@/utils/category";
import { notifyApiError, parseRouteNumericId } from "@/utils/query";
import { Context } from "../../context";
import { EditDialog } from "./edit-dialog";

function IsEnabledCell({ row }: { row: Row<GameChallengeView> }) {
  const { t } = useTranslation();
  const { game_id } = useParams<{ game_id: string }>();
  const routeGameId = parseRouteNumericId(game_id);
  const { game } = useContext(Context);
  const title = row.original.challenge_title;
  const challenge_id = row.original.challenge_id;
  const [isPending, startTransition] = useTransition();
  const [isEnabled, setIsEnabled] = useState(row.original.enabled);
  const [optimisticEnabled, setOptimisticEnabled] = useOptimistic(isEnabled);

  function handlePublicnessChange() {
    const gid = routeGameId ?? game?.id ?? row.original.game_id;
    if (gid == null || challenge_id == null) return;

    const newValue = !optimisticEnabled;
    startTransition(async () => {
      setOptimisticEnabled(newValue);
      await updateGameChallenge({
        game_id: gid,
        challenge_id,
        enabled: newValue,
      });
      setIsEnabled(newValue);
      const enabledLabel = newValue
        ? t("game:enabled.true")
        : t("game:enabled.false");
      toast.success(
        t("game:challenge.enabled.toast", { enabled: enabledLabel, title }),
        { id: "publicness_change" }
      );
    });
  }

  return (
    <Switch
      checked={optimisticEnabled}
      onCheckedChange={handlePublicnessChange}
      disabled={isPending}
      aria-label={t("game:challenge.enabled.aria_label")}
    />
  );
}

function ChallengeCell({ row }: { row: Row<GameChallengeView> }) {
  const challenge = row.original;
  const id = challenge.challenge_id!;
  const category = getCategory(challenge.challenge_category);
  const CategoryIcon = category.icon!;
  const { t } = useTranslation();
  const { isCopied, copyToClipboard } = useClipboard();
  return (
    <div className={cn(["flex", "min-w-0", "items-center", "gap-3"])}>
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
        <CategoryIcon className="size-4" />
      </div>
      <div className={cn(["min-w-0", "flex-1"])}>
        <div className={cn(["truncate", "text-sm", "font-semibold"])}>
          {challenge.challenge_title || "-"}
        </div>
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
            <TooltipTrigger
              render={
                <Button
                  icon={
                    isCopied ? <ClipboardCheckIcon /> : <ClipboardCopyIcon />
                  }
                  square
                  size="sm"
                  variant="ghost"
                  className={cn([
                    "size-6",
                    "shrink-0",
                    "text-muted-foreground",
                  ])}
                  aria-label={t("common:tooltip.copy")}
                  onClick={() => copyToClipboard(String(id))}
                />
              }
            ></TooltipTrigger>
            <TooltipContent>{t("common:tooltip.copy")}</TooltipContent>
          </Tooltip>
        </div>
      </div>
    </div>
  );
}

function ActionsCell({ row }: { row: Row<GameChallengeView> }) {
  const { t } = useTranslation();

  const { game_id } = useParams<{ game_id: string }>();
  const routeGameId = parseRouteNumericId(game_id);
  const { game } = useContext(Context);
  const challenge_id = row.original.challenge_id;
  const title = row.original.challenge_title;

  const sharedStore = useSharedStore();

  const [editDialogOpen, setEditDialogOpen] = useState<boolean>(false);
  const [deleteDialogOpen, setDeleteDialogOpen] = useState<boolean>(false);

  async function handleDelete() {
    const gid = routeGameId ?? game?.id ?? row.original.game_id;
    if (gid == null || challenge_id == null) return;

    try {
      await deleteGameChallenge({ game_id: gid, challenge_id });
      toast.success(
        t("game:actions.delete.success", {
          title,
        })
      );
      setDeleteDialogOpen(false);
    } catch (error) {
      await notifyApiError(error);
    } finally {
      sharedStore?.setRefresh();
    }
  }

  return (
    <div
      className={cn([
        "flex",
        "items-center",
        "justify-center",
        "gap-2",
        "*:shrink-0",
      ])}
    >
      <Button
        variant={"ghost"}
        size={"sm"}
        square
        icon={<SettingsIcon />}
        onClick={() => setEditDialogOpen(true)}
      />
      <Dialog open={editDialogOpen} onOpenChange={setEditDialogOpen}>
        <DialogContent size="wide">
          <EditDialog
            gameChallenge={row.original}
            onClose={() => setEditDialogOpen(false)}
          />
        </DialogContent>
      </Dialog>
      <Button
        icon={<EditIcon />}
        square
        size={"sm"}
        render={<Link to={`/admin/challenges/${row.original.challenge_id}`} />}
      />
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
              "w-full",
              "max-w-xl",
              "rounded-elevated",
              "shadow-lg",
              "overflow-hidden",
              "flex",
              "flex-col",
            ])}
          >
            <DialogHeader
              className="p-5 pb-0"
              icon={<TrashIcon />}
              level="error"
              title={t("game:challenge.actions.delete._")}
            />
            <DialogBody className="px-5">
              <p className="text-sm leading-relaxed text-muted-foreground">
                <Trans
                  i18nKey="game:challenge.actions.delete.message"
                  values={{ title }}
                  components={{
                    muted: <span className={cn(["text-muted-foreground"])} />,
                  }}
                />
              </p>
            </DialogBody>
            <DialogFooter className="p-5 pt-0">
              <Button
                level={"error"}
                variant={"solid"}
                size={"sm"}
                onClick={handleDelete}
              >
                {t("common:actions.confirm")}
              </Button>
            </DialogFooter>
          </Card>
        </DialogContent>
      </Dialog>
    </div>
  );
}

function useColumns() {
  const { t } = useTranslation();

  const columns: Array<ColumnDef<GameChallengeView>> = useMemo(() => {
    return [
      {
        accessorKey: "game_id",
        enableHiding: false,
      },
      {
        accessorKey: "enabled",
        header: t("game:challenge.enabled._"),
        cell: IsEnabledCell,
      },
      {
        id: "challenge_id",
        header: t("challenge:title"),
        cell: ChallengeCell,
      },
      {
        id: "challenge_category",
        header: t("challenge:category"),
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
        header: t("game:challenge.pts"),
        cell: ({ row }) => (
          <span>
            {row.original.pts}{" "}
            <span className={cn(["text-muted-foreground"])}>
              {t("game:filter.pts")}
            </span>
          </span>
        ),
      },
      {
        id: "actions",
        header: () => (
          <div className={cn(["justify-self-center"])}>
            {t("game:challenge.actions._")}
          </div>
        ),
        cell: ActionsCell,
      },
    ];
  }, [t]);

  return columns;
}

export { useColumns };
