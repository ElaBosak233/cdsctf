import { Box, Button, Flex, Group, Stack, Text } from "@mantine/core";
import MDIcon from "@/components/ui/MDIcon";
import { useLocation, useNavigate, useParams } from "react-router-dom";
import { useGameApi } from "@/api/game";
import { Game } from "@/types/game";
import { useEffect, useState } from "react";

export default function withGame(WrappedComponent: React.ComponentType<any>) {
	return function withGame(props: any) {
		const { id } = useParams<{ id: string }>();
		const gameApi = useGameApi();
		const location = useLocation();
		const path = location.pathname.split(`/games/${id}`)[1];
		const navigate = useNavigate();

		const [game, setGame] = useState<Game>();

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

		return (
			<>
				<Stack m={36}>
					<Flex justify={"space-between"} align={"center"}>
						<Box w={"50%"}></Box>
						<Box
							sx={{
								flexShrink: 0,
							}}
						>
							<Text fw={700} size="1.5rem">
								{game?.title}
							</Text>
						</Box>
						<Flex w={"50%"} justify={"end"}>
							<Group>
								<Button
									size="md"
									leftSection={<MDIcon>flag</MDIcon>}
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
									leftSection={<MDIcon>trending_up</MDIcon>}
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
