import { useTeamApi } from "@/api/team";
import TeamCreateModal from "@/components/modals/TeamCreateModal";
import TeamEditModal from "@/components/modals/TeamEditModal";
import TeamJoinModal from "@/components/modals/TeamJoinModal";
import MDIcon from "@/components/ui/MDIcon";
import TeamCard from "@/components/widgets/TeamCard";
import { useAuthStore } from "@/stores/auth";
import { useConfigStore } from "@/stores/config";
import { Team } from "@/types/team";
import { Button, Flex, Group, Stack, UnstyledButton } from "@mantine/core";
import { useDisclosure } from "@mantine/hooks";
import { useEffect, useState } from "react";

export default function Page() {
	const configStore = useConfigStore();
	const authStore = useAuthStore();
	const teamApi = useTeamApi();

	const [refresh, setRefresh] = useState<number>(0);

	const [teams, setTeams] = useState<Array<Team>>([]);

	const [createOpened, { open: createOpen, close: createClose }] =
		useDisclosure(false);

	const [editOpened, { open: editOpen, close: editClose }] =
		useDisclosure(false);
	const [editTeam, setEditTeam] = useState<Team>();

	const [joinOpened, { open: joinOpen, close: joinClose }] =
		useDisclosure(false);

	function getTeams() {
		teamApi
			.getTeams({
				user_id: authStore?.user?.id,
			})
			.then((res) => {
				const r = res.data;
				setTeams(r.data);
			});
	}

	useEffect(() => {
		getTeams();
	}, [refresh]);

	useEffect(() => {
		document.title = `团队 - ${configStore?.pltCfg?.site?.title}`;
	}, []);

	return (
		<>
			<Stack mx={150} my={36}>
				<Flex justify={"end"} gap={15}>
					<Button
						size="lg"
						leftSection={<MDIcon>waving_hand</MDIcon>}
						onClick={joinOpen}
					>
						加入团队
					</Button>
					<Button
						size="lg"
						leftSection={<MDIcon>group_add</MDIcon>}
						onClick={createOpen}
					>
						创建团队
					</Button>
				</Flex>
				<Group gap={20} my={20}>
					{teams?.map((team) => (
						<UnstyledButton
							key={team?.id}
							onClick={() => {
								editOpen();
								setEditTeam(team);
							}}
						>
							<TeamCard team={team} />
						</UnstyledButton>
					))}
				</Group>
			</Stack>
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
				team={editTeam}
				opened={editOpened}
				onClose={editClose}
				centered
			/>
			<TeamJoinModal
				setRefresh={() => {
					setRefresh((prev) => prev + 1);
				}}
				opened={joinOpened}
				onClose={joinClose}
				centered
			/>
		</>
	);
}
