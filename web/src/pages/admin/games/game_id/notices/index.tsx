import { Button } from "@/components/ui/button";
import { MessageCircleIcon, PlusCircleIcon } from "lucide-react";
import { useContext, useEffect, useState } from "react";
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
    Table,
    TableBody,
    TableCell,
    TableHead,
    TableHeader,
    TableRow,
} from "@/components/ui/table";
import { cn } from "@/utils";
import { columns } from "./columns";
import { useDebounce } from "@/hooks/use-debounce";
import { Dialog, DialogContent } from "@/components/ui/dialog";
import { useSharedStore } from "@/storages/shared";
import { Context } from "../context";
import { CreateDialog } from "./create-dialog";
import { GameNotice } from "@/models/game_notice";
import { getGameNotice } from "@/api/games/game_id/notices";

export default function Index() {
    const sharedStore = useSharedStore();

    const { game } = useContext(Context);

    const [createDialogOpen, setCreateDialogOpen] = useState<boolean>(false);

    const [total, setTotal] = useState<number>(0);
    const [notices, setNotices] = useState<Array<GameNotice>>([]);

    const [page, _setPage] = useState<number>(1);
    const [size, _setSize] = useState<number>(10);

    const [sorting, setSorting] = useState<SortingState>([]);
    const [columnVisibility, setColumnVisibility] = useState<VisibilityState>({
        game_id: false,
    });
    const [columnFilters, setColumnFilters] = useState<ColumnFiltersState>([]);
    const debouncedColumnFilters = useDebounce(columnFilters, 100);

    const table = useReactTable<GameNotice>({
        data: notices,
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
            columnFilters,
        },
    });

    useEffect(() => {
        if (!game?.id) return;

        getGameNotice({
            game_id: game?.id!,
        }).then((res) => {
            setTotal(res?.total || 0);
            setNotices(res?.data || []);
        });
    }, [
        page,
        size,
        sorting,
        debouncedColumnFilters,
        sharedStore.refresh,
        game,
    ]);

    return (
        <div className={cn(["container", "mx-auto"])}>
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
                    <MessageCircleIcon />
                    通知
                </h1>
                <div
                    className={cn([
                        "flex",
                        "flex-1",
                        "justify-end",
                        "items-center",
                        "gap-3",
                    ])}
                >
                    <Button
                        icon={PlusCircleIcon}
                        variant={"solid"}
                        onClick={() => setCreateDialogOpen(true)}
                    >
                        添加通知
                    </Button>
                    <Dialog
                        open={createDialogOpen}
                        onOpenChange={setCreateDialogOpen}
                    >
                        <DialogContent>
                            <CreateDialog
                                onClose={() => setCreateDialogOpen(false)}
                            />
                        </DialogContent>
                    </Dialog>
                </div>
            </div>

            <div className={cn(["rounded-md", "border", "bg-card"])}>
                <Table className={cn(["text-foreground"])}>
                    <TableHeader>
                        {table.getHeaderGroups().map((headerGroup) => (
                            <TableRow key={headerGroup.id}>
                                {headerGroup.headers.map((header) => {
                                    return (
                                        <TableHead key={header.id}>
                                            {!header.isPlaceholder &&
                                                flexRender(
                                                    header.column.columnDef
                                                        .header,
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
                                    data-state={
                                        row.getIsSelected() && "selected"
                                    }
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
                                    哎呀，好像还没有通知呢。
                                </TableCell>
                            </TableRow>
                        )}
                    </TableBody>
                </Table>
            </div>
        </div>
    );
}
