import { useTeamApi } from "@/api/team";
import { Team } from "@/types/team";
import {
	ActionIcon,
	Avatar,
	Card,
	Divider,
	Flex,
	Group,
	Modal,
	ModalProps,
	Stack,
	Text,
} from "@mantine/core";
import { useState, useEffect } from "react";
import MDIcon from "@/components/ui/MDIcon";
import { useGameApi } from "@/api/game";
import { useParams } from "react-router-dom";
import {
	showErrNotification,
	showSuccessNotification,
} from "@/utils/notification";
import { useAuthStore } from "@/stores/auth";
import { GameTeam } from "@/types/game_team";

interface GameTeamApplyModalProps extends ModalProps {}

export default function GameTeamApplyModal(props: GameTeamApplyModalProps) {
	const { ...modalProps } = props;

	const teamApi = useTeamApi();
	const gameApi = useGameApi();
	const authStore = useAuthStore();

	const { id } = useParams<{ id: string }>();

	const [teams, setTeams] = useState<Array<Team>>([]);
	const [gameTeams, setGameTeams] = useState<Array<GameTeam>>([]);

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

	function getTeams() {
		teamApi
			.getTeams({
				user_id: authStore?.user?.id,
			})
			.then((res) => {
				const r = res.data;
				const t = r.data as Array<Team>;
				t?.map((team) => {
					// 判断是否是队长且未加入比赛
					if (
						team?.captain_id === authStore?.user?.id &&
						!gameTeams?.find(
							(gameTeam) => gameTeam.team_id === team.id
						)
					) {
						setTeams([...teams, team]);
					}
				});
			});
	}

	function createGameTeam(team?: Team) {
		gameApi
			.createGameTeam({
				game_id: Number(id),
				team_id: team?.id,
			})
			.then((_) => {
				showSuccessNotification({
					message: "已递交申请",
				});
			})
			.catch((e) => {
				showErrNotification({
					message: e.response.data.msg || "申请失败",
				});
			})
			.finally(() => {
				modalProps.onClose();
			});
	}

	useEffect(() => {
		if (gameTeams) {
			getTeams();
		}
	}, [gameTeams]);

	useEffect(() => {
		if (modalProps.opened) {
			setTeams([]);
			getGameTeams();
		}
	}, [modalProps.opened]);

	return (
		<>
			<Modal.Root {...modalProps}>
				<Modal.Overlay />
				<Modal.Content
					sx={{
						flex: "none",
						backgroundColor: "transparent",
					}}
				>
					<Card
						shadow="md"
						padding={"lg"}
						radius={"md"}
						withBorder
						w={"40rem"}
					>
						<Flex gap={10} align={"center"}>
							<MDIcon>people</MDIcon>
							<Text fw={600}>选择团队</Text>
						</Flex>
						<Divider my={10} />
						<Stack p={10} gap={20} align="center">
							<Stack w={"100%"}>
								{teams?.map((team) => (
									<Flex
										key={team?.id}
										justify={"space-between"}
										align={"center"}
									>
										<Group gap={15}>
											<Avatar
												color="brand"
												src={`${import.meta.env.VITE_BASE_API}/media/teams/${team?.id}/${team?.avatar?.name}`}
												radius="xl"
											>
												<MDIcon>person</MDIcon>
											</Avatar>
											<Text fw={700} size="1rem">
												{team?.name}
											</Text>
										</Group>
										<ActionIcon
											onClick={() => {
												createGameTeam(team);
											}}
										>
											<MDIcon>check</MDIcon>
										</ActionIcon>
									</Flex>
								))}
							</Stack>
						</Stack>
					</Card>
				</Modal.Content>
			</Modal.Root>
		</>
	);
}
