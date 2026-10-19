import { Maximize2Icon, XIcon } from "lucide-react";
import { useMemo, useState } from "react";
import { useTranslation } from "react-i18next";
import {
  Brush,
  CartesianGrid,
  Line,
  LineChart,
  Tooltip,
  XAxis,
  YAxis,
} from "recharts";
import { Button } from "@/components/ui/button";
import { type ChartConfig, ChartContainer } from "@/components/ui/chart";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTrigger,
} from "@/components/ui/dialog";
import { useTickerTime } from "@/hooks/use-ticker-time";
import type { ScoreboardEntry } from "@/models/game";
import { useGameStore } from "@/storages/game";
import { cn } from "@/utils";
import { toEpochMilliseconds } from "@/utils/time";

export const CHART_TEAM_LIMIT = 10;

const COLORS = [
  "var(--color-info)",
  "var(--color-success)",
  "var(--color-chart-4)",
  "var(--color-error)",
  "var(--color-chart-3)",
];
type ChartPoint = { ts: number; [key: string]: number };

function ScoreTooltip({
  active,
  payload,
  label,
}: {
  active?: boolean;
  payload?: Array<{
    dataKey?: string;
    name?: string;
    value?: number;
    color?: string;
  }>;
  label?: number;
}) {
  if (!active || !payload?.length) return null;
  return (
    <div className="min-w-44 rounded-lg border bg-popover px-3 py-2.5 text-xs shadow-md">
      <p className="mb-2 font-mono text-[11px] text-muted-foreground">
        {typeof label === "number" ? new Date(label).toLocaleString() : ""}
      </p>
      <div className="grid gap-1.5">
        {payload
          .filter((item) => typeof item.value === "number")
          .sort((a, b) => (b.value ?? 0) - (a.value ?? 0))
          .map((item) => (
            <div
              className="flex items-center justify-between gap-5"
              key={item.dataKey}
            >
              <span className="flex min-w-0 items-center gap-2">
                <span
                  className="size-2 shrink-0 rounded-full"
                  style={{ backgroundColor: item.color }}
                />
                <span className="max-w-44 truncate">{item.name}</span>
              </span>
              <span className="font-mono font-semibold tabular-nums">
                {item.value?.toLocaleString()}
              </span>
            </div>
          ))}
      </div>
    </div>
  );
}

function ChampionChart({ scoreboard }: { scoreboard?: ScoreboardEntry[] }) {
  const { t } = useTranslation();
  const { currentGame } = useGameStore();
  const now = useTickerTime();
  const nowMs = Math.floor(now.getTime() / 60000) * 60000;
  const [expanded, setExpanded] = useState(false);
  const [hidden, setHidden] = useState<Set<string>>(new Set());
  const [focused, setFocused] = useState<string>();
  const { lines, data, config } = useMemo(() => {
    const teams = (scoreboard ?? [])
      .filter((record) => record.team)
      .slice(0, CHART_TEAM_LIMIT);
    const events = teams
      .flatMap((record) =>
        record.submissions.map((submission) => ({
          ts: toEpochMilliseconds(submission.created_at) ?? Number.NaN,
          teamId: record.team.id,
          pts: submission.pts,
        }))
      )
      .filter((event) => Number.isFinite(event.ts))
      .sort((a, b) => a.ts - b.ts);
    const firstEvent = events[0]?.ts;
    const lastEvent = events[events.length - 1]?.ts;
    const started =
      toEpochMilliseconds(currentGame?.started_at) ?? firstEvent ?? nowMs;
    const ended = toEpochMilliseconds(currentGame?.ended_at) ?? nowMs;
    const start = Math.min(started, firstEvent ?? started);
    const end = Math.max(start + 1, lastEvent ?? start, Math.min(nowMs, ended));
    const totals = new Map<number, number>();
    const points: ChartPoint[] = [];
    const pointAt = (ts: number): ChartPoint =>
      Object.fromEntries([
        ["ts", ts],
        ...teams.map(({ team }) => [
          `team-${team.id}`,
          totals.get(team.id) ?? 0,
        ]),
      ]);
    // Seed the zero-score baseline, then process simultaneous solves together.
    if (firstEvent === undefined || start < firstEvent)
      points.push(pointAt(start));
    for (let index = 0; index < events.length; ) {
      const ts = events[index].ts;
      while (index < events.length && events[index].ts === ts) {
        const event = events[index++];
        totals.set(event.teamId, (totals.get(event.teamId) ?? 0) + event.pts);
      }
      points.push(pointAt(ts));
    }
    if (points[points.length - 1]?.ts !== end) points.push(pointAt(end));
    const chartLines = teams.map(({ team }, index) => ({
      id: `team-${team.id}`,
      name: team.name,
      rank: team.rank,
      pts: team.pts,
      color: COLORS[index % COLORS.length],
      dash: index >= COLORS.length ? "6 3" : undefined,
    }));
    return {
      lines: chartLines,
      data: points,
      config: Object.fromEntries(
        chartLines.map((line) => [
          line.id,
          { label: line.name, color: line.color },
        ])
      ) satisfies ChartConfig,
    };
  }, [scoreboard, currentGame?.started_at, currentGame?.ended_at, nowMs]);
  if (!lines.length) return null;
  const spansDays =
    new Date(data[0].ts).toDateString() !==
    new Date(data[data.length - 1].ts).toDateString();
  const formatTime = (value: number) =>
    new Date(value).toLocaleString(undefined, {
      ...(spansDays ? ({ month: "2-digit", day: "2-digit" } as const) : {}),
      hour: "2-digit",
      minute: "2-digit",
    });
  const visible = lines.filter((line) => !hidden.has(line.id));
  const legend = () => (
    <div className="grid grid-cols-2 gap-2 md:grid-cols-3 xl:grid-cols-5">
      {lines.map((line) => {
        const isHidden = hidden.has(line.id);
        return (
          <button
            type="button"
            key={line.id}
            aria-pressed={!isHidden}
            aria-label={t("game:scoreboard.chart.toggle_team", {
              name: line.name,
            })}
            disabled={!isHidden && visible.length === 1}
            onClick={() =>
              setHidden((previous) => {
                const next = new Set(previous);
                if (next.has(line.id)) next.delete(line.id);
                else next.add(line.id);
                return next;
              })
            }
            onMouseEnter={() => setFocused(line.id)}
            onMouseLeave={() => setFocused(undefined)}
            onFocus={() => setFocused(line.id)}
            onBlur={() => setFocused(undefined)}
            className={cn(
              "flex min-w-0 cursor-pointer items-center gap-2 rounded-md px-2 py-2 text-left text-xs hover:bg-muted/60 focus-visible:outline-2 focus-visible:outline-ring",
              isHidden && "opacity-40"
            )}
          >
            <svg
              className="h-2 w-6 shrink-0"
              viewBox="0 0 24 8"
              aria-hidden="true"
            >
              <line
                x1="0"
                y1="4"
                x2="24"
                y2="4"
                stroke={line.color}
                strokeWidth="2"
                strokeDasharray={line.dash}
              />
            </svg>
            <span className="font-mono text-[11px] text-muted-foreground">
              {line.rank > 0 ? `#${line.rank}` : "—"}
            </span>
            <span className="min-w-0 flex-1 truncate" title={line.name}>
              {line.name}
            </span>
            <span className="font-mono font-medium tabular-nums">
              {line.pts.toLocaleString()}
            </span>
          </button>
        );
      })}
    </div>
  );
  const chart = (large: boolean) => (
    <ChartContainer
      config={config}
      className={cn(
        "aspect-auto w-full",
        large ? "min-h-0 flex-1" : "h-56 sm:h-64"
      )}
    >
      <LineChart
        accessibilityLayer
        data={data}
        margin={{ top: 12, right: 16, bottom: 0, left: 0 }}
      >
        <CartesianGrid
          vertical={false}
          strokeDasharray="3 3"
          className="stroke-border/60"
        />
        <XAxis
          dataKey="ts"
          type="number"
          domain={["dataMin", "dataMax"]}
          tickFormatter={formatTime}
          tickLine={false}
          axisLine={false}
          minTickGap={spansDays ? 60 : 36}
          tickMargin={12}
          height={40}
        />
        <YAxis
          tickLine={false}
          axisLine={false}
          width={48}
          tickMargin={8}
          allowDecimals={false}
          tickFormatter={(value: number) => value.toLocaleString()}
        />
        <Tooltip
          cursor={{ stroke: "var(--border)", strokeDasharray: "4 4" }}
          content={<ScoreTooltip />}
        />
        {lines.map((line) => (
          <Line
            key={line.id}
            type="stepAfter"
            dataKey={line.id}
            name={line.name}
            stroke={line.color}
            strokeWidth={2}
            strokeDasharray={line.dash}
            strokeOpacity={
              focused &&
              visible.some((item) => item.id === focused) &&
              focused !== line.id
                ? 0.2
                : 1
            }
            hide={hidden.has(line.id)}
            dot={false}
            activeDot={{ r: 4, strokeWidth: 2, fill: "var(--background)" }}
            isAnimationActive={false}
          />
        ))}
        {large && data.length > 2 && (
          <Brush
            ariaLabel={t("game:scoreboard.chart.time_range")}
            dataKey="ts"
            height={24}
            travellerWidth={8}
            tickFormatter={formatTime}
            stroke="var(--border)"
            fill="var(--muted)"
          />
        )}
      </LineChart>
    </ChartContainer>
  );
  return (
    <section
      className="min-w-0 rounded-lg border bg-card p-4 sm:p-5"
      aria-labelledby="progression-heading"
    >
      <div className="mb-4 flex flex-wrap items-center justify-between gap-3">
        <div className="flex items-baseline gap-3">
          <h2 id="progression-heading" className="text-base font-semibold">
            {t("game:scoreboard.chart.progression")}
          </h2>
          <span className="text-xs text-muted-foreground">
            {t("game:scoreboard.chart.top_teams", { count: lines.length })}
          </span>
        </div>
        <Dialog open={expanded} onOpenChange={setExpanded}>
          <DialogTrigger
            render={
              <Button size="sm" variant="ghost" icon={<Maximize2Icon />} />
            }
          >
            {t("game:scoreboard.chart.expand")}
          </DialogTrigger>
          <DialogContent
            className="flex h-[calc(100dvh-2rem)] max-w-none flex-col gap-4 overflow-hidden rounded-elevated border bg-card p-4 shadow-lg sm:p-6 motion-reduce:animate-none"
            slotProps={{
              title: { children: t("game:scoreboard.chart.progression") },
            }}
          >
            <DialogHeader
              title={t("game:scoreboard.chart.progression")}
              description={currentGame?.title}
              trailing={
                <Button
                  size="sm"
                  square
                  icon={<XIcon />}
                  aria-label={t("game:scoreboard.chart.close")}
                  title={t("game:scoreboard.chart.close")}
                  onClick={() => setExpanded(false)}
                />
              }
            />
            {expanded && chart(true)}
            <div className="max-h-[40dvh] shrink-0 overflow-y-auto border-t pt-3">
              {legend()}
            </div>
          </DialogContent>
        </Dialog>
      </div>
      {chart(false)}
      <div className="mt-3 border-t pt-2">{legend()}</div>
    </section>
  );
}

export { ChampionChart };
