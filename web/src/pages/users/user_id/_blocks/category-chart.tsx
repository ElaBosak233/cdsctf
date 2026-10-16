import { Pie, PieChart } from "recharts";
import { Card } from "@/components/ui/card";
import {
  type ChartConfig,
  ChartContainer,
  ChartTooltip,
  ChartTooltipContent,
} from "@/components/ui/chart";
import { LoadingOverlay } from "@/components/ui/loading-overlay";
import { getCategory } from "@/utils/category";

interface CategoryChartProps {
  solved?: number;
  solvedByCategory?: Array<{
    category: number;
    solved: number;
  }>;
  title: string;
  description: string;
  emptyLabel: string;
  totalLabel: string;
  loading?: boolean;
  errorLabel?: string;
}

export function CategoryChart({
  solved = 0,
  solvedByCategory = [],
  title,
  description,
  emptyLabel,
  totalLabel,
  loading = false,
  errorLabel,
}: CategoryChartProps) {
  const data = solvedByCategory.map((item) => {
    const category = getCategory(item.category);
    return {
      key: String(item.category),
      label: category.name ?? String(item.category),
      value: item.solved,
      fill: category.color,
    };
  });

  const chartConfig = Object.fromEntries(
    data.map((item) => [item.key, { label: item.label, color: item.fill }])
  ) satisfies ChartConfig;

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
      ) : data.length ? (
        <div className="grid items-center gap-5 sm:grid-cols-[minmax(0,1fr)_9rem]">
          <div className="relative min-w-0">
            <ChartContainer
              config={chartConfig}
              className="mx-auto h-56 w-full"
            >
              <PieChart accessibilityLayer>
                <ChartTooltip
                  cursor={false}
                  content={<ChartTooltipContent hideLabel />}
                />
                <Pie
                  data={data}
                  dataKey="value"
                  nameKey="key"
                  innerRadius={62}
                  outerRadius={88}
                  strokeWidth={2}
                  stroke="var(--color-card)"
                  isAnimationActive={false}
                />
              </PieChart>
            </ChartContainer>
            <div className="pointer-events-none absolute inset-0 flex flex-col items-center justify-center">
              <span className="font-mono text-2xl font-semibold tabular-nums">
                {solved.toLocaleString()}
              </span>
              <span className="text-xs text-muted-foreground">
                {totalLabel}
              </span>
            </div>
          </div>
          <div className="grid gap-3">
            {data.map((item) => (
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
      ) : (
        <div className="flex min-h-56 items-center justify-center text-sm text-muted-foreground">
          {emptyLabel}
        </div>
      )}
    </Card>
  );
}
