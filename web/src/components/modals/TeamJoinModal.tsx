import { useTeamApi } from "@/api/team";
import {
	showErrNotification,
	showSuccessNotification,
} from "@/utils/notification";
import {
	Box,
	Card,
	Divider,
	Flex,
	Modal,
	ModalProps,
	ThemeIcon,
	Text,
	TextInput,
	Button,
} from "@mantine/core";
import { useForm } from "@mantine/form";
import MDIcon from "@/components/ui/MDIcon";

interface TeamJoinModalProps extends ModalProps {
	setRefresh: () => void;
}

export default function TeamJoinModal(props: TeamJoinModalProps) {
	const { setRefresh, ...modalProps } = props;

	const teamApi = useTeamApi();

	const form = useForm({
		initialValues: {
			inviteToken: "",
		},
		validate: {
			inviteToken: (value) => {
				if (value.split(":").length != 2) {
					return "邀请码格式错误";
				}
				return null;
			},
		},
	});

	function joinTeam() {
		teamApi
			.joinTeam({
				id: Number(form.getValues().inviteToken.split(":")[0]),
				invite_token: form.getValues().inviteToken.split(":")[1],
			})
			.then((_) => {
				showSuccessNotification({
					message: "加入团队成功",
				});
			})
			.catch((e) => {
				showErrNotification({
					message: "邀请码无效或团队已被锁定",
				});
			})
			.finally(() => {
				modalProps.onClose();
				setRefresh();
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
							<ThemeIcon variant="transparent">
								<MDIcon>waving_hand</MDIcon>
							</ThemeIcon>
							<Text fw={600}>加入团队</Text>
						</Flex>
						<Divider my={10} />
						<Box p={10}>
							<form
								onSubmit={form.onSubmit((_) => {
									joinTeam();
								})}
							>
								<TextInput
									label="邀请码"
									size="md"
									placeholder="n:xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"
									key={form.key("inviteToken")}
									{...form.getInputProps("inviteToken")}
								/>
								<Flex mt={20} justify={"end"}>
									<Button
										type="submit"
										leftSection={<MDIcon>check</MDIcon>}
									>
										加入
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
