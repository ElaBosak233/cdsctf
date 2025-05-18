import {
  ColumnFiltersState,
  flexRender,
  getCoreRowModel,
  getFilteredRowModel,
  SortingState,
  useReactTable,
  VisibilityState,
} from "@tanstack/react-table";
import {
  FlagIcon,
  HashIcon,
  ListOrderedIcon,
  PlusCircleIcon,
  TypeIcon,
} from "lucide-react";
import { useEffect, useState } from "react";

import { columns } from "./columns";
import { CreateDialog } from "./create-dialog";

import { getGames } from "@/api/admin/games";
import { Button } from "@/components/ui/button";
import { Dialog, DialogContent } from "@/components/ui/dialog";
import { Field, FieldIcon } from "@/components/ui/field";
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
import { Game } from "@/models/game";
import { useSharedStore } from "@/storages/shared";
import { cn } from "@/utils";

export default function Index() {
  const [createDialogOpen, setCreateDialogOpen] = useState<boolean>(false);
  const sharedStore = useSharedStore();

  const [page, setPage] = useState<number>(1);
  const [size, setSize] = useState<number>(10);
  const [total, setTotal] = useState<number>(0);
  const [sorting, setSorting] = useState<SortingState>([
    {
      id: "started_at",
      desc: true,
    },
  ]);

  const [columnVisibility, setColumnVisibility] = useState<VisibilityState>({});
  const [columnFilters, setColumnFilters] = useState<ColumnFiltersState>([]);
  const debouncedColumnFilters = useDebounce(columnFilters, 100);
  const [games, setGames] = useState<Array<Game>>([]);
  const table = useReactTable<Game>({
    data: games,
    columns,
    getCoreRowModel: getCoreRowModel(),
    manualPagination: true,
    rowCount: total,
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

  useEffect(() => {
    getGames({
      id: debouncedColumnFilters.find((c) => c.id === "id")?.value as number,
      title: debouncedColumnFilters.find((c) => c.id === "title")
        ?.value as string,
      sorts: sorting
        .map((value) => (value.desc ? `-${value.id}` : `${value.id}`))
        .join(","),
      page,
      size,
    }).then((res) => {
      setTotal(res?.total || 0);
      setGames(res?.data || []);
    });
  }, [page, size, sorting, debouncedColumnFilters, sharedStore?.refresh]);

  return (
    <div className={cn(["container", "mx-auto", "p-10"])}>
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
          比赛管理
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
              value={(table.getColumn("id")?.getFilterValue() as number) ?? ""}
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
              placeholder={"比赛名"}
              value={
                (table.getColumn("title")?.getFilterValue() as string) ?? ""
              }
              onChange={(e) =>
                table.getColumn("title")?.setFilterValue(e.target.value)
              }
            />
          </Field>
          <Button
            icon={<PlusCircleIcon />}
            variant={"solid"}
            onClick={() => setCreateDialogOpen(true)}
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
            {table.getRowModel().rows?.length ? (
              table.getRowModel().rows.map((row) => (
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
              ))
            ) : (
              <TableRow>
                <TableCell
                  colSpan={columns.length}
                  className="h-24 text-center"
                >
                  哎呀，好像还没有比赛呢。
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
