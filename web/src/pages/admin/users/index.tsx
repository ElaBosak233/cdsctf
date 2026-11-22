import { keepPreviousData, useQuery } from "@tanstack/react-query";
import { ListOrderedIcon } from "lucide-react";
import {
  createParser,
  parseAsNumberLiteral,
  parseAsString,
  parseAsStringLiteral,
  useQueryStates,
} from "nuqs";
import { useEffect, useMemo, useState } from "react";
import { useTranslation } from "react-i18next";
import { type GetUsersRequest, getUsers } from "@/api/management/users";
import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";
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
import type { UserAccountView } from "@/models/user";
import { useConfigStore } from "@/storages/config";
import { useSharedStore } from "@/storages/shared";
import { cn } from "@/utils";
import { useColumns } from "./_blocks/columns";
import { CreateUserDialog } from "./_blocks/create-dialog";
import {
  UserListFilters,
  userAccessOptions,
  userStatuses,
} from "./_blocks/filters";

const pageSizes = [10, 20, 40, 60] as const;

const userSearchParams = {
  q: parseAsString.withDefault(""),
  status: parseAsStringLiteral(userStatuses),
  access: parseAsStringLiteral(userAccessOptions),
  page: createParser({
    parse: (value) => {
      const page = Number(value);
      return /^\d+$/.test(value) && Number.isSafeInteger(page) && page > 0
        ? page
        : null;
    },
    serialize: String,
  }).withDefault(1),
  size: parseAsNumberLiteral(pageSizes).withDefault(10),
};

export default function Index() {
  const { t } = useTranslation();
  const { config } = useConfigStore();
  const { refresh } = useSharedStore();
  const [createDialogOpen, setCreateDialogOpen] = useState(false);
  const [filters, setFilters] = useQueryStates(userSearchParams);
  const { q, status, access, page, size } = filters;
  const [sorting, setSorting] = useState<SortingState>([
    { id: "created_at", desc: false },
  ]);
  const sorts = sorting
    .map((sort) => (sort.desc ? `-${sort.id}` : sort.id))
    .join(",");
  const request = useMemo<GetUsersRequest>(
    () => ({
      q: q.trim() || undefined,
      banned: status === null ? undefined : status === "banned",
      ...(access === "management" || access === "no_management"
        ? { access }
        : { permission: access ?? undefined }),
      page,
      size,
      sorts,
    }),
    [q, status, access, page, size, sorts]
  );
  const params = useDebounce(request, 150);
  const usersQuery = useQuery({
    queryKey: ["users", params, refresh],
    queryFn: () => getUsers(params),
    placeholderData: keepPreviousData,
  });
  const usersData = usersQuery.data;
  const loading =
    request !== params || usersQuery.isPending || usersQuery.isPlaceholderData;
  const totalPages = Math.ceil((usersData?.total ?? 0) / size);

  useEffect(() => {
    if (loading || !usersQuery.isSuccess) return;
    const lastPage = Math.max(totalPages, 1);
    if (page > lastPage) void setFilters({ page: lastPage });
  }, [loading, usersQuery.isSuccess, page, totalPages, setFilters]);

  const columns = useColumns();
  const table = useDataTable<UserAccountView>({
    data: usersData?.users || [],
    columns,
    manualPagination: true,
    rowCount: usersData?.total,
    manualFiltering: true,
    manualSorting: true,
    onSortingChange: (updater) => {
      setSorting(updater);
      void setFilters({ page: 1 });
    },
    state: { sorting },
  });

  return (
    <>
      <title>{`${t("user:_")} - ${config?.meta?.title}`}</title>
      <Dialog open={createDialogOpen} onOpenChange={setCreateDialogOpen}>
        <DialogContent>
          <CreateUserDialog onClose={() => setCreateDialogOpen(false)} />
        </DialogContent>
      </Dialog>
      <div className="flex min-h-0 min-w-0 flex-1 flex-col xl:min-h-(--app-content-height) xl:flex-row xl:items-stretch">
        <UserListFilters
          filters={filters}
          onChange={(next) => void setFilters({ ...next, page: 1 })}
          onCreate={() => setCreateDialogOpen(true)}
        />
        <Card className="flex min-h-0 min-w-0 flex-1 flex-col gap-4 overflow-hidden rounded-none border-y-0 p-4 sm:p-6 lg:p-8 xl:h-(--app-content-height)">
          <ScrollArea
            className={cn([
              "flex-1",
              "min-h-0",
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
                          header.column.id === "status" && ["w-56"],
                          header.column.id === "created_at" && ["w-48"],
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
                {usersQuery.isError && !loading ? (
                  <TableRow>
                    <TableCell
                      colSpan={columns.length}
                      className="h-40 text-center"
                    >
                      <div className="flex flex-col items-center gap-2 text-muted-foreground">
                        <p role="alert">{t("user:search.load_error")}</p>
                        <Button
                          size="sm"
                          onClick={() => void usersQuery.refetch()}
                        >
                          {t("common:actions.refresh")}
                        </Button>
                      </div>
                    </TableCell>
                  </TableRow>
                ) : table.getRowModel().rows?.length ? (
                  table.getRowModel().rows.map((row) => (
                    <TableRow
                      key={row.original.id}
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
                      {t(
                        q || status || access
                          ? "user:search.no_matches"
                          : "user:empty"
                      )}
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
            <p role="status" className="text-sm text-muted-foreground">
              {usersData && t("user:result_count", { count: usersData.total })}
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
                onChange={(page) => void setFilters({ page })}
              />
              <Field size="sm" className={cn(["w-32", "sm:w-36"])}>
                <FieldIcon>
                  <ListOrderedIcon className="size-4" />
                </FieldIcon>
                <Select
                  value={String(size)}
                  onValueChange={(value) => {
                    const size = pageSizes.find(
                      (size) => String(size) === value
                    );
                    if (size) void setFilters({ size, page: 1 });
                  }}
                >
                  <SelectTrigger>
                    <SelectValue />
                  </SelectTrigger>
                  <SelectContent>
                    {pageSizes.map((value) => (
                      <SelectItem key={value} value={String(value)}>
                        {value}
                      </SelectItem>
                    ))}
                  </SelectContent>
                </Select>
              </Field>
            </div>
          </footer>
        </Card>
      </div>
    </>
  );
}
