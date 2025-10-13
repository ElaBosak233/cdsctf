import { keepPreviousData, useQuery } from "@tanstack/react-query";
import {
  type ColumnFiltersState,
  flexRender,
  getCoreRowModel,
  getFilteredRowModel,
  type SortingState,
  useReactTable,
  type VisibilityState,
} from "@tanstack/react-table";
import {
  FlagIcon,
  HashIcon,
  ListOrderedIcon,
  PlusCircleIcon,
  TypeIcon,
} from "lucide-react";
import { useState } from "react";
import { type GetGamesRequest, getGames } from "@/api/admin/games";
import { Button } from "@/components/ui/button";
import { Dialog, DialogContent } from "@/components/ui/dialog";
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
import type { Game } from "@/models/game";
import { useConfigStore } from "@/storages/config";
import { useSharedStore } from "@/storages/shared";
import { cn } from "@/utils";
import { columns } from "./columns";
import { CreateDialog } from "./create-dialog";

function useGameQuery(params: GetGamesRequest) {
  const { refresh } = useSharedStore();

  return useQuery({
    queryKey: [
      "games",
      params.id,
      params.title,
      params.size,
      params.page,
      params.is_enabled,
      refresh,
    ],
    queryFn: () => getGames(params),
    select: (response) => ({
      games: response.data || [],
      total: response.total || 0,
    }),
    enabled: !!params,
    placeholderData: keepPreviousData,
  });
}

export default function Index() {
  const configStore = useConfigStore();

  const [createDialogOpen, setCreateDialogOpen] = useState<boolean>(false);

  const [page, setPage] = useState<number>(1);
  const [size, setSize] = useState<number>(10);
  const [sorting, setSorting] = useState<SortingState>([
    {
      id: "started_at",
      desc: true,
    },
  ]);

  const [columnVisibility, setColumnVisibility] = useState<VisibilityState>({});
  const [columnFilters, setColumnFilters] = useState<ColumnFiltersState>([]);
  const debouncedColumnFilters = useDebounce(columnFilters, 100);

  const { data: gamesData, isFetching: loading } = useGameQuery({
    id: debouncedColumnFilters.find((c) => c.id === "id")?.value as number,
    title: debouncedColumnFilters.find((c) => c.id === "title")
      ?.value as string,
    sorts: sorting
      .map((value) => (value.desc ? `-${value.id}` : `${value.id}`))
      .join(","),
    page,
    size,
  });

  const table = useReactTable<Game>({
    data: gamesData?.games || [],
    columns,
    getCoreRowModel: getCoreRowModel(),
    manualPagination: true,
    rowCount: gamesData?.total,
    manualFiltering: true,
    getFilteredRowModel: getFilteredRowModel(),
    onColumnFiltersChange: setColumnFilters,
    onColumnVisibilityChange: setColumnVisibility,
    manualSorting: true,
    onSortingChange: setSorting,
    state: {
      sorting,
      columnVisibility,
    },
  });

  return (
    <>
      <title>{`比赛 - ${configStore?.config?.meta?.title}`}</title>
      <div className={cn(["container", "mx-auto", "p-10"])}>
        <div
          className={cn([
            "flex",
            "flex-col",
            "lg:flex-row",
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
            比赛
          </h1>
          <div
            className={cn([
              "flex",
              "flex-1",
              "flex-col",
              "lg:flex-row",
              "items-center",
              "gap-3",
              "w-full",
            ])}
          >
            <Field size={"sm"} className={cn(["w-full", "lg:w-1/6"])}>
              <FieldIcon>
                <HashIcon />
              </FieldIcon>
              <TextField
                placeholder="ID"
                value={table.getColumn("id")?.getFilterValue() as number}
                onChange={(e) =>
                  table.getColumn("id")?.setFilterValue(e.target.value)
                }
              />
            </Field>
            <Field size={"sm"} className={cn(["w-full", "lg:w-4/6"])}>
              <FieldIcon>
                <TypeIcon />
              </FieldIcon>
              <TextField
                placeholder={"比赛名"}
                value={table.getColumn("title")?.getFilterValue() as string}
                onChange={(e) =>
                  table.getColumn("title")?.setFilterValue(e.target.value)
                }
              />
            </Field>
            <Button
              icon={<PlusCircleIcon />}
              variant={"solid"}
              onClick={() => setCreateDialogOpen(true)}
              className={cn(["w-full", "lg:w-1/6"])}
            >
              添加比赛
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
            <TableBody>
              {table.getRowModel().rows?.length
                ? table.getRowModel().rows.map((row) => (
                    <TableRow
                      key={row.original.id}
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
                        哎呀，好像还没有比赛呢。
                      </TableCell>
                    </TableRow>
                  )}
            </TableBody>
          </Table>
        </ScrollArea>
        <div
          className={cn([
            "flex",
            "items-center",
            "justify-between",
            "space-x-2",
            "py-4",
            "px-4",
          ])}
        >
          <div className={cn(["flex-1", "text-sm", "text-muted-foreground"])}>
            {table.getFilteredRowModel().rows.length} / {gamesData?.total}
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
              total={Math.ceil((gamesData?.total || 0) / size)}
              onChange={setPage}
            />
          </div>
        </div>
      </div>
    </>
  );
}
