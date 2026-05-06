import type { Column, ColumnDef, Row } from "@tanstack/react-table";
import {
  ArrowDownIcon,
  ArrowUpDownIcon,
  ArrowUpIcon,
  BoxIcon,
  CheckIcon,
  ClipboardCheckIcon,
  ClipboardCopyIcon,
  EditIcon,
  EyeClosedIcon,
  EyeIcon,
  LockIcon,
  ShipWheelIcon,
  TrashIcon,
  XIcon,
} from "lucide-react";
import {
  createContext,
  useCallback,
  useContext,
  useMemo,
  useOptimistic,
  useState,
  useTransition,
  type ReactNode,
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
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from "@/components/ui/tooltip";
import { useClipboard } from "@/hooks/use-clipboard";
import type { Challenge } from "@/models/challenge";
import { useSharedStore } from "@/storages/shared";
import { cn } from "@/utils";
import { getCategory } from "@/utils/category";

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
  challenge: Challenge;
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
    [optimisticPublic, challenge.id, startTransition, setOptimisticPublic, t],
  );

  return (
    <RowContext.Provider value={{ optimisticPublic, togglePublic }}>
      {children}
    </RowContext.Provider>
  );
}

function IdCell({ row }: { row: Row<Challenge> }) {
  const id = row.original.id!;
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
            onClick={() => copyToClipboard(String(id))}
          />
        </TooltipTrigger>
        <TooltipContent>{t("common:tooltip.copy")}</TooltipContent>
      </Tooltip>
    </div>
  );
}

function TitleCell({ row }: { row: Row<Challenge> }) {
  const ctx = useRowContext();
  const isPublic = ctx ? ctx.optimisticPublic : row.original.public ?? false;
  return (
    <div
      className={cn([
        "w-42",
        "flex",
        "gap-2",
        "items-center",
        "overflow-hidden",
        "text-ellipsis",
        "whitespace-nowrap",
      ])}
    >
      {!isPublic && (
        <LockIcon className={cn(["size-[1em]", "text-warning"])} />
      )}
      {row.original.title || "-"}
    </div>
  );
}

function UpdatedAtHeader({ column }: { column: Column<Challenge> }) {
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
      {t("challenge:updated_at")}
      <Button
        icon={icon}
        square
        size={"sm"}
        onClick={() => column.toggleSorting()}
      />
    </div>
  );
}

function CreatedAtHeader({ column }: { column: Column<Challenge> }) {
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
      {t("challenge:created_at")}
      <Button
        icon={icon}
        square
        size={"sm"}
        onClick={() => column.toggleSorting()}
      />
    </div>
  );
}

function ActionsCell({ row }: { row: Row<Challenge> }) {
  const { t } = useTranslation();

  const id = row.original.id;
  const title = row.original.title;

  const sharedStore = useSharedStore();

  const [deleteDialogOpen, setDeleteDialogOpen] = useState<boolean>(false);

  const { optimisticPublic, togglePublic } = useRowContext()!;

  function handlePublicnessChange() {
    togglePublic(title!);
  }

  async function handleDelete() {
    try {
      await deleteChallenge({
        id,
      });

      toast.success(t("challenge:actions.delete.success", { title }));
      setDeleteDialogOpen(false);
    } finally {
      sharedStore?.setRefresh();
    }
  }

  return (
    <div className={cn(["flex", "items-center", "justify-center", "gap-2"])}>
      <Button variant={"ghost"} size={"sm"} square icon={<EditIcon />} asChild>
        <Link to={`/admin/challenges/${id}`} />
      </Button>

      <Tooltip>
        <TooltipTrigger asChild>
          <Button
            level={optimisticPublic ? "warning" : "success"}
            variant={"ghost"}
            size={"sm"}
            square
            icon={optimisticPublic ? <EyeClosedIcon /> : <EyeIcon />}
            onClick={handlePublicnessChange}
          />
        </TooltipTrigger>
        <TooltipContent>
          {optimisticPublic
            ? t("challenge:public.actions.false")
            : t("challenge:public.actions.true")}
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
              {t("challenge:actions.delete._")}
            </div>
            <p className={cn(["text-sm"])}>
              <Trans
                i18nKey={"challenge:actions.delete.message"}
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
          </Card>
        </DialogContent>
      </Dialog>
    </div>
  );
}

function useColumns() {
  const { t } = useTranslation();

  const columns: Array<ColumnDef<Challenge>> = useMemo(() => {
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
        header: t("challenge:title"),
        cell: TitleCell,
      },
      {
        accessorKey: "category",
        header: t("challenge:category"),
        cell: ({ row }) => {
          const categoryId = row.original.category!;
          const category = getCategory(categoryId);

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
        accessorKey: "tags",
        id: "tags",
        header: t("challenge:tags"),
        cell: ({ row }) => {
          const tags = row.original.tags;

          return (
            <div className={cn(["flex", "flex-wrap", "gap-1", "w-36"])}>
              {tags?.map((tag, index) => (
                <Badge key={index}>{tag}</Badge>
              ))}
            </div>
          );
        },
      },
      {
        accessorKey: "has_attachment",
        header: t("challenge:has_attachment"),
        cell: ({ row }) => {
          const hasAttachment = row.original.has_attachment;

          const options = [
            {
              className: ["bg-warning", "text-warning-foreground"],
              icon: <XIcon />,
            },
            {
              className: ["bg-info", "text-info-foreground"],
              icon: <CheckIcon />,
            },
          ];

          return (
            <Badge className={cn([options[Number(hasAttachment)]?.className])}>
              {options[Number(hasAttachment)]?.icon}
            </Badge>
          );
        },
      },
      {
        accessorKey: "has_instance",
        header: t("challenge:has_instance._"),
        cell: ({ row }) => {
          const hasInstance = row.original.has_instance;

          return (
            <Badge
              className={cn([
                hasInstance
                  ? ["bg-info", "text-info-foreground"]
                  : ["bg-success", "text-success-foreground"],
              ])}
            >
              {hasInstance ? <ShipWheelIcon /> : <BoxIcon />}
              {hasInstance
                ? t("challenge:has_instance.true")
                : t("challenge:has_instance.false")}
            </Badge>
          );
        },
      },
      {
        accessorKey: "updated_at",
        id: "updated_at",
        header: UpdatedAtHeader,
        cell: ({ row }) => (
          <span className={cn(["text-secondary-foreground", "text-sm"])}>
            {new Date(
              row.getValue<number>("updated_at") * 1000
            ).toLocaleString()}
          </span>
        ),
      },
      {
        accessorKey: "created_at",
        id: "created_at",
        header: CreatedAtHeader,
        cell: ({ row }) => (
          <span className={cn(["text-secondary-foreground", "text-sm"])}>
            {new Date(
              row.getValue<number>("created_at") * 1000
            ).toLocaleString()}
          </span>
        ),
      },
      {
        id: "actions",
        header: () => (
          <div className={cn(["justify-self-center"])}>
            {t("challenge:actions._")}
          </div>
        ),
        cell: ActionsCell,
      },
    ];
  }, [t]);

  return columns;
}

export { RowProvider, useColumns };
