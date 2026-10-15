import { keepPreviousData, useQuery } from "@tanstack/react-query";
import {
  FlagIcon,
  HashIcon,
  LibraryIcon,
  ListOrderedIcon,
  UserRoundIcon,
  UsersRoundIcon,
} from "lucide-react";
import {
  createParser,
  parseAsNumberLiteral,
  parseAsStringEnum,
  useQueryStates,
} from "nuqs";
import React, { useContext, useMemo, useState } from "react";
import { useTranslation } from "react-i18next";
import { useParams } from "react-router";
import { getGameChallenges } from "@/api/management/games/game_id/challenges";
import { getSubmissions } from "@/api/management/games/game_id/submissions";
import { getTeams } from "@/api/management/games/game_id/teams";
import { getGameUsers } from "@/api/management/games/game_id/users";
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
import { TextField } from "@/components/ui/text-field";
import {
  type ColumnVisibilityState,
  flexRender,
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

const positiveInteger = createParser({
  parse: (value) => {
    if (!/^\d+$/.test(value)) return null;
    const number = Number(value);
    return Number.isSafeInteger(number) && number > 0 ? number : null;
  },
  serialize: String,
});
const pageSizes = [10, 20, 40, 60] as const;
const submissionSearchParams = {
  page: positiveInteger.withDefault(1),
  size: parseAsNumberLiteral(pageSizes).withDefault(10),
  id: positiveInteger,
  team_id: positiveInteger,
  challenge_id: positiveInteger,
  user_id: positiveInteger,
  status: parseAsStringEnum<Status>(Object.values(Status)),
};

type TeamOption = Pick<TeamView, "id" | "name" | "avatar_hash">;
type ChallengeOption = Pick<
  GameChallengeView,
  "challenge_id" | "challenge_title"
>;
type UserOption = Pick<
  UserAccountView,
  "id" | "name" | "username" | "avatar_hash"
>;

export default function Index() {
  const { t } = useTranslation();

  const sharedStore = useSharedStore();

  const { game_id } = useParams<{ game_id: string }>();
  const routeGameId = parseRouteNumericId(game_id);
  const { game } = useContext(Context);

  const [search, setSearch] = useQueryStates(submissionSearchParams, {
    history: "push",
  });
  const { page, size, id, team_id, challenge_id, user_id, status } = search;
  const debouncedId = useDebounce(id, 100);

  const [columnVisibility, setColumnVisibility] =
    useState<ColumnVisibilityState>({
      team_id: false,
      challenge_id: false,
      user_id: false,
    });
  const [teamQuery, setTeamQuery] = useState("");
  const [challengeQuery, setChallengeQuery] = useState("");
  const [userQuery, setUserQuery] = useState("");
  const debouncedTeamQuery = useDebounce(teamQuery.trim(), 150);
  const debouncedUserQuery = useDebounce(userQuery.trim(), 150);

  const gameId = routeGameId ?? game?.id;
  const selectedTeamQuery = useQuery({
    queryKey: ["admin", "game-team-option", gameId, team_id],
    queryFn: () =>
      getTeams({ game_id: gameId!, id: team_id!, size: 1, page: 1 }),
    enabled: gameId != null && team_id != null,
  });
  const selectedTeam: TeamOption | null =
    team_id == null
      ? null
      : (selectedTeamQuery.data?.teams.find((team) => team.id === team_id) ?? {
          id: team_id,
          name: "",
          avatar_hash: null,
        });
  const teamOptionsQuery = useQuery({
    queryKey: ["admin", "game-team-options", gameId, debouncedTeamQuery],
    queryFn: async () => {
      const numericId = /^\d+$/.test(debouncedTeamQuery)
        ? Number(debouncedTeamQuery)
        : undefined;
      const [exact, named] = await Promise.all([
        numericId == null
          ? Promise.resolve({ teams: [] as TeamView[] })
          : getTeams({ game_id: gameId!, id: numericId, size: 5, page: 1 }),
        getTeams({
          game_id: gameId!,
          name: debouncedTeamQuery || undefined,
          size: 10,
          page: 1,
        }),
      ]);
      const seen = new Set<number>();
      return [...exact.teams, ...named.teams]
        .filter((item) => {
          if (seen.has(item.id)) return false;
          seen.add(item.id);
          return true;
        })
        .slice(0, 10);
    },
    enabled: gameId != null,
    placeholderData: keepPreviousData,
  });
  const fetchedTeamOptions = teamOptionsQuery.data ?? [];
  const teamOptions =
    selectedTeam &&
    !fetchedTeamOptions.some((team) => team.id === selectedTeam.id)
      ? [selectedTeam, ...fetchedTeamOptions]
      : fetchedTeamOptions;

  const challengesQuery = useQuery({
    queryKey: ["admin", "game-challenge-options", gameId],
    queryFn: () => getGameChallenges({ game_id: gameId! }),
    enabled: gameId != null,
  });
  const gameChallenges = challengesQuery.data?.challenges ?? [];
  const selectedChallenge: ChallengeOption | null =
    challenge_id == null
      ? null
      : (gameChallenges.find(
          (challenge) => challenge.challenge_id === challenge_id
        ) ?? { challenge_id, challenge_title: "" });

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

    const options = [...exact, ...titleMatches].slice(0, 10);
    const selected = gameChallenges.find(
      (challenge) => challenge.challenge_id === challenge_id
    );
    return selected &&
      !options.some(
        (challenge) => challenge.challenge_id === selected.challenge_id
      )
      ? [selected, ...options]
      : options;
  }, [challengeQuery, gameChallenges, challenge_id]);

  const userOptionsQuery = useQuery({
    queryKey: ["admin", "game-user-options", gameId, debouncedUserQuery],
    queryFn: async () => {
      const numericId = /^\d+$/.test(debouncedUserQuery)
        ? Number(debouncedUserQuery)
        : undefined;
      const [exact, named] = await Promise.all([
        numericId == null
          ? Promise.resolve({ users: [] as UserAccountView[] })
          : getGameUsers({ game_id: gameId!, id: numericId, size: 1, page: 1 }),
        getGameUsers({
          game_id: gameId!,
          name: debouncedUserQuery || undefined,
          size: 10,
          page: 1,
        }),
      ]);
      const seen = new Set<number>();
      return [...exact.users, ...named.users]
        .filter((item) => {
          if (seen.has(item.id)) return false;
          seen.add(item.id);
          return true;
        })
        .slice(0, 10);
    },
    enabled: gameId != null,
    placeholderData: keepPreviousData,
  });
  const selectedUserQuery = useQuery({
    queryKey: ["admin", "game-user-option", gameId, user_id],
    queryFn: () =>
      getGameUsers({ game_id: gameId!, id: user_id!, size: 1, page: 1 }),
    enabled: gameId != null && user_id != null,
  });
  const selectedUser: UserOption | null =
    user_id == null
      ? null
      : (selectedUserQuery.data?.users.find((user) => user.id === user_id) ?? {
          id: user_id,
          name: "",
          username: "",
          avatar_hash: null,
        });
  const fetchedUserOptions = userOptionsQuery.data ?? [];
  const userOptions =
    selectedUser &&
    !fetchedUserOptions.some((user) => user.id === selectedUser.id)
      ? [selectedUser, ...fetchedUserOptions]
      : fetchedUserOptions;

  const submissionsQuery = useQuery({
    queryKey: [
      "admin",
      "game-submissions",
      gameId,
      page,
      size,
      debouncedId,
      team_id,
      challenge_id,
      user_id,
      status,
      sharedStore.refresh,
    ],
    queryFn: () =>
      getSubmissions({
        game_id: gameId!,
        id: debouncedId ?? undefined,
        user_id: user_id ?? undefined,
        team_id: team_id ?? undefined,
        challenge_id: challenge_id ?? undefined,
        status: status ?? undefined,
        sorts: "-created_at",
        page,
        size,
      }),
    enabled: gameId != null,
    placeholderData: keepPreviousData,
  });
  const submissions = submissionsQuery.data?.submissions ?? [];
  const total = submissionsQuery.data?.total ?? 0;
  const loading = submissionsQuery.isFetching;

  const columns = useColumns();

  const table = useDataTable<SubmissionView>({
    data: submissions,
    columns,
    manualPagination: true,
    rowCount: total,
    manualFiltering: true,
    onColumnVisibilityChange: setColumnVisibility,
    manualSorting: true,
    state: {
      columnVisibility,
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
              inputMode="numeric"
              value={id == null ? "" : String(id)}
              onChange={(event) => {
                const value = event.target.value;
                const nextId = positiveInteger.parse(value);
                if (value !== "" && nextId == null) return;
                void setSearch({ id: nextId, page: 1 });
              }}
            />
          </Field>
          <Field size={"sm"} className={cn(["min-w-48", "flex-1"])}>
            <FieldIcon>
              <UsersRoundIcon />
            </FieldIcon>
            <Combobox<TeamOption>
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
                void setSearch({ team_id: team?.id ?? null, page: 1 });
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
            <Combobox<ChallengeOption>
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
                void setSearch({
                  challenge_id: challenge?.challenge_id ?? null,
                  page: 1,
                });
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
            <Combobox<UserOption>
              value={selectedUser}
              options={userOptions.map((user) => ({
                value: user,
                content: `#${user.id} ${user.name}`,
              }))}
              itemToStringLabel={(user) => `#${user.id} ${user.name}`}
              isItemEqualToValue={(item, value) => item.id === value.id}
              filter={null}
              onInputValueChange={(value, details) => {
                if (details.reason === "input-change" || value === "") {
                  setUserQuery(value);
                }
              }}
              onValueChange={(user) => {
                void setSearch({ user_id: user?.id ?? null, page: 1 });
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
                        {user.name || user.username}
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
              value={status ?? "all"}
              onValueChange={(value) =>
                void setSearch({
                  status: value === "all" ? null : (value as Status),
                  page: 1,
                })
              }
            >
              <SelectTrigger>
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="all">{t("common:all")}</SelectItem>
                {statusOptions.map((status) => (
                  <SelectItem key={status.id} value={String(status.id)}>
                    {status.name}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
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
                          header.column.id === "id" && "w-16",
                          header.column.id === "team_name" && "w-56",
                          header.column.id === "challenge_title" && "w-56",
                          header.column.id === "content" && "min-w-56",
                          header.column.id === "status" && "w-32 min-w-32",
                          header.column.id === "score_rank" && "w-28 min-w-28",
                          header.column.id === "user_name" && "w-56",
                          header.column.id === "processing_duration" && "w-32",
                          header.column.id === "created_at" && "w-48 min-w-48",
                          header.column.id === "actions" && [
                            "w-24 min-w-24 text-center",
                            "sticky",
                            "right-0",
                            "z-3",
                            "backdrop-blur-xs",
                          ],
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
                              cell.column.id === "actions" && [
                                "w-24",
                                "sticky",
                                "right-0",
                                "z-3",
                                "backdrop-blur-xs",
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
            onChange={(value) => void setSearch({ page: value })}
          />
          <Field size={"sm"} className={cn(["w-32", "sm:w-36"])}>
            <FieldIcon>
              <ListOrderedIcon />
            </FieldIcon>
            <Select
              value={String(size)}
              onValueChange={(value) => {
                void setSearch({
                  size: Number(value) as (typeof pageSizes)[number],
                  page: 1,
                });
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
    </div>
  );
}
