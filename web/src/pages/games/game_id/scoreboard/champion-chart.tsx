import { Card } from "@/components/ui/card";
import {
    ChartConfig,
    ChartContainer,
    ChartTooltip,
    ChartTooltipContent,
} from "@/components/ui/chart";
import { ScoreRecord } from "@/models/game";
import { cn } from "@/utils";
import { useEffect, useMemo, useState } from "react";
import {
    CartesianGrid,
    XAxis,
    LineChart,
    Line,
    Legend,
    Brush,
    YAxis,
} from "recharts";

const COLORS = [
    "#ff4d4f", // 红
    "#1890ff", // 蓝
    "#52c41a", // 绿
    "#faad14", // 金黄
    "#722ed1", // 紫
    "#13c2c2", // 青
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

        // 收集所有 submission
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

        // 时间排序
        allSubmissions.sort((a, b) => a.ts - b.ts);

        const cumulativeScores: Record<number, number> = {};
        const result: Array<{ ts: number; [key: number]: number }> = [];

        allSubmissions.forEach(({ ts, teamId, pts }) => {
            cumulativeScores[teamId] = (cumulativeScores[teamId] || 0) + pts;

            result.push({
                ts,
                [teamId]: cumulativeScores[teamId], // 只更新当前队伍的累计值
            });
        });

        return result;
    }, [scoreboard]);

    const [lines, setLines] = useState<
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
                id: team?.id!,
                name: team?.name,
            });
        });

        setLines(result);
    }, [scoreboard]);

    const chartConfig = {} satisfies ChartConfig;

    return (
        <Card className={cn(["p-10", "w-full"])}>
            <ChartContainer
                config={chartConfig}
                className={cn(["h-[20rem]", "w-full"])}
            >
                <LineChart
                    accessibilityLayer
                    data={data}
                    margin={{
                        left: 12,
                        right: 12,
                    }}
                >
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
                    <Legend />
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
                    />
                    <ChartTooltip
                        cursor={false}
                        content={
                            <ChartTooltipContent indicator="dot" hideLabel />
                        }
                    />

                    {lines?.map((line, index) => (
                        <Line
                            key={line?.id}
                            type="stepAfter"
                            dataKey={line.id}
                            name={line.name}
                            stroke={COLORS[index % COLORS.length]}
                            connectNulls
                        />
                    ))}
                </LineChart>
            </ChartContainer>
        </Card>
    );
}

export { ChampionChart };
