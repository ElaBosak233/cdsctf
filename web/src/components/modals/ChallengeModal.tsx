import { Challenge, ChallengeStatus } from "@/types/challenge";
import {
    Box,
    Card,
    Group,
    Tooltip,
    Text,
    ThemeIcon,
    Divider,
    Flex,
    TextInput,
    Button,
    Stack,
    ActionIcon,
    ModalProps,
    Modal,
} from "@mantine/core";
import MDIcon from "@/components/ui/MDIcon";
import FirstBloodIcon from "@/components/icons/hexagons/FirstBloodIcon";
import ThirdBloodIcon from "@/components/icons/hexagons/ThirdBloodIcon";
import SecondBloodIcon from "@/components/icons/hexagons/SecondBloodIcon";
import MarkdownRender from "../utils/MarkdownRender";
import { useEffect, useState } from "react";
import { Pod } from "@/types/pod";
import { usePodApi } from "@/api/pod";
import { useAuthStore } from "@/stores/auth";
import { useSubmissionApi } from "@/api/submission";
import {
    showErrNotification,
    showInfoNotification,
    showSuccessNotification,
    showWarnNotification,
} from "@/utils/notification";
import { useForm } from "@mantine/form";
import { useTeamStore } from "@/stores/team";
import { useClipboard, useInterval } from "@mantine/hooks";
import { Metadata } from "@/types/media";
import { useChallengeApi } from "@/api/challenge";
import { useCategoryStore } from "@/stores/category";
import { Status } from "@/types/submission";

interface ChallengeModalProps extends ModalProps {
    challenge?: Challenge;
    gameID?: number;
    setRefresh: () => void;
    status?: ChallengeStatus;
}

export default function ChallengeModal(props: ChallengeModalProps) {
    const { challenge, gameID, setRefresh, status, ...modalProps } = props;

    const clipboard = useClipboard({ timeout: 500 });
    const podApi = usePodApi();
    const challengeApi = useChallengeApi();
    const submissionApi = useSubmissionApi();
    const authStore = useAuthStore();
    const teamStore = useTeamStore();
    const categoryStore = useCategoryStore();

    const [attachmentMetadata, setAttachmentMetadata] = useState<Metadata>();

    const [pod, setPod] = useState<Pod>();
    const [podTime, setPodTime] = useState<number>(0);
    const interval = useInterval(() => setPodTime((s) => s - 1), 1000);
    const [podCreateLoading, setPodCreateLoading] = useState(false);
    const [podRemoveLoading, setPodRemoveLoading] = useState(false);
    const [podRenewLoading, setPodRenewLoading] = useState(false);

    const category = categoryStore.getCategory(Number(challenge?.category_id));

    const form = useForm({
        mode: "uncontrolled",
        initialValues: {
            flag: "",
        },
    });

    function getAttachmentMetadata() {
        challengeApi
            .getChallengeAttachmentMetadata(Number(challenge?.id))
            .then((res) => {
                const r = res.data;
                setAttachmentMetadata(r?.data);
            });
    }

    function getPod() {
        podApi
            .getPods({
                challenge_id: challenge?.id,
                user_id: !gameID ? authStore?.user?.id : undefined,
                team_id: gameID ? teamStore?.selectedTeamID : undefined,
                game_id: gameID ? gameID : undefined,
                is_available: true,
            })
            .then((res) => {
                const r = res.data;
                setPod(r.data?.[0] as Pod);
            });
    }

    function createPod() {
        setPodCreateLoading(true);
        podApi
            .createPod({
                challenge_id: challenge?.id,
                team_id: gameID ? teamStore?.selectedTeamID : undefined,
                game_id: gameID ? gameID : undefined,
            })
            .then((res) => {
                const r = res.data;
                setPod(r?.data);
            })
            .catch((e) => {
                showErrNotification({
                    title: "错误",
                    message: e.response.data.msg,
                });
            })
            .finally(() => {
                setPodCreateLoading(false);
            });
    }

    function removePod() {
        setPodRemoveLoading(true);
        podApi
            .removePod({
                id: pod?.id as number,
            })
            .then((_) => {
                setPod(undefined);
                setPodTime(0);
                showSuccessNotification({
                    title: "操作成功",
                    message: "实例已销毁！",
                });
            })
            .finally(() => {
                setPodRemoveLoading(false);
            });
    }

    function renewPod() {
        setPodRenewLoading(true);
        podApi
            .renewPod({
                id: pod?.id!,
            })
            .then((_) => {
                getPod();
            })
            .finally(() => {
                setPodRenewLoading(false);
            });
    }

    function submitFlag(flag?: string) {
        if (!flag?.trim()) {
            showErrNotification({
                title: "错误",
                message: "Flag 不能为空！",
            });
            return;
        }

        submissionApi
            .createSubmission({
                challenge_id: challenge?.id,
                flag: flag,
                team_id: gameID ? teamStore?.selectedTeamID : undefined,
                game_id: gameID ? gameID : undefined,
            })
            .then((res) => {
                const r = res.data;
                switch (r?.data?.status) {
                    case Status.Incorrect:
                        showWarnNotification({
                            title: "错误",
                            message: "再试试，你可以的！",
                        });
                        break;
                    case Status.Correct:
                        showSuccessNotification({
                            title: "正确",
                            message: "恭喜你，答对了！",
                        });
                        setRefresh();
                        form.reset();
                        break;
                    case Status.Cheat:
                        showErrNotification({
                            title: "作弊",
                            message:
                                "你提交了禁止提交的 Flag 或者他人的 Flag，该行为已记录！",
                        });
                        break;
                    case Status.Invalid:
                        showInfoNotification({
                            title: "无效",
                            message: "提交入口已关闭或你已提交过正确的 Flag！",
                        });
                        setRefresh();
                        form.reset();
                        break;
                }
            })
            .catch((e) => {
                showErrNotification({
                    title: "错误",
                    message: e.response.data.msg,
                });
            });
    }

    useEffect(() => {
        if (podTime > 0) {
            interval.start();
            return interval.stop;
        }
    }, [podTime]);

    useEffect(() => {
        if (pod) {
            setPodTime(
                Math.ceil(pod?.removed_at - new Date().getTime() / 1000)
            );
        }
    }, [pod]);

    useEffect(() => {
        if (challenge?.is_dynamic) {
            getPod();
        }
        if (challenge?.has_attachment) {
            getAttachmentMetadata();
        }
    }, [challenge, modalProps.opened]);

    useEffect(() => {
        form.reset();
        setPodTime(0);
        setPod(undefined);
        setAttachmentMetadata(undefined);
    }, [modalProps.opened]);

    return (
        <Modal.Root {...modalProps}>
            <Modal.Overlay />
            <Modal.Content
                sx={{
                    flex: "none",
                    backgroundColor: "transparent",
                }}
            >
                <Card
                    shadow="md"
                    padding="lg"
                    radius="md"
                    withBorder
                    miw={"40vw"}
                    mih={"20rem"}
                    sx={{
                        position: "relative",
                        display: "flex",
                        flexDirection: "column",
                        justifyContent: "space-between",
                    }}
                >
                    <Box>
                        <Group justify={"space-between"}>
                            <Group gap={6}>
                                <MDIcon color={category?.color}>
                                    {category?.icon}
                                </MDIcon>
                                <Text fw={700}>{challenge?.title}</Text>
                            </Group>
                            <Group gap={0}>
                                {(status?.bloods?.length as number) > 0 && (
                                    <Tooltip
                                        label={`一血 ${status?.bloods?.[0]?.team?.name || status?.bloods?.[0]?.user?.nickname}`}
                                        position={"top"}
                                    >
                                        <ThemeIcon variant="transparent">
                                            <FirstBloodIcon />
                                        </ThemeIcon>
                                    </Tooltip>
                                )}
                                {(status?.bloods?.length as number) > 1 && (
                                    <Tooltip
                                        label={`二血 ${status?.bloods?.[1]?.team?.name || status?.bloods?.[1]?.user?.nickname}`}
                                        position={"top"}
                                    >
                                        <Box
                                            sx={{
                                                display: "flex",
                                                alignItems: "center",
                                            }}
                                        >
                                            <ThemeIcon variant="transparent">
                                                <SecondBloodIcon />
                                            </ThemeIcon>
                                        </Box>
                                    </Tooltip>
                                )}
                                {(status?.bloods?.length as number) > 2 && (
                                    <Tooltip
                                        label={`三血 ${status?.bloods?.[2]?.team?.name || status?.bloods?.[2]?.user?.nickname}`}
                                        position={"top"}
                                    >
                                        <Box
                                            sx={{
                                                display: "flex",
                                                alignItems: "center",
                                            }}
                                        >
                                            <ThemeIcon variant="transparent">
                                                <ThirdBloodIcon />
                                            </ThemeIcon>
                                        </Box>
                                    </Tooltip>
                                )}
                            </Group>
                        </Group>
                        <Divider my={10} />
                        <Flex justify={"space-between"}>
                            <MarkdownRender
                                src={challenge?.description || ""}
                            />
                            {attachmentMetadata?.filename && (
                                <Tooltip
                                    label={`下载附件 ${attachmentMetadata?.filename}`}
                                    withArrow
                                    position={"top"}
                                >
                                    <ActionIcon
                                        onClick={() => {
                                            window.open(
                                                `/api/challenges/${challenge?.id}/attachment`
                                            );
                                        }}
                                    >
                                        <MDIcon c={category?.color}>
                                            download
                                        </MDIcon>
                                    </ActionIcon>
                                </Tooltip>
                            )}
                        </Flex>
                    </Box>
                    <Box>
                        {challenge?.is_dynamic && (
                            <Stack mt={50}>
                                <Stack gap={5}>
                                    {pod?.nats?.map((nat) => (
                                        <TextInput
                                            value={
                                                nat?.proxy
                                                    ? `ws://${window.location.host}/api/proxies/${pod.name}?port=${nat.src}`
                                                    : nat.entry
                                            }
                                            readOnly
                                            sx={{
                                                input: {
                                                    "&:focus": {
                                                        borderColor:
                                                            category?.color,
                                                    },
                                                },
                                            }}
                                            leftSectionWidth={135}
                                            leftSection={
                                                <Flex
                                                    w={"100%"}
                                                    px={10}
                                                    gap={10}
                                                >
                                                    <MDIcon c={"gray"}>
                                                        lan
                                                    </MDIcon>
                                                    <Flex
                                                        align={"center"}
                                                        justify={
                                                            "space-between"
                                                        }
                                                        sx={{
                                                            flexGrow: 1,
                                                        }}
                                                    >
                                                        <Text>{nat.src}</Text>
                                                        <MDIcon c={"gray"}>
                                                            arrow_right_alt
                                                        </MDIcon>
                                                    </Flex>
                                                </Flex>
                                            }
                                            rightSectionWidth={100}
                                            rightSection={
                                                <Flex>
                                                    <Divider
                                                        mx={10}
                                                        orientation={"vertical"}
                                                    />
                                                    <Tooltip
                                                        withArrow
                                                        label={
                                                            clipboard.copied
                                                                ? "已复制"
                                                                : "复制到剪贴板"
                                                        }
                                                    >
                                                        <ActionIcon
                                                            onClick={() =>
                                                                clipboard.copy(
                                                                    nat?.entry
                                                                )
                                                            }
                                                        >
                                                            <MDIcon c={"gray"}>
                                                                content_copy
                                                            </MDIcon>
                                                        </ActionIcon>
                                                    </Tooltip>
                                                    <Tooltip
                                                        withArrow
                                                        label={"在浏览器中打开"}
                                                    >
                                                        <ActionIcon
                                                            onClick={() => {
                                                                window.open(
                                                                    `http://${nat?.entry}`
                                                                );
                                                            }}
                                                        >
                                                            <MDIcon c={"gray"}>
                                                                open_in_new
                                                            </MDIcon>
                                                        </ActionIcon>
                                                    </Tooltip>
                                                </Flex>
                                            }
                                        />
                                    ))}
                                </Stack>
                                <Flex
                                    justify={"space-between"}
                                    align={"center"}
                                >
                                    <Stack gap={5}>
                                        <Text fw={700} size="0.8rem">
                                            本题为动态容器题目，解题需开启容器实例
                                        </Text>
                                        <Text size="0.8rem" c="secondary">
                                            本题容器时间{" "}
                                            {podTime || challenge?.duration}s
                                        </Text>
                                    </Stack>
                                    <Flex gap={10}>
                                        {pod?.id && (
                                            <>
                                                <Button
                                                    loading={podRenewLoading}
                                                    color={"blue"}
                                                    onClick={renewPod}
                                                >
                                                    实例续期
                                                </Button>
                                                <Button
                                                    loading={podRemoveLoading}
                                                    color={"red"}
                                                    onClick={removePod}
                                                >
                                                    销毁实例
                                                </Button>
                                            </>
                                        )}
                                        {!pod?.id && (
                                            <Button
                                                size="sm"
                                                color={category?.color}
                                                loading={podCreateLoading}
                                                onClick={createPod}
                                            >
                                                开启容器
                                            </Button>
                                        )}
                                    </Flex>
                                </Flex>
                            </Stack>
                        )}
                        <Divider my={20} />
                        <form
                            onSubmit={form.onSubmit((values) =>
                                submitFlag(values.flag)
                            )}
                        >
                            <Flex align="center" gap={6}>
                                <TextInput
                                    variant="filled"
                                    placeholder="Flag"
                                    w={"85%"}
                                    leftSection={
                                        <MDIcon color={category?.color}>
                                            flag
                                        </MDIcon>
                                    }
                                    sx={{
                                        input: {
                                            "&:focus": {
                                                borderColor: category?.color,
                                            },
                                        },
                                    }}
                                    key={form.key("flag")}
                                    {...form.getInputProps("flag")}
                                />
                                <Button
                                    color={category?.color}
                                    w={"15%"}
                                    type="submit"
                                >
                                    提交
                                </Button>
                            </Flex>
                        </form>
                    </Box>
                </Card>
            </Modal.Content>
        </Modal.Root>
    );
}
