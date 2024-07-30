import { useGameApi } from "@/api/game";
import withGameEdit from "@/components/layouts/admin/withGameEdit";
import GameChallengeCreateModal from "@/components/modals/admin/GameChallengeCreateModal";
import MDIcon from "@/components/ui/MDIcon";
import GameChallengeAccordion from "@/components/widgets/admin/GameChallengeAccordion";
import { Game } from "@/types/game";
import { GameChallenge } from "@/types/game_challenge";
import {
    Accordion,
    ActionIcon,
    Divider,
    Flex,
    Group,
    LoadingOverlay,
    Stack,
    Text,
    Tooltip,
} from "@mantine/core";
import { useDisclosure } from "@mantine/hooks";
import { useEffect, useState } from "react";
import { useParams } from "react-router-dom";

function Page() {
    const { id } = useParams<{ id: string }>();
    const gameApi = useGameApi();

    const [refresh, setRefresh] = useState<number>(0);

    const [game, setGame] = useState<Game>();
    const [gameChallenges, setGameChallenges] = useState<Array<GameChallenge>>(
        []
    );

    const [loading, setLoading] = useState<boolean>(false);

    const [createOpened, { open: createOpen, close: createClose }] =
        useDisclosure(false);

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

    function getChallenges() {
        setLoading(true);
        gameApi
            .getGameChallenges({
                game_id: Number(id),
            })
            .then((res) => {
                const r = res.data;
                setGameChallenges(r.data);
            })
            .finally(() => {
                setLoading(false);
            });
    }

    useEffect(() => {
        getGame();
    }, []);

    useEffect(() => {
        if (game) {
            getChallenges();
        }
    }, [game, refresh]);

    useEffect(() => {
        document.title = `题目管理 - ${game?.title}`;
    }, [game]);

    return (
        <>
            <Stack m={36}>
                <Stack gap={10}>
                    <Flex justify={"space-between"} align={"center"}>
                        <Group>
                            <MDIcon>collections_bookmark</MDIcon>
                            <Text fw={700} size="xl">
                                题目
                            </Text>
                        </Group>
                        <Tooltip label="添加题目" withArrow>
                            <ActionIcon onClick={() => createOpen()}>
                                <MDIcon>add</MDIcon>
                            </ActionIcon>
                        </Tooltip>
                    </Flex>
                    <Divider />
                </Stack>
                <Stack mx={20} mih={"calc(100vh - 360px)"} pos={"relative"}>
                    <LoadingOverlay visible={loading} />
                    <Accordion variant="separated">
                        {gameChallenges?.map((gameChallenge) => (
                            <GameChallengeAccordion
                                key={gameChallenge?.id}
                                gameChallenge={gameChallenge}
                                setRefresh={() => {
                                    setRefresh((prev) => prev + 1);
                                }}
                            />
                        ))}
                    </Accordion>
                </Stack>
            </Stack>
            <GameChallengeCreateModal
                setRefresh={() => {
                    setRefresh((prev) => prev + 1);
                }}
                opened={createOpened}
                onClose={createClose}
                centered
            />
        </>
    );
}

export default withGameEdit(Page);
