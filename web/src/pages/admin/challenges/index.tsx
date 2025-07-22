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
  EyeIcon,
  HashIcon,
  LibraryIcon,
  ListOrderedIcon,
  PlusCircleIcon,
  TypeIcon,
} from "lucide-react";
import { useState } from "react";
import {
  type GetChallengesRequest,
  getChallenges,
} from "@/api/admin/challenges";
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
import type { Challenge } from "@/models/challenge";
import { useConfigStore } from "@/storages/config";
import { useSharedStore } from "@/storages/shared";
import { cn } from "@/utils";
import { categories } from "@/utils/category";
import { columns } from "./columns";
import { CreateDialog } from "./create-dialog";

function useChallengeQuery(params: GetChallengesRequest) {
  const { refresh } = useSharedStore();

  return useQuery({
    queryKey: [
      "challenges",
      params.id,
      params.title,
      params.size,
      params.page,
      params.category,
      params.is_public,
      refresh,
    ],
    queryFn: () => getChallenges(params),
    select: (response) => ({
      challenges: response.data || [],
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
      id: "created_at",
      desc: true,
    },
  ]);
  const [columnVisibility, setColumnVisibility] = useState<VisibilityState>({});
  const [columnFilters, setColumnFilters] = useState<ColumnFiltersState>([
    {
      id: "category",
      value: "all",
    },
    {
      id: "is_public",
      value: "all",
    },
  ]);
  const debouncedColumnFilters = useDebounce(columnFilters, 100);

  const category =
    (debouncedColumnFilters.find((c) => c.id === "category")
      ?.value as string) !== "all"
      ? (debouncedColumnFilters.find((c) => c.id === "category")
          ?.value as number)
      : undefined;

  const isPublic =
    (debouncedColumnFilters.find((c) => c.id === "is_public")
      ?.value as string) !== "all"
      ? (debouncedColumnFilters.find((c) => c.id === "is_public")
          ?.value as string) === "true"
      : undefined;

  const { data: challengesData, isFetching: loading } = useChallengeQuery({
    id: debouncedColumnFilters.find((c) => c.id === "id")?.value as string,
    title: debouncedColumnFilters.find((c) => c.id === "title")
      ?.value as string,
    category: category,
    is_public: isPublic,
    sorts: sorting
      .map((value) => (value.desc ? `-${value.id}` : `${value.id}`))
      .join(","),
    page,
    size,
  });

  const table = useReactTable<Challenge>({
    data: challengesData?.challenges || [],
    columns,
    getCoreRowModel: getCoreRowModel(),
    manualPagination: true,
    rowCount: challengesData?.total,
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

  return (
    <>
      <title>{`题库 - ${configStore?.config?.meta?.title}`}</title>
      <div
        className={cn([
          "container",
          "mx-auto",
          "p-10",
          "flex",
          "flex-col",
          "flex-1",
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
            <LibraryIcon />
            题库
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
            <Field size={"sm"} className={cn(["flex-1/6"])}>
              <FieldIcon>
                <HashIcon />
              </FieldIcon>
              <TextField
                placeholder="ID"
                value={table.getColumn("id")?.getFilterValue() as string}
                onChange={(e) =>
                  table.getColumn("id")?.setFilterValue(e.target.value)
                }
              />
            </Field>
            <Field size={"sm"} className={cn(["flex-3/6"])}>
              <FieldIcon>
                <TypeIcon />
              </FieldIcon>
              <TextField
                placeholder={"题目名"}
                value={table.getColumn("title")?.getFilterValue() as string}
                onChange={(e) =>
                  table.getColumn("title")?.setFilterValue(e.target.value)
                }
              />
            </Field>

            <Field size={"sm"} className={cn(["flex-1/6"])}>
              <FieldIcon>
                <LibraryIcon />
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
                  table.getColumn("category")?.setFilterValue(value)
                }
                value={
                  (table.getColumn("category")?.getFilterValue() as string) ??
                  ""
                }
              />
            </Field>

            <Field size={"sm"} className={cn(["flex-1/6"])}>
              <FieldIcon>
                <EyeIcon />
              </FieldIcon>
              <Select
                options={[
                  {
                    value: "all",
                    content: "全部",
                  },
                  {
                    value: "true",
                    content: "公开",
                  },
                  {
                    value: "false",
                    content: "非公开",
                  },
                ]}
                onValueChange={(value) =>
                  setColumnFilters((prev) => {
                    const otherFilters = prev.filter(
                      (f) => f.id !== "is_public"
                    );
                    return [...otherFilters, { id: "is_public", value }];
                  })
                }
                value={
                  (columnFilters.find((f) => f.id === "is_public")
                    ?.value as string) ?? "all"
                }
              />
            </Field>

            <Button
              icon={<PlusCircleIcon />}
              variant={"solid"}
              onClick={() => setCreateDialogOpen(true)}
              className={cn(["flex-1/6"])}
            >
              添加题目
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
                : !loading && (
                    <TableRow>
                      <TableCell
                        colSpan={columns.length}
                        className={cn(["h-24", "text-center"])}
                      >
                        哎呀，好像还没有题目呢。
                      </TableCell>
                    </TableRow>
                  )}
            </TableBody>
          </Table>
        </ScrollArea>
        <div className="flex items-center justify-between space-x-2 py-4 px-4">
          <div className="flex-1 text-sm text-muted-foreground">
            {table.getFilteredRowModel().rows.length} / {challengesData?.total}
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
              total={Math.ceil((challengesData?.total || 0) / size)}
              onChange={setPage}
            />
          </div>
        </div>
      </div>
    </>
  );
}
