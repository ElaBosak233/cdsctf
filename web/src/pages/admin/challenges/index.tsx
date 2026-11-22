import { keepPreviousData, useQuery } from "@tanstack/react-query";
import { ListOrderedIcon } from "lucide-react";
import { parseAsInteger, useQueryState } from "nuqs";
import { useContext, useEffect, useMemo, useState } from "react";
import { useTranslation } from "react-i18next";
import {
  type GetChallengesRequest,
  getChallenges,
} from "@/api/management/challenges";
import { Dialog, DialogContent } from "@/components/ui/dialog";
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
import {
  flexRender,
  type SortingState,
  useDataTable,
} from "@/hooks/use-data-table";
import { useDebounce } from "@/hooks/use-debounce";
import type { ChallengeDetail } from "@/models/challenge";
import { useConfigStore } from "@/storages/config";
import { useSharedStore } from "@/storages/shared";
import { cn } from "@/utils";
import { RowProvider, useColumns } from "./_blocks/columns";
import { CreateDialog } from "./_blocks/create-dialog";
import { ChallengeListContext } from "./context";

export default function Index() {
  const { t } = useTranslation();
  const configStore = useConfigStore();
  const refresh = useSharedStore((state) => state.refresh);
  const { createDialogOpen, setCreateDialogOpen, filters } =
    useContext(ChallengeListContext)!;

  const [page, setPage] = useQueryState("page", parseAsInteger.withDefault(1));
  const [size, setSize] = useQueryState("size", parseAsInteger.withDefault(10));
  const [sorting, setSorting] = useState<SortingState>([
    { id: "updated_at", desc: true },
  ]);
  const sorts = sorting.map((s) => (s.desc ? `-${s.id}` : s.id)).join(",");
  const request = useMemo<GetChallengesRequest>(
    () => ({
      q: filters.q.trim() || undefined,
      category: filters.category ?? undefined,
      public: filters.public ?? undefined,
      sorts,
      page,
      size,
    }),
    [filters.category, filters.public, filters.q, sorts, page, size]
  );
  const params = useDebounce(request, 150);
  const query = useQuery({
    queryKey: ["admin", "challenges", params, refresh],
    queryFn: () => getChallenges(params),
    placeholderData: keepPreviousData,
  });
  const challengesData = query.data;
  const loading =
    request !== params || query.isPending || query.isPlaceholderData;
  const totalPages = Math.ceil((challengesData?.total ?? 0) / size);

  useEffect(() => {
    if (loading || !query.isSuccess) return;
    const lastPage = Math.max(totalPages, 1);
    if (page > lastPage) void setPage(lastPage);
  }, [loading, query.isSuccess, page, totalPages, setPage]);

  const columns = useColumns();
  const table = useDataTable<ChallengeDetail>({
    data: challengesData?.challenges || [],
    columns,
    manualPagination: true,
    rowCount: challengesData?.total,
    manualSorting: true,
    onSortingChange: (updater) => {
      setSorting(updater);
      void setPage(1);
    },
    state: { sorting },
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
            "min-w-0",
            "w-full",
            "max-w-full",
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
            aria-busy={loading}
            className={cn([
              "w-full",
              "min-w-224",
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
                  {headerGroup.headers.map((header) => (
                    <TableHead
                      key={header.id}
                      className={cn([
                        "bg-muted/95",
                        header.column.id === "classification" && ["w-44"],
                        header.column.id === "status" && ["w-52"],
                        header.column.id === "updated_at" && ["w-44"],
                        header.column.id === "actions" && [
                          "sticky",
                          "right-0",
                          "z-3",
                          "w-28",
                        ],
                      ])}
                    >
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
                  <RowProvider key={row.original.id} challenge={row.original}>
                    <TableRow
                      data-state={row.getIsSelected() ? "selected" : undefined}
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
                            cell.column.id === "actions" && [
                              "sticky",
                              "right-0",
                              "z-1",
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
                  </RowProvider>
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
            {t("challenge:result_count", {
              count: challengesData?.total ?? 0,
            })}
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
              total={totalPages}
              onChange={setPage}
            />
            <Field size="sm" className={cn(["w-32", "sm:w-36"])}>
              <FieldIcon>
                <ListOrderedIcon className="size-4" />
              </FieldIcon>
              <Select
                value={String(size)}
                onValueChange={(value) => {
                  void setSize(Number(value));
                  void setPage(1);
                }}
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
          </div>
        </footer>
      </div>
    </>
  );
}
