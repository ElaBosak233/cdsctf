import { useGameApi } from "@/api/game";
import { useSubmissionApi } from "@/api/submission";
import withGameEdit from "@/components/layouts/admin/withGameEdit";
import MDIcon from "@/components/ui/MDIcon";
import { Game } from "@/types/game";
import { Submission } from "@/types/submission";
import { showSuccessNotification } from "@/utils/notification";
import {
    Divider,
    Group,
    Stack,
    Text,
    Table,
    Pagination,
    Flex,
    Badge,
    Avatar,
    ActionIcon,
    Tooltip,
    LoadingOverlay,
} from "@mantine/core";
import { modals } from "@mantine/modals";
import dayjs from "dayjs";
import { useEffect, useState } from "react";
import { useParams } from "react-router-dom";

function Page() {
    const { id } = useParams<{ id: string }>();
    const submissionApi = useSubmissionApi();
    const gameApi = useGameApi();

    const [game, setGame] = useState<Game>();
    const [submissions, setSubmissions] = useState<Array<Submission>>([]);

    const [total, setTotal] = useState<number>(0);
    const [rowsPerPage, _] = useState<number>(10);
    const [page, setPage] = useState<number>(1);

    const [loading, setLoading] = useState<boolean>(false);

    const [refresh, setRefresh] = useState<number>(0);

    const statusMap = new Map<number, { color: string; label: string }>([
        [
            0,
            {
                color: "gray",
                label: "Unjudged",
            },
        ],
        [
            1,
            {
                color: "red",
                label: "Wrong",
            },
        ],
        [
            2,
            {
                color: "green",
                label: "Accpet",
            },
        ],
        [
            3,
            {
                color: "orange",
                label: "Cheat",
            },
        ],
        [
            4,
            {
                color: "blue",
                label: "Invalid",
            },
        ],
    ]);

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
        submissionApi
            .getSubmissions({
                game_id: Number(id),
                page: page,
                size: rowsPerPage,
                is_detailed: true,
            })
            .then((res) => {
                const r = res.data;
                setSubmissions(r.data);
                setTotal(r.total);
            })
            .finally(() => {
                setLoading(false);
            });
    }

    function deleteSubmission(submission?: Submission) {
        if (submission) {
            submissionApi
                .deleteSubmission({
                    id: submission?.id,
                })
                .then(() => {
                    showSuccessNotification({
                        message: "提交记录已移除",
                    });
                    setRefresh((prev) => prev + 1);
                });
        }
    }

    const openDeleteSubmissionModal = (submission?: Submission) =>
        modals.openConfirmModal({
            centered: true,
            children: (
                <>
                    <Flex gap={10} align={"center"}>
                        <MDIcon>verified</MDIcon>
                        <Text fw={600}>删除提交记录</Text>
                    </Flex>
                    <Divider my={10} />
                    <Text>你确定要删除提交记录 {submission?.flag} 吗？</Text>
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
                deleteSubmission(submission);
            },
        });

    useEffect(() => {
        if (game) {
            getSubmissions();
        }
    }, [game, page, rowsPerPage, refresh]);

    useEffect(() => {
        getGame();
    }, []);

    useEffect(() => {
        document.title = `提交记录 - ${game?.title}`;
    }, [game]);

    return (
        <>
            <Stack m={36}>
                <Stack gap={10}>
                    <Group>
                        <MDIcon>verified</MDIcon>
                        <Text fw={700} size="xl">
                            提交记录
                        </Text>
                    </Group>
                    <Divider />
                </Stack>
                <Stack mx={20} mih={"calc(100vh - 360px)"} pos={"relative"}>
                    <LoadingOverlay visible={loading} />
                    <Table stickyHeader horizontalSpacing={"md"} striped>
                        <Table.Thead>
                            <Table.Tr
                                sx={{
                                    lineHeight: 3,
                                }}
                            >
                                <Table.Th />
                                <Table.Th>Flag</Table.Th>
                                <Table.Th>题目</Table.Th>
                                <Table.Th>相关团队</Table.Th>
                                <Table.Th>提交者</Table.Th>
                                <Table.Th>提交时间</Table.Th>
                                <Table.Th />
                            </Table.Tr>
                        </Table.Thead>
                        <Table.Tbody>
                            {submissions?.map((submission) => (
                                <Table.Tr key={submission?.id}>
                                    <Table.Td>
                                        <Badge
                                            color={
                                                statusMap?.get(
                                                    Number(submission?.status)
                                                )?.color
                                            }
                                        >
                                            {
                                                statusMap?.get(
                                                    Number(submission?.status)
                                                )?.label
                                            }
                                        </Badge>
                                    </Table.Td>
                                    <Table.Td
                                        maw={200}
                                        sx={{
                                            overflow: "hidden",
                                            textOverflow: "ellipsis",
                                            whiteSpace: "nowrap",
                                        }}
                                    >
                                        {submission?.flag}
                                    </Table.Td>
                                    <Table.Td>
                                        {submission?.challenge?.title}
                                    </Table.Td>
                                    <Table.Td>
                                        <Group gap={15}>
                                            <Avatar
                                                color="brand"
                                                src={`/api/teams/${submission?.team?.id}/avatar`}
                                                radius="xl"
                                            >
                                                <MDIcon>people</MDIcon>
                                            </Avatar>
                                            {submission?.team?.name}
                                        </Group>
                                    </Table.Td>
                                    <Table.Td>
                                        <Group gap={15}>
                                            <Avatar
                                                color="brand"
                                                src={`/api/users/${submission?.user?.id}/avatar`}
                                                radius="xl"
                                            >
                                                <MDIcon>person</MDIcon>
                                            </Avatar>
                                            {submission?.user?.nickname}
                                        </Group>
                                    </Table.Td>
                                    <Table.Td>
                                        <Badge>
                                            {dayjs(
                                                Number(submission?.created_at)
                                            ).format("YYYY/MM/DD HH:mm:ss")}
                                        </Badge>
                                    </Table.Td>
                                    <Table.Td>
                                        <Group>
                                            <Tooltip
                                                withArrow
                                                label="删除提交记录"
                                            >
                                                <ActionIcon
                                                    onClick={() =>
                                                        openDeleteSubmissionModal(
                                                            submission
                                                        )
                                                    }
                                                >
                                                    <MDIcon color={"red"}>
                                                        delete
                                                    </MDIcon>
                                                </ActionIcon>
                                            </Tooltip>
                                        </Group>
                                    </Table.Td>
                                </Table.Tr>
                            ))}
                        </Table.Tbody>
                    </Table>
                </Stack>
                <Flex justify={"center"}>
                    <Pagination
                        withEdges
                        total={Math.ceil(total / rowsPerPage)}
                        value={page}
                        onChange={setPage}
                    />
                </Flex>
            </Stack>
        </>
    );
}

export default withGameEdit(Page);
