import { useMantineColorScheme, useMantineTheme } from "@mantine/core";
import ReactEcharts from "echarts-for-react";

interface CurveProps {
    maxPts: number;
    difficulty: number;
    minPts: number;
    sovledTimes: number;
}

export default function Curve({
    maxPts,
    difficulty,
    minPts,
    sovledTimes,
}: CurveProps) {
    const toX = (x: number) => (x * 6 * difficulty) / 100;
    const func = (x: number) => {
        const ratio: number = minPts / maxPts;
        const result: number = Math.floor(
            maxPts * (ratio + (1 - ratio) * Math.exp((1 - x) / difficulty))
        );
        return Math.min(result, maxPts);
    };

    const curScore = func(sovledTimes);
    const showCount =
        sovledTimes > 5.8 * difficulty ? 5.8 * difficulty : sovledTimes;
    const theme = useMantineTheme();
    const plotData = [...Array(100).keys()].map((x) => [toX(x), func(toX(x))]);
    const { colorScheme } = useMantineColorScheme();

    return (
        <ReactEcharts
            theme={colorScheme}
            option={{
                animation: false,
                backgroundColor: "transparent",
                grid: {
                    top: 30,
                    left: 40,
                    right: 70,
                    bottom: 20,
                    backgroundColor: "transparent",
                },
                xAxis: {
                    name: "解决次数",
                },
                yAxis: {
                    name: "分数",
                    min: 0,
                    max: Math.ceil((maxPts * 1.2) / 100) * 100,
                },
                series: [
                    {
                        type: "line",
                        showSymbol: false,
                        clip: true,
                        color: theme.colors[theme.primaryColor][8],
                        data: plotData,
                        markPoint: {
                            label: {
                                show: true,
                                fontSize: 10,
                                formatter: "{c}",
                            },
                            symbol: "pin",
                            symbolSize: 40,
                            symbolOffset: [0, 0],
                            data: [
                                {
                                    value: curScore,
                                    xAxis: showCount,
                                    yAxis: curScore,
                                },
                            ],
                        },
                        markLine: {
                            symbol: "none",
                            data: [
                                {
                                    yAxis: Math.floor(minPts),
                                    label: {
                                        position: "end",
                                        formatter: "最低分 {c}",
                                    },
                                },
                            ],
                        },
                    },
                ],
            }}
            opts={{
                renderer: "svg",
            }}
            style={{
                width: "45%",
                height: "150%",
                display: "flex",
            }}
        />
    );
}
