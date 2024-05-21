import { useChallengeApi } from "@/api/challenge";
import MDIcon from "@/components/ui/MDIcon";
import { Challenge } from "@/types/challenge";
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
	Group,
	ActionIcon,
	Pagination,
	Badge,
} from "@mantine/core";
import { useEffect, useState } from "react";

interface ChallengeSelectModalProps extends ModalProps {
	setChallenge: (challenge: Challenge) => void;
}

export default function ChallengeSelectModal(props: ChallengeSelectModalProps) {
	const { setChallenge, ...modalProps } = props;

	const challengeApi = useChallengeApi();
	const [challenges, setChallenges] = useState<Array<Challenge>>([]);
	const [search, setSearch] = useState<string>("");
	const [page, setPage] = useState<number>(1);
	const [total, setTotal] = useState<number>(0);
	const [rowsPerPage, _] = useState<number>(10);

	function getChallenges() {
		challengeApi
			.getChallenges({
				size: 10,
				page: page,
				title: search,
			})
			.then((res) => {
				const r = res.data;
				setChallenges(r.data);
				setTotal(r.total);
			});
	}

	useEffect(() => {
		getChallenges();
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
								<MDIcon>collections_bookmark</MDIcon>
							</ThemeIcon>
							<Text fw={600}>选择题目</Text>
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
								{challenges?.map((challenge) => (
									<Flex
										key={challenge?.id}
										justify={"space-between"}
										align={"center"}
									>
										<Group gap={15}>
											<Badge>{challenge?.id}</Badge>
											<ThemeIcon
												variant="transparent"
												color={
													challenge?.category?.color
												}
											>
												<MDIcon>
													{challenge?.category?.icon}
												</MDIcon>
											</ThemeIcon>
											<Text fw={700} size="1rem">
												{challenge?.title}
											</Text>
										</Group>
										<ActionIcon
											variant="transparent"
											onClick={() => {
												setChallenge(challenge);
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
