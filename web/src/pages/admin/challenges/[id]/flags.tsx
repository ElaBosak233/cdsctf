import { useChallengeApi } from "@/api/challenge";
import withChallengeEdit from "@/components/layouts/admin/withChallengeEdit";
import ChallengeFlagCreateModal from "@/components/modals/admin/ChallengeFlagCreateModal";
import MDIcon from "@/components/ui/MDIcon";
import ChallengeFlagAccordion from "@/components/widgets/admin/ChallengeFlagAccordion";
import { Challenge } from "@/types/challenge";
import {
	Accordion,
	Button,
	Flex,
	Group,
	Stack,
	ThemeIcon,
	Text,
	Divider,
	ActionIcon,
	Tooltip,
} from "@mantine/core";
import { useDisclosure } from "@mantine/hooks";
import { useEffect, useState } from "react";
import { useParams } from "react-router-dom";

function Page() {
	const { id } = useParams<{ id: string }>();
	const challengeApi = useChallengeApi();

	const [refresh, setRefresh] = useState<number>(0);
	const [challenge, setChallenge] = useState<Challenge>();

	const [createOpened, { open: createOpen, close: createClose }] =
		useDisclosure(false);

	function getChallenge() {
		challengeApi
			.getChallenges({
				id: Number(id),
				is_detailed: true,
			})
			.then((res) => {
				const r = res.data;
				setChallenge(r.data[0]);
			});
	}

	useEffect(() => {
		getChallenge();
	}, [refresh]);

	useEffect(() => {
		document.title = `Flags - ${challenge?.title}`;
	}, [challenge]);

	return (
		<>
			<Stack m={36}>
				<Stack gap={10}>
					<Flex justify={"space-between"} align={"center"}>
						<Group>
							<MDIcon>flag</MDIcon>
							<Text fw={700} size="xl">
								Flags
							</Text>
						</Group>
						<Tooltip label="创建 Flag" withArrow>
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
				<Stack mx={20}>
					<Accordion variant="separated">
						{challenge?.flags?.map((flag) => (
							<ChallengeFlagAccordion
								key={flag?.id}
								flag={flag}
								setRefresh={() => {
									setRefresh((prev) => prev + 1);
								}}
							/>
						))}
					</Accordion>
				</Stack>
			</Stack>
			<ChallengeFlagCreateModal
				centered
				opened={createOpened}
				onClose={createClose}
				setRefresh={() => {
					setRefresh((prev) => prev + 1);
				}}
			/>
		</>
	);
}

export default withChallengeEdit(Page);
