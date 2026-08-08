import {
  ArrowDownIcon,
  ArrowUpDownIcon,
  ArrowUpIcon,
  BookOpenCheckIcon,
  BoxIcon,
  CalendarPlusIcon,
  ClipboardCheckIcon,
  ClipboardCopyIcon,
  Clock3Icon,
  EditIcon,
  EllipsisIcon,
  EyeClosedIcon,
  EyeIcon,
  Globe2Icon,
  LockKeyholeIcon,
  PaperclipIcon,
  ShipWheelIcon,
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
import {
  deleteChallenge,
  updateChallenge,
} from "@/api/admin/challenges/challenge_id";
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
import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from "@/components/ui/tooltip";
import { useClipboard } from "@/hooks/use-clipboard";
import type { ChallengeDetail } from "@/models/challenge";
import { useSharedStore } from "@/storages/shared";
import { cn } from "@/utils";
import { getCategory } from "@/utils/category";
import type { Column, ColumnDef, Row } from "@/utils/data-table";

const RowContext = createContext<{
  optimisticPublic: boolean;
  togglePublic: (title: string) => void;
} | null>(null);

function useRowContext() {
  return useContext(RowContext);
}

function RowProvider({
  challenge,
  children,
}: {
  challenge: ChallengeDetail;
  children: ReactNode;
}) {
  const { t } = useTranslation();
  const [isPublic, setIsPublic] = useState(challenge.public ?? false);
  const [, startTransition] = useTransition();
  const [optimisticPublic, setOptimisticPublic] = useOptimistic(isPublic);

  const togglePublic = useCallback(
    (title: string) => {
      const newValue = !optimisticPublic;
      startTransition(async () => {
        setOptimisticPublic(newValue);
        await updateChallenge({
          id: challenge.id,
          public: newValue,
        });
        setIsPublic(newValue);
        toast.success(t("challenge:public.actions.success", { title }), {
          id: "publicness_change",
        });
      });
    },
    [optimisticPublic, challenge.id, setOptimisticPublic, t]
  );

  return (
    <RowContext.Provider value={{ optimisticPublic, togglePublic }}>
      {children}
    </RowContext.Provider>
  );
}

function ChallengeCell({ row }: { row: Row<ChallengeDetail> }) {
  const challenge = row.original;
  const category = getCategory(challenge.category);
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
        ])}
        style={{
          backgroundColor: `${category.color}1a`,
          color: category.color,
        }}
      >
        <CategoryIcon className="size-4" />
      </div>
      <div className={cn(["min-w-0", "flex-1"])}>
        <Link
          to={`/admin/challenges/${challenge.id}`}
          className={cn([
            "block",
            "truncate",
            "text-sm",
            "font-semibold",
            "hover:underline",
            "underline-offset-4",
          ])}
        >
          {challenge.title || "-"}
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
          <span className={cn(["shrink-0", "font-mono"])}>#{challenge.id}</span>
          <Tooltip>
            <TooltipTrigger asChild>
              <Button
                icon={isCopied ? <ClipboardCheckIcon /> : <ClipboardCopyIcon />}
                square
                size="sm"
                variant="ghost"
                className={cn(["size-6", "shrink-0", "text-muted-foreground"])}
                aria-label={t("common:tooltip.copy")}
                onClick={() => copyToClipboard(String(challenge.id))}
              />
            </TooltipTrigger>
            <TooltipContent>{t("common:tooltip.copy")}</TooltipContent>
          </Tooltip>
          {challenge.description && (
            <>
              <span className="shrink-0">·</span>
              <span className="truncate">{challenge.description}</span>
            </>
          )}
        </div>
      </div>
    </div>
  );
}

function ClassificationCell({ row }: { row: Row<ChallengeDetail> }) {
  const category = getCategory(row.original.category);
  const CategoryIcon = category.icon!;

  return (
    <div className={cn(["flex", "min-w-0", "flex-col", "gap-1.5"])}>
      <div className={cn(["flex", "items-center", "gap-1.5", "text-sm"])}>
        <CategoryIcon className="size-3.5" style={{ color: category.color }} />
        <span className="font-medium">{category.name?.toUpperCase()}</span>
      </div>
      {row.original.tags.length > 0 && (
        <div className={cn(["flex", "min-w-0", "gap-1", "overflow-hidden"])}>
          {row.original.tags.map((tag, index) => (
            <Badge
              key={`${tag}-${index}`}
              variant="outline"
              className={cn(["max-w-24", "truncate", "text-muted-foreground"])}
            >
              {tag}
            </Badge>
          ))}
        </div>
      )}
    </div>
  );
}

function StatusCell({ row }: { row: Row<ChallengeDetail> }) {
  const { t } = useTranslation();
  const { optimisticPublic } = useRowContext()!;

  return (
    <div className={cn(["flex", "flex-wrap", "items-center", "gap-1.5"])}>
      <Badge
        variant="outline"
        className={cn(
          optimisticPublic
            ? ["border-success/20", "bg-success/10", "text-success"]
            : ["text-muted-foreground"]
        )}
      >
        {optimisticPublic ? <Globe2Icon /> : <LockKeyholeIcon />}
        {t(
          optimisticPublic
            ? "challenge:list.visibility.public"
            : "challenge:list.visibility.private"
        )}
      </Badge>
      <Badge
        variant="outline"
        className={cn(
          row.original.has_instance
            ? ["border-info/20", "bg-info/10", "text-info"]
            : ["text-muted-foreground"]
        )}
      >
        {row.original.has_instance ? <ShipWheelIcon /> : <BoxIcon />}
        {t(
          row.original.has_instance
            ? "challenge:has_instance.true"
            : "challenge:has_instance.false"
        )}
      </Badge>
      {row.original.has_attachment && (
        <Badge variant="outline" className="text-muted-foreground">
          <PaperclipIcon />
          {t("challenge:has_attachment")}
        </Badge>
      )}
      {row.original.has_writeup && (
        <Badge variant="outline" className="text-muted-foreground">
          <BookOpenCheckIcon />
          {t("challenge:edit.writeup")}
        </Badge>
      )}
    </div>
  );
}

function TimeCell({
  row,
  formatter,
}: {
  row: Row<ChallengeDetail>;
  formatter: Intl.DateTimeFormat;
}) {
  const { t } = useTranslation();
  const format = (timestamp: number) =>
    formatter.format(new Date(timestamp * 1000));

  return (
    <div className={cn(["flex", "flex-col", "gap-1", "whitespace-nowrap"])}>
      <div className={cn(["flex", "items-center", "gap-1.5", "text-sm"])}>
        <Clock3Icon className="size-3.5 text-muted-foreground" />
        <span>{format(row.original.updated_at)}</span>
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
        <CalendarPlusIcon className="size-3.5" />
        {t("challenge:list.created_at", {
          time: format(row.original.created_at),
        })}
      </div>
    </div>
  );
}

function ActionsCell({ row }: { row: Row<ChallengeDetail> }) {
  const { t } = useTranslation();
  const challenge = row.original;
  const sharedStore = useSharedStore();
  const [deleteDialogOpen, setDeleteDialogOpen] = useState(false);
  const { optimisticPublic, togglePublic } = useRowContext()!;

  async function handleDelete() {
    try {
      await deleteChallenge({ id: challenge.id });
      toast.success(
        t("challenge:actions.delete.success", { title: challenge.title })
      );
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
            aria-label={t("challenge:edit._")}
            asChild
          >
            <Link to={`/admin/challenges/${challenge.id}`} />
          </Button>
        </TooltipTrigger>
        <TooltipContent>{t("challenge:edit._")}</TooltipContent>
      </Tooltip>

      <DropdownMenu>
        <DropdownMenuTrigger asChild>
          <Button
            square
            size="sm"
            variant="ghost"
            icon={<EllipsisIcon />}
            aria-label={t("challenge:actions._")}
          />
        </DropdownMenuTrigger>
        <DropdownMenuContent>
          <DropdownMenuItem onClick={() => togglePublic(challenge.title)}>
            {optimisticPublic ? <EyeClosedIcon /> : <EyeIcon />}
            {optimisticPublic
              ? t("challenge:public.actions.false")
              : t("challenge:public.actions.true")}
          </DropdownMenuItem>
          <DropdownMenuItem
            onClick={() => setDeleteDialogOpen(true)}
            className="text-error"
          >
            <TrashIcon />
            {t("challenge:actions.delete._")}
          </DropdownMenuItem>
        </DropdownMenuContent>
      </DropdownMenu>

      <Dialog open={deleteDialogOpen} onOpenChange={setDeleteDialogOpen}>
        <DialogContent>
          <Card
            className={cn([
              "flex",
              "w-full",
              "max-w-xl",
              "flex-col",
              "overflow-hidden",
              "rounded-elevated",
              "shadow-lg",
            ])}
          >
            <div className={cn(["flex", "flex-col", "gap-5", "p-5"])}>
              <div className={cn(["flex", "items-center", "gap-3"])}>
                <div
                  className={cn([
                    "flex",
                    "size-10",
                    "shrink-0",
                    "items-center",
                    "justify-center",
                    "rounded-badge",
                    "bg-error/10",
                    "text-error",
                  ])}
                >
                  <TrashIcon className="size-5" />
                </div>
                <h3 className="text-base font-semibold">
                  {t("challenge:actions.delete._")}
                </h3>
              </div>
              <p className="text-sm">
                <Trans
                  i18nKey="challenge:actions.delete.message"
                  values={{ title: challenge.title }}
                  components={{
                    muted: <span className="text-muted-foreground" />,
                  }}
                />
              </p>
              <div className="flex justify-end">
                <Button
                  level="error"
                  variant="solid"
                  size="sm"
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

function UpdatedAtHeader({ column }: { column: Column<ChallengeDetail> }) {
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
      {t("challenge:list.columns.time")}
    </Button>
  );
}

function useColumns() {
  const { t, i18n } = useTranslation();
  const language = i18n.resolvedLanguage ?? i18n.language;
  const formatter = useMemo(
    () =>
      new Intl.DateTimeFormat(language, {
        year: "numeric",
        month: "2-digit",
        day: "2-digit",
        hour: "2-digit",
        minute: "2-digit",
      }),
    [language]
  );

  const columns: Array<ColumnDef<ChallengeDetail>> = useMemo(
    () => [
      {
        id: "challenge",
        accessorFn: (challenge) => challenge.title,
        header: () => t("challenge:list.columns.challenge"),
        cell: ChallengeCell,
      },
      {
        id: "classification",
        header: () => t("challenge:list.columns.classification"),
        cell: ClassificationCell,
      },
      {
        id: "status",
        header: () => t("challenge:list.columns.status"),
        cell: StatusCell,
      },
      {
        accessorKey: "updated_at",
        id: "updated_at",
        header: UpdatedAtHeader,
        cell: ({ row }) => <TimeCell row={row} formatter={formatter} />,
      },
      {
        id: "actions",
        header: () => (
          <div className="justify-self-center">{t("challenge:actions._")}</div>
        ),
        cell: ActionsCell,
      },
    ],
    [formatter, t]
  );

  return columns;
}

export { RowProvider, useColumns };
