import {
	Card,
	Flex,
	Modal,
	ModalProps,
	ThemeIcon,
	Text,
	Divider,
	TextInput,
	Stack,
	Textarea,
	Button,
	Box,
} from "@mantine/core";
import MDIcon from "@/components/ui/MDIcon";
import { isEmail, useForm } from "@mantine/form";
import { useTeamApi } from "@/api/team";
import { useAuthStore } from "@/stores/auth";
import {
	showErrNotification,
	showSuccessNotification,
} from "@/utils/notification";

interface TeamCreateModalProps extends ModalProps {
	setRefresh: () => void;
}

export default function TeamCreateModal(props: TeamCreateModalProps) {
	const { setRefresh, ...modalProps } = props;

	const teamApi = useTeamApi();
	const authStore = useAuthStore();

	const form = useForm({
		mode: "uncontrolled",
		initialValues: {
			name: "",
			description: "",
			email: "",
		},
		validate: {
			name: (value) => {
				if (value === "") {
					return "团队名称不能为空";
				}
				return null;
			},
			description: (value) => {
				if (value === "") {
					return "团队简介不能为空";
				}
				return null;
			},
			email: isEmail("邮箱格式不正确"),
		},
	});

	function createTeam(name: string, description: string, email: string) {
		teamApi
			.createTeam({
				name: name,
				description: description,
				email: email,
				captain_id: Number(authStore.user?.id),
			})
			.then((_) => {
				showSuccessNotification({
					message: `团队 ${form.values.name} 创建成功`,
				});
				setRefresh();
			})
			.catch((e) => {
				showErrNotification({
					message: e.response.data.error || "创建团队失败",
				});
			})
			.finally(() => {
				form.reset();
				modalProps.onClose();
			});
	}

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
							<MDIcon>group_add</MDIcon>
							<Text fw={600}>创建团队</Text>
						</Flex>
						<Divider my={10} />
						<Box p={10}>
							<form
								onSubmit={form.onSubmit((values) =>
									createTeam(
										values.name,
										values.description,
										values.email
									)
								)}
							>
								<Stack gap={10}>
									<TextInput
										label="团队名称"
										size="md"
										leftSection={<MDIcon>people</MDIcon>}
										key={form.key("name")}
										{...form.getInputProps("name")}
									/>
									<Textarea
										label="团队简介"
										size="md"
										key={form.key("description")}
										{...form.getInputProps("description")}
									/>
									<TextInput
										label="邮箱"
										size="md"
										leftSection={<MDIcon>email</MDIcon>}
										key={form.key("email")}
										{...form.getInputProps("email")}
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
