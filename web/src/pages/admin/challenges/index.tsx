import { keepPreviousData, useQuery } from "@tanstack/react-query";
import {
  flexRender,
  getCoreRowModel,
  getFilteredRowModel,
  type SortingState,
  useReactTable,
  type VisibilityState,
} from "@tanstack/react-table";
import { ListOrderedIcon } from "lucide-react";
import { parseAsInteger, useQueryState } from "nuqs";
import { useContext, useState } from "react";
import { useTranslation } from "react-i18next";
import {
  type GetChallengesRequest,
  getChallenges,
} from "@/api/admin/challenges";
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
import { useDebounce } from "@/hooks/use-debounce";
import type { Challenge } from "@/models/challenge";
import { useConfigStore } from "@/storages/config";
import { useSharedStore } from "@/storages/shared";
import { cn } from "@/utils";
import { useColumns } from "./_blocks/columns";
import { CreateDialog } from "./_blocks/create-dialog";
import { ChallengeListContext } from "./context";

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
      params.public,
      params.sorts,
      refresh,
    ],
    queryFn: () => getChallenges(params),
    select: (response) => ({
      challenges: response.challenges || [],
      total: response.total || 0,
    }),
    enabled: !!params,
    placeholderData: keepPreviousData,
  });
}

export default function Index() {
  const { t } = useTranslation();
  const configStore = useConfigStore();
  const { createDialogOpen, setCreateDialogOpen, columnFilters, setColumnFilters } =
    useContext(ChallengeListContext)!;

  const [page, setPage] = useQueryState("page", parseAsInteger.withDefault(1));
  const [size, setSize] = useQueryState("size", parseAsInteger.withDefault(10));
  const [sorting, setSorting] = useState<SortingState>([
    { id: "created_at", desc: true },
  ]);
  const [columnVisibility, setColumnVisibility] = useState<VisibilityState>({});
  const debouncedColumnFilters = useDebounce(columnFilters, 100);

  const category =
    (debouncedColumnFilters.find((c) => c.id === "category")
      ?.value as string) !== "all"
      ? (debouncedColumnFilters.find((c) => c.id === "category")
          ?.value as number)
      : undefined;

  const isPublic =
    (debouncedColumnFilters.find((c) => c.id === "public")?.value as string) !==
    "all"
      ? (debouncedColumnFilters.find((c) => c.id === "public")
          ?.value as string) === "true"
      : undefined;

  const { data: challengesData, isLoading: loading } = useChallengeQuery({
    id: debouncedColumnFilters.find((c) => c.id === "id")?.value as number,
    title: debouncedColumnFilters.find((c) => c.id === "title")
      ?.value as string,
    category,
    public: isPublic,
    sorts: sorting
      .map((s) => (s.desc ? `-${s.id}` : s.id))
      .join(","),
    page,
    size,
  });

  const columns = useColumns();
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
    state: { sorting, columnVisibility, columnFilters },
  });

  return (
    <>
      <title>{`${t("challenge:_")} - ${configStore?.config?.meta?.title}`}</title>
      <Dialog open={createDialogOpen} onOpenChange={setCreateDialogOpen}>
        <DialogContent>
          <CreateDialog onClose={() => setCreateDialogOpen(false)} />
        </DialogContent>
      </Dialog>
      <div
        className={cn([
          "overflow-hidden",
          "flex",
          "flex-col",
          "min-h-0",
          "h-full",
          "px-4",
          "py-4",
          "sm:px-6",
          "sm:py-6",
          "lg:px-8",
          "lg:py-8",
          "gap-4",
        ])}
      >
        <ScrollArea
          className={cn([
            "flex-1",
            "min-h-0",
            "rounded-xl",
            "border",
            "ring-1",
            "ring-border/50",
            "shadow-sm",
          ])}
        >
          <LoadingOverlay loading={loading} />
          <Table className={cn(["text-foreground", "w-full", "min-w-160"])}>
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
                  {headerGroup.headers.map((header) => (
                    <TableHead key={header.id}>
                      {!header.isPlaceholder &&
                        flexRender(
                          header.column.columnDef.header,
                          header.getContext()
                        )}
                    </TableHead>
                  ))}
                </TableRow>
              ))}
            </TableHeader>
            <TableBody>
              {table.getRowModel().rows?.length ? (
                table.getRowModel().rows.map((row) => (
                  <TableRow
                    key={row.getValue("id")}
                    data-state={row.getIsSelected() ? "selected" : undefined}
                    className={cn(["transition-colors"])}
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
              ) : !loading ? (
                <TableRow>
                  <TableCell
                    colSpan={columns.length}
                    className={cn([
                      "h-40",
                      "text-center",
                      "text-muted-foreground",
                    ])}
                  >
                    {t("challenge:empty")}
                  </TableCell>
                </TableRow>
              ) : null}
            </TableBody>
          </Table>
        </ScrollArea>
        <footer
          className={cn([
            "flex",
            "flex-col",
            "gap-3",
            "sm:flex-row",
            "sm:items-center",
            "sm:justify-between",
            "shrink-0",
          ])}
        >
          <p className={cn(["text-sm", "text-muted-foreground"])}>
            {table.getFilteredRowModel().rows.length} /{" "}
            {challengesData?.total ?? 0}
          </p>
          <div
            className={cn([
              "flex",
              "flex-wrap",
              "items-center",
              "gap-3",
              "min-h-10",
            ])}
          >
            <Pagination
              size="sm"
              value={page}
              total={Math.ceil((challengesData?.total || 0) / size)}
              onChange={setPage}
            />
            <Field size="sm" className={cn(["w-32", "sm:w-36"])}>
              <FieldIcon>
                <ListOrderedIcon className="size-4" />
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
          </div>
        </footer>
      </div>
    </>
  );
}
