import {
    deleteChallenge,
    getChallenges,
    updateChallenge,
} from "@/api/challenge";
import ChallengeCreateModal from "@/components/modals/admin/ChallengeCreateModal";
import MDIcon from "@/components/ui/MDIcon";
import { useCategoryStore } from "@/stores/category";
import { useConfigStore } from "@/stores/config";
import { Challenge } from "@/types/challenge";
import {
    showErrNotification,
    showSuccessNotification,
} from "@/utils/notification";
import {
    ActionIcon,
    Badge,
    Divider,
    Flex,
    Group,
    LoadingOverlay,
    Pagination,
    Paper,
    Select,
    Stack,
    Switch,
    Table,
    Text,
    TextInput,
    Tooltip,
    lighten,
    useMantineColorScheme,
} from "@mantine/core";
import { useDisclosure } from "@mantine/hooks";
import { modals } from "@mantine/modals";
import { useEffect, useState } from "react";
import { useNavigate } from "react-router-dom";

export default function Page() {
    const configStore = useConfigStore();
    const categoryStore = useCategoryStore();

    const { colorScheme } = useMantineColorScheme();

    const [refresh, setRefresh] = useState<number>(0);
    const [loading, setLoading] = useState<boolean>(false);

    const [challenges, setChallenges] = useState<Array<Challenge>>([]);
    const [page, setPage] = useState<number>(1);
    const [total, setTotal] = useState<number>(0);
    const [rowsPerPage, setRowsPerPage] = useState<number>(15);
    const [search, setSearch] = useState<string>("");
    const [searchInput, setSearchInput] = useState<string>("");
    const [sort, setSort] = useState<string>("id_asc");

    const navigate = useNavigate();

    const [createOpened, { open: createOpen, close: createClose }] =
        useDisclosure(false);

    function handleGetChallenges() {
        setLoading(true);
        getChallenges({
            page: page,
            size: rowsPerPage,
            title: search,
            sort_key: sort.split("_")[0],
            sort_order: sort.split("_")[1],
        })
            .then((res) => {
                const r = res.data;
                setChallenges(r.data);
                setTotal(r.total);
            })
            .finally(() => {
                setLoading(false);
            });
    }

    function switchIsPracticable(challenge?: Challenge) {
        updateChallenge({
            id: Number(challenge?.id),
            is_practicable: !challenge?.is_practicable,
        })
            .then((_) => {
                showSuccessNotification({
                    message: !challenge?.is_practicable
                        ? `题目 ${challenge?.title} 已投放进练习场`
                        : `题目 ${challenge?.title} 已从练习场移除`,
                });
            })
            .finally(() => {
                setRefresh((prev) => prev + 1);
            });
    }

    function handleDeleteChallenge(challenge?: Challenge) {
        deleteChallenge({
            id: Number(challenge?.id),
        })
            .then((_) => {
                showSuccessNotification({
                    message: `题目 ${challenge?.title} 已被删除`,
                });
            })
            .catch((e) => {
                showErrNotification({
                    message: e.response.data.message,
                });
            })
            .finally(() => {
                setRefresh((prev) => prev + 1);
            });
    }

    const openDeleteChallengeModal = (challenge?: Challenge) =>
        modals.openConfirmModal({
            centered: true,
            children: (
                <>
                    <Flex gap={10} align={"center"}>
                        <MDIcon>bookmark_remove</MDIcon>
                        <Text fw={600}>删除题目</Text>
                    </Flex>
                    <Divider my={10} />
                    <Text>你确定要删除题目 {challenge?.title} 吗？</Text>
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
                handleDeleteChallenge(challenge);
            },
        });

    useEffect(() => {
        handleGetChallenges();
    }, [search, page, rowsPerPage, sort, refresh]);

    useEffect(() => {
        document.title = `题库管理 - ${configStore?.pltCfg?.site?.title}`;
    }, []);

    return (
        <>
            <Flex my={36} mx={"10%"} justify={"space-between"} gap={36}>
                <Stack w={"15%"} gap={0}>
                    <Flex justify={"space-between"} align={"center"}>
                        <TextInput
                            variant="filled"
                            placeholder="搜索"
                            mr={5}
                            value={searchInput}
                            onChange={(e) => setSearchInput(e.target.value)}
                            flex={1}
                        />
                        <ActionIcon
                            variant={"filled"}
                            onClick={() => setSearch(searchInput)}
                        >
                            <MDIcon size={15} c={"white"}>
                                search
                            </MDIcon>
                        </ActionIcon>
                    </Flex>
                    <Select
                        label="每页显示"
                        description="选择每页显示的题目数量"
                        value={String(rowsPerPage)}
                        allowDeselect={false}
                        data={["15", "25", "50", "100"]}
                        onChange={(_, option) =>
                            setRowsPerPage(Number(option.value))
                        }
                        mt={15}
                    />
                    <Select
                        label="排序"
                        description="选择题目排序方式"
                        value={sort}
                        allowDeselect={false}
                        data={[
                            {
                                label: "ID 升序",
                                value: "id_asc",
                            },
                            {
                                label: "ID 降序",
                                value: "id_desc",
                            },
                            {
                                label: "标题升序",
                                value: "title_asc",
                            },
                            {
                                label: "标题降序",
                                value: "title_desc",
                            },
                        ]}
                        onChange={(_, option) => setSort(option.value)}
                        mt={15}
                    />
                </Stack>
                <Stack
                    w={"85%"}
                    align="center"
                    gap={36}
                    mih={"calc(100vh - 10rem)"}
                >
                    <Paper w={"100%"} shadow={"md"} flex={1} pos={"relative"}>
                        <LoadingOverlay visible={loading} />
                        <Table stickyHeader horizontalSpacing={"md"} striped>
                            <Table.Thead>
                                <Table.Tr
                                    sx={{
                                        lineHeight: 3,
                                    }}
                                >
                                    <Table.Th>
                                        <Flex justify={"start"}>
                                            <Tooltip label="投放到题库">
                                                <MDIcon>
                                                    collections_bookmark
                                                </MDIcon>
                                            </Tooltip>
                                        </Flex>
                                    </Table.Th>
                                    <Table.Th>标题</Table.Th>
                                    <Table.Th>描述</Table.Th>
                                    <Table.Th>
                                        <Flex align={"center"}>
                                            分类
                                            <ActionIcon
                                                onClick={() =>
                                                    navigate(
                                                        "/admin/categories"
                                                    )
                                                }
                                            >
                                                <MDIcon>tune</MDIcon>
                                            </ActionIcon>
                                        </Flex>
                                    </Table.Th>
                                    <Table.Th>
                                        <Flex justify={"center"}>
                                            <ActionIcon onClick={createOpen}>
                                                <MDIcon>add</MDIcon>
                                            </ActionIcon>
                                        </Flex>
                                    </Table.Th>
                                </Table.Tr>
                            </Table.Thead>
                            <Table.Tbody>
                                {challenges?.map((challenge) => (
                                    <Table.Tr key={challenge?.id}>
                                        <Table.Th>
                                            <Group wrap={"nowrap"}>
                                                <Badge>{challenge?.id}</Badge>
                                                <Switch
                                                    size="sm"
                                                    checked={
                                                        challenge?.is_practicable
                                                    }
                                                    onChange={(_) => {
                                                        switchIsPracticable(
                                                            challenge
                                                        );
                                                    }}
                                                />
                                            </Group>
                                        </Table.Th>
                                        <Table.Th maw={100}>
                                            <Text fw={700} size="1rem">
                                                {challenge?.title}
                                            </Text>
                                        </Table.Th>
                                        <Table.Th
                                            maw={200}
                                            sx={{
                                                textWrap: "nowrap",
                                                overflow: "hidden",
                                                textOverflow: "ellipsis",
                                            }}
                                        >
                                            {challenge?.description}
                                        </Table.Th>
                                        <Table.Th>
                                            <Group gap={10} wrap={"nowrap"}>
                                                <MDIcon
                                                    color={
                                                        colorScheme === "dark"
                                                            ? lighten(
                                                                  categoryStore.getCategory(
                                                                      Number(
                                                                          challenge?.category_id
                                                                      )
                                                                  )?.color ||
                                                                      "#3F51B5",
                                                                  0.2
                                                              )
                                                            : categoryStore.getCategory(
                                                                  Number(
                                                                      challenge?.category_id
                                                                  )
                                                              )?.color
                                                    }
                                                >
                                                    {
                                                        categoryStore.getCategory(
                                                            Number(
                                                                challenge?.category_id
                                                            )
                                                        )?.icon
                                                    }
                                                </MDIcon>
                                                <Text
                                                    c={
                                                        colorScheme === "dark"
                                                            ? lighten(
                                                                  categoryStore.getCategory(
                                                                      Number(
                                                                          challenge?.category_id
                                                                      )
                                                                  )?.color ||
                                                                      "#3F51B5",
                                                                  0.1
                                                              )
                                                            : categoryStore.getCategory(
                                                                  Number(
                                                                      challenge?.category_id
                                                                  )
                                                              )?.color
                                                    }
                                                    fw={600}
                                                >
                                                    {categoryStore
                                                        .getCategory(
                                                            Number(
                                                                challenge?.category_id
                                                            )
                                                        )
                                                        ?.name?.toUpperCase()}
                                                </Text>
                                            </Group>
                                        </Table.Th>
                                        <Table.Th>
                                            <Group
                                                justify="center"
                                                wrap={"nowrap"}
                                            >
                                                <ActionIcon
                                                    onClick={() =>
                                                        navigate(
                                                            `/admin/challenges/${challenge?.id}`
                                                        )
                                                    }
                                                >
                                                    <MDIcon>edit</MDIcon>
                                                </ActionIcon>
                                                <ActionIcon
                                                    onClick={() =>
                                                        openDeleteChallengeModal(
                                                            challenge
                                                        )
                                                    }
                                                >
                                                    <MDIcon color={"red"}>
                                                        delete
                                                    </MDIcon>
                                                </ActionIcon>
                                            </Group>
                                        </Table.Th>
                                    </Table.Tr>
                                ))}
                            </Table.Tbody>
                        </Table>
                    </Paper>
                    <Pagination
                        total={Math.max(Math.ceil(total / rowsPerPage), 1)}
                        value={page}
                        onChange={setPage}
                        withEdges
                    />
                </Stack>
            </Flex>
            <ChallengeCreateModal
                opened={createOpened}
                onClose={createClose}
                setRefresh={() => setRefresh((prev) => prev + 1)}
                centered
            />
        </>
    );
}
