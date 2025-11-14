import {
  type ColumnFiltersState,
  flexRender,
  getCoreRowModel,
  getFilteredRowModel,
  type SortingState,
  useReactTable,
  type VisibilityState,
} from "@tanstack/react-table";
import { HashIcon, LibraryIcon, PlusCircleIcon } from "lucide-react";
import { useContext, useEffect, useState } from "react";
import { useTranslation } from "react-i18next";
import { getGameChallenges } from "@/api/admin/games/game_id/challenges";
import { Button } from "@/components/ui/button";
import { Dialog, DialogContent } from "@/components/ui/dialog";
import { Field, FieldIcon } from "@/components/ui/field";
import { LoadingOverlay } from "@/components/ui/loading-overlay";
import { ScrollArea } from "@/components/ui/scroll-area";
import { Select } from "@/components/ui/select";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import { TextField } from "@/components/ui/text-field";
import { useDebounce } from "@/hooks/use-debounce";
import type { GameChallenge } from "@/models/game_challenge";
import { useSharedStore } from "@/storages/shared";
import { cn } from "@/utils";
import { categories } from "@/utils/category";
import { Context } from "../context";
import { useColumns } from "./columns";
import { CreateDialog } from "./create-dialog";

export default function Index() {
  const { t } = useTranslation();

  const sharedStore = useSharedStore();

  const { game } = useContext(Context);

  const [createDialogOpen, setCreateDialogOpen] = useState<boolean>(false);

  const [challenges, setChallenges] = useState<Array<GameChallenge>>([]);
  const [loading, setLoading] = useState<boolean>(false);

  const [sorting, setSorting] = useState<SortingState>([]);
  const [columnVisibility, setColumnVisibility] = useState<VisibilityState>({
    game_id: false,
  });
  const [columnFilters, setColumnFilters] = useState<ColumnFiltersState>([
    {
      id: "challenge_category",
      value: "all",
    },
  ]);
  const debouncedColumnFilters = useDebounce(columnFilters, 100);

  const columns = useColumns();
  const table = useReactTable<GameChallenge>({
    data: challenges,
    columns,
    getCoreRowModel: getCoreRowModel(),
    manualPagination: true,
    manualFiltering: true,
    getFilteredRowModel: getFilteredRowModel(),
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

  useEffect(() => {
    void sharedStore.refresh;

    if (!game) return;

    setLoading(true);
    getGameChallenges({
      game_id: game.id!,
      challenge_id: debouncedColumnFilters.find((c) => c.id === "challenge_id")
        ?.value as number,
      category:
        (debouncedColumnFilters.find((c) => c.id === "challenge_category")
          ?.value as string) !== "all"
          ? (debouncedColumnFilters.find((c) => c.id === "challenge_category")
              ?.value as number)
          : undefined,
    })
      .then((res) => {
        setChallenges(res?.data || []);
      })
      .finally(() => {
        setLoading(false);
      });
  }, [debouncedColumnFilters, sharedStore.refresh, game]);

  return (
    <div className={cn(["container", "mx-auto", "flex-1", "flex", "flex-col"])}>
      <div
        className={cn([
          "flex",
          "justify-between",
          "items-center",
          "mb-6",
          "gap-10",
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
          {t("challenge._")}
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
              placeholder={"ID"}
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
              options={[
                {
                  value: "all",
                  content: (
                    <div className={cn(["flex", "gap-2", "items-center"])}>
                      {t("common.all")}
                    </div>
                  ),
                },
                ...(categories || []).map((category) => {
                  const Icon = category.icon!;

                  return {
                    value: String(category?.id),
                    content: (
                      <div className={cn(["flex", "gap-2", "items-center"])}>
                        <Icon />
                        {category?.name?.toUpperCase()}
                      </div>
                    ),
                  };
                }),
              ]}
              onValueChange={(value) =>
                table.getColumn("challenge_category")?.setFilterValue(value)
              }
              value={
                (table
                  .getColumn("challenge_category")
                  ?.getFilterValue() as string) ?? ""
              }
            />
          </Field>

          <Button
            icon={<PlusCircleIcon />}
            variant={"solid"}
            onClick={() => setCreateDialogOpen(true)}
          >
            {t("common.actions.add")}
          </Button>
          <Dialog open={createDialogOpen} onOpenChange={setCreateDialogOpen}>
            <DialogContent>
              <CreateDialog onClose={() => setCreateDialogOpen(false)} />
            </DialogContent>
          </Dialog>
        </div>
      </div>

      <ScrollArea
        className={cn([
          "rounded-md",
          "border",
          "bg-card",
          "min-h-100",
          "h-[calc(100vh-13rem)]",
        ])}
      >
        <LoadingOverlay loading={loading} />
        <Table className={cn(["text-foreground"])}>
          <TableHeader
            className={cn([
              "sticky",
              "top-0",
              "z-2",
              "bg-muted/70",
              "backdrop-blur-md",
            ])}
          >
            {table.getHeaderGroups().map((headerGroup) => (
              <TableRow key={headerGroup.id}>
                {headerGroup.headers.map((header) => {
                  return (
                    <TableHead key={header.id}>
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
                  >
                    {row.getVisibleCells().map((cell) => (
                      <TableCell key={cell.id}>
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
                      {t("game.challenge.empty")}
                    </TableCell>
                  </TableRow>
                )}
          </TableBody>
        </Table>
      </ScrollArea>
    </div>
  );
}
