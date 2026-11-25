import { TrophyIcon } from "lucide-react";
import { type CSSProperties, useMemo } from "react";
import { useTranslation } from "react-i18next";
import { Avatar } from "@/components/ui/avatar";
import { Button } from "@/components/ui/button";
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from "@/components/ui/popover";
import { ScrollArea } from "@/components/ui/scroll-area";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import type {
  ScoreboardChallenge,
  ScoreboardEntry,
  ScoreboardSubmission,
} from "@/models/game";
import { cn } from "@/utils";
import { getCategory } from "@/utils/category";
import { parseTimestamp } from "@/utils/time";

function SolveCell({
  submissions,
  title,
}: {
  submissions: ScoreboardSubmission[];
  title: string;
}) {
  const { t } = useTranslation();
  if (!submissions.length)
    return (
      <span
        className="text-muted-foreground/50"
        aria-label={t("game:scoreboard.unsolved")}
      >
        —
      </span>
    );
  const pts = submissions.reduce((sum, submission) => sum + submission.pts, 0);
  return (
    <Popover>
      <PopoverTrigger
        render={
          <Button
            size="sm"
            variant="link"
            level="success"
            className="h-8 min-w-12 px-2 font-mono text-xs font-semibold tabular-nums focus-visible:outline-2 focus-visible:outline-ring"
            aria-label={t("game:scoreboard.solve_details", { title, pts })}
          />
        }
      >
        +{pts.toLocaleString()}
      </PopoverTrigger>
      <PopoverContent
        className="w-72 gap-3 p-4"
        aria-label={t("game:scoreboard.solve_details", { title, pts })}
      >
        {submissions.map((submission) => (
          <div
            key={submission.id}
            className="flex items-center gap-3 text-xs not-first:border-t not-first:pt-3"
          >
            <Avatar
              className="size-8"
              src={
                submission.user_avatar_hash
                  ? `/api/media?hash=${submission.user_avatar_hash}`
                  : undefined
              }
              fallback={submission.user_name.charAt(0)}
              aria-hidden="true"
            />
            <div className="grid min-w-0 flex-1 grid-cols-1 gap-1">
              <div className="flex items-center justify-between gap-3">
                <span
                  className="min-w-0 flex-1 truncate"
                  title={submission.user_name}
                >
                  {submission.user_name}
                </span>
                <span className="shrink-0 font-mono font-semibold text-success tabular-nums">
                  +{submission.pts.toLocaleString()} {t("game:filter.pts")}
                </span>
              </div>
              <time
                className="font-mono text-[11px] text-muted-foreground"
                dateTime={submission.created_at}
              >
                {parseTimestamp(submission.created_at)?.toLocaleString() ?? "—"}
              </time>
            </div>
          </div>
        ))}
      </PopoverContent>
    </Popover>
  );
}

export function ScoreboardTable({
  records,
  challenges,
  selfTeamId,
}: {
  records: ScoreboardEntry[];
  challenges: ScoreboardChallenge[];
  selfTeamId?: number;
}) {
  const { t } = useTranslation();
  const rows = useMemo(
    () =>
      records.map((record) => {
        const solves = new Map<number, ScoreboardSubmission[]>();
        for (const submission of record.submissions) {
          const group = solves.get(submission.challenge_id) ?? [];
          group.push(submission);
          solves.set(submission.challenge_id, group);
        }
        return { ...record, solves };
      }),
    [records]
  );
  const { categoryGroups, groupStartIds } = useMemo(() => {
    const groups: Array<{
      key: string;
      category: ReturnType<typeof getCategory>;
      challenges: ScoreboardChallenge[];
    }> = [];
    const startIds = new Set<number>();

    for (const challenge of challenges) {
      const last = groups[groups.length - 1];
      if (
        last &&
        last.challenges[0].challenge_category === challenge.challenge_category
      ) {
        last.challenges.push(challenge);
      } else {
        startIds.add(challenge.challenge_id);
        groups.push({
          key: `${challenge.challenge_category}-${challenge.challenge_id}`,
          category: getCategory(challenge.challenge_category),
          challenges: [challenge],
        });
      }
    }

    return { categoryGroups: groups, groupStartIds: startIds };
  }, [challenges]);

  const stickyCell = "sticky z-10 bg-inherit";
  const headerSurface = "bg-[color-mix(in_oklab,var(--muted)_60%,var(--card))]";
  const hasChallenges = challenges.length > 0;
  const pinnedRowSpan = hasChallenges ? 2 : 1;

  return (
    <ScrollArea
      vertical={false}
      className="w-full rounded-lg border bg-card [--rank-width:2.5rem] [--team-width:8rem] [--score-width:5rem] [--solves-width:0rem] sm:[--rank-width:3rem] sm:[--team-width:14rem] sm:[--score-width:6rem] sm:[--solves-width:5rem] 2xl:h-full"
    >
      <Table
        className="w-full table-fixed border-separate border-spacing-0"
        style={{
          minWidth: `calc(var(--rank-width) + var(--team-width) + var(--score-width) + var(--solves-width) + ${challenges.length} * 9rem)`,
        }}
        aria-label={t("game:scoreboard._")}
      >
        <colgroup>
          <col className="w-(--rank-width)" />
          <col className="w-(--team-width)" />
          <col className="w-(--score-width)" />
          <col className="hidden w-(--solves-width) sm:table-column" />
          {challenges.map((challenge) => (
            <col key={challenge.challenge_id} className="w-36" />
          ))}
          {/* Let unused space expand here instead of stretching the data columns. */}
          <col />
        </colgroup>
        <TableHeader className={cn("sticky top-0 z-20", headerSurface)}>
          <TableRow className="hover:bg-transparent">
            <TableHead
              scope="col"
              rowSpan={pinnedRowSpan}
              className={cn(
                "sticky left-0 z-30 border-b px-2 text-center align-middle",
                headerSurface
              )}
            >
              #
            </TableHead>
            <TableHead
              scope="col"
              rowSpan={pinnedRowSpan}
              className={cn(
                "sticky left-(--rank-width) z-30 border-b px-2 align-middle",
                headerSurface
              )}
            >
              {t("game:scoreboard.columns.team")}
            </TableHead>
            <TableHead
              scope="col"
              rowSpan={pinnedRowSpan}
              className={cn(
                "sticky left-[calc(var(--rank-width)+var(--team-width))] z-30 border-r border-b border-r-border/60 px-3 text-right align-middle dark:border-r-foreground/15 sm:px-5",
                headerSurface
              )}
            >
              {t("game:scoreboard.columns.score")}
            </TableHead>
            <TableHead
              scope="col"
              rowSpan={pinnedRowSpan}
              className="hidden border-b px-2 text-center align-middle sm:table-cell"
            >
              {t("game:scoreboard.columns.solves")}
            </TableHead>
            {categoryGroups.map((group) => {
              const CategoryIcon = group.category.icon;
              return (
                <TableHead
                  key={group.key}
                  scope="colgroup"
                  colSpan={group.challenges.length}
                  className={cn(
                    "h-9 border-b border-l border-l-border/60 px-3 py-1.5 text-center align-middle dark:border-l-foreground/15",
                    "bg-[color-mix(in_oklab,var(--color-category)_16%,var(--card))]",
                    "border-b-[color-mix(in_oklab,var(--color-category)_32%,var(--border))]",
                    "text-[color-mix(in_oklab,var(--color-category)_86%,var(--foreground))]",
                    "dark:bg-[color-mix(in_oklab,var(--color-category)_28%,var(--card))]",
                    "dark:border-b-[color-mix(in_oklab,var(--color-category)_42%,var(--foreground)_15%)]",
                    "dark:text-[color-mix(in_oklab,var(--color-category)_58%,white)]"
                  )}
                  style={
                    {
                      "--color-category": group.category.color,
                    } as CSSProperties
                  }
                >
                  <span className="inline-flex items-center justify-center gap-1.5 text-xs font-semibold">
                    {CategoryIcon && (
                      <CategoryIcon
                        className="size-3.5 shrink-0"
                        aria-hidden="true"
                      />
                    )}
                    {group.category.name?.toUpperCase()}
                  </span>
                </TableHead>
              );
            })}
            <TableHead
              rowSpan={pinnedRowSpan}
              aria-hidden="true"
              className="border-b p-0"
            />
          </TableRow>
          {hasChallenges && (
            <TableRow className="hover:bg-transparent">
              {challenges.map((challenge) => (
                <TableHead
                  scope="col"
                  key={challenge.challenge_id}
                  className={cn(
                    "h-15 border-b px-3 py-2 text-center align-middle whitespace-normal",
                    groupStartIds.has(challenge.challenge_id) &&
                      "border-l border-l-border/60 dark:border-l-foreground/15"
                  )}
                >
                  <span
                    className="line-clamp-2 text-xs font-medium leading-4 text-foreground"
                    title={challenge.challenge_title}
                  >
                    {challenge.challenge_title}
                  </span>
                  <span className="mt-1 block font-mono text-[11px] leading-none font-normal text-muted-foreground tabular-nums">
                    {challenge.pts.toLocaleString()} {t("game:filter.pts")}
                  </span>
                </TableHead>
              ))}
            </TableRow>
          )}
        </TableHeader>
        <TableBody>
          {rows.map(({ team, solves }) => (
            <TableRow
              key={team.id}
              className="h-16 bg-card hover:bg-[color-mix(in_oklab,var(--muted)_35%,var(--card))] data-[state=selected]:bg-[color-mix(in_oklab,var(--info)_6%,var(--card))]"
              data-state={team.id === selfTeamId ? "selected" : undefined}
            >
              <TableCell
                className={cn(
                  stickyCell,
                  "left-0 border-b border-b-border/60 px-2 py-3 text-center"
                )}
              >
                <span
                  className={cn(
                    "inline-flex flex-col items-center gap-0.5 font-mono text-sm tabular-nums",
                    team.rank === 1 && "text-warning"
                  )}
                >
                  {team.rank === 1 && (
                    <TrophyIcon className="size-3" aria-hidden="true" />
                  )}
                  {team.rank > 0 ? team.rank : "—"}
                </span>
              </TableCell>
              <TableCell
                className={cn(
                  stickyCell,
                  "left-(--rank-width) border-b border-b-border/60 py-3 pr-4 pl-2"
                )}
              >
                <div className="flex w-full min-w-0 items-center gap-2.5 text-left">
                  <Avatar
                    className="hidden size-8 shrink-0 sm:flex"
                    src={
                      team.avatar_hash
                        ? `/api/media?hash=${team.avatar_hash}`
                        : undefined
                    }
                    fallback={team.name.charAt(0)}
                  />
                  <span className="min-w-0 flex-1">
                    <span
                      className="block truncate text-sm font-semibold"
                      title={team.name}
                    >
                      {team.name}
                    </span>
                    {team.slogan && (
                      <span
                        className="mt-0.5 block truncate text-[11px] text-muted-foreground"
                        title={team.slogan}
                      >
                        {team.slogan}
                      </span>
                    )}
                  </span>
                </div>
              </TableCell>
              <TableCell
                className={cn(
                  stickyCell,
                  "left-[calc(var(--rank-width)+var(--team-width))] border-r border-b border-r-border/60 border-b-border/60 px-3 py-3 text-right sm:px-5 font-mono font-semibold tabular-nums"
                )}
              >
                {team.pts.toLocaleString()}
              </TableCell>
              <TableCell className="hidden border-b border-b-border/60 px-3 py-3 text-center font-mono text-muted-foreground tabular-nums sm:table-cell">
                {solves.size}
              </TableCell>
              {challenges.map((challenge) => (
                <TableCell
                  key={challenge.challenge_id}
                  className={cn(
                    "border-b border-b-border/60 px-4 py-2 text-center",
                    groupStartIds.has(challenge.challenge_id) &&
                      "border-l border-l-border/40"
                  )}
                >
                  <SolveCell
                    title={challenge.challenge_title}
                    submissions={solves.get(challenge.challenge_id) ?? []}
                  />
                </TableCell>
              ))}
              <TableCell
                aria-hidden="true"
                className="border-b border-b-border/60 p-0"
              />
            </TableRow>
          ))}
        </TableBody>
      </Table>
    </ScrollArea>
  );
}
