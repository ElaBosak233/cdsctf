import { useGameApi } from "@/api/game";
import { useSubmissionApi } from "@/api/submission";
import withGame from "@/components/layouts/withGame";
import ChallengeModal from "@/components/modals/ChallengeModal";
import MDIcon from "@/components/ui/MDIcon";
import ChallengeCard from "@/components/widgets/ChallengeCard";
import GameNoticeArea from "@/components/widgets/GameNoticeArea";
import { useConfigStore } from "@/stores/config";
import { useTeamStore } from "@/stores/team";
import { Category } from "@/types/category";
import { Game } from "@/types/game";
import { GameChallenge } from "@/types/game_challenge";
import { GameTeam } from "@/types/game_team";
import { Submission } from "@/types/submission";
import { calculateAndSort } from "@/utils/game";
import { showErrNotification } from "@/utils/notification";
import {
	Avatar,
	Box,
	Button,
	Card,
	Divider,
	Flex,
	Group,
	LoadingOverlay,
	ScrollArea,
	Stack,
	Text,
	UnstyledButton,
} from "@mantine/core";
import { useDisclosure } from "@mantine/hooks";
import { useEffect, useState } from "react";
import { useNavigate, useParams } from "react-router-dom";

function Page() {
	const { id } = useParams<{ id: string }>();
	const gameApi = useGameApi();
	const submissionApi = useSubmissionApi();
	const configStore = useConfigStore();
	const teamStore = useTeamStore();

	const navigate = useNavigate();

	const [game, setGame] = useState<Game>();
	const [gameChallenges, setGameChallenges] = useState<Array<GameChallenge>>(
		[]
	);
	const [categories, setCategories] = useState<Record<number, Category>>({});
	const [selectedGameChallenges, setSelectedGameChallenges] = useState<
		Array<GameChallenge>
	>([]);
	const [selectedCategory, setSelectedCategory] = useState<number>(0);
	const [gameTeam, setGameTeam] = useState<GameTeam>();
	const [submissions, setSubmissions] = useState<Array<Submission>>([]);

	const [loadingTeamStatus, setLoadingTeamStatus] = useState<boolean>(false);

	const [opened, { open, close }] = useDisclosure(false);
	const [selectedChallenge, setSelectedChallenge] = useState<GameChallenge>();

	const [refresh, setRefresh] = useState<number>(0);

	const [score, setScore] = useState<number>(0);
	const [rank, setRank] = useState<number>(0);
	const [solves, setSolves] = useState<number>(0);

	function getSubmissions() {
		setLoadingTeamStatus(true);
		submissionApi
			.getSubmissions({
				game_id: Number(id),
				status: 2,
				is_detailed: false,
			})
			.then((res) => {
				const r = res.data;
				setSubmissions(r.data);
			})
			.finally(() => {
				setLoadingTeamStatus(false);
			});
	}

	function getGame() {
		gameApi
			.getGames({
				id: Number(id),
			})
			.then((res) => {
				const r = res.data;
				setGame(r.data?.[0]);
			});
	}

	function getGameChallenges() {
		gameApi
			.getGameChallenges({
				game_id: Number(id),
				team_id: teamStore?.selectedTeamID,
				is_enabled: true,
			})
			.then((res) => {
				const r = res.data;
				setGameChallenges(r.data);
			});
	}

	function getTeam() {
		gameApi
			.getGameTeams({
				game_id: Number(id),
				team_id: teamStore?.selectedTeamID,
			})
			.then((res) => {
				const r = res.data;
				if (!r.data) {
					showErrNotification({
						title: "获取队伍信息失败",
						message: "请检查是否已加入可参赛的队伍",
					});
					navigate(`/games/${id}`);
				}
				setGameTeam(r.data[0]);
			});
	}

	useEffect(() => {
		if (gameTeam) {
			const rows = calculateAndSort(submissions);
			console.log(rows);
			if (rows) {
				rows?.forEach((row) => {
					if (row?.team?.id === gameTeam?.team_id) {
						setScore(row.totalScore);
						setRank(row.rank as number);
						setSolves(row.solvedCount);
					}
				});
			}
		}
	}, [submissions, gameTeam]);

	useEffect(() => {
		if (selectedCategory != 0) {
			setSelectedGameChallenges(
				gameChallenges.filter((gameChallenge) => {
					return (
						gameChallenge?.challenge?.category_id ===
						selectedCategory
					);
				})
			);
		} else {
			setSelectedGameChallenges(gameChallenges);
		}
	}, [gameChallenges, selectedCategory]);

	useEffect(() => {
		if (gameChallenges) {
			gameChallenges.forEach((gameChallenge) => {
				if (
					!(categories as Record<number, Category>)[
						gameChallenge?.challenge?.category_id as number
					]
				) {
					setCategories((categories) => {
						return {
							...categories,
							[gameChallenge?.challenge?.category_id as number]:
								gameChallenge?.challenge?.category as Category,
						};
					});
				}
			});
		}
	}, [gameChallenges]);

	useEffect(() => {
		getGame();
		getTeam();
		getSubmissions();
	}, [teamStore?.selectedTeamID]);

	useEffect(() => {
		getGameChallenges();
	}, [refresh]);

	useEffect(() => {
		document.title = `${game?.title} - ${configStore?.pltCfg?.site?.title}`;
	}, [game]);

	return (
		<>
			<Stack my={10} mx={"2%"}>
				<Flex justify={"space-between"}>
					<Stack mx={10} miw={200} maw={200}>
						<Button
							size="lg"
							leftSection={<MDIcon c={"white"}>upload</MDIcon>}
						>
							上传题解
						</Button>
						<Divider />
						<Stack gap={10}>
							<Button
								variant={
									selectedCategory === 0 ? "filled" : "subtle"
								}
								size="lg"
								color="brand"
								leftSection={
									<MDIcon
										c={
											selectedCategory === 0
												? "white"
												: "brand"
										}
									>
										extension
									</MDIcon>
								}
								onClick={() => {
									setSelectedCategory(0);
								}}
							>
								All
							</Button>
							{Object.entries(categories)?.map(
								([_, category]) => (
									<Button
										key={category?.id}
										variant={
											selectedCategory === category?.id
												? "filled"
												: "subtle"
										}
										color={category?.color || "brand"}
										size="lg"
										leftSection={
											<MDIcon
												c={
													selectedCategory ===
													category?.id
														? "white"
														: category?.color ||
															"brand"
												}
											>
												{category?.icon}
											</MDIcon>
										}
										onClick={() => {
											setSelectedCategory(
												category?.id as number
											);
										}}
									>
										{category?.name?.toUpperCase()}
									</Button>
								)
							)}
						</Stack>
					</Stack>
					<Box mx={20} w={"100%"}>
						<ScrollArea h={"calc(100vh - 250px)"}>
							<Group gap={"lg"} justify={"start"}>
								{selectedGameChallenges?.map(
									(gameChallenge) => (
										<UnstyledButton
											onClick={() => {
												open();
												setSelectedChallenge(
													gameChallenge
												);
											}}
											key={gameChallenge?.id}
										>
											<ChallengeCard
												challenge={
													gameChallenge?.challenge
												}
												pts={gameChallenge?.pts}
											/>
										</UnstyledButton>
									)
								)}
							</Group>
						</ScrollArea>
					</Box>
					<Stack miw={330} maw={330} mx={10}>
						<Card mih={200} shadow="md" p={25} pos={"relative"}>
							<LoadingOverlay
								visible={loadingTeamStatus}
								zIndex={2}
							/>
							<Group gap={20}>
								<Avatar
									color="brand"
									size={72}
									src={`${import.meta.env.VITE_BASE_API}/media/teams/${gameTeam?.team_id}/${gameTeam?.team?.avatar?.name}`}
								>
									<MDIcon size={36}>people</MDIcon>
								</Avatar>
								<Text fw={700} size="1rem">
									{gameTeam?.team?.name}
								</Text>
							</Group>
							<Flex justify={"space-between"} mt={20} mx={36}>
								<Stack align={"center"}>
									<Text fw={700} size="1.2rem">
										{score || 0}
									</Text>
									<Text fw={700}>得分</Text>
								</Stack>
								<Stack align={"center"}>
									<Text fw={700} size="1.2rem">
										{rank > 0 ? rank : "无排名"}
									</Text>
									<Text fw={700}>排名</Text>
								</Stack>
								<Stack align={"center"}>
									<Text fw={700} size="1.2rem">
										{solves || 0}
									</Text>
									<Text fw={700}>已解决</Text>
								</Stack>
							</Flex>
						</Card>
						<Card h={"calc(100vh - 450px)"} shadow="md">
							<GameNoticeArea />
						</Card>
					</Stack>
				</Flex>
			</Stack>
			<ChallengeModal
				opened={opened}
				onClose={close}
				centered
				setRefresh={() => setRefresh((prev) => prev + 1)}
				challenge={selectedChallenge?.challenge}
				gameID={selectedChallenge?.game_id}
				mode="game"
			/>
		</>
	);
}

export default withGame(Page);
