import { keepPreviousData, useQuery } from "@tanstack/react-query";
import {
  HashIcon,
  ListOrderedIcon,
  SatelliteIcon,
  TypeIcon,
  UsersRoundIcon,
} from "lucide-react";
import React, { useContext, useState } from "react";
import { useTranslation } from "react-i18next";
import { useParams } from "react-router";
import { getTeams } from "@/api/admin/games/game_id/teams";
import { Field, FieldIcon } from "@/components/ui/field";
import { LoadingOverlay } from "@/components/ui/loading-overlay";
import { Pagination } from "@/components/ui/pagination";
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
  type ExpandedState,
  flexRender,
  type SortingState,
  useDataTable,
} from "@/hooks/use-data-table";
import { useDebounce } from "@/hooks/use-debounce";
import { State, type TeamView } from "@/models/team";
import { useSharedStore } from "@/storages/shared";
import { cn } from "@/utils";
import { parseRouteNumericId } from "@/utils/query";
import { Context } from "../context";
import { useColumns } from "./_blocks/columns";
import { ExpandedCard } from "./_blocks/expanded-card";

export default function Index() {
  const { t } = useTranslation();

  const sharedStore = useSharedStore();

  const { game_id } = useParams<{ game_id: string }>();
  const routeGameId = parseRouteNumericId(game_id);
  const { game } = useContext(Context);

  const [page, setPage] = useState<number>(1);
  const [size, setSize] = useState<number>(10);

  const [sorting, setSorting] = useState<SortingState>([]);
  const [expanded, setExpanded] = useState<ExpandedState>({});
  const [columnVisibility, setColumnVisibility] =
    useState<ColumnVisibilityState>({
      id: false,
      game_id: false,
    });
  const [columnFilters, setColumnFilters] = useState<ColumnFiltersState>([
    {
      id: "state",
      value: "all",
    },
  ]);
  const debouncedColumnFilters = useDebounce(columnFilters, 100);

  const gameId = routeGameId ?? game?.id;
  const teamQuery = useQuery({
    queryKey: [
      "admin",
      "game-teams",
      gameId,
      page,
      size,
      sorting,
      debouncedColumnFilters,
      sharedStore.refresh,
    ],
    queryFn: () =>
      getTeams({
        game_id: gameId!,
        id: debouncedColumnFilters.find((c) => c.id === "id")?.value as number,
        name: debouncedColumnFilters.find((c) => c.id === "name")
          ?.value as string,
        state:
          debouncedColumnFilters.find((c) => c.id === "state")?.value !== "all"
            ? Number(
                debouncedColumnFilters.find((c) => c.id === "state")?.value
              )
            : undefined,
        sorts: "rank",
        page,
        size,
      }),
    enabled: gameId != null,
    placeholderData: keepPreviousData,
  });
  const teams = teamQuery.data?.teams ?? [];
  const total = teamQuery.data?.total ?? 0;
  const loading = teamQuery.isFetching;

  const columns = useColumns();

  const table = useDataTable<TeamView>({
    data: teams,
    columns,
    manualPagination: true,
    rowCount: total,
    manualFiltering: true,
    onColumnFiltersChange: setColumnFilters,
    onColumnVisibilityChange: setColumnVisibility,
    manualSorting: true,
    onSortingChange: setSorting,
    getRowCanExpand: () => true,
    onExpandedChange: setExpanded,
    state: {
      sorting,
      columnVisibility,
      columnFilters,
      expanded,
    },
  });

  const stateOptions = [
    { id: State.Banned.toString(), name: t("team:state.banned") },
    { id: State.Preparing.toString(), name: t("team:state.preparing") },
    { id: State.Pending.toString(), name: t("team:state.pending") },
    { id: State.Passed.toString(), name: t("team:state.passed") },
  ];

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
          <UsersRoundIcon />
          {t("team:_")}
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
              placeholder={t("team:id")}
              value={(table.getColumn("id")?.getFilterValue() as string) ?? ""}
              onChange={(e) =>
                table.getColumn("id")?.setFilterValue(e.target.value)
              }
            />
          </Field>
          <Field size={"sm"} className={cn(["flex-1"])}>
            <FieldIcon>
              <TypeIcon />
            </FieldIcon>
            <TextField
              placeholder={t("team:search.name")}
              value={
                (table.getColumn("name")?.getFilterValue() as string) ?? ""
              }
              onChange={(e) =>
                table.getColumn("name")?.setFilterValue(e.target.value)
              }
            />
          </Field>
          <Field size={"sm"} className={cn(["flex-1"])}>
            <FieldIcon>
              <SatelliteIcon />
            </FieldIcon>
            <Select
              value={
                (table.getColumn("state")?.getFilterValue() as string) ?? ""
              }
              onValueChange={(value) =>
                table.getColumn("state")?.setFilterValue(value)
              }
            >
              <SelectTrigger>
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="all">{t("common:all")}</SelectItem>
                {stateOptions.map((state) => (
                  <SelectItem key={state.id} value={String(state.id)}>
                    {state.name}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
          </Field>
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
              "min-w-full",
              "table-auto",
              "max-w-none",
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
                          header.column.id === "name" && "min-w-64",
                          header.column.id === "rank" && "w-16 min-w-16",
                          header.column.id === "pts" && "w-20 min-w-20",
                          header.column.id === "state" && "w-32 min-w-32",
                          header.column.id === "has_writeup" && "w-32 min-w-32",
                          header.column.id === "actions" && "w-28 min-w-28",
                          header.column.id === "expand" && "w-12 min-w-12",
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
            <TableBody className={cn(["flex-1"])}>
              {table.getRowModel().rows?.length
                ? table.getRowModel().rows.map((row) => (
                    <React.Fragment key={row.id}>
                      <TableRow
                        key={row.getValue("id")}
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
                              cell.column.id === "actions" && ["w-28"],
                              cell.column.id === "expand" && "w-12",
                            ])}
                          >
                            {flexRender(
                              cell.column.columnDef.cell,
                              cell.getContext()
                            )}
                          </TableCell>
                        ))}
                      </TableRow>

                      <TableRow
                        data-state={row.getIsExpanded() ? "open" : "closed"}
                        className={cn([
                          "overflow-hidden",
                          "bg-muted/30",
                          "transition-all",
                          "data-[state=open]:animate-accordion-down",
                          "data-[state=closed]:animate-accordion-up",
                        ])}
                        style={{
                          display: row.getIsExpanded() ? "table-row" : "none",
                        }}
                      >
                        {row.getIsExpanded() && (
                          <TableCell
                            colSpan={row.getVisibleCells().length}
                            className="p-0"
                          >
                            <ExpandedCard team={row.original} />
                          </TableCell>
                        )}
                      </TableRow>
                    </React.Fragment>
                  ))
                : !loading && (
                    <TableRow>
                      <TableCell
                        colSpan={columns.length}
                        className={cn(["h-24", "text-center"])}
                      >
                        {t("game:team.empty")}
                      </TableCell>
                    </TableRow>
                  )}
            </TableBody>
          </Table>
        </ScrollArea>
      </div>
      <div className="flex items-center justify-between space-x-2 py-4 px-4">
        <div className="flex-1 text-sm text-muted-foreground">
          {table.getFilteredRowModel().rows.length} / {total}
        </div>
        <div className={cn(["flex", "items-center", "gap-5"])}>
          <Field size={"sm"} className={cn(["w-48"])}>
            <FieldIcon>
              <ListOrderedIcon />
            </FieldIcon>
            <Select
              value={String(size)}
              onValueChange={(value) => setSize(Number(value))}
            >
              <SelectTrigger>
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                {[10, 20, 40, 60].map((value) => (
                  <SelectItem key={value} value={String(value)}>
                    {value}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
          </Field>

          <Pagination
            size={"sm"}
            value={page}
            total={Math.ceil(total / size)}
            onChange={setPage}
          />
        </div>
      </div>
    </div>
  );
}
