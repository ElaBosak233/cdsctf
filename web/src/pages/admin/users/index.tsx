import { User, Group } from "@/models/user";
import { Button } from "@/components/ui/button";
import {
    HashIcon,
    ListOrderedIcon,
    PlusCircle,
    TypeIcon,
    ShieldIcon,
    MailIcon,
    UserRoundIcon,
    UserRoundXIcon,
    UserRoundCheckIcon,
} from "lucide-react";
import { useEffect, useState } from "react";
import { getUsers } from "@/api/users";
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
import { Pagination } from "@/components/ui/pagination";
import { cn } from "@/utils";
import { columns } from "./columns";
import { Field, FieldIcon } from "@/components/ui/field";
import { TextField } from "@/components/ui/text-field";
import { useDebounce } from "@/hooks/use-debounce";
import { Select } from "@/components/ui/select";
import { Dialog, DialogContent } from "@/components/ui/dialog";
import { CreateUserDialog } from "./create-dialog";
import { useSharedStore } from "@/storages/shared";

export default function Index() {
    const sharedStore = useSharedStore();

    const [createDialogOpen, setCreateDialogOpen] = useState<boolean>(false);

    const [total, setTotal] = useState<number>(0);
    const [users, setUsers] = useState<Array<User>>([]);

    const [page, setPage] = useState<number>(1);
    const [size, setSize] = useState<number>(10);

    const [sorting, setSorting] = useState<SortingState>([
        {
            id: "created_at",
            desc: false,
        },
    ]);
    const [columnVisibility, setColumnVisibility] = useState<VisibilityState>(
        {}
    );
    const [columnFilters, setColumnFilters] = useState<ColumnFiltersState>([
        {
            id: "group",
            value: "all",
        },
    ]);
    const debouncedColumnFilters = useDebounce(columnFilters, 100);

    const groupOptions = [
        { id: "all", name: "全部", icon: UserRoundIcon },
        { id: Group.Banned.toString(), name: "封禁", icon: UserRoundXIcon },
        { id: Group.User.toString(), name: "用户", icon: UserRoundCheckIcon },
        { id: Group.Admin.toString(), name: "管理员", icon: ShieldIcon },
    ];

    const table = useReactTable<User>({
        data: users,
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
        getUsers({
            id: debouncedColumnFilters.find((c) => c.id === "id")
                ?.value as number,
            name: debouncedColumnFilters.find((c) => c.id === "username")
                ?.value as string,
            email: debouncedColumnFilters.find((c) => c.id === "email")
                ?.value as string,
            group:
                debouncedColumnFilters.find((c) => c.id === "group")?.value !==
                "all"
                    ? Number(
                          debouncedColumnFilters.find((c) => c.id === "group")
                              ?.value
                      )
                    : undefined,
            sorts: sorting
                .map((value) => (value.desc ? `-${value.id}` : `${value.id}`))
                .join(","),
            page,
            size,
        }).then((res) => {
            setTotal(res?.total || 0);
            setUsers(res?.data || []);
        });
    }, [page, size, sorting, debouncedColumnFilters, sharedStore.refresh]);

    return (
        <div className={cn(["container", "mx-auto", "py-10"])}>
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
                    <UserRoundIcon />
                    用户管理
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
                            value={
                                (table
                                    .getColumn("id")
                                    ?.getFilterValue() as string) ?? ""
                            }
                            onChange={(e) =>
                                table
                                    .getColumn("id")
                                    ?.setFilterValue(e.target.value)
                            }
                        />
                    </Field>
                    <Field size={"sm"} className={cn(["flex-1"])}>
                        <FieldIcon>
                            <TypeIcon />
                        </FieldIcon>
                        <TextField
                            placeholder={"用户名"}
                            value={
                                (table
                                    .getColumn("username")
                                    ?.getFilterValue() as string) ?? ""
                            }
                            onChange={(e) =>
                                table
                                    .getColumn("username")
                                    ?.setFilterValue(e.target.value)
                            }
                        />
                    </Field>
                    <Field size={"sm"} className={cn(["flex-1"])}>
                        <FieldIcon>
                            <MailIcon />
                        </FieldIcon>
                        <TextField
                            placeholder={"邮箱"}
                            value={
                                (table
                                    .getColumn("email")
                                    ?.getFilterValue() as string) ?? ""
                            }
                            onChange={(e) =>
                                table
                                    .getColumn("email")
                                    ?.setFilterValue(e.target.value)
                            }
                        />
                    </Field>
                    <Field size={"sm"} className={cn(["flex-1"])}>
                        <FieldIcon>
                            <UserRoundIcon />
                        </FieldIcon>
                        <Select
                            options={groupOptions.map((group) => ({
                                value: group.id,
                                content: (
                                    <div
                                        className={cn([
                                            "flex",
                                            "gap-2",
                                            "items-center",
                                        ])}
                                    >
                                        <group.icon className="size-4" />
                                        {group.name}
                                    </div>
                                ),
                            }))}
                            onValueChange={(value) =>
                                table.getColumn("group")?.setFilterValue(value)
                            }
                            value={
                                (table
                                    .getColumn("group")
                                    ?.getFilterValue() as string) ?? "all"
                            }
                        />
                    </Field>

                    <Button
                        icon={PlusCircle}
                        variant={"solid"}
                        onClick={() => setCreateDialogOpen(true)}
                    >
                        添加用户
                    </Button>
                    <Dialog
                        open={createDialogOpen}
                        onOpenChange={setCreateDialogOpen}
                    >
                        <DialogContent>
                            <CreateUserDialog
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
                                    用户列表空空如也呢...要不要加我一个?
                                    天天给你干活，也有点无聊呢=v=
                                </TableCell>
                            </TableRow>
                        )}
                    </TableBody>
                </Table>
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
                                onValueChange={(value) =>
                                    setSize(Number(value))
                                }
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
        </div>
    );
}
