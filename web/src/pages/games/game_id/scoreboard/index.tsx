import { cn } from "@/utils";
import { ChampionChart } from "./champion-chart";
import { useEffect, useState } from "react";
import { ScoreRecord } from "@/models/game";
import { useGameStore } from "@/storages/game";
import { getGameScoreboard } from "@/api/games/game_id";
import { Pagination } from "@/components/ui/pagination";
import {
  flexRender,
  getCoreRowModel,
  getFilteredRowModel,
  useReactTable,
} from "@tanstack/react-table";
import { columns } from "./columns";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import { Field, FieldIcon } from "@/components/ui/field";
import { ListOrderedIcon } from "lucide-react";
import { Select } from "@/components/ui/select";
import { Dialog, DialogContent, DialogTrigger } from "@/components/ui/dialog";
import { TeamDetailsDialog } from "./team-details-dialog";
import { ScrollArea } from "@/components/ui/scroll-area";

export default function Index() {
  const { currentGame } = useGameStore();
  const [scoreboard, setScoreboard] = useState<Array<ScoreRecord>>([]);
  const [total, setTotal] = useState<number>(0);
  const [size, setSize] = useState<number>(10);
  const [page, setPage] = useState<number>(1);

  useEffect(() => {
    getGameScoreboard({
      id: currentGame?.id!,
      size,
      page,
    }).then((res) => {
      setTotal(res.total || 0);
      setScoreboard(res.data || []);
    });
  }, [currentGame?.id, page, size]);

  const table = useReactTable<ScoreRecord>({
    data: scoreboard,
    columns,
    getCoreRowModel: getCoreRowModel(),
    manualPagination: true,
    rowCount: total,
    manualFiltering: true,
    getFilteredRowModel: getFilteredRowModel(),
    manualSorting: true,
  });

  return (
    <>
      <title>{`积分榜 - ${currentGame?.title}`}</title>
      <div
        className={cn([
          "xl:mx-60",
          "mx-10",
          "my-10",
          "flex",
          "flex-col",
          "gap-10",
          "items-center",
        ])}
      >
        <ChampionChart scoreboard={scoreboard} />
        <div className={cn(["flex", "items-center", "gap-10"])}>
          <div className="flex-1 text-sm text-muted-foreground">
            {table.getFilteredRowModel().rows.length} / {total}
          </div>
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
              onValueChange={(value) => setSize(Number(value))}
            />
          </Field>
          <Pagination
            value={page}
            onChange={setPage}
            total={Math.ceil(total / size)}
          />
        </div>
        <ScrollArea className={cn(["rounded-md", "w-full"])}>
          <Table className={cn(["text-foreground"])}>
            <TableHeader className={cn(["bg-muted/70", "backdrop-blur-md"])}>
              {table.getHeaderGroups().map((headerGroup) => (
                <TableRow key={headerGroup.id}>
                  {headerGroup.headers.map((header) => {
                    return (
                      <TableHead key={header.id}>
                        {!header.isPlaceholder &&
                          flexRender(
                            header.column.columnDef.header,
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
                  <Dialog key={row.original.team?.id}>
                    <DialogTrigger>
                      <TableRow
                        data-state={row.getIsSelected() && "selected"}
                        className={cn(["cursor-pointer"])}
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
                    </DialogTrigger>
                    <DialogContent>
                      <TeamDetailsDialog team={row.original.team!} />
                    </DialogContent>
                  </Dialog>
                ))
              ) : (
                <TableRow>
                  <TableCell
                    colSpan={columns.length}
                    className={cn(["h-24", "text-center"])}
                  >
                    但是谁也没有来。
                  </TableCell>
                </TableRow>
              )}
            </TableBody>
          </Table>
        </ScrollArea>
      </div>
    </>
  );
}
