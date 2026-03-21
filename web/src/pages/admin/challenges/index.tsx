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
  TypeIcon,
} from "lucide-react";
import { parseAsInteger, useQueryState } from "nuqs";
import { useContext, useEffect, useState } from "react";
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
import { TextField } from "@/components/ui/text-field";
import { useDebounce } from "@/hooks/use-debounce";
import type { Challenge } from "@/models/challenge";
import { useConfigStore } from "@/storages/config";
import { useSharedStore } from "@/storages/shared";
import { cn } from "@/utils";
import { categories } from "@/utils/category";
import { AdminListContext, AdminListPageView } from "../_list";
import { useColumns } from "./_blocks/columns";
import { CreateDialog } from "./_blocks/create-dialog";

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
      challenges: response.items || [],
      total: response.total || 0,
    }),
    enabled: !!params,
    placeholderData: keepPreviousData,
  });
}

export default function Index() {
  const { t } = useTranslation();

  const configStore = useConfigStore();
  const listContext = useContext(AdminListContext);
  const hasSidebar = listContext != null;

  const [localPage, setLocalPage] = useQueryState(
    "page",
    parseAsInteger.withDefault(1)
  );
  const [localSize, setLocalSize] = useQueryState(
    "size",
    parseAsInteger.withDefault(10)
  );
  const page = listContext?.page ?? localPage;
  const setPage = listContext?.setPage ?? setLocalPage;
  const size = listContext?.size ?? localSize;
  const setSize = listContext?.setSize ?? setLocalSize;

  const [localColumnFilters, setLocalColumnFilters] =
    useState<ColumnFiltersState>([
      { id: "category", value: "all" },
      { id: "public", value: "all" },
    ]);
  const columnFilters = listContext?.columnFilters ?? localColumnFilters;
  const setColumnFilters =
    listContext?.setColumnFilters ?? setLocalColumnFilters;

  const [localCreateDialogOpen, setLocalCreateDialogOpen] = useState(false);
  const createDialogOpen =
    listContext?.createDialogOpen ?? localCreateDialogOpen;
  const setCreateDialogOpen =
    listContext?.setCreateDialogOpen ?? setLocalCreateDialogOpen;

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
      .map((value) => (value.desc ? `-${value.id}` : `${value.id}`))
      .join(","),
    page,
    size,
  });

  useEffect(() => {
    if (listContext) listContext.setTotal(challengesData?.total ?? 0);
  }, [listContext, challengesData?.total]);

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

  const filterContent = (
    <div
      className={cn(
        "grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-12 gap-3 items-end"
      )}
    >
      <Field size="sm" className={cn("lg:col-span-2")}>
        <FieldIcon>
          <HashIcon className="size-4" />
        </FieldIcon>
        <TextField
          placeholder="ID"
          value={table.getColumn("id")?.getFilterValue() as string}
          onChange={(e) =>
            table.getColumn("id")?.setFilterValue(e.target.value)
          }
        />
      </Field>
      <Field size="sm" className={cn("lg:col-span-4")}>
        <FieldIcon>
          <TypeIcon className="size-4" />
        </FieldIcon>
        <TextField
          placeholder={t("challenge:title")}
          value={table.getColumn("title")?.getFilterValue() as string}
          onChange={(e) =>
            table.getColumn("title")?.setFilterValue(e.target.value)
          }
        />
      </Field>
      <Field size="sm" className={cn("lg:col-span-2")}>
        <FieldIcon>
          <LibraryIcon className="size-4" />
        </FieldIcon>
        <Select
          options={[
            {
              value: "all",
              content: (
                <div className={cn("flex gap-2 items-center")}>
                  {t("common:all")}
                </div>
              ),
            },
            ...(categories || []).map((cat) => {
              const Icon = cat.icon!;
              return {
                value: String(cat?.id),
                content: (
                  <div className={cn("flex gap-2 items-center")}>
                    <Icon className="size-4" />
                    {cat?.name?.toUpperCase()}
                  </div>
                ),
              };
            }),
          ]}
          onValueChange={(value) =>
            table.getColumn("category")?.setFilterValue(value)
          }
          value={
            (table.getColumn("category")?.getFilterValue() as string) ?? ""
          }
        />
      </Field>
      <Field size="sm" className={cn("lg:col-span-2")}>
        <FieldIcon>
          <EyeIcon className="size-4" />
        </FieldIcon>
        <Select
          options={[
            { value: "all", content: t("common:all") },
            { value: "true", content: t("challenge:search.public.true") },
            { value: "false", content: t("challenge:search.public.false") },
          ]}
          onValueChange={(value) =>
            setColumnFilters((prev) => {
              const other = prev.filter((f) => f.id !== "public");
              return [...other, { id: "public", value }];
            })
          }
          value={
            (columnFilters.find((f) => f.id === "public")?.value as string) ??
            "all"
          }
        />
      </Field>
    </div>
  );

  const tableContent = (
    <ScrollArea className={cn("h-full w-full")}>
      <LoadingOverlay loading={loading} />
      <Table className={cn("text-foreground w-full min-w-160")}>
        <TableHeader
          className={cn(
            "sticky top-0 z-2 bg-muted/80 backdrop-blur-sm border-b"
          )}
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
                className={cn("transition-colors")}
              >
                {row.getVisibleCells().map((cell) => (
                  <TableCell key={cell.id}>
                    {flexRender(cell.column.columnDef.cell, cell.getContext())}
                  </TableCell>
                ))}
              </TableRow>
            ))
          ) : !loading ? (
            <TableRow>
              <TableCell
                colSpan={columns.length}
                className={cn("h-40 text-center text-muted-foreground")}
              >
                <div
                  className={cn(
                    "flex flex-col items-center justify-center gap-2"
                  )}
                >
                  <LibraryIcon
                    className={cn("size-10 opacity-30")}
                    aria-hidden
                  />
                  <span>{t("challenge:empty")}</span>
                </div>
              </TableCell>
            </TableRow>
          ) : null}
        </TableBody>
      </Table>
    </ScrollArea>
  );

  const footerContent = !hasSidebar ? (
    <>
      <p className={cn("text-sm text-muted-foreground order-2 sm:order-1")}>
        {table.getFilteredRowModel().rows.length} / {challengesData?.total ?? 0}
      </p>
      <div
        className={cn(
          "flex flex-wrap items-center gap-3 order-1 sm:order-2 min-h-10"
        )}
      >
        <Field size="sm" className={cn("w-32 sm:w-36")}>
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
        <Pagination
          size="sm"
          value={page}
          total={Math.ceil((challengesData?.total || 0) / size)}
          onChange={setPage}
        />
      </div>
    </>
  ) : null;

  return (
    <>
      <title>{`${t("challenge:_")} - ${configStore?.config?.meta?.title}`}</title>
      <Dialog open={createDialogOpen} onOpenChange={setCreateDialogOpen}>
        <DialogContent>
          <CreateDialog onClose={() => setCreateDialogOpen(false)} />
        </DialogContent>
      </Dialog>
      <AdminListPageView
        hasSidebar={hasSidebar}
        title={t("challenge:_")}
        icon={<LibraryIcon className="size-5" />}
        addButtonLabel={t("common:actions.add")}
        onAddClick={() => setCreateDialogOpen(true)}
        filterContent={filterContent}
        tableContent={tableContent}
        footerContent={footerContent}
      />
    </>
  );
}
