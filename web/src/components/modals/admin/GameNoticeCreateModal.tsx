import { useGameApi } from "@/api/game";
import MDIcon from "@/components/ui/MDIcon";
import { showSuccessNotification } from "@/utils/notification";
import {
	Box,
	Button,
	Card,
	Divider,
	Flex,
	Modal,
	ModalProps,
	Stack,
	TextInput,
	Text,
} from "@mantine/core";
import { useForm } from "@mantine/form";
import { useEffect } from "react";
import { useParams } from "react-router-dom";

interface GameNoticeCreateModalProps extends ModalProps {
	setRefresh: () => void;
}

export default function GameNoticeCreateModal(
	props: GameNoticeCreateModalProps
) {
	const { setRefresh, ...modalProps } = props;
	const gameApi = useGameApi();
	const { id } = useParams<{ id: string }>();

	const form = useForm({
		mode: "controlled",
		initialValues: {
			content: "",
		},
	});

	function createGameNotice() {
		gameApi
			.createGameNotice({
				content: form.getValues().content,
				type: "normal",
				game_id: Number(id),
			})
			.then((_) => {
				showSuccessNotification({
					message: "公告创建成功",
				});
				setRefresh();
				modalProps.onClose();
			});
	}

	useEffect(() => {
		form.reset();
	}, [modalProps.opened]);

	return (
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
						<MDIcon>campaign</MDIcon>
						<Text fw={600}>创建公告</Text>
					</Flex>
					<Divider my={10} />
					<Box p={10}>
						<form
							onSubmit={form.onSubmit((_) => createGameNotice())}
						>
							<Stack gap={10}>
								<TextInput
									label="内容"
									withAsterisk
									key={form.key("content")}
									{...form.getInputProps("content")}
								/>
							</Stack>
							<Flex mt={20} justify={"end"}>
								<Button
									type="submit"
									leftSection={
										<MDIcon c={"white"}>check</MDIcon>
									}
								>
									创建
								</Button>
							</Flex>
						</form>
					</Box>
				</Card>
			</Modal.Content>
		</Modal.Root>
	);
}
