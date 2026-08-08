import {
  columnFilteringFeature,
  columnVisibilityFeature,
  createExpandedRowModel,
  createFilteredRowModel,
  createSortedRowModel,
  filterFns,
  type RowData,
  rowExpandingFeature,
  rowPaginationFeature,
  rowSelectionFeature,
  rowSortingFeature,
  sortFns,
  type TableOptions,
  type Column as TanStackColumn,
  type ColumnDef as TanStackColumnDef,
  type Row as TanStackRow,
  tableFeatures,
  useTable,
} from "@tanstack/react-table";

const dataTableFeatures = tableFeatures({
  columnFilteringFeature,
  columnVisibilityFeature,
  rowExpandingFeature,
  rowPaginationFeature,
  rowSelectionFeature,
  rowSortingFeature,
  expandedRowModel: createExpandedRowModel(),
  filteredRowModel: createFilteredRowModel(),
  sortedRowModel: createSortedRowModel(),
  filterFns,
  sortFns,
});

type DataTableFeatures = typeof dataTableFeatures;

export type Column<TData extends RowData, TValue = unknown> = TanStackColumn<
  DataTableFeatures,
  TData,
  TValue
>;

export type ColumnDef<
  TData extends RowData,
  TValue = unknown,
> = TanStackColumnDef<DataTableFeatures, TData, TValue>;

export type Row<TData extends RowData> = TanStackRow<DataTableFeatures, TData>;

export function useDataTable<TData extends RowData>(
  options: Omit<TableOptions<DataTableFeatures, TData>, "features">
) {
  return useTable({ ...options, features: dataTableFeatures });
}

export {
  type ColumnFiltersState,
  type ColumnVisibilityState,
  flexRender,
  type SortingState,
} from "@tanstack/react-table";
