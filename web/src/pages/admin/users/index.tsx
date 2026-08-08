import { keepPreviousData, useQuery } from "@tanstack/react-query";
import { ListOrderedIcon } from "lucide-react";
import { parseAsInteger, useQueryState } from "nuqs";
import { useContext, useState } from "react";
import { useTranslation } from "react-i18next";
import { type GetUsersRequest, getUsers } from "@/api/admin/users";
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
import type { Group, UserAccountView } from "@/models/user";
import { useConfigStore } from "@/storages/config";
import { useSharedStore } from "@/storages/shared";
import { cn } from "@/utils";
import {
  flexRender,
  type SortingState,
  useDataTable,
} from "@/hooks/use-data-table";
import { useColumns } from "./_blocks/columns";
import { CreateUserDialog } from "./_blocks/create-dialog";
import { UserListContext } from "./context";

function useUserQuery(params: GetUsersRequest) {
  const { refresh } = useSharedStore();

  return useQuery({
    queryKey: [
      "users",
      params.id,
      params.name,
      params.size,
      params.page,
      params.group,
      params.sorts,
      refresh,
    ],
    queryFn: () => getUsers(params),
    select: (response) => ({
      users: response.users || [],
      total: response.total || 0,
    }),
    enabled: !!params,
    placeholderData: keepPreviousData,
  });
}

export default function Index() {
  const { t } = useTranslation();
  const configStore = useConfigStore();
  const {
    createDialogOpen,
    setCreateDialogOpen,
    columnFilters,
    setColumnFilters,
  } = useContext(UserListContext)!;

  const [page, setPage] = useQueryState("page", parseAsInteger.withDefault(1));
  const [size, setSize] = useQueryState("size", parseAsInteger.withDefault(10));
  const [sorting, setSorting] = useState<SortingState>([
    { id: "created_at", desc: false },
  ]);
  const debouncedColumnFilters = useDebounce(columnFilters, 100);

  const groupValue = debouncedColumnFilters.find(
    (filter) => filter.id === "group"
  )?.value as string | undefined;
  const groupFilter =
    groupValue && groupValue !== "all"
      ? (Number(groupValue) as Group)
      : undefined;

  const { data: usersData, isLoading: loading } = useUserQuery({
    id:
      Number(debouncedColumnFilters.find((c) => c.id === "id")?.value) ||
      undefined,
    name: debouncedColumnFilters.find((c) => c.id === "username")
      ?.value as string,
    group: groupFilter,
    sorts: sorting.map((s) => (s.desc ? `-${s.id}` : s.id)).join(","),
    page,
    size,
  });

  const columns = useColumns();
  const table = useDataTable<UserAccountView>({
    data: usersData?.users || [],
    columns,
    manualPagination: true,
    rowCount: usersData?.total,
    manualFiltering: true,
    onColumnFiltersChange: setColumnFilters,
    manualSorting: true,
    onSortingChange: (updater) => {
      setSorting(updater);
      void setPage(1);
    },
    state: { sorting, columnFilters },
  });

  return (
    <>
      <title>{`${t("user:_")} - ${configStore?.config?.meta?.title}`}</title>
      <Dialog open={createDialogOpen} onOpenChange={setCreateDialogOpen}>
        <DialogContent>
          <CreateUserDialog onClose={() => setCreateDialogOpen(false)} />
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
                          "w-24",
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
                    {t("user:empty")}
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
            {t("user:result_count", { count: usersData?.total ?? 0 })}
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
              total={Math.ceil((usersData?.total || 0) / size)}
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
                onValueChange={(value) => {
                  void setSize(Number(value));
                  void setPage(1);
                }}
              />
            </Field>
          </div>
        </footer>
      </div>
    </>
  );
}
