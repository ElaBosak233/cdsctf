import { useGameApi } from "@/api/game";
import withGameEdit from "@/components/layouts/admin/withGameEdit";
import GameTeamCreateModal from "@/components/modals/admin/GameTeamCreateModal";
import MDIcon from "@/components/ui/MDIcon";
import { Game } from "@/types/game";
import { GameTeam } from "@/types/game_team";
import { Notice } from "@/types/notice";
import { showSuccessNotification } from "@/utils/notification";
import {
	ActionIcon,
	Divider,
	Flex,
	Group,
	Stack,
	ThemeIcon,
	Tooltip,
	Text,
	Avatar,
	Badge,
	Switch,
	Pagination,
	LoadingOverlay,
	Table,
} from "@mantine/core";
import { useDisclosure } from "@mantine/hooks";
import { useEffect, useState } from "react";
import { useParams } from "react-router-dom";

function Page() {
	const gameApi = useGameApi();
	const { id } = useParams<{ id: string }>();

	const [loading, setLoading] = useState<boolean>(true);

	const [game, setGame] = useState<Game>();
	const [refresh, setRefresh] = useState<number>(0);
	const [notices, setNotices] = useState<Array<Notice>>();
	const [rowsPerPage, setRowsPerPage] = useState<number>(10);
	const [page, setPage] = useState<number>(1);

	const [displayedNotices, setDisplayedNotices] = useState<Array<Notice>>([]);

	const [createOpened, { open: createOpen, close: createClose }] =
		useDisclosure(false);

	const [total, setTotal] = useState<number>(0);

	const typeMap = new Map([
		["new_challenge", "新题目"],
		["first_blood", "一血"],
		["second_blood", "二血"],
		["third_blood", "三血"],
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

	function getGameNotices() {
		setLoading(true);
		gameApi
			.getGameNotices({
				game_id: Number(id),
			})
			.then((res) => {
				const r = res.data;
				setNotices(r.data);
				setTotal(r.total);
			})
			.finally(() => {
				setLoading(false);
			});
	}

	useEffect(() => {
		getGame();
	}, [refresh]);

	useEffect(() => {
		if (notices) {
			setDisplayedNotices(
				notices.slice((page - 1) * rowsPerPage, page * rowsPerPage)
			);
		}
	}, [page, notices]);

	useEffect(() => {
		if (game) {
			getGameNotices();
		}
	}, [game]);

	useEffect(() => {
		document.title = `公告管理 - ${game?.title}`;
	}, [game]);

	return (
		<>
			<Stack m={36}>
				<Stack gap={10}>
					<Flex justify={"space-between"} align={"center"}>
						<Group>
							<ThemeIcon variant="transparent">
								<MDIcon>campaign</MDIcon>
							</ThemeIcon>
							<Text fw={700} size="xl">
								公告
							</Text>
						</Group>
						<Tooltip label="添加公告" withArrow>
							<ActionIcon
								variant="transparent"
								onClick={() => createOpen()}
							>
								<MDIcon>add</MDIcon>
							</ActionIcon>
						</Tooltip>
					</Flex>
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
								<Table.Th>内容</Table.Th>
								<Table.Th>类型</Table.Th>
								<Table.Th>相关题目</Table.Th>
								<Table.Th>相关团队</Table.Th>
								<Table.Th />
							</Table.Tr>
						</Table.Thead>
						<Table.Tbody>
							{displayedNotices?.map((notice) => (
								<Table.Tr key={notice?.id}>
									<Table.Td>
										<Badge>{notice?.id}</Badge>
									</Table.Td>
									<Table.Td>{notice?.content}</Table.Td>
									<Table.Td>
										{typeMap.get(String(notice?.type))}
									</Table.Td>
									<Table.Td>
										{notice?.challenge?.title}
									</Table.Td>
									<Table.Td>{notice?.team?.name}</Table.Td>
									<Table.Td>
										<Tooltip label="删除公告" withArrow>
											<ActionIcon
												variant="transparent"
												color="red"
											>
												<MDIcon>delete</MDIcon>
											</ActionIcon>
										</Tooltip>
									</Table.Td>
								</Table.Tr>
							))}
						</Table.Tbody>
					</Table>
				</Stack>
				<Flex justify={"center"}>
					<Pagination
						total={Math.ceil(total / rowsPerPage)}
						value={page}
						onChange={setPage}
						withEdges
					/>
				</Flex>
			</Stack>
		</>
	);
}

export default withGameEdit(Page);
