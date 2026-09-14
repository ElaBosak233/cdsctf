import {
  FlagIcon,
  HashIcon,
  LibraryIcon,
  ListOrderedIcon,
  UserRoundIcon,
  UsersRoundIcon,
} from "lucide-react";
import React, { useContext, useEffect, useMemo, useState } from "react";
import { useTranslation } from "react-i18next";
import { useParams } from "react-router";
import { getGameChallenges } from "@/api/admin/games/game_id/challenges";
import { getSubmissions } from "@/api/admin/games/game_id/submissions";
import { getTeams } from "@/api/admin/games/game_id/teams";
import { getGameUsers } from "@/api/admin/games/game_id/users";
import { Avatar } from "@/components/ui/avatar";
import {
  Combobox,
  ComboboxContent,
  ComboboxEmpty,
  ComboboxInput,
  ComboboxItem,
  ComboboxList,
} from "@/components/ui/combobox";
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
import {
  type ColumnFiltersState,
  type ColumnVisibilityState,
  flexRender,
  type SortingState,
  useDataTable,
} from "@/hooks/use-data-table";
import { useDebounce } from "@/hooks/use-debounce";
import type { GameChallengeView } from "@/models/game_challenge";
import { Status, type SubmissionView } from "@/models/submission";
import type { TeamView } from "@/models/team";
import type { UserAccountView } from "@/models/user";
import { useSharedStore } from "@/storages/shared";
import { cn } from "@/utils";
import { parseRouteNumericId } from "@/utils/query";
import { Context } from "../context";
import { useColumns } from "./_blocks/columns";

export default function Index() {
  const { t } = useTranslation();

  const sharedStore = useSharedStore();

  const { game_id } = useParams<{ game_id: string }>();
  const routeGameId = parseRouteNumericId(game_id);
  const { game } = useContext(Context);

  const [total, setTotal] = useState<number>(0);
  const [submissions, setSubmissions] = useState<Array<SubmissionView>>([]);
  const [loading, setLoading] = useState<boolean>(false);

  const [page, setPage] = useState<number>(1);
  const [size, setSize] = useState<number>(10);

  const [sorting, setSorting] = useState<SortingState>([]);
  const [columnVisibility, setColumnVisibility] =
    useState<ColumnVisibilityState>({
      team_id: false,
      challenge_id: false,
      user_id: false,
    });
  const [columnFilters, setColumnFilters] = useState<ColumnFiltersState>([
    {
      id: "status",
      value: "all",
    },
  ]);
  const debouncedColumnFilters = useDebounce(columnFilters, 100);
  const [teamQuery, setTeamQuery] = useState("");
  const [challengeQuery, setChallengeQuery] = useState("");
  const [userQuery, setUserQuery] = useState("");
  const debouncedTeamQuery = useDebounce(teamQuery.trim(), 150);
  const debouncedUserQuery = useDebounce(userQuery.trim(), 150);
  const [teamOptions, setTeamOptions] = useState<TeamView[]>([]);
  const [gameChallenges, setGameChallenges] = useState<GameChallengeView[]>([]);
  const [userOptions, setUserOptions] = useState<UserAccountView[]>([]);
  const [selectedTeam, setSelectedTeam] = useState<TeamView | null>(null);
  const [selectedChallenge, setSelectedChallenge] =
    useState<GameChallengeView | null>(null);
  const [selectedUser, setSelectedUser] = useState<UserAccountView | null>(
    null
  );

  useEffect(() => {
    const gameId = routeGameId ?? game?.id;
    if (gameId == null) return;
    const numericId = /^\d+$/.test(debouncedTeamQuery)
      ? Number(debouncedTeamQuery)
      : undefined;
    let cancelled = false;
    Promise.all([
      numericId == null
        ? Promise.resolve({ teams: [] as TeamView[] })
        : getTeams({ game_id: gameId, id: numericId, size: 5, page: 1 }),
      getTeams({
        game_id: gameId,
        name: debouncedTeamQuery || undefined,
        size: 10,
        page: 1,
      }),
    ]).then(([exact, named]) => {
      if (cancelled) return;
      const seen = new Set<number>();
      setTeamOptions(
        [...exact.teams, ...named.teams]
          .filter((item) => {
            if (seen.has(item.id)) return false;
            seen.add(item.id);
            return true;
          })
          .slice(0, 10)
      );
    });
    return () => {
      cancelled = true;
    };
  }, [debouncedTeamQuery, game, routeGameId]);

  useEffect(() => {
    const gameId = routeGameId ?? game?.id;
    if (gameId == null) {
      setGameChallenges([]);
      return;
    }

    let cancelled = false;
    getGameChallenges({ game_id: gameId }).then((response) => {
      if (!cancelled) setGameChallenges(response.challenges);
    });

    return () => {
      cancelled = true;
    };
  }, [game, routeGameId]);

  const challengeOptions = useMemo(() => {
    const query = challengeQuery.trim();
    const normalizedQuery = query.toLocaleLowerCase();
    const numericId = /^\d+$/.test(query) ? Number(query) : null;
    const exact =
      numericId == null
        ? []
        : gameChallenges.filter(
            (challenge) => challenge.challenge_id === numericId
          );
    const exactIds = new Set(exact.map((challenge) => challenge.challenge_id));
    const titleMatches = gameChallenges.filter(
      (challenge) =>
        !exactIds.has(challenge.challenge_id) &&
        challenge.challenge_title.toLocaleLowerCase().includes(normalizedQuery)
    );

    return [...exact, ...titleMatches].slice(0, 10);
  }, [challengeQuery, gameChallenges]);

  useEffect(() => {
    const numericId = /^\d+$/.test(debouncedUserQuery)
      ? Number(debouncedUserQuery)
      : undefined;
    const gameId = routeGameId ?? game?.id;
    if (gameId == null) return;
    let cancelled = false;
    Promise.all([
      numericId == null
        ? Promise.resolve({ users: [] as UserAccountView[] })
        : getGameUsers({ game_id: gameId, id: numericId, size: 1, page: 1 }),
      getGameUsers({
        game_id: gameId,
        name: debouncedUserQuery || undefined,
        size: 10,
        page: 1,
      }),
    ]).then(([exact, named]) => {
      if (cancelled) return;
      const seen = new Set<number>();
      setUserOptions(
        [...exact.users, ...named.users]
          .filter((item) => {
            if (seen.has(item.id)) return false;
            seen.add(item.id);
            return true;
          })
          .slice(0, 10)
      );
    });
    return () => {
      cancelled = true;
    };
  }, [debouncedUserQuery, game, routeGameId]);

  const columns = useColumns();

  const table = useDataTable<SubmissionView>({
    data: submissions,
    columns,
    manualPagination: true,
    rowCount: total,
    manualFiltering: true,
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

  const statusOptions = [
    { id: Status.Queued, name: t("submission:status.queued") },
    { id: Status.Processing, name: t("submission:status.processing") },
    { id: Status.Correct, name: t("submission:status.correct") },
    { id: Status.Incorrect, name: t("submission:status.incorrect") },
    { id: Status.Cheat, name: t("submission:status.cheat") },
    { id: Status.Expired, name: t("submission:status.expired") },
    { id: Status.Duplicate, name: t("submission:status.duplicate") },
  ];

  useEffect(() => {
    void sorting;
    void sharedStore.refresh;

    const gid = routeGameId ?? game?.id;
    if (gid == null) return;

    setLoading(true);

    const rawStatus = debouncedColumnFilters.find(
      (c) => c.id === "status"
    )?.value;
    const status = Object.values(Status).includes(rawStatus as Status)
      ? (rawStatus as Status)
      : undefined;

    getSubmissions({
      game_id: gid,
      id: debouncedColumnFilters.find((c) => c.id === "id")?.value as number,
      user_id: debouncedColumnFilters.find((c) => c.id === "user_id")
        ?.value as number,
      team_id: debouncedColumnFilters.find((c) => c.id === "team_id")
        ?.value as number,
      challenge_id: debouncedColumnFilters.find((c) => c.id === "challenge_id")
        ?.value as number,
      status,
      sorts: "-created_at",
      page,
      size,
    })
      .then((res) => {
        setTotal(res?.total || 0);
        setSubmissions(res?.submissions || []);
      })
      .finally(() => {
        setLoading(false);
      });
  }, [
    page,
    size,
    sorting,
    debouncedColumnFilters,
    sharedStore.refresh,
    game,
    routeGameId,
  ]);

  return (
    <div
      className={cn([
        "h-full",
        "w-full",
        "min-w-0",
        "min-h-0",
        "flex",
        "flex-col",
        "gap-4",
        "px-4",
        "py-4",
        "sm:px-6",
        "sm:py-6",
        "lg:px-8",
        "lg:py-8",
      ])}
    >
      <div
        className={cn([
          "flex",
          "justify-between",
          "items-center",
          "shrink-0",
          "gap-6",
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
          <FlagIcon />
          {t("submission:_")}
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
              placeholder={t("game:id")}
              value={(table.getColumn("id")?.getFilterValue() as string) ?? ""}
              onChange={(e) =>
                table.getColumn("id")?.setFilterValue(e.target.value)
              }
            />
          </Field>
          <Field size={"sm"} className={cn(["min-w-48", "flex-1"])}>
            <FieldIcon>
              <UsersRoundIcon />
            </FieldIcon>
            <Combobox<TeamView>
              value={selectedTeam}
              options={teamOptions.map((team) => ({
                value: team,
                content: `#${team.id} ${team.name}`,
              }))}
              itemToStringLabel={(team) => `#${team.id} ${team.name}`}
              isItemEqualToValue={(item, value) => item.id === value.id}
              filter={null}
              onInputValueChange={(value, details) => {
                if (details.reason === "input-change" || value === "") {
                  setTeamQuery(value);
                }
              }}
              onValueChange={(team) => {
                setSelectedTeam(team);
                table
                  .getColumn("team_id")
                  ?.setFilterValue(team ? String(team.id) : undefined);
              }}
              placeholder={t("submission:team_id")}
            >
              <ComboboxInput
                showClear
                placeholder={t("submission:team_id")}
                startContent={
                  selectedTeam && (
                    <Avatar
                      className="size-6"
                      src={
                        selectedTeam.avatar_hash &&
                        `/api/media?hash=${selectedTeam.avatar_hash}`
                      }
                      fallback={selectedTeam.name.charAt(0)}
                    />
                  )
                }
              />
              <ComboboxContent>
                <ComboboxEmpty>{t("game:team.empty")}</ComboboxEmpty>
                <ComboboxList>
                  {teamOptions.map((team) => (
                    <ComboboxItem key={team.id} value={team}>
                      <Avatar
                        className="size-6"
                        src={
                          team.avatar_hash &&
                          `/api/media?hash=${team.avatar_hash}`
                        }
                        fallback={team.name.charAt(0)}
                      />
                      <span className="shrink-0 font-mono text-xs text-muted-foreground">
                        #{team.id}
                      </span>
                      <span className="truncate">{team.name}</span>
                    </ComboboxItem>
                  ))}
                </ComboboxList>
              </ComboboxContent>
            </Combobox>
          </Field>
          <Field size={"sm"} className={cn(["min-w-48", "flex-1"])}>
            <FieldIcon>
              <LibraryIcon />
            </FieldIcon>
            <Combobox<GameChallengeView>
              value={selectedChallenge}
              options={challengeOptions.map((challenge) => ({
                value: challenge,
                content: `#${challenge.challenge_id} ${challenge.challenge_title}`,
              }))}
              itemToStringLabel={(challenge) =>
                `#${challenge.challenge_id} ${challenge.challenge_title}`
              }
              isItemEqualToValue={(item, value) =>
                item.challenge_id === value.challenge_id
              }
              filter={null}
              placeholder={t("submission:challenge_id")}
              onInputValueChange={(value, details) => {
                if (details.reason === "input-change" || value === "") {
                  setChallengeQuery(value);
                }
              }}
              onValueChange={(challenge) => {
                setSelectedChallenge(challenge);
                table
                  .getColumn("challenge_id")
                  ?.setFilterValue(
                    challenge ? String(challenge.challenge_id) : undefined
                  );
              }}
            >
              <ComboboxInput
                showClear
                placeholder={t("submission:challenge_id")}
              />
              <ComboboxContent>
                <ComboboxEmpty>{t("challenge:empty")}</ComboboxEmpty>
                <ComboboxList>
                  {challengeOptions.map((challenge) => (
                    <ComboboxItem
                      key={challenge.challenge_id}
                      value={challenge}
                    >
                      <span className="shrink-0 font-mono text-xs text-muted-foreground">
                        #{challenge.challenge_id}
                      </span>
                      <span className="min-w-0 truncate">
                        {challenge.challenge_title}
                      </span>
                    </ComboboxItem>
                  ))}
                </ComboboxList>
              </ComboboxContent>
            </Combobox>
          </Field>
          <Field size={"sm"} className={cn(["min-w-48", "flex-1"])}>
            <FieldIcon>
              <UserRoundIcon />
            </FieldIcon>
            <Combobox<UserAccountView>
              value={selectedUser}
              options={userOptions.map((user) => ({
                value: user,
                content: `#${user.id} ${user.username}`,
              }))}
              itemToStringLabel={(user) => `#${user.id} ${user.username}`}
              isItemEqualToValue={(item, value) => item.id === value.id}
              filter={null}
              onInputValueChange={(value, details) => {
                if (details.reason === "input-change" || value === "") {
                  setUserQuery(value);
                }
              }}
              onValueChange={(user) => {
                setSelectedUser(user);
                table
                  .getColumn("user_id")
                  ?.setFilterValue(user ? String(user.id) : undefined);
              }}
              placeholder={t("submission:user_id")}
            >
              <ComboboxInput
                showClear
                placeholder={t("submission:user_id")}
                startContent={
                  selectedUser && (
                    <Avatar
                      className="size-6"
                      src={
                        selectedUser.avatar_hash &&
                        `/api/media?hash=${selectedUser.avatar_hash}`
                      }
                      fallback={selectedUser.name.charAt(0)}
                    />
                  )
                }
              />
              <ComboboxContent>
                <ComboboxEmpty>{t("user:empty")}</ComboboxEmpty>
                <ComboboxList>
                  {userOptions.map((user) => (
                    <ComboboxItem key={user.id} value={user}>
                      <Avatar
                        className="size-6"
                        src={
                          user.avatar_hash &&
                          `/api/media?hash=${user.avatar_hash}`
                        }
                        fallback={user.name.charAt(0)}
                      />
                      <span className="shrink-0 font-mono text-xs text-muted-foreground">
                        #{user.id}
                      </span>
                      <span className="truncate">
                        {user.username || user.name}
                      </span>
                    </ComboboxItem>
                  ))}
                </ComboboxList>
              </ComboboxContent>
            </Combobox>
          </Field>
          <Field size={"sm"} className={cn(["min-w-44", "flex-1"])}>
            <FieldIcon>
              <ListOrderedIcon />
            </FieldIcon>
            <Select
              options={[
                {
                  value: "all",
                  content: (
                    <div className={cn(["flex", "gap-2", "items-center"])}>
                      {t("common:all")}
                    </div>
                  ),
                },
                ...statusOptions.map((status) => {
                  return {
                    value: String(status?.id),
                    content: (
                      <div className={cn(["flex", "gap-2", "items-center"])}>
                        {status?.name}
                      </div>
                    ),
                  };
                }),
              ]}
              onValueChange={(value) =>
                table.getColumn("status")?.setFilterValue(value)
              }
              value={
                (table.getColumn("status")?.getFilterValue() as string) ?? "all"
              }
            />
          </Field>
        </div>
      </div>

      <div className={cn(["flex-1", "min-h-0", "flex", "flex-col"])}>
        <ScrollArea
          className={cn([
            "flex-1",
            "min-h-0",
            "min-w-0",
            "w-full",
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
            className={cn([
              "w-full",
              "min-w-full",
              "table-auto",
              "max-w-none",
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
                  {headerGroup.headers.map((header) => {
                    return (
                      <TableHead
                        key={header.id}
                        className={cn([
                          "bg-muted/95",
                          header.column.id === "id" && "w-16 min-w-16",
                          header.column.id === "team_name" && "min-w-56",
                          header.column.id === "challenge_title" && "min-w-56",
                          header.column.id === "content" && "w-56 min-w-56",
                          header.column.id === "status" && "w-32 min-w-32",
                          header.column.id === "user_name" && "min-w-56",
                          header.column.id === "processing_duration" &&
                            "w-32 min-w-32",
                          header.column.id === "created_at" && "w-48 min-w-48",
                          header.column.id === "actions" && "w-24 min-w-24",
                        ])}
                      >
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
            <TableBody className={cn(["flex-1"])}>
              {table.getRowModel().rows?.length
                ? table.getRowModel().rows.map((row) => (
                    <React.Fragment key={row.id}>
                      <TableRow
                        key={row.getValue("id")}
                        data-state={row.getIsSelected() && "selected"}
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
                              cell.column.id === "actions" && "w-24",
                            ])}
                          >
                            {flexRender(
                              cell.column.columnDef.cell,
                              cell.getContext()
                            )}
                          </TableCell>
                        ))}
                      </TableRow>
                    </React.Fragment>
                  ))
                : !loading && (
                    <TableRow>
                      <TableCell
                        colSpan={columns.length}
                        className={cn(["h-24", "text-center"])}
                      >
                        {t("submission:empty")}
                      </TableCell>
                    </TableRow>
                  )}
            </TableBody>
          </Table>
        </ScrollArea>
      </div>
      <footer className="flex shrink-0 flex-col gap-3 py-1 sm:flex-row sm:items-center sm:justify-between">
        <p className="text-sm text-muted-foreground">
          {table.getFilteredRowModel().rows.length} / {total}
        </p>
        <div
          className={cn([
            "flex",
            "min-h-10",
            "flex-wrap",
            "items-center",
            "gap-3",
          ])}
        >
          <Pagination
            size={"sm"}
            value={page}
            total={Math.ceil(total / size)}
            onChange={setPage}
          />
          <Field size={"sm"} className={cn(["w-32", "sm:w-36"])}>
            <FieldIcon>
              <ListOrderedIcon />
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
                setPage(1);
                setSize(Number(value));
              }}
            />
          </Field>
        </div>
      </footer>
    </div>
  );
}
