import { useTeamApi } from "@/api/team";
import MDIcon from "@/components/ui/MDIcon";
import { Team } from "@/types/team";
import {
	Card,
	Divider,
	Flex,
	Modal,
	ModalProps,
	TextInput,
	ThemeIcon,
	Text,
	Stack,
	Avatar,
	Group,
	ActionIcon,
	Pagination,
} from "@mantine/core";
import { useEffect, useState } from "react";

interface TeamSelectModalProps extends ModalProps {
	setTeam: (team: Team) => void;
}

export default function TeamSelectModal(props: TeamSelectModalProps) {
	const { setTeam, ...modalProps } = props;

	const teamApi = useTeamApi();
	const [teams, setTeams] = useState<Array<Team>>([]);
	const [search, setSearch] = useState<string>("");
	const [page, setPage] = useState<number>(1);
	const [total, setTotal] = useState<number>(0);
	const [rowsPerPage, _] = useState<number>(10);

	function getTeams() {
		teamApi
			.getTeams({
				size: 10,
				page: page,
				name: search,
			})
			.then((res) => {
				const r = res.data;
				setTeams(r.data);
				setTotal(r.total);
			});
	}

	useEffect(() => {
		getTeams();
	}, [search, page]);

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
							<ThemeIcon variant="transparent">
								<MDIcon>people</MDIcon>
							</ThemeIcon>
							<Text fw={600}>选择团队</Text>
						</Flex>
						<Divider my={10} />
						<Stack p={10} gap={20} align="center">
							<TextInput
								label="搜索"
								value={search}
								onChange={(e) => setSearch(e.target.value)}
								w={"100%"}
							/>
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
											variant="transparent"
											onClick={() => {
												setTeam(team);
												modalProps.onClose();
											}}
										>
											<MDIcon>check</MDIcon>
										</ActionIcon>
									</Flex>
								))}
							</Stack>
							<Pagination
								total={Math.ceil(total / rowsPerPage)}
								value={page}
								onChange={setPage}
							/>
						</Stack>
					</Card>
				</Modal.Content>
			</Modal.Root>
		</>
	);
}
