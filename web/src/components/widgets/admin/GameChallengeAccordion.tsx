import { useGameApi } from "@/api/game";
import MDIcon from "@/components/ui/MDIcon";
import { useCategoryStore } from "@/stores/category";
import { ChallengeStatus } from "@/types/challenge";
import { GameChallenge } from "@/types/game_challenge";
import { showSuccessNotification } from "@/utils/notification";
import {
    Accordion,
    Flex,
    Group,
    Switch,
    Tooltip,
    ActionIcon,
    Text,
    NumberInput,
    Button,
    Badge,
    Center,
    Divider,
    Stack,
} from "@mantine/core";
import { useForm } from "@mantine/form";
import { modals } from "@mantine/modals";
import { useEffect } from "react";
import { useNavigate } from "react-router-dom";
import Curve from "./Curve";

export default function GameChallengeAccordion({
    gameChallenge,
    status,
    setRefresh,
}: {
    gameChallenge: GameChallenge;
    status: ChallengeStatus;
    setRefresh: () => void;
}) {
    const navigate = useNavigate();
    const gameApi = useGameApi();
    const categoryStore = useCategoryStore();

    const form = useForm({
        initialValues: {
            min_pts: 0,
            max_pts: 0,
            difficulty: 0,
            first_blood_reward_ratio: 0,
            second_blood_reward_ratio: 0,
            third_blood_reward_ratio: 0,
        },
    });

    function switchIsEnabled() {
        gameApi
            .updateGameChallenge({
                game_id: gameChallenge?.game_id,
                challenge_id: gameChallenge?.challenge_id,
                is_enabled: !gameChallenge?.is_enabled,
            })
            .then((_) => {
                showSuccessNotification({
                    message: !gameChallenge?.is_enabled
                        ? `题目 ${gameChallenge?.challenge?.title} 已投放至比赛`
                        : `题目 ${gameChallenge?.challenge?.title} 已从比赛移除`,
                });
            })
            .finally(() => {
                setRefresh();
            });
    }

    function updateGameChallenge() {
        gameApi
            .updateGameChallenge({
                game_id: gameChallenge?.game_id,
                challenge_id: gameChallenge?.challenge_id,
                min_pts: form.getValues().min_pts,
                max_pts: form.getValues().max_pts,
                difficulty: form.getValues().difficulty,
                first_blood_reward_ratio:
                    form.getValues().first_blood_reward_ratio,
                second_blood_reward_ratio:
                    form.getValues().second_blood_reward_ratio,
                third_blood_reward_ratio:
                    form.getValues().third_blood_reward_ratio,
            })
            .then((_) => {
                showSuccessNotification({
                    message: "题目分值已更新",
                });
            })
            .finally(() => {
                setRefresh();
            });
    }

    function deleteGameChallenge(gameChallenge?: GameChallenge) {
        if (gameChallenge) {
            gameApi
                .deleteGameChallenge({
                    game_id: gameChallenge?.game_id,
                    challenge_id: gameChallenge?.challenge_id,
                })
                .then(() => {
                    showSuccessNotification({
                        message: "题目已删除",
                    });
                    setRefresh();
                });
        }
    }

    const openDeleteGameChallengeModal = (gameChallenge?: GameChallenge) =>
        modals.openConfirmModal({
            centered: true,
            children: (
                <>
                    <Flex gap={10} align={"center"}>
                        <MDIcon>bookmark_remove</MDIcon>
                        <Text fw={600}>删除题目</Text>
                    </Flex>
                    <Divider my={10} />
                    <Text>
                        你确定要删除题目 {gameChallenge?.challenge?.title} 吗？
                    </Text>
                </>
            ),
            withCloseButton: false,
            labels: {
                confirm: "确定",
                cancel: "取消",
            },
            confirmProps: {
                color: "red",
            },
            onConfirm: () => {
                deleteGameChallenge(gameChallenge);
            },
        });

    useEffect(() => {
        if (gameChallenge) {
            form.setValues({
                min_pts: Number(gameChallenge?.min_pts),
                max_pts: Number(gameChallenge?.max_pts),
                difficulty: Number(gameChallenge?.difficulty),
                first_blood_reward_ratio: Number(
                    gameChallenge?.first_blood_reward_ratio
                ),
                second_blood_reward_ratio: Number(
                    gameChallenge?.second_blood_reward_ratio
                ),
                third_blood_reward_ratio: Number(
                    gameChallenge?.third_blood_reward_ratio
                ),
            });
        }
    }, [gameChallenge]);

    return (
        <Accordion.Item value={`${gameChallenge?.challenge_id}`}>
            <Center mx={20}>
                <Switch
                    checked={gameChallenge?.is_enabled}
                    onChange={(_) => switchIsEnabled()}
                />
                <Accordion.Control>
                    <Flex justify={"space-between"}>
                        <Flex gap={15} align={"center"}>
                            <Badge>{gameChallenge?.challenge?.id}</Badge>
                            <MDIcon
                                color={
                                    categoryStore.getCategory(
                                        Number(
                                            gameChallenge?.challenge
                                                ?.category_id
                                        )
                                    )?.color
                                }
                            >
                                {
                                    categoryStore.getCategory(
                                        Number(
                                            gameChallenge?.challenge
                                                ?.category_id
                                        )
                                    )?.icon
                                }
                            </MDIcon>
                            <Text fw={700} size="1rem">
                                {gameChallenge?.challenge?.title}
                            </Text>
                        </Flex>
                        <Group mx={15}>
                            <Tooltip label="当前分值" withArrow>
                                <Badge>{status?.pts}</Badge>
                            </Tooltip>
                        </Group>
                    </Flex>
                </Accordion.Control>
                <Flex gap={10}>
                    <Tooltip label="编辑题目" withArrow>
                        <ActionIcon
                            variant="transparent"
                            onClick={() =>
                                navigate(
                                    `/admin/challenges/${gameChallenge?.challenge?.id}`
                                )
                            }
                        >
                            <MDIcon>edit</MDIcon>
                        </ActionIcon>
                    </Tooltip>
                    <Tooltip label="删除题目" withArrow>
                        <ActionIcon
                            variant="transparent"
                            onClick={() =>
                                openDeleteGameChallengeModal(gameChallenge)
                            }
                        >
                            <MDIcon color={"red"}>delete</MDIcon>
                        </ActionIcon>
                    </Tooltip>
                </Flex>
            </Center>
            <Accordion.Panel>
                <form onSubmit={form.onSubmit((_) => updateGameChallenge())}>
                    <Stack align={"end"}>
                        <Flex w={"100%"} gap={10}>
                            <Stack>
                                <Group gap={10} grow>
                                    <NumberInput
                                        label="一血奖励（%）"
                                        key={form.key(
                                            "first_blood_reward_ratio"
                                        )}
                                        {...form.getInputProps(
                                            "first_blood_reward_ratio"
                                        )}
                                    />
                                    <NumberInput
                                        label="二血奖励（%）"
                                        key={form.key(
                                            "second_blood_reward_ratio"
                                        )}
                                        {...form.getInputProps(
                                            "second_blood_reward_ratio"
                                        )}
                                    />
                                    <NumberInput
                                        label="三血奖励（%）"
                                        key={form.key(
                                            "third_blood_reward_ratio"
                                        )}
                                        {...form.getInputProps(
                                            "third_blood_reward_ratio"
                                        )}
                                    />
                                </Group>
                                <Group gap={10} grow>
                                    <NumberInput
                                        label="难度系数"
                                        withAsterisk
                                        key={form.key("difficulty")}
                                        {...form.getInputProps("difficulty")}
                                    />
                                    <NumberInput
                                        label="最小分值"
                                        withAsterisk
                                        key={form.key("min_pts")}
                                        {...form.getInputProps("min_pts")}
                                    />
                                    <NumberInput
                                        label="最大分值"
                                        withAsterisk
                                        key={form.key("max_pts")}
                                        {...form.getInputProps("max_pts")}
                                    />
                                </Group>
                            </Stack>
                            <Curve
                                maxPts={form.values.max_pts}
                                minPts={form.values.min_pts}
                                difficulty={form.values.difficulty}
                                sovledTimes={Number(status?.solved_times)}
                            />
                        </Flex>
                        <Button
                            type="submit"
                            leftSection={<MDIcon c={"white"}>check</MDIcon>}
                        >
                            保存
                        </Button>
                    </Stack>
                </form>
            </Accordion.Panel>
        </Accordion.Item>
    );
}
