import { Box, Button, Flex, Group, Progress, Stack, Text } from "@mantine/core";
import MDIcon from "@/components/ui/MDIcon";
import { useLocation, useNavigate, useParams } from "react-router-dom";
import { useGameApi } from "@/api/game";
import { Game } from "@/types/game";
import { useEffect, useState } from "react";
import { useInterval } from "@mantine/hooks";

export default function withGame(WrappedComponent: React.ComponentType<any>) {
	return function withGame(props: any) {
		const { id } = useParams<{ id: string }>();
		const gameApi = useGameApi();
		const location = useLocation();
		const path = location.pathname.split(`/games/${id}`)[1];
		const navigate = useNavigate();

		const [game, setGame] = useState<Game>();

		const [progress, setProgress] = useState<number>(0);
		const [seconds, setSeconds] = useState(0);
		const interval = useInterval(() => setSeconds((s) => s + 1), 1000);

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
			setProgress(
				((Math.floor(Date.now() / 1000) - Number(game?.started_at)) /
					(Number(game?.ended_at) - Number(game?.started_at))) *
					100
			);
		}, [game, seconds]);

		useEffect(() => {
			interval.start();
			return interval.stop;
		}, []);

		useEffect(() => {
			getGame();
		}, []);

		return (
			<>
				<Stack m={36}>
					<Flex justify={"space-between"} align={"center"}>
						<Box w={"50%"}></Box>
						<Stack
							align={"center"}
							sx={{
								flexShrink: 0,
							}}
						>
							<Text fw={700} size="2rem">
								{game?.title}
							</Text>
							<Progress
								w={"25vw"}
								radius={"xl"}
								size={"md"}
								value={progress}
								animated
								transitionDuration={200}
							/>
						</Stack>
						<Flex w={"50%"} justify={"end"}>
							<Group wrap={"nowrap"}>
								<Button
									size="md"
									leftSection={
										<MDIcon
											c={
												path === "/challenges"
													? "white"
													: "brand"
											}
										>
											flag
										</MDIcon>
									}
									variant={
										path === "/challenges"
											? "filled"
											: "outline"
									}
									onClick={() =>
										navigate(`/games/${id}/challenges`)
									}
								>
									题目
								</Button>
								<Button
									size="md"
									leftSection={
										<MDIcon
											c={
												path === "/scoreboard"
													? "white"
													: "brand"
											}
										>
											trending_up
										</MDIcon>
									}
									variant={
										path === "/scoreboard"
											? "filled"
											: "outline"
									}
									onClick={() =>
										navigate(`/games/${id}/scoreboard`)
									}
								>
									积分榜
								</Button>
							</Group>
						</Flex>
					</Flex>
					<WrappedComponent {...props} />
				</Stack>
			</>
		);
	};
}
