import { useGameApi } from "@/api/game";
import MDIcon from "@/components/ui/MDIcon";
import { showSuccessNotification } from "@/utils/notification";
import {
	Box,
	Button,
	Divider,
	Flex,
	Modal,
	ModalProps,
	Stack,
	TextInput,
	Text,
	Card,
	Group,
	Input,
	Textarea,
	ThemeIcon,
} from "@mantine/core";
import { useForm, zodResolver } from "@mantine/form";
import { DateTimePicker } from "@mantine/dates";
import { useEffect } from "react";
import { z } from "zod";

interface GameCreateModalProps extends ModalProps {
	setRefresh: () => void;
}

export default function GameCreateModal(props: GameCreateModalProps) {
	const { setRefresh, ...modalProps } = props;

	const gameApi = useGameApi();

	const form = useForm({
		mode: "controlled",
		initialValues: {
			title: "",
			bio: "",
			started_at: new Date().getTime() / 1000,
			ended_at: new Date().getTime() / 1000,
		},
		validate: zodResolver(
			z.object({
				title: z.string(),
				bio: z.string().optional(),
				started_at: z.number(),
				ended_at: z.number(),
			})
		),
	});

	function createGame() {
		gameApi
			.createGame({
				title: form.getValues().title,
				bio: form.getValues().bio,
				started_at: Math.ceil(form.getValues().started_at),
				ended_at: Math.ceil(form.getValues().ended_at),
				is_enabled: false,
			})
			.then((_) => {
				showSuccessNotification({
					message: `比赛 ${form.getValues().title} 创建成功`,
				});
				setRefresh();
				modalProps.onClose();
			});
	}

	useEffect(() => {
		form.reset();
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
							<MDIcon>flag</MDIcon>
							<Text fw={600}>创建比赛</Text>
						</Flex>
						<Divider my={10} />
						<Box p={10}>
							<form onSubmit={form.onSubmit((_) => createGame())}>
								<Stack gap={10}>
									<TextInput
										label="比赛名称"
										size="md"
										withAsterisk
										key={form.key("title")}
										{...form.getInputProps("title")}
									/>
									<Textarea
										label="比赛简介"
										size="md"
										key={form.key("bio")}
										{...form.getInputProps("bio")}
									/>
									<DateTimePicker
										withSeconds
										withAsterisk
										label="开始时间"
										placeholder="请选择比赛开始的时间"
										valueFormat="YYYY/MM/DD HH:mm:ss"
										value={
											new Date(
												form.getValues().started_at *
													1000
											)
										}
										onChange={(value) => {
											form.setFieldValue(
												"started_at",
												Number(value?.getTime()) / 1000
											);
										}}
									/>
									<DateTimePicker
										withSeconds
										withAsterisk
										label="结束时间"
										placeholder="请选择比赛结束的时间"
										valueFormat="YYYY/MM/DD HH:mm:ss"
										value={
											new Date(
												form.getValues().ended_at * 1000
											)
										}
										onChange={(value) => {
											form.setFieldValue(
												"ended_at",
												Number(value?.getTime()) / 1000
											);
										}}
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
		</>
	);
}
