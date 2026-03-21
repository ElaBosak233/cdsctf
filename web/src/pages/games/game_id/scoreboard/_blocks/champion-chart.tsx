import { useEffect, useMemo, useState } from "react";
import {
  Brush,
  CartesianGrid,
  Legend,
  Line,
  LineChart,
  XAxis,
  YAxis,
} from "recharts";

import {
  type ChartConfig,
  ChartContainer,
  ChartTooltip,
  ChartTooltipContent,
} from "@/components/ui/chart";
import type { ScoreRecord } from "@/models/game";
import { cn } from "@/utils";

const COLORS = [
  "#ff4d4f", // red
  "#1890ff", // blue
  "#52c41a", // green
  "#faad14", // gold
  "#722ed1", // purple
  "#13c2c2", // cyan
];

interface ChampionChartProps {
  scoreboard?: Array<ScoreRecord>;
}

function ChampionChart(props: ChampionChartProps) {
  const { scoreboard } = props;

  const data = useMemo(() => {
    if (!scoreboard) return [];

    const allSubmissions: Array<{
      ts: number;
      teamId: number;
      pts: number;
    }> = [];

    // Flatten all submissions from the scoreboard payload.
    scoreboard.forEach((record) => {
      const team = record?.team;
      const submissions = record?.submissions;
      if (!team) return;

      submissions?.forEach((submission) => {
        allSubmissions.push({
          ts: Number(submission?.created_at),
          teamId: team.id!,
          pts: Number(submission?.pts),
        });
      });
    });

    // Sort by event time for a proper timeline.
    allSubmissions.sort((a, b) => a.ts - b.ts);

    const cumulativeScores: Record<number, number> = {};
    const result: Array<{ ts: number; [key: number]: number }> = [];

    allSubmissions.forEach(({ ts, teamId, pts }) => {
      cumulativeScores[teamId] = (cumulativeScores[teamId] || 0) + pts;

      result.push({
        ts,
        [teamId]: cumulativeScores[teamId], // cumulative score for this team at this timestamp
      });
    });

    return result;
  }, [scoreboard]);

  const [lines, setLines] =
    useState<
      Array<{
        id: number;
        name?: string;
      }>
    >();

  useEffect(() => {
    const result: Array<{
      id: number;
      name?: string;
    }> = [];

    scoreboard?.forEach((record) => {
      const team = record?.team;

      if (!team) return;

      result.push({
        id: team.id!,
        name: team.name,
      });
    });

    setLines(result);
  }, [scoreboard]);

  const chartConfig = {} satisfies ChartConfig;

  return (
    <ChartContainer config={chartConfig} className={cn(["h-100", "w-full"])}>
      <LineChart accessibilityLayer data={data}>
        <CartesianGrid vertical={false} />
        <XAxis
          dataKey={"ts"}
          tickFormatter={(value: number) =>
            new Date(value * 1000).toLocaleString(undefined, {
              month: "2-digit",
              day: "2-digit",
              hour: "2-digit",
              minute: "2-digit",
              second: "2-digit",
            })
          }
          scale={"auto"}
        />
        <YAxis />
        <Legend
          verticalAlign={"top"}
          align={"center"}
          wrapperStyle={{ marginTop: -15 }}
        />
        <Brush
          dataKey={"ts"}
          height={25}
          tickFormatter={(value: number) =>
            new Date(value * 1000).toLocaleString(undefined, {
              month: "2-digit",
              day: "2-digit",
              hour: "2-digit",
              minute: "2-digit",
              second: "2-digit",
            })
          }
          fill={"none"}
        />
        <ChartTooltip
          cursor={false}
          content={<ChartTooltipContent indicator="dot" hideLabel />}
        />

        {lines?.map((line, index) => (
          <Line
            key={line?.id}
            type="stepAfter"
            dataKey={line.id}
            name={line.name}
            stroke={COLORS[index % COLORS.length]}
            connectNulls
            dot={false}
          />
        ))}
      </LineChart>
    </ChartContainer>
  );
}

export { ChampionChart };
