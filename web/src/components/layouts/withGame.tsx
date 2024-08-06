import { Box, Button, Flex, Group, Progress, Stack, Text } from "@mantine/core";
import MDIcon from "@/components/ui/MDIcon";
import { Link, useLocation, useParams } from "react-router-dom";
import { Game } from "@/types/game";
import { useEffect, useState } from "react";
import { useInterval } from "@mantine/hooks";
import { getGames } from "@/api/game";

export default function withGame(WrappedComponent: React.ComponentType<any>) {
    return function withGame(props: any) {
        const { id } = useParams<{ id: string }>();
        const location = useLocation();
        const path = location.pathname.split(`/games/${id}`)[1];

        const [game, setGame] = useState<Game>();

        const [progress, setProgress] = useState<number>(0);
        const [seconds, setSeconds] = useState(0);
        const interval = useInterval(() => setSeconds((s) => s + 1), 1000);

        function handleGetGame() {
            getGames({
                id: Number(id),
            }).then((res) => {
                const r = res.data;
                setGame(r.data[0]);
            });
        }

        useEffect(() => {
            setProgress(
                ((Math.floor(Date.now() / 1000) - Number(game?.started_at)) /
                    (Number(game?.ended_at) - Number(game?.started_at))) *
                    100
            );
        }, [game, seconds]);

        useEffect(() => {
            interval.start();
            return interval.stop;
        }, []);

        useEffect(() => {
            handleGetGame();
        }, []);

        return (
            <>
                <Stack m={36}>
                    <Flex justify={"space-between"} align={"center"}>
                        <Box w={"50%"}></Box>
                        <Stack
                            align={"center"}
                            sx={{
                                flexShrink: 0,
                            }}
                        >
                            <Text fw={700} size="2rem">
                                {game?.title}
                            </Text>
                            <Progress
                                w={"25vw"}
                                radius={"xl"}
                                size={"md"}
                                value={progress}
                                animated
                                transitionDuration={200}
                            />
                        </Stack>
                        <Flex w={"50%"} justify={"end"}>
                            <Group wrap={"nowrap"}>
                                <Button
                                    component={Link}
                                    size="md"
                                    leftSection={
                                        <MDIcon
                                            c={
                                                path === "/challenges"
                                                    ? "white"
                                                    : "brand"
                                            }
                                        >
                                            flag
                                        </MDIcon>
                                    }
                                    variant={
                                        path === "/challenges"
                                            ? "filled"
                                            : "outline"
                                    }
                                    to={`/games/${id}/challenges`}
                                >
                                    题目
                                </Button>
                                <Button
                                    component={Link}
                                    size="md"
                                    leftSection={
                                        <MDIcon
                                            c={
                                                path === "/scoreboard"
                                                    ? "white"
                                                    : "brand"
                                            }
                                        >
                                            trending_up
                                        </MDIcon>
                                    }
                                    variant={
                                        path === "/scoreboard"
                                            ? "filled"
                                            : "outline"
                                    }
                                    to={`/games/${id}/scoreboard`}
                                >
                                    积分榜
                                </Button>
                            </Group>
                        </Flex>
                    </Flex>
                    <WrappedComponent {...props} />
                </Stack>
            </>
        );
    };
}
