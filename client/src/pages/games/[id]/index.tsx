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
	ThemeIcon,
	Progress,
	Button,
} from "@mantine/core";
import { useEffect, useState } from "react";
import { useParams } from "react-router-dom";
import dayjs from "dayjs";

export default function Page() {
	const { id } = useParams<{ id: string }>();
	const gameApi = useGameApi();
	const configStore = useConfigStore();

	const [game, setGame] = useState<Game>();

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

	useEffect(() => {
		getGame();
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
									<ThemeIcon variant="transparent">
										<MDIcon>arrow_right_alt</MDIcon>
									</ThemeIcon>
									<Badge>{endedAt}</Badge>
								</Group>
								<Progress
									value={progress}
									animated
									w={"100%"}
								/>
								<Group gap={20}>
									<Button>查看榜单</Button>
									<Button>报名参赛</Button>
									<Button>进入比赛</Button>
								</Group>
							</Stack>
						</Stack>
						<BackgroundImage
							src={
								"https://raw.githubusercontent.com/mantinedev/mantine/master/.demo/images/bg-6.png"
							}
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
		</>
	);
}
