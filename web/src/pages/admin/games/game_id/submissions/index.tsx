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
  FlagIcon,
  HashIcon,
  ListOrderedIcon,
  SatelliteIcon,
  TypeIcon,
} from "lucide-react";
import React, { useContext, useEffect, useState } from "react";
import { useTranslation } from "react-i18next";
import { useParams } from "react-router";
import { getSubmissions } from "@/api/admin/games/game_id/submissions";
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
import { Status, type Submission } from "@/models/submission";
import { useSharedStore } from "@/storages/shared";
import { cn } from "@/utils";
import { parseRouteNumericId } from "@/utils/query";
import { Context } from "../context";
import { useColumns } from "./_blocks/columns";

export default function Index() {
  const { t } = useTranslation();

  const sharedStore = useSharedStore();

  const { game_id } = useParams<{ game_id: string }>();
  const routeGameId = parseRouteNumericId(game_id);
  const { game } = useContext(Context);

  const [total, setTotal] = useState<number>(0);
  const [submissions, setSubmissions] = useState<Array<Submission>>([]);
  const [loading, setLoading] = useState<boolean>(false);

  const [page, setPage] = useState<number>(1);
  const [size, setSize] = useState<number>(10);

  const [sorting, setSorting] = useState<SortingState>([]);
  const [columnVisibility, setColumnVisibility] = useState<VisibilityState>({
    team_id: false,
    challenge_id: false,
  });
  const [columnFilters, setColumnFilters] = useState<ColumnFiltersState>([
    {
      id: "status",
      value: "all",
    },
  ]);
  const debouncedColumnFilters = useDebounce(columnFilters, 100);

  const columns = useColumns();

  const table = useReactTable<Submission>({
    data: submissions,
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

  const statusOptions = [
    { id: Status.Pending.toString(), name: t("submission:status.pending") },
    { id: Status.Correct.toString(), name: t("submission:status.correct") },
    { id: Status.Incorrect.toString(), name: t("submission:status.incorrect") },
    { id: Status.Cheat.toString(), name: t("submission:status.cheat") },
    { id: Status.Expired.toString(), name: t("submission:status.expired") },
    { id: Status.Duplicate.toString(), name: t("submission:status.duplicate") },
  ];

  useEffect(() => {
    void sorting;
    void sharedStore.refresh;

    const gid = routeGameId ?? game?.id;
    if (gid == null) return;

    setLoading(true);

    const rawStatus = debouncedColumnFilters.find(
      (c) => c.id === "status"
    )?.value;
    const parsedStatus =
      rawStatus !== undefined && rawStatus !== null && rawStatus !== "all"
        ? Number(rawStatus)
        : undefined;

    getSubmissions({
      game_id: gid,
      id: debouncedColumnFilters.find((c) => c.id === "id")?.value as number,
      team_id: debouncedColumnFilters.find((c) => c.id === "team_id")
        ?.value as number,
      challenge_id: debouncedColumnFilters.find((c) => c.id === "challenge_id")
        ?.value as number,
      status: Number.isFinite(parsedStatus) ? parsedStatus : undefined,
      sorts: "-created_at",
      page,
      size,
    })
      .then((res) => {
        setTotal(res?.total || 0);
        setSubmissions(res?.submissions || []);
      })
      .finally(() => {
        setLoading(false);
      });
  }, [
    page,
    size,
    sorting,
    debouncedColumnFilters,
    sharedStore.refresh,
    game,
    routeGameId,
  ]);

  return (
    <div
      className={cn([
        "container",
        "mx-auto",
        "h-full",
        "min-h-0",
        "flex",
        "flex-col",
      ])}
    >
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
          <FlagIcon />
          {t("submission:_")}
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
              placeholder={t("game:id")}
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
              placeholder={t("submission:team_id")}
              value={
                (table.getColumn("team_id")?.getFilterValue() as string) ?? ""
              }
              onChange={(e) =>
                table.getColumn("team_id")?.setFilterValue(e.target.value)
              }
            />
          </Field>
          <Field size={"sm"} className={cn(["flex-1"])}>
            <FieldIcon>
              <SatelliteIcon />
            </FieldIcon>
            <TextField
              placeholder={t("submission:challenge_id")}
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
              <ListOrderedIcon />
            </FieldIcon>
            <Select
              options={[
                {
                  value: "all",
                  content: (
                    <div className={cn(["flex", "gap-2", "items-center"])}>
                      {t("common:all")}
                    </div>
                  ),
                },
                ...statusOptions.map((status) => {
                  return {
                    value: String(status?.id),
                    content: (
                      <div className={cn(["flex", "gap-2", "items-center"])}>
                        {status?.name}
                      </div>
                    ),
                  };
                }),
              ]}
              onValueChange={(value) =>
                table.getColumn("status")?.setFilterValue(value)
              }
              value={
                (table.getColumn("status")?.getFilterValue() as string) ?? "all"
              }
            />
          </Field>
        </div>
      </div>

      <div className={cn(["flex-1", "min-h-0", "flex", "flex-col"])}>
        <ScrollArea
          className={cn([
            "rounded-md",
            "border",
            "bg-card",
            "h-full",
            "min-h-0",
            "overflow-hidden",
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
                    </React.Fragment>
                  ))
                : !loading && (
                    <TableRow>
                      <TableCell
                        colSpan={columns.length}
                        className={cn(["h-24", "text-center"])}
                      >
                        {t("submission:empty")}
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
