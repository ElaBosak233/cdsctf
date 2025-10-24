import {
  type ColumnFiltersState,
  flexRender,
  getCoreRowModel,
  getExpandedRowModel,
  getFilteredRowModel,
  type SortingState,
  useReactTable,
  type VisibilityState,
} from "@tanstack/react-table";
import {
  HashIcon,
  ListOrderedIcon,
  SatelliteIcon,
  TypeIcon,
  UsersRoundIcon,
} from "lucide-react";
import React, { useContext, useEffect, useState } from "react";
import { getTeams } from "@/api/admin/games/game_id/teams";
import { Field, FieldIcon } from "@/components/ui/field";
import { LoadingOverlay } from "@/components/ui/loading-overlay";
import { Pagination } from "@/components/ui/pagination";
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
import { State, type Team } from "@/models/team";
import { useSharedStore } from "@/storages/shared";
import { cn } from "@/utils";
import { Context } from "../context";
import { columns } from "./columns";
import { ExpandedCard } from "./expanded-card";

export default function Index() {
  const sharedStore = useSharedStore();

  const { game } = useContext(Context);

  const [total, setTotal] = useState<number>(0);
  const [teams, setTeams] = useState<Array<Team>>([]);
  const [loading, setLoading] = useState<boolean>(false);

  const [page, setPage] = useState<number>(1);
  const [size, setSize] = useState<number>(10);

  const [sorting, setSorting] = useState<SortingState>([]);
  const [columnVisibility, setColumnVisibility] = useState<VisibilityState>({
    game_id: false,
  });
  const [columnFilters, setColumnFilters] = useState<ColumnFiltersState>([
    {
      id: "state",
      value: "all",
    },
  ]);
  const debouncedColumnFilters = useDebounce(columnFilters, 100);

  const table = useReactTable<Team>({
    data: teams,
    columns,
    getCoreRowModel: getCoreRowModel(),
    manualPagination: true,
    rowCount: total,
    manualFiltering: true,
    getFilteredRowModel: getFilteredRowModel(),
    getExpandedRowModel: getExpandedRowModel(),
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

  const stateOptions = [
    { id: State.Banned.toString(), name: "禁赛中" },
    { id: State.Preparing.toString(), name: "准备中" },
    { id: State.Pending.toString(), name: "待审核" },
    { id: State.Passed.toString(), name: "正常参赛" },
  ];

  useEffect(() => {
    void sorting;
    void sharedStore.refresh;

    if (!game) return;

    setLoading(true);
    getTeams({
      game_id: game.id!,
      id: debouncedColumnFilters.find((c) => c.id === "id")?.value as number,
      name: debouncedColumnFilters.find((c) => c.id === "name")
        ?.value as string,
      state:
        debouncedColumnFilters.find((c) => c.id === "state")?.value !== "all"
          ? Number(debouncedColumnFilters.find((c) => c.id === "state")?.value)
          : undefined,
      sorts: "rank",
      page,
      size,
    })
      .then((res) => {
        setTotal(res?.total || 0);
        setTeams(res?.data || []);
      })
      .finally(() => {
        setLoading(false);
      });
  }, [page, size, sorting, debouncedColumnFilters, sharedStore.refresh, game]);

  return (
    <div className={cn(["container", "mx-auto"])}>
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
          <UsersRoundIcon />
          团队
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
              placeholder="ID"
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
              placeholder={"团队名"}
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
              options={[
                {
                  value: "all",
                  content: (
                    <div className={cn(["flex", "gap-2", "items-center"])}>
                      全部
                    </div>
                  ),
                },
                ...stateOptions.map((state) => {
                  return {
                    value: String(state?.id),
                    content: (
                      <div className={cn(["flex", "gap-2", "items-center"])}>
                        {state?.name}
                      </div>
                    ),
                  };
                }),
              ]}
              onValueChange={(value) =>
                table.getColumn("state")?.setFilterValue(value)
              }
              value={
                (table.getColumn("state")?.getFilterValue() as string) ?? ""
              }
            />
          </Field>
        </div>
      </div>

      <ScrollArea
        className={cn([
          "rounded-md",
          "border",
          "bg-card",
          "min-h-100",
          "h-[calc(100vh-18rem)]",
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
          <TableBody className={cn(["flex-1"])}>
            {table.getRowModel().rows?.length
              ? table.getRowModel().rows.map((row) => (
                  <React.Fragment key={row.id}>
                    <TableRow
                      key={row.getValue("id")}
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
                      但是谁也没有来。
                    </TableCell>
                  </TableRow>
                )}
          </TableBody>
        </Table>
      </ScrollArea>
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
              placeholder={"每页显示"}
              options={[
                { value: "10" },
                { value: "20" },
                { value: "40" },
                { value: "60" },
              ]}
              value={String(size)}
              onValueChange={(value) => setSize(Number(value))}
            />
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
