import { getChallengeStatus } from "@/api/challenge";
import {
    getGameChallenges,
    getGames,
    getGameSubmissions,
    getGameTeams,
} from "@/api/game";
import withGame from "@/components/layouts/withGame";
import ChallengeModal from "@/components/modals/ChallengeModal";
import MDIcon from "@/components/ui/MDIcon";
import ChallengeCard from "@/components/widgets/ChallengeCard";
import GameNoticeArea from "@/components/widgets/GameNoticeArea";
import { useAuthStore } from "@/stores/auth";
import { useCategoryStore } from "@/stores/category";
import { useConfigStore } from "@/stores/config";
import { useTeamStore } from "@/stores/team";
import { Category } from "@/types/category";
import { ChallengeStatus } from "@/types/challenge";
import { Game } from "@/types/game";
import { GameChallenge } from "@/types/game_challenge";
import { GameTeam } from "@/types/game_team";
import { GameSubmission, Status } from "@/types/submission";
import { calculateAndSort } from "@/utils/game";
import { showErrNotification } from "@/utils/notification";
import {
    Avatar,
    Badge,
    Box,
    Button,
    Card,
    Divider,
    Flex,
    Group,
    LoadingOverlay,
    ScrollArea,
    Stack,
    Text,
    Title,
    UnstyledButton,
} from "@mantine/core";
import { useDisclosure } from "@mantine/hooks";
import { useEffect, useState } from "react";
import { useNavigate, useParams } from "react-router-dom";

function Page() {
    const { id } = useParams<{ id: string }>();
    const configStore = useConfigStore();
    const categoryStore = useCategoryStore();
    const authStore = useAuthStore();
    const teamStore = useTeamStore();

    const navigate = useNavigate();

    const [game, setGame] = useState<Game>();
    const [gameChallenges, setGameChallenges] = useState<Array<GameChallenge>>(
        []
    );
    const [status, setStatus] = useState<Record<number, ChallengeStatus>>();

    const [categories, setCategories] = useState<Record<number, Category>>({});
    const [selectedGameChallenges, setSelectedGameChallenges] = useState<
        Array<GameChallenge>
    >([]);
    const [selectedCategory, setSelectedCategory] = useState<number>(0);

    const [gameTeam, setGameTeam] = useState<GameTeam>();
    const [gameTeams, setGameTeams] = useState<Array<GameTeam>>([]);
    const [submissions, setSubmissions] = useState<Array<GameSubmission>>([]);

    const [loadingTeamStatus, setLoadingTeamStatus] = useState<boolean>(false);
    const [loadingChallenges, setLoadingChallenges] = useState<boolean>(false);

    const [opened, { open, close }] = useDisclosure(false);
    const [selectedChallenge, setSelectedChallenge] = useState<GameChallenge>();

    const [refresh, setRefresh] = useState<number>(0);

    const [score, setScore] = useState<number>(0);
    const [rank, setRank] = useState<number>(0);
    const [solves, setSolves] = useState<number>(0);

    function handleGetGameSubmissions() {
        setLoadingTeamStatus(true);
        getGameSubmissions({
            id: Number(id),
            status: Status.Correct,
        })
            .then((res) => {
                const r = res.data;
                setSubmissions(r.data);
            })
            .finally(() => {
                setLoadingTeamStatus(false);
            });
    }

    function handleGetGame() {
        getGames({
            id: Number(id),
        }).then((res) => {
            const r = res.data;
            setGame(r.data?.[0]);
        });
    }

    function handleGetGameChallenges() {
        setLoadingChallenges(true);
        getGameChallenges({
            game_id: Number(id),
            is_enabled: true,
        })
            .then((res) => {
                const r = res.data;
                setGameChallenges(r.data);
            })
            .finally(() => {
                setLoadingChallenges(false);
            });
    }

    function handleGetChallengeStatus() {
        getChallengeStatus({
            game_id: Number(id),
            cids: gameChallenges.map((c) => c.challenge_id!),
            team_id: gameTeam?.team_id,
        }).then((res) => {
            const r = res.data;
            setStatus(r.data);
        });
    }

    function handleGetGameTeams() {
        getGameTeams({
            game_id: Number(id),
        }).then((res) => {
            const r = res.data;
            setGameTeams(r.data);
        });
    }

    useEffect(() => {
        if (gameTeam) {
            const rows = calculateAndSort(submissions);
            if (rows) {
                rows?.forEach((row) => {
                    if (row?.team?.id === gameTeam?.team_id) {
                        setScore(row.totalScore);
                        setRank(row.rank as number);
                        setSolves(row.solvedCount);
                    }
                });
            }
        }
    }, [submissions, gameTeam]);

    useEffect(() => {
        if (gameTeams && gameTeams?.length) {
            for (const gameTeam of gameTeams) {
                for (const user of gameTeam?.team?.users || []) {
                    if (user?.id === authStore?.user?.id) {
                        setGameTeam(gameTeam);
                        teamStore.setSelectedTeamID(gameTeam?.team_id!);
                        if (gameTeam?.is_allowed) {
                            return;
                        }
                    }
                }
            }
            showErrNotification({
                title: "获取队伍信息失败",
                message: "请检查是否已加入可参赛的队伍",
            });
            navigate(`/games/${id}`);
        }
    }, [gameTeams]);

    useEffect(() => {
        if (selectedCategory != 0) {
            setSelectedGameChallenges(
                gameChallenges.filter((gameChallenge) => {
                    return (
                        gameChallenge?.challenge?.category_id ===
                        selectedCategory
                    );
                })
            );
        } else {
            setSelectedGameChallenges(gameChallenges);
        }
    }, [gameChallenges, selectedCategory]);

    useEffect(() => {
        if (gameChallenges.length) {
            gameChallenges.forEach((gameChallenge) => {
                if (
                    !(categories as Record<number, Category>)[
                        gameChallenge?.challenge?.category_id as number
                    ]
                ) {
                    setCategories((categories) => {
                        return {
                            ...categories,
                            [gameChallenge?.challenge?.category_id as number]:
                                categoryStore.getCategory(
                                    Number(
                                        gameChallenge?.challenge?.category_id
                                    )
                                ) as Category,
                        };
                    });
                }
            });
            handleGetChallengeStatus();
        }
    }, [gameChallenges]);

    useEffect(() => {
        handleGetGame();
        handleGetGameTeams();
        handleGetGameSubmissions();
    }, []);

    useEffect(() => {
        if (gameTeam) {
            handleGetGameChallenges();
        }
    }, [gameTeam, refresh]);

    useEffect(() => {
        document.title = `${game?.title} - ${configStore?.pltCfg?.site?.title}`;
    }, [game]);

    return (
        <>
            <Stack my={10} mx={"2%"}>
                <Flex justify={"space-between"}>
                    <Stack mx={10} miw={200} maw={200}>
                        <Button
                            size="lg"
                            leftSection={
                                <MDIcon
                                    c={
                                        !game?.is_need_write_up
                                            ? "gray.5"
                                            : "white"
                                    }
                                >
                                    upload
                                </MDIcon>
                            }
                            disabled={!game?.is_need_write_up}
                        >
                            上传题解
                        </Button>
                        <Divider />
                        <Stack gap={10}>
                            <Button
                                variant={
                                    selectedCategory === 0 ? "filled" : "subtle"
                                }
                                size="lg"
                                color="brand"
                                leftSection={
                                    <MDIcon
                                        c={
                                            selectedCategory === 0
                                                ? "white"
                                                : "brand"
                                        }
                                    >
                                        extension
                                    </MDIcon>
                                }
                                onClick={() => {
                                    setSelectedCategory(0);
                                }}
                            >
                                All
                            </Button>
                            {Object.entries(categories)?.map(
                                ([_, category]) => (
                                    <Button
                                        key={category?.id}
                                        variant={
                                            selectedCategory === category?.id
                                                ? "filled"
                                                : "subtle"
                                        }
                                        color={category?.color || "brand"}
                                        size="lg"
                                        leftSection={
                                            <MDIcon
                                                c={
                                                    selectedCategory ===
                                                    category?.id
                                                        ? "white"
                                                        : category?.color ||
                                                          "brand"
                                                }
                                            >
                                                {category?.icon}
                                            </MDIcon>
                                        }
                                        onClick={() => {
                                            setSelectedCategory(
                                                category?.id as number
                                            );
                                        }}
                                    >
                                        {category?.name?.toUpperCase()}
                                    </Button>
                                )
                            )}
                        </Stack>
                    </Stack>
                    <Box mx={20} w={"100%"}>
                        <ScrollArea h={"calc(100vh - 250px)"}>
                            <LoadingOverlay visible={loadingChallenges} />
                            <Group gap={"lg"} justify={"start"}>
                                {selectedGameChallenges?.map(
                                    (gameChallenge) => (
                                        <UnstyledButton
                                            onClick={() => {
                                                open();
                                                setSelectedChallenge(
                                                    gameChallenge
                                                );
                                            }}
                                            key={gameChallenge?.challenge_id}
                                        >
                                            <ChallengeCard
                                                challenge={
                                                    gameChallenge?.challenge
                                                }
                                                status={
                                                    status?.[
                                                        gameChallenge
                                                            ?.challenge_id!
                                                    ]
                                                }
                                            />
                                        </UnstyledButton>
                                    )
                                )}
                            </Group>
                        </ScrollArea>
                    </Box>
                    <Stack miw={330} maw={330} mx={10}>
                        <Card mih={185} shadow="md" p={25} pos={"relative"}>
                            <LoadingOverlay
                                visible={loadingTeamStatus}
                                zIndex={2}
                            />
                            <Stack>
                                <Flex gap={20} align={"center"}>
                                    <Avatar
                                        color="brand"
                                        size={64}
                                        src={`/api/teams/${gameTeam?.team_id}/avatar`}
                                    >
                                        <MDIcon size={36}>people</MDIcon>
                                    </Avatar>
                                    <Title
                                        fw={700}
                                        size={"1.25rem"}
                                        sx={{
                                            overflow: "hidden",
                                            textOverflow: "ellipsis",
                                            whiteSpace: "nowrap",
                                            flexGrow: 1,
                                        }}
                                    >
                                        {gameTeam?.team?.name}
                                    </Title>
                                </Flex>
                                <Flex justify={"space-between"} mx={36}>
                                    <Stack align={"center"} gap={10}>
                                        <Text fw={700} size="1.2rem">
                                            {rank > 0 ? rank : "无排名"}
                                        </Text>
                                        <Badge>排名</Badge>
                                    </Stack>
                                    <Stack align={"center"} gap={10}>
                                        <Text fw={700} size="1.2rem">
                                            {score || 0}
                                        </Text>
                                        <Badge>得分</Badge>
                                    </Stack>
                                    <Stack align={"center"} gap={10}>
                                        <Text fw={700} size="1.2rem">
                                            {solves || 0}
                                        </Text>
                                        <Badge>已解决</Badge>
                                    </Stack>
                                </Flex>
                            </Stack>
                        </Card>
                        <Card h={"calc(100vh - 450px)"} shadow="md">
                            <GameNoticeArea />
                        </Card>
                    </Stack>
                </Flex>
            </Stack>
            <ChallengeModal
                opened={opened}
                onClose={close}
                centered
                setRefresh={() => setRefresh((prev) => prev + 1)}
                challenge={selectedChallenge?.challenge}
                gameID={selectedChallenge?.game_id}
            />
        </>
    );
}

export default withGame(Page);
