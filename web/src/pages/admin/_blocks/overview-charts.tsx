import { Pie, PieChart } from "recharts";
import { Card } from "@/components/ui/card";
import {
  type ChartConfig,
  ChartContainer,
  ChartTooltip,
  ChartTooltipContent,
} from "@/components/ui/chart";
import { cn } from "@/utils";

type PieDatum = {
  key: string;
  label: string;
  value: number;
};

interface OverviewPieChartProps {
  title: string;
  description: string;
  total: number;
  totalLabel: string;
  data: Array<PieDatum>;
  colors: Array<string>;
}

function OverviewPieChart({
  title,
  description,
  total,
  totalLabel,
  data,
  colors,
}: OverviewPieChartProps) {
  const chartData = data.map((item, index) => ({
    ...item,
    fill: colors[index % colors.length],
  }));

  const chartConfig = Object.fromEntries(
    data.map((item, index) => [
      item.key,
      {
        label: item.label,
        color: colors[index % colors.length],
      },
    ])
  ) satisfies ChartConfig;

  return (
    <Card className="min-w-0 p-5">
      <div className="mb-2">
        <h3 className="font-medium">{title}</h3>
        <p className="text-sm text-muted-foreground">{description}</p>
      </div>
      <div className="grid items-center gap-5 sm:grid-cols-[minmax(0,1fr)_9rem]">
        <div className="relative min-w-0">
          <ChartContainer config={chartConfig} className="mx-auto h-52 w-full">
            <PieChart accessibilityLayer>
              <ChartTooltip
                cursor={false}
                content={<ChartTooltipContent hideLabel />}
              />
              <Pie
                data={chartData}
                dataKey="value"
                nameKey="key"
                innerRadius={58}
                outerRadius={82}
                strokeWidth={2}
                stroke="var(--color-card)"
              />
            </PieChart>
          </ChartContainer>
          <div className="pointer-events-none absolute inset-0 flex flex-col items-center justify-center">
            <span className="font-mono text-2xl font-semibold tabular-nums">
              {total.toLocaleString()}
            </span>
            <span className="text-xs text-muted-foreground">{totalLabel}</span>
          </div>
        </div>
        <div className="grid gap-3">
          {chartData.map((item) => (
            <div
              key={item.key}
              className="flex items-center justify-between gap-3 text-sm"
            >
              <div className="flex min-w-0 items-center gap-2">
                <span
                  className="size-2.5 shrink-0 rounded-full"
                  style={{ backgroundColor: item.fill }}
                />
                <span className="truncate text-muted-foreground">
                  {item.label}
                </span>
              </div>
              <span className="font-mono font-medium tabular-nums">
                {item.value.toLocaleString()}
              </span>
            </div>
          ))}
        </div>
      </div>
    </Card>
  );
}

interface OverviewChartsProps {
  submissionsTotal?: number;
  submissionsSolved?: number;
  challengesTotal?: number;
  challengesInGame?: number;
  challengesStandalone?: number;
  labels: {
    submissions: string;
    submissionsDescription: string;
    total: string;
    solved: string;
    otherSubmissions: string;
    challenges: string;
    challengesDescription: string;
    inGame: string;
    standalone: string;
  };
}

export function OverviewCharts({
  submissionsTotal = 0,
  submissionsSolved = 0,
  challengesTotal = 0,
  challengesInGame = 0,
  challengesStandalone,
  labels,
}: OverviewChartsProps) {
  const solved = Math.min(Math.max(submissionsSolved, 0), submissionsTotal);
  const inGame = Math.min(Math.max(challengesInGame, 0), challengesTotal);
  const standalone = Math.min(
    Math.max(challengesStandalone ?? challengesTotal - inGame, 0),
    challengesTotal
  );

  return (
    <div className={cn("grid", "gap-5", "lg:grid-cols-2")}>
      <OverviewPieChart
        title={labels.submissions}
        description={labels.submissionsDescription}
        total={submissionsTotal}
        totalLabel={labels.total}
        data={[
          { key: "solved", label: labels.solved, value: solved },
          {
            key: "other",
            label: labels.otherSubmissions,
            value: submissionsTotal - solved,
          },
        ]}
        colors={["var(--color-chart-2)", "var(--color-chart-1)"]}
      />
      <OverviewPieChart
        title={labels.challenges}
        description={labels.challengesDescription}
        total={challengesTotal}
        totalLabel={labels.total}
        data={[
          { key: "in-game", label: labels.inGame, value: inGame },
          {
            key: "standalone",
            label: labels.standalone,
            value: standalone,
          },
        ]}
        colors={["var(--color-chart-3)", "var(--color-chart-4)"]}
      />
    </div>
  );
}
