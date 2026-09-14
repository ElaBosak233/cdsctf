import { keepPreviousData, useQuery } from "@tanstack/react-query";
import { HashIcon, LibraryIcon, PlusCircleIcon } from "lucide-react";
import { useContext, useState } from "react";
import { useTranslation } from "react-i18next";
import { useParams } from "react-router";
import { getGameChallenges } from "@/api/admin/games/game_id/challenges";
import { Button } from "@/components/ui/button";
import { Dialog, DialogContent } from "@/components/ui/dialog";
import { Field, FieldIcon } from "@/components/ui/field";
import { LoadingOverlay } from "@/components/ui/loading-overlay";
import { ScrollArea } from "@/components/ui/scroll-area";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import { TextField } from "@/components/ui/text-field";
import {
  type ColumnFiltersState,
  type ColumnVisibilityState,
  flexRender,
  type SortingState,
  useDataTable,
} from "@/hooks/use-data-table";
import { useDebounce } from "@/hooks/use-debounce";
import type { GameChallengeView } from "@/models/game_challenge";
import { useSharedStore } from "@/storages/shared";
import { cn } from "@/utils";
import { categories } from "@/utils/category";
import { parseRouteNumericId } from "@/utils/query";
import { Context } from "../context";
import { useColumns } from "./_blocks/columns";
import { CreateDialog } from "./_blocks/create-dialog";

export default function Index() {
  const { t } = useTranslation();

  const sharedStore = useSharedStore();

  const { game_id } = useParams<{ game_id: string }>();
  const routeGameId = parseRouteNumericId(game_id);
  const { game } = useContext(Context);

  const [createDialogOpen, setCreateDialogOpen] = useState<boolean>(false);

  const [sorting, setSorting] = useState<SortingState>([]);
  const [columnVisibility, setColumnVisibility] =
    useState<ColumnVisibilityState>({
      game_id: false,
    });
  const [columnFilters, setColumnFilters] = useState<ColumnFiltersState>([
    {
      id: "challenge_category",
      value: "all",
    },
  ]);
  const debouncedColumnFilters = useDebounce(columnFilters, 100);
  const gameId = routeGameId ?? game?.id;
  const challengeQuery = useQuery({
    queryKey: [
      "admin",
      "game-challenges",
      gameId,
      sorting,
      debouncedColumnFilters,
      sharedStore.refresh,
    ],
    queryFn: () =>
      getGameChallenges({
        game_id: gameId!,
        challenge_id: debouncedColumnFilters.find(
          (c) => c.id === "challenge_id"
        )?.value as number,
        category:
          (debouncedColumnFilters.find((c) => c.id === "challenge_category")
            ?.value as string) !== "all"
            ? (debouncedColumnFilters.find((c) => c.id === "challenge_category")
                ?.value as number)
            : undefined,
      }),
    enabled: gameId != null,
    placeholderData: keepPreviousData,
  });
  const challenges = challengeQuery.data?.challenges ?? [];
  const loading = challengeQuery.isFetching;

  const columns = useColumns();
  const table = useDataTable<GameChallengeView>({
    data: challenges,
    columns,
    manualPagination: true,
    manualFiltering: true,
    onColumnFiltersChange: setColumnFilters,
    onColumnVisibilityChange: setColumnVisibility,
    manualSorting: true,
    onSortingChange: setSorting,
    state: {
      sorting,
      columnVisibility,
      columnFilters,
    },
  });

  return (
    <div
      className={cn([
        "h-full",
        "w-full",
        "min-w-0",
        "min-h-0",
        "flex",
        "flex-col",
        "gap-4",
        "px-4",
        "py-4",
        "sm:px-6",
        "sm:py-6",
        "lg:px-8",
        "lg:py-8",
      ])}
    >
      <div
        className={cn([
          "flex",
          "justify-between",
          "items-center",
          "shrink-0",
          "gap-6",
        ])}
      >
        <h1
          className={cn([
            "text-2xl",
            "font-bold",
            "flex",
            "gap-2",
            "items-center",
          ])}
        >
          <LibraryIcon />
          {t("challenge:_")}
        </h1>
        <div
          className={cn([
            "flex",
            "flex-1",
            "justify-center",
            "items-center",
            "gap-3",
          ])}
        >
          <Field size={"sm"} className={cn(["flex-1"])}>
            <FieldIcon>
              <HashIcon />
            </FieldIcon>
            <TextField
              placeholder={t("challenge:form.id._")}
              value={
                (table.getColumn("challenge_id")?.getFilterValue() as string) ??
                ""
              }
              onChange={(e) =>
                table.getColumn("challenge_id")?.setFilterValue(e.target.value)
              }
            />
          </Field>
          <Field size={"sm"} className={cn(["flex-1"])}>
            <FieldIcon>
              <LibraryIcon />
            </FieldIcon>
            <Select
              value={
                (table
                  .getColumn("challenge_category")
                  ?.getFilterValue() as string) ?? ""
              }
              onValueChange={(value) =>
                table.getColumn("challenge_category")?.setFilterValue(value)
              }
            >
              <SelectTrigger>
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="all">{t("common:all")}</SelectItem>
                {(categories || []).map((category) => {
                  const Icon = category.icon!;
                  return (
                    <SelectItem key={category.id} value={String(category.id)}>
                      <Icon />
                      {category.name?.toUpperCase()}
                    </SelectItem>
                  );
                })}
              </SelectContent>
            </Select>
          </Field>

          <Button
            icon={<PlusCircleIcon />}
            variant={"solid"}
            onClick={() => setCreateDialogOpen(true)}
          >
            {t("common:actions.add")}
          </Button>
          <Dialog open={createDialogOpen} onOpenChange={setCreateDialogOpen}>
            <DialogContent size="wide">
              <CreateDialog onClose={() => setCreateDialogOpen(false)} />
            </DialogContent>
          </Dialog>
        </div>
      </div>

      <div className={cn(["flex-1", "min-h-0", "flex", "flex-col"])}>
        <ScrollArea
          className={cn([
            "flex-1",
            "min-h-0",
            "min-w-0",
            "w-full",
            "overflow-hidden",
            "rounded-lg",
            "border",
            "ring-1",
            "ring-border/50",
            "shadow-sm",
          ])}
        >
          <LoadingOverlay loading={loading} />
          <Table
            className={cn([
              "w-full",
              "min-w-4xl",
              "table-fixed",
              "text-foreground",
            ])}
          >
            <TableHeader
              className={cn([
                "sticky",
                "top-0",
                "z-2",
                "bg-muted/80",
                "backdrop-blur-sm",
                "border-b",
              ])}
            >
              {table.getHeaderGroups().map((headerGroup) => (
                <TableRow key={headerGroup.id}>
                  {headerGroup.headers.map((header) => {
                    return (
                      <TableHead
                        key={header.id}
                        className={cn([
                          "bg-muted/95",
                          header.column.id === "actions" && [
                            "sticky",
                            "right-0",
                            "z-3",
                            "bg-muted/95",
                          ],
                          header.column.id === "enabled" && "w-1 px-2",
                          header.column.id === "challenge_id" && "w-24",
                          header.column.id === "challenge_title" && "min-w-64",
                          header.column.id === "challenge_category" && "w-44",
                          header.column.id === "pts" && "w-24",
                          header.column.id === "actions" && "w-12",
                        ])}
                      >
                        {!header.isPlaceholder &&
                          flexRender(
                            header.column.columnDef.header,
                            header.getContext()
                          )}
                      </TableHead>
                    );
                  })}
                </TableRow>
              ))}
            </TableHeader>
            <TableBody>
              {table.getRowModel().rows?.length
                ? table.getRowModel().rows.map((row) => (
                    <TableRow
                      key={row.original.challenge_id}
                      data-state={row.getIsSelected() && "selected"}
                      className={cn([
                        "group",
                        "transition-colors",
                        "hover:bg-transparent",
                      ])}
                    >
                      {row.getVisibleCells().map((cell) => (
                        <TableCell
                          key={cell.id}
                          className={cn([
                            "py-3",
                            "transition-colors",
                            "group-hover:bg-muted/50",
                            cell.column.id === "enabled" && "px-2",
                            cell.column.id === "actions" && [
                              "sticky",
                              "right-0",
                              "z-1",
                              "w-28",
                              "bg-card",
                            ],
                          ])}
                        >
                          {flexRender(
                            cell.column.columnDef.cell,
                            cell.getContext()
                          )}
                        </TableCell>
                      ))}
                    </TableRow>
                  ))
                : !loading && (
                    <TableRow>
                      <TableCell
                        colSpan={columns.length}
                        className={cn(["h-24", "text-center"])}
                      >
                        {t("game:challenge.empty")}
                      </TableCell>
                    </TableRow>
                  )}
            </TableBody>
          </Table>
        </ScrollArea>
      </div>
    </div>
  );
}
