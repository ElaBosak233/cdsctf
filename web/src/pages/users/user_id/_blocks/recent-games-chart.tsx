import {
  Bar,
  BarChart,
  CartesianGrid,
  LabelList,
  XAxis,
  YAxis,
} from "recharts";
import { Card } from "@/components/ui/card";
import {
  type ChartConfig,
  ChartContainer,
  ChartLegend,
  ChartLegendContent,
  ChartTooltip,
  ChartTooltipContent,
} from "@/components/ui/chart";
import { LoadingOverlay } from "@/components/ui/loading-overlay";

interface RecentGamesChartProps {
  games?: Array<{
    game_id: number;
    title: string;
    solved: number;
    total: number;
  }>;
  title: string;
  description: string;
  emptyLabel: string;
  solvedLabel: string;
  remainingLabel: string;
  loading?: boolean;
  errorLabel?: string;
}

function truncateTitle(title: string) {
  return title.length > 18 ? `${title.slice(0, 18)}...` : title;
}

export function RecentGamesChart({
  games = [],
  title,
  description,
  emptyLabel,
  solvedLabel,
  remainingLabel,
  loading = false,
  errorLabel,
}: RecentGamesChartProps) {
  const chartConfig = {
    solved: { label: solvedLabel, color: "var(--color-chart-2)" },
    remaining: { label: remainingLabel, color: "var(--color-muted)" },
  } satisfies ChartConfig;
  const chartData = games.map((game) => ({
    ...game,
    remaining: Math.max(game.total - game.solved, 0),
    progress: `${game.solved}/${game.total}`,
  }));

  return (
    <Card className="min-w-0 p-5 sm:p-6">
      <div className="mb-2">
        <h2 className="font-medium">{title}</h2>
        <p className="text-sm text-muted-foreground">{description}</p>
      </div>
      {loading || errorLabel ? (
        <div
          className="relative flex min-h-56 items-center justify-center text-sm text-muted-foreground"
          aria-busy={loading}
        >
          <LoadingOverlay loading={loading} />
          {errorLabel ? <p role="alert">{errorLabel}</p> : null}
        </div>
      ) : games.length ? (
        <ChartContainer config={chartConfig} className="h-56 w-full">
          <BarChart
            accessibilityLayer
            data={chartData}
            layout="vertical"
            margin={{ left: 0, right: 40, top: 8, bottom: 8 }}
            barCategoryGap="18%"
          >
            <CartesianGrid horizontal={false} strokeDasharray="3 3" />
            <XAxis
              type="number"
              allowDecimals={false}
              axisLine={false}
              tickLine={false}
            />
            <YAxis
              type="category"
              dataKey="title"
              axisLine={false}
              tickLine={false}
              width={120}
              tickFormatter={truncateTitle}
            />
            <ChartTooltip
              cursor={false}
              content={<ChartTooltipContent indicator="line" />}
            />
            <ChartLegend content={<ChartLegendContent />} />
            <Bar
              dataKey="solved"
              stackId="total"
              fill="var(--color-solved)"
              radius={[3, 0, 0, 3]}
              isAnimationActive={false}
            />
            <Bar
              dataKey="remaining"
              stackId="total"
              fill="var(--color-remaining)"
              radius={[0, 3, 3, 0]}
              isAnimationActive={false}
            >
              <LabelList
                dataKey="progress"
                position="right"
                className="fill-muted-foreground"
              />
            </Bar>
          </BarChart>
        </ChartContainer>
      ) : (
        <div className="flex min-h-56 items-center justify-center text-sm text-muted-foreground">
          {emptyLabel}
        </div>
      )}
    </Card>
  );
}
