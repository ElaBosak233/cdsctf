import { useGameApi } from "@/api/game";
import MDIcon from "@/components/ui/MDIcon";
import GameCard from "@/components/widgets/GameCard";
import { useConfigStore } from "@/stores/config";
import { Game } from "@/types/game";
import {
	Button,
	Flex,
	Pagination,
	Stack,
	TextInput,
	UnstyledButton,
} from "@mantine/core";
import { useEffect, useState } from "react";
import { useNavigate } from "react-router-dom";

export default function Page() {
	const configStore = useConfigStore();
	const gameApi = useGameApi();
	const navigate = useNavigate();

	const [games, setGames] = useState<Array<Game>>([]);
	const [page, setPage] = useState<number>(1);
	const [total, setTotal] = useState<number>(0);

	function getGames() {
		gameApi
			.getGames({
				page: page,
				is_enabled: true,
			})
			.then((res) => {
				const r = res.data;
				setGames(r.data);
				setTotal(r.total);
			});
	}

	useEffect(() => {
		getGames();
	}, [page]);

	useEffect(() => {
		document.title = `比赛 - ${configStore?.pltCfg?.site?.title}`;
	}, []);

	return (
		<>
			<Stack
				my={36}
				mx={"15%"}
				mih={"calc(100vh - 10rem)"}
				align="center"
				gap={36}
			>
				<Flex w={"100%"} gap={20}>
					<TextInput
						variant="filled"
						size="lg"
						w={"90%"}
						placeholder={"搜索比赛"}
					/>
					<Button
						w={"10%"}
						size="lg"
						leftSection={<MDIcon c={"white"}>search</MDIcon>}
					>
						搜索
					</Button>
				</Flex>
				<Stack
					w={"100%"}
					sx={{
						flexGrow: 1,
					}}
				>
					{games.map((game) => (
						<UnstyledButton
							key={game?.id}
							onClick={() => navigate(`/games/${game?.id}`)}
						>
							<GameCard game={game} />
						</UnstyledButton>
					))}
				</Stack>
				<Pagination total={total} value={page} onChange={setPage} />
			</Stack>
		</>
	);
}
