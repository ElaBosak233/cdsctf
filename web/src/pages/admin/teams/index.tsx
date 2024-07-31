import { useTeamApi } from "@/api/team";
import TeamEditModal from "@/components/modals/admin/TeamEditModal";
import TeamCreateModal from "@/components/modals/admin/TeamCreateModal";
import MDIcon from "@/components/ui/MDIcon";
import { useConfigStore } from "@/stores/config";
import { Team } from "@/types/team";
import {
    showErrNotification,
    showSuccessNotification,
} from "@/utils/notification";
import {
    ActionIcon,
    Avatar,
    Badge,
    Divider,
    Flex,
    Group,
    Pagination,
    Paper,
    Select,
    Stack,
    Table,
    Text,
    TextInput,
    Tooltip,
} from "@mantine/core";
import { useDisclosure } from "@mantine/hooks";
import { modals } from "@mantine/modals";
import { useEffect, useState } from "react";

export default function Page() {
    const teamApi = useTeamApi();
    const configStore = useConfigStore();

    const [refresh, setRefresh] = useState<number>(0);

    const [teams, setTeams] = useState<Array<Team>>([]);
    const [page, setPage] = useState<number>(1);
    const [total, setTotal] = useState<number>(0);
    const [rowsPerPage, setRowsPerPage] = useState<number>(20);
    const [search, setSearch] = useState<string>("");
    const [searchInput, setSearchInput] = useState<string>("");
    const [sort, setSort] = useState<string>("id_asc");

    const [createOpened, { open: createOpen, close: createClose }] =
        useDisclosure(false);
    const [editOpened, { open: editOpen, close: editClose }] =
        useDisclosure(false);
    const [selectedTeam, setSelectedTeam] = useState<Team>();

    function getTeams() {
        teamApi
            .getTeams({
                page: page,
                size: rowsPerPage,
                name: search,
                sort_key: sort.split("_")[0],
                sort_order: sort.split("_")[1],
            })
            .then((res) => {
                const r = res.data;
                setTeams(r.data);
                setTotal(r.total);
            });
    }

    function deleteTeam(team?: Team) {
        teamApi
            .deleteTeam({
                id: Number(team?.id),
            })
            .then((_) => {
                showSuccessNotification({
                    message: `团队 ${team?.name} 已被删除`,
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

    const openDeleteTeamModal = (team?: Team) =>
        modals.openConfirmModal({
            centered: true,
            children: (
                <>
                    <Flex gap={10} align={"center"}>
                        <MDIcon>person_remove</MDIcon>
                        <Text fw={600}>删除团队</Text>
                    </Flex>
                    <Divider my={10} />
                    <Text>你确定要删除团队 {team?.name} 吗？</Text>
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
                deleteTeam(team);
            },
        });

    useEffect(() => {
        getTeams();
    }, [search, page, rowsPerPage, sort, refresh]);

    useEffect(() => {
        document.title = `团队管理 - ${configStore?.pltCfg?.site?.title}`;
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
                            sx={{
                                flexGrow: 1,
                            }}
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
                        description="选择每页显示的团队数量"
                        value={String(rowsPerPage)}
                        allowDeselect={false}
                        data={["20", "25", "50", "100"]}
                        onChange={(_, option) =>
                            setRowsPerPage(Number(option.value))
                        }
                        mt={15}
                    />
                    <Select
                        label="排序"
                        description="选择团队排序方式"
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
                                label: "队名升序",
                                value: "name_asc",
                            },
                            {
                                label: "队名降序",
                                value: "name_desc",
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
                    <Paper
                        w={"100%"}
                        sx={{
                            flexGrow: 1,
                        }}
                    >
                        <Table stickyHeader horizontalSpacing={"md"} striped>
                            <Table.Thead>
                                <Table.Tr
                                    sx={{
                                        lineHeight: 3,
                                    }}
                                >
                                    <Table.Th>#</Table.Th>
                                    <Table.Th>队名</Table.Th>
                                    <Table.Th>成员</Table.Th>
                                    <Table.Th>电子邮箱</Table.Th>
                                    <Table.Th>描述</Table.Th>
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
                                {teams?.map((team) => (
                                    <Table.Tr key={team?.id}>
                                        <Table.Th>
                                            <Badge>{team?.id}</Badge>
                                        </Table.Th>
                                        <Table.Th>
                                            <Group gap={15} wrap={"nowrap"}>
                                                <Avatar
                                                    color="brand"
                                                    src={`/api/teams/${team?.id}/avatar`}
                                                    radius="xl"
                                                >
                                                    <MDIcon>people</MDIcon>
                                                </Avatar>
                                                <Text fw={700} size="1rem">
                                                    {team?.name}
                                                </Text>
                                            </Group>
                                        </Table.Th>
                                        <Table.Th>
                                            <Tooltip.Group>
                                                <Avatar.Group spacing="sm">
                                                    {team?.users?.map(
                                                        (user) => (
                                                            <Tooltip
                                                                key={user?.id}
                                                                label={
                                                                    user?.nickname
                                                                }
                                                                withArrow
                                                            >
                                                                <Avatar
                                                                    color="brand"
                                                                    src={`/api/users/${user?.id}/avatar`}
                                                                    radius="xl"
                                                                >
                                                                    <MDIcon>
                                                                        person
                                                                    </MDIcon>
                                                                </Avatar>
                                                            </Tooltip>
                                                        )
                                                    )}
                                                </Avatar.Group>
                                            </Tooltip.Group>
                                        </Table.Th>
                                        <Table.Th>{team?.email}</Table.Th>
                                        <Table.Th>{team?.description}</Table.Th>
                                        <Table.Th>
                                            <Group
                                                justify="center"
                                                wrap={"nowrap"}
                                            >
                                                <ActionIcon
                                                    onClick={() => {
                                                        setSelectedTeam(team);
                                                        editOpen();
                                                    }}
                                                >
                                                    <MDIcon>edit</MDIcon>
                                                </ActionIcon>
                                                <ActionIcon
                                                    onClick={() =>
                                                        openDeleteTeamModal(
                                                            team
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
                        total={Math.ceil(total / rowsPerPage)}
                        value={page}
                        onChange={setPage}
                        withEdges
                    />
                </Stack>
            </Flex>
            <TeamCreateModal
                setRefresh={() => {
                    setRefresh((prev) => prev + 1);
                }}
                opened={createOpened}
                onClose={createClose}
                centered
            />
            <TeamEditModal
                setRefresh={() => {
                    setRefresh((prev) => prev + 1);
                }}
                opened={editOpened}
                onClose={editClose}
                teamID={Number(selectedTeam?.id)}
                centered
            />
        </>
    );
}
