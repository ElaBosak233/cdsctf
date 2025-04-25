import { getSubmission } from "@/api/submissions";
import { Avatar } from "@/components/ui/avatar";
import { Badge } from "@/components/ui/badge";
import { Card } from "@/components/ui/card";
import { Pagination } from "@/components/ui/pagination";
import { ScrollArea } from "@/components/ui/scroll-area";
import { Separator } from "@/components/ui/separator";
import {
    Table,
    TableBody,
    TableCell,
    TableHead,
    TableHeader,
    TableRow,
} from "@/components/ui/table";
import { Status, Submission } from "@/models/submission";
import { Team } from "@/models/team";
import { useGameStore } from "@/storages/game";
import { cn } from "@/utils";
import {
    ColumnDef,
    flexRender,
    getCoreRowModel,
    getFilteredRowModel,
    useReactTable,
} from "@tanstack/react-table";
import { StarIcon } from "lucide-react";
import { useEffect, useState } from "react";

interface TeamDetailsDialogProps {
    team: Team;
}

function TeamDetailsDialog(props: TeamDetailsDialogProps) {
    const { team } = props;
    const { currentGame } = useGameStore();

    const [submissions, setSubmissions] = useState<Array<Submission>>([]);
    const [total, setTotal] = useState<number>(0);
    const [size, setSize] = useState<number>(10);
    const [page, setPage] = useState<number>(1);

    useEffect(() => {
        getSubmission({
            game_id: currentGame?.id!,
            team_id: team.id,
            status: Status.Correct,
            page: page,
            size: size,
            sorts: "-created_at",
        }).then((res) => {
            if (res.code === 200) {
                setTotal(res.total || 0);
                setSubmissions(res.data || []);
            }
        });
    }, []);

    const columns: ColumnDef<Submission>[] = [
        {
            accessorKey: "user_id",
            id: "user_id",
            header: "用户",
            cell: ({ row }) => (
                <div className={cn(["flex", "items-center", "gap-4"])}>
                    <Avatar
                        className={cn(["size-7"])}
                        src={`/api/users/${row.original.user_id}/avatar`}
                        fallback={row.original.user?.nickname?.charAt(0)}
                    />
                    <span>{row.original.user?.nickname}</span>
                </div>
            ),
        },
        {
            accessorKey: "challenge_id",
            id: "challenge_id",
            header: "题目",
            cell: ({ row }) => {
                return (
                    <div className={cn(["flex", "items-center", "gap-3"])}>
                        {row.original.challenge?.title}
                    </div>
                );
            },
        },
        {
            accessorKey: "pts",
            id: "pts",
            header: "分数",
            cell: ({ row }) => (
                <span className={cn(["font-mono"])}>{row.original.pts}</span>
            ),
        },
        {
            accessorKey: "created_at",
            id: "created_at",
            header: "时间",
            cell: ({ row }) => (
                <span
                    className={cn(["font-mono", "text-secondary-foreground"])}
                >
                    {new Date(
                        Number(row.original.created_at) * 1000
                    ).toLocaleString()}
                </span>
            ),
        },
    ];

    const table = useReactTable<Submission>({
        data: submissions,
        columns: columns,
        getCoreRowModel: getCoreRowModel(),
        manualPagination: true,
        rowCount: total,
        manualFiltering: true,
        getFilteredRowModel: getFilteredRowModel(),
        manualSorting: true,
    });

    return (
        <Card
            className={cn([
                "p-6",
                "min-h-128",
                "w-screen",
                "md:w-3xl",
                "flex",
                "flex-col",
                "gap-6",
            ])}
        >
            <div
                className={cn([
                    "flex",
                    "gap-5",
                    "select-none",
                    "justify-between",
                    "items-center",
                    "mx-6",
                ])}
            >
                <div className={cn(["flex", "gap-5", "my-5"])}>
                    <Avatar
                        className={cn(["size-15"])}
                        src={`/api/games/${currentGame?.id}/teams/${team.id}/avatar`}
                        fallback={team.name?.charAt(0)}
                    />
                    <div className={cn(["flex", "flex-col", "gap-1"])}>
                        <span className={cn(["text-2xl", "font-semibold"])}>
                            {team.name}
                        </span>
                        <span
                            className={cn([
                                "text-secondary-foreground",
                                "text-sm",
                            ])}
                        >
                            {team.slogan || "这个小队很懒，什么也没留下"}
                        </span>
                    </div>
                </div>
                <Badge
                    className={cn([
                        "font-mono",
                        "flex",
                        "gap-1",
                        "items-center",
                    ])}
                >
                    <StarIcon />
                    <span>{team?.pts} pts</span>
                </Badge>
            </div>
            <ScrollArea className={cn(["h-128", "rounded-md", "border"])}>
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
                                    key={row.original.id}
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
                                    className={cn(["h-24", "text-center"])}
                                >
                                    积分榜空空如也呢
                                </TableCell>
                            </TableRow>
                        )}
                    </TableBody>
                </Table>
            </ScrollArea>
            <div
                className={cn([
                    "flex",
                    "items-center",
                    "justify-between",
                    "space-x-2",
                    "px-4",
                ])}
            >
                <div
                    className={cn([
                        "flex-1",
                        "text-sm",
                        "text-muted-foreground",
                    ])}
                >
                    {table.getFilteredRowModel().rows.length} / {total}
                </div>
                <div className={cn(["flex", "items-center", "gap-5"])}>
                    <Pagination
                        size={"sm"}
                        value={page}
                        total={Math.ceil(total / size)}
                        onChange={setPage}
                    />
                </div>
            </div>
        </Card>
    );
}

export { TeamDetailsDialog };
