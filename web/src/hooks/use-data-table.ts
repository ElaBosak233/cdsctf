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

// TanStack Table v9 compares data by reference. Reusing one empty value keeps
// loading states from rebuilding row models on every render.
const EMPTY_DATA: never[] = [];

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
  return useTable({
    ...options,
    data: options.data.length === 0 ? EMPTY_DATA : options.data,
    features: dataTableFeatures,
  });
}

export {
  type ColumnFiltersState,
  type ColumnVisibilityState,
  flexRender,
  type SortingState,
} from "@tanstack/react-table";
