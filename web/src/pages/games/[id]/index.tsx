import { useGameApi } from "@/api/game";
import MDIcon from "@/components/ui/MDIcon";
import MarkdownRender from "@/components/utils/MarkdownRender";
import { useConfigStore } from "@/stores/config";
import { Game } from "@/types/game";
import {
	Text,
	Box,
	Flex,
	Paper,
	BackgroundImage,
	Stack,
	Group,
	Badge,
	Progress,
	Button,
} from "@mantine/core";
import { useEffect, useState } from "react";
import { useNavigate, useParams } from "react-router-dom";
import dayjs from "dayjs";
import { GameTeam } from "@/types/game_team";
import { useTeamStore } from "@/stores/team";
import { useAuthStore } from "@/stores/auth";
import { showErrNotification } from "@/utils/notification";
import { useDisclosure } from "@mantine/hooks";
import GameTeamApplyModal from "@/components/modals/GameTeamApplyModal";

export default function Page() {
	const { id } = useParams<{ id: string }>();
	const gameApi = useGameApi();
	const configStore = useConfigStore();
	const navigate = useNavigate();

	const teamStore = useTeamStore();
	const authStore = useAuthStore();

	const [game, setGame] = useState<Game>();
	const [gameTeams, setGameTeams] = useState<Array<GameTeam>>([]);

	const startedAt = dayjs(Number(game?.started_at) * 1000).format(
		"YYYY/MM/DD HH:mm:ss"
	);
	const endedAt = dayjs(Number(game?.ended_at) * 1000).format(
		"YYYY/MM/DD HH:mm:ss"
	);

	const progress =
		((Math.floor(Date.now() / 1000) - Number(game?.started_at)) /
			(Number(game?.ended_at) - Number(game?.started_at))) *
		100;

	const [applyOpened, { open: applyOpen, close: applyClose }] =
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

	function getGameTeams() {
		gameApi
			.getGameTeams({
				game_id: Number(id),
			})
			.then((res) => {
				const r = res.data;
				setGameTeams(r.data);
			});
	}

	function enter() {
		for (const gameTeam of gameTeams) {
			if (gameTeam?.is_allowed) {
				for (const user of gameTeam?.team?.users || []) {
					if (user?.id === authStore?.user?.id) {
						teamStore.setSelectedTeamID(Number(gameTeam?.team?.id));
						navigate(`/games/${game?.id}/challenges`);
						return;
					}
				}
			}
		}
		showErrNotification({
			message: "你没有加入任何一个可参赛的队伍",
		});
	}

	useEffect(() => {
		getGame();
		getGameTeams();
	}, []);

	useEffect(() => {
		document.title = `${game?.title} - ${configStore?.pltCfg?.site?.title}`;
	}, [game]);

	return (
		<>
			<Box>
				<Paper mih={"22rem"} px={"25%"} py={"5rem"} shadow="md">
					<Flex justify={"space-between"} gap={50}>
						<Stack w={"55%"} justify={"space-between"}>
							<Stack>
								<Text fw={700} size="2.5rem">
									{game?.title}
								</Text>
								<Text>{game?.bio}</Text>
							</Stack>
							<Stack my={20}>
								<Group gap={5}>
									<Badge>{startedAt}</Badge>
									<MDIcon>arrow_right_alt</MDIcon>
									<Badge>{endedAt}</Badge>
								</Group>
								<Progress
									value={progress}
									animated
									w={"100%"}
								/>
								<Group gap={20}>
									<Button
										onClick={() =>
											navigate(
												`/games/${game?.id}/scoreboard`
											)
										}
									>
										查看榜单
									</Button>
									<Button onClick={() => applyOpen()}>
										报名参赛
									</Button>
									<Button onClick={() => enter()}>
										进入比赛
									</Button>
								</Group>
							</Stack>
						</Stack>
						<BackgroundImage
							src={`${import.meta.env.VITE_BASE_API}/media/games/${game?.id}/poster/${game?.poster?.name}`}
							radius={"md"}
							h={250}
							w={"45%"}
						/>
					</Flex>
				</Paper>
				<Box mx={"25%"} my={50}>
					<MarkdownRender src={game?.description || ""} />
				</Box>
			</Box>
			<GameTeamApplyModal
				centered
				opened={applyOpened}
				onClose={applyClose}
			/>
		</>
	);
}
