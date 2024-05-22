import { useGameApi } from "@/api/game";
import withGameEdit from "@/components/layouts/admin/withGameEdit";
import GameTeamCreateModal from "@/components/modals/admin/GameTeamCreateModal";
import MDIcon from "@/components/ui/MDIcon";
import { Game } from "@/types/game";
import { GameTeam } from "@/types/game_team";
import { Team } from "@/types/team";
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
	Accordion,
	Center,
	Avatar,
	Box,
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
	const [gameTeams, setGameTeams] = useState<Array<GameTeam>>([]);
	const [rowsPerPage, setRowsPerPage] = useState<number>(10);
	const [page, setPage] = useState<number>(1);

	const [displayedGameTeams, setDisplayedGameTeams] = useState<
		Array<GameTeam>
	>([]);

	const [createOpened, { open: createOpen, close: createClose }] =
		useDisclosure(false);

	const [total, setTotal] = useState<number>(0);

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

	function getGameTeams() {
		setLoading(true);
		gameApi
			.getGameTeams({
				game_id: Number(id),
			})
			.then((res) => {
				const r = res.data;
				setGameTeams(r.data);
				setTotal(r.total);
			})
			.finally(() => {
				setLoading(false);
			});
	}

	function switchIsAllowed(gameTeam?: GameTeam) {
		gameApi
			.updateGameTeam({
				game_id: Number(id),
				team_id: gameTeam?.team_id,
				is_allowed: !gameTeam?.is_allowed,
			})
			.then((_) => {
				showSuccessNotification({
					message: !gameTeam?.is_allowed
						? `已允许队伍 ${gameTeam?.team?.name} 参赛`
						: `已禁止队伍 ${gameTeam?.team?.name} 参赛`,
				});
				setGameTeams(
					gameTeams?.map((gt) =>
						gt.id === gameTeam?.id
							? {
									...gt,
									is_allowed: !gameTeam?.is_allowed,
								}
							: gt
					)
				);
			});
	}

	useEffect(() => {
		getGame();
	}, [refresh]);

	useEffect(() => {
		if (gameTeams) {
			setDisplayedGameTeams(
				gameTeams.slice((page - 1) * rowsPerPage, page * rowsPerPage)
			);
		}
	}, [page, gameTeams]);

	useEffect(() => {
		if (game) {
			getGameTeams();
		}
	}, [game]);

	useEffect(() => {
		document.title = `团队管理 - ${game?.title}`;
	}, [game]);

	return (
		<>
			<Stack m={36}>
				<Stack gap={10}>
					<Flex justify={"space-between"} align={"center"}>
						<Group>
							<ThemeIcon variant="transparent">
								<MDIcon>people</MDIcon>
							</ThemeIcon>
							<Text fw={700} size="xl">
								参赛团队
							</Text>
						</Group>
						<Tooltip label="添加团队" withArrow>
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
								<Table.Th>团队名</Table.Th>
								<Table.Th>成员</Table.Th>
								<Table.Th>邮箱</Table.Th>
								<Table.Th />
							</Table.Tr>
						</Table.Thead>
						<Table.Tbody>
							{displayedGameTeams?.map((gameTeam) => (
								<Table.Tr key={gameTeam?.id}>
									<Table.Td>
										<Group>
											<Switch
												checked={gameTeam?.is_allowed}
												onChange={() =>
													switchIsAllowed(gameTeam)
												}
											/>
											<Badge>{gameTeam?.team_id}</Badge>
										</Group>
									</Table.Td>
									<Table.Td>
										<Group gap={20} align="center">
											<Avatar
												color="brand"
												src={`${import.meta.env.VITE_BASE_API}/media/teams/${gameTeam?.team_id}/${gameTeam?.team?.avatar?.name}`}
											>
												<MDIcon>people</MDIcon>
											</Avatar>
											<Text fw={700} size="1rem">
												{gameTeam?.team?.name}
											</Text>
										</Group>
									</Table.Td>
									<Table.Td>
										<Avatar.Group spacing="sm">
											{gameTeam?.team?.users?.map(
												(user) => (
													<Tooltip
														key={user?.id}
														label={user?.nickname}
														withArrow
													>
														<Avatar
															color="brand"
															src={`${import.meta.env.VITE_BASE_API}/media/users/${user?.id}/${user?.avatar?.name}`}
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
									</Table.Td>
									<Table.Td>{gameTeam?.team?.email}</Table.Td>
									<Table.Td>
										<Tooltip label="移除团队" withArrow>
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
			<GameTeamCreateModal
				opened={createOpened}
				onClose={createClose}
				setRefresh={() => setRefresh((prev) => prev + 1)}
				centered
			/>
		</>
	);
}

export default withGameEdit(Page);
