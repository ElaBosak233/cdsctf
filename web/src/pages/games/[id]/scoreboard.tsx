import { useGameApi } from "@/api/game";
import { Game } from "@/types/game";
import { GameSubmission, Status } from "@/types/submission";
import {
    Flex,
    Stack,
    useMantineColorScheme,
    useMantineTheme,
    Text,
    Group,
    LoadingOverlay,
    Table,
    Avatar,
    Tooltip,
    ThemeIcon,
    alpha,
} from "@mantine/core";
import { useEffect, useState } from "react";
import { useParams } from "react-router-dom";
import ReactECharts from "echarts-for-react";
import withGame from "@/components/layouts/withGame";
import { GameChallenge } from "@/types/game_challenge";
import { Category } from "@/types/category";
import { Challenge } from "@/types/challenge";
import MDIcon from "@/components/ui/MDIcon";
import React from "react";
import FirstBloodIcon from "@/components/icons/hexagons/FirstBloodIcon";
import SecondBloodIcon from "@/components/icons/hexagons/SecondBloodIcon";
import ThirdBloodIcon from "@/components/icons/hexagons/ThirdBloodIcon";
import { Row, calculateAndSort } from "@/utils/game";
import { useCategoryStore } from "@/stores/category";

interface ScoreSeries {
    name: string;
    data: [number, number][];
    type: string;
    step: string;
}

function Page() {
    const { id } = useParams<{ id: string }>();
    const { colorScheme } = useMantineColorScheme();
    const theme = useMantineTheme();

    const gameApi = useGameApi();

    const categoryStore = useCategoryStore();

    const [game, setGame] = useState<Game>();
    const [submissions, setSubmissions] = useState<Array<GameSubmission>>([]);
    const [gameChallenges, setGameChallenges] = useState<Array<GameChallenge>>(
        []
    );
    const [categoriedChallenges, setCategoriedChallenges] = useState<
        Record<number, { category: Category; challenges: Array<Challenge> }>
    >({});
    const [rows, setRows] = useState<Array<Row> | undefined>([]);

    const [series, setSeries] = useState<Array<ScoreSeries>>([]);

    const [loading, setLoading] = useState(true);

    function getGame() {
        gameApi
            .getGames({
                id: Number(id),
            })
            .then((res) => {
                const r = res.data;
                setGame(r.data[0]);
            });
    }

    function getSubmissions() {
        setLoading(true);
        gameApi
            .getGameSubmissions({
                id: Number(id),
                status: Status.Correct,
            })
            .then((res) => {
                const r = res.data;
                setSubmissions(r.data);
            })
            .finally(() => {
                setLoading(false);
            });
    }

    function getGameChallenges() {
        gameApi
            .getGameChallenges({
                game_id: Number(id),
                is_enabled: true,
            })
            .then((res) => {
                const r = res.data;
                setGameChallenges(r.data);
            });
    }

    // 用于表头
    useEffect(() => {
        if (!gameChallenges) return;

        const categoriedChallenges: Record<
            number,
            { category: Category; challenges: Array<Challenge> }
        > = {};

        gameChallenges?.forEach((gameChallenge) => {
            const category_id = gameChallenge?.challenge?.category_id;
            if (!category_id) return;

            if (!categoriedChallenges[category_id]) {
                categoriedChallenges[category_id] = {
                    category: categoryStore.getCategory(
                        Number(gameChallenge?.challenge?.category_id)
                    ) as Category,
                    challenges: [],
                };
            }

            categoriedChallenges[category_id].challenges.push(
                gameChallenge?.challenge!
            );
        });
        setCategoriedChallenges(categoriedChallenges);
    }, [gameChallenges]);

    // 用于表格
    useEffect(() => {
        setRows(calculateAndSort(submissions));
    }, [submissions]);

    // 用于折线图
    useEffect(() => {
        if (submissions) {
            const teamScores: Record<number, [number, number][]> = {};
            const teamMaps: Record<number, string> = {};

            // 遍历数据，按照团队和时间戳进行分组
            for (const submission of submissions) {
                const { team, team_id, created_at, pts } = submission;
                if (!teamScores[Number(team_id)]) {
                    teamMaps[Number(team_id)] = `${team?.name}`;
                    teamScores[Number(team_id)] = [];
                }
                teamScores[Number(team_id)].push([
                    Number(created_at),
                    Number(pts),
                ]);
            }

            // 对每个团队的得分进行汇总
            for (const team_id in teamScores) {
                teamScores[team_id].sort((a, b) => a[0] - b[0]); // 按照时间戳排序
                let total = 0;
                for (const score of teamScores[team_id]) {
                    total += score[1]; // 累加得分
                    score[1] = total; // 更新为累计得分
                }
            }

            // 将字典转换为数组并按总分排序
            const teamsArray: Array<ScoreSeries> = Object.keys(teamScores).map(
                (team_id) => {
                    return {
                        name: teamMaps[Number(team_id)],
                        data: teamScores[Number(team_id)],
                        type: "line",
                        step: "end",
                    };
                }
            );

            // 按照总分排序并获取前十名
            teamsArray.sort((a, b) => {
                return (
                    b.data[b.data.length - 1][1] - a.data[a.data.length - 1][1]
                );
            });
            setSeries(teamsArray.slice(0, 10));
        }
    }, [submissions]);

    useEffect(() => {
        getGame();
    }, []);

    useEffect(() => {
        getSubmissions();
        getGameChallenges();
    }, [game]);

    useEffect(() => {
        document.title = `积分榜 - ${game?.title}`;
    }, [game]);

    return (
        <Stack m={36} gap={20} pos={"relative"}>
            <LoadingOverlay visible={loading} />
            <ReactECharts
                theme={colorScheme}
                option={{
                    backgroundColor: "transparent",
                    toolbox: {
                        show: true,
                        feature: {
                            dataZoom: {},
                            saveAsImage: {},
                        },
                    },
                    xAxis: {
                        name: "时间",
                        type: "time",
                        splitLine: {
                            show: true,
                        },
                    },
                    yAxis: {
                        name: "分数",
                        type: "value",
                    },
                    series: series,
                    grid: {
                        x: 70,
                        y: 50,
                        y2: 120,
                        x2: 90,
                    },
                    legend: {
                        orient: "horizontal",
                        bottom: 60,
                        textStyle: {
                            fontSize: 12,
                            color:
                                colorScheme === "dark"
                                    ? theme.colors.light[1]
                                    : theme.colors.dark[5],
                        },
                    },
                    tooltip: {
                        trigger: "axis",
                        borderWidth: 0,
                        textStyle: {
                            fontSize: 10,
                            color:
                                colorScheme === "dark"
                                    ? theme.colors.light[1]
                                    : theme.colors.dark[5],
                        },
                        backgroundColor:
                            colorScheme === "dark"
                                ? theme.colors.gray[6]
                                : theme.colors.light[1],
                    },
                    dataZoom: [
                        {
                            type: "inside",
                            start: game?.started_at,
                            end: game?.ended_at,
                            xAxisIndex: 0,
                            filterMode: "none",
                        },
                        {
                            start: game?.started_at,
                            end: game?.ended_at,
                            xAxisIndex: 0,
                            showDetail: false,
                        },
                        {
                            type: "inside",
                            start: 0,
                            end: 100,
                            yAxisIndex: 0,
                            filterMode: "none",
                        },
                        {
                            start: 0,
                            end: 100,
                            yAxisIndex: 0,
                            showDetail: false,
                        },
                    ],
                }}
                style={{
                    width: "100%",
                    height: "512px",
                    display: "flex",
                }}
                opts={{
                    renderer: "svg",
                }}
            />

            <Table stickyHeader horizontalSpacing={"md"}>
                <Table.Thead>
                    <Table.Tr h={50}>
                        <Table.Th colSpan={4} w={"25%"}>
                            <Flex gap={20} justify={"center"}></Flex>
                        </Table.Th>
                        {Object.values(categoriedChallenges)?.map(
                            (categoriedChallenge) => (
                                <Table.Th
                                    colSpan={
                                        categoriedChallenge?.challenges?.length
                                    }
                                    bg={alpha(
                                        categoriedChallenge?.category?.color ||
                                            "#FFF",
                                        0.3
                                    )}
                                    variant={"subtle"}
                                    sx={{
                                        whiteSpace: "nowrap",
                                    }}
                                    align={"center"}
                                    key={categoriedChallenge.category.id}
                                >
                                    <Group
                                        gap={20}
                                        align={"center"}
                                        justify={"center"}
                                    >
                                        <MDIcon
                                            color={
                                                categoriedChallenge?.category
                                                    ?.color
                                            }
                                        >
                                            {
                                                categoriedChallenge?.category
                                                    ?.icon
                                            }
                                        </MDIcon>
                                        <Text
                                            fw={700}
                                            c={
                                                colorScheme === "dark"
                                                    ? "#FFF"
                                                    : categoriedChallenge
                                                          ?.category?.color
                                            }
                                        >
                                            {categoriedChallenge.category.name}
                                        </Text>
                                    </Group>
                                </Table.Th>
                            )
                        )}
                    </Table.Tr>
                    <Table.Tr>
                        <Table.Th>排名</Table.Th>
                        <Table.Th>队伍</Table.Th>
                        <Table.Th>总分</Table.Th>
                        <Table.Th>攻克</Table.Th>
                        {Object.values(categoriedChallenges)?.map(
                            (categoriedChallenge) =>
                                categoriedChallenge.challenges?.map(
                                    (challenge) => (
                                        <Table.Th
                                            key={challenge.id}
                                            align="center"
                                            sx={{
                                                whiteSpace: "nowrap",
                                            }}
                                        >
                                            <Flex justify={"center"}>
                                                {challenge.title}
                                            </Flex>
                                        </Table.Th>
                                    )
                                )
                        )}
                    </Table.Tr>
                </Table.Thead>
                <Table.Tbody>
                    {rows?.map((row, i) => (
                        <Table.Tr key={`${i}`}>
                            <Table.Td>{row?.rank}</Table.Td>
                            <Table.Td>
                                <Group wrap={"nowrap"}>
                                    <Avatar
                                        color="brand"
                                        src={`/api/teams/${row?.team?.id}/avatar`}
                                    >
                                        <MDIcon>people</MDIcon>
                                    </Avatar>
                                    <Text>{row?.team?.name}</Text>
                                </Group>
                            </Table.Td>
                            <Table.Td>{row?.totalScore}</Table.Td>
                            <Table.Td>{row?.solvedCount}</Table.Td>
                            {Object.values(categoriedChallenges)?.map(
                                (categoriedChallenge, j) => (
                                    <React.Fragment key={`${i}-${j}`}>
                                        {categoriedChallenge?.challenges?.map(
                                            (challenge, k) => {
                                                const submission =
                                                    row?.submissions?.find(
                                                        (submission) =>
                                                            submission?.challenge_id ===
                                                            challenge.id
                                                    );
                                                return (
                                                    <Table.Td
                                                        key={`${i}-${j}-${k}`}
                                                        align="center"
                                                    >
                                                        {submission?.pts && (
                                                            <>
                                                                <Tooltip
                                                                    label={` + ${submission?.pts} ${submission?.user?.nickname}`}
                                                                >
                                                                    <ThemeIcon
                                                                        size={
                                                                            25
                                                                        }
                                                                        variant="transparent"
                                                                        color="brand"
                                                                    >
                                                                        {submission?.rank ===
                                                                        1 ? (
                                                                            <FirstBloodIcon />
                                                                        ) : submission?.rank ===
                                                                          2 ? (
                                                                            <SecondBloodIcon />
                                                                        ) : submission?.rank ===
                                                                          3 ? (
                                                                            <ThirdBloodIcon />
                                                                        ) : (
                                                                            <MDIcon>
                                                                                flag
                                                                            </MDIcon>
                                                                        )}
                                                                    </ThemeIcon>
                                                                </Tooltip>
                                                            </>
                                                        )}
                                                    </Table.Td>
                                                );
                                            }
                                        )}
                                    </React.Fragment>
                                )
                            )}
                        </Table.Tr>
                    ))}
                </Table.Tbody>
            </Table>
        </Stack>
    );
}

export default withGame(Page);
