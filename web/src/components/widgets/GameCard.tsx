import { Game } from "@/types/game";
import {
    BackgroundImage,
    Badge,
    Card,
    Group,
    Indicator,
    Stack,
    Text,
    Tooltip,
} from "@mantine/core";
import { useEffect, useState } from "react";

export default function GameCard({ game }: { game?: Game }) {
    const [status, setStatus] = useState<number>(0);

    useEffect(() => {
        if (
            Number(game?.started_at) > Math.floor(new Date().getTime() / 1000)
        ) {
            setStatus(0);
        } else if (
            Number(game?.started_at) <
                Math.floor(new Date().getTime() / 1000) &&
            Math.floor(new Date().getTime() / 1000) < Number(game?.ended_at)
        ) {
            setStatus(1);
        } else if (
            Number(game?.ended_at) < Math.floor(new Date().getTime() / 1000)
        ) {
            setStatus(2);
        }
    }, []);

    return (
        <>
            <Card h={200} shadow="sm" className="no-select">
                <Card.Section
                    sx={{
                        display: "flex",
                        position: "relative",
                    }}
                >
                    <BackgroundImage
                        src={`${import.meta.env.VITE_BASE_API}/games/${game?.id}/poster`}
                        h={200}
                        w={"30%"}
                    ></BackgroundImage>
                    <Stack my={20} mx={20}>
                        <Group>
                            <Badge size="lg">
                                {game?.member_limit_max === 1
                                    ? "单人赛"
                                    : "多人赛"}
                            </Badge>
                            <Badge size="lg">
                                {Math.ceil(
                                    (Number(game?.ended_at) -
                                        Number(game?.started_at)) /
                                        3600
                                )}{" "}
                                小时
                            </Badge>
                        </Group>
                        <Text size="2rem" fw={900} ff={"san-serif"}>
                            {game?.title}
                        </Text>
                        <Text>{game?.bio}</Text>
                    </Stack>
                    <Tooltip
                        label={
                            status === 0
                                ? "未开始"
                                : status === 1
                                  ? "进行中"
                                  : "已结束"
                        }
                        offset={20}
                        withArrow
                    >
                        <Indicator
                            size={15}
                            color={
                                status === 0
                                    ? "orange"
                                    : status === 2
                                      ? "red"
                                      : undefined
                            }
                            zIndex={3}
                            processing={status === 1 ? true : false}
                            sx={{
                                position: "absolute",
                                right: 25,
                                top: 25,
                            }}
                        />
                    </Tooltip>
                </Card.Section>
            </Card>
        </>
    );
}
