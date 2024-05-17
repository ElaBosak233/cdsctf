import { useTeamApi } from "@/api/team";
import { useAuthStore } from "@/stores/auth";
import { Team, TeamUpdateRequest } from "@/types/team";
import {
	showErrNotification,
	showSuccessNotification,
} from "@/utils/notification";
import {
	Card,
	Flex,
	Modal,
	ModalProps,
	ThemeIcon,
	Text,
	Divider,
	Box,
	Stack,
	TextInput,
	Textarea,
	Button,
	ActionIcon,
	Avatar,
	Tooltip,
	Group,
} from "@mantine/core";
import { isEmail, useForm } from "@mantine/form";
import MDIcon from "@/components/ui/MDIcon";
import { useEffect, useState } from "react";
import { modals } from "@mantine/modals";
import { User } from "@/types/user";

interface TeamEditModalProps extends ModalProps {
	setRefresh: () => void;
	team?: Team;
}

export default function TeamEditModal(props: TeamEditModalProps) {
	const { setRefresh, team, ...modalProps } = props;

	const teamApi = useTeamApi();
	const authStore = useAuthStore();

	const [isCaptain, setIsCaptain] = useState<boolean>(false);
	const [inviteToken, setInviteToken] = useState<string>("");
	const [users, setUsers] = useState<Array<User> | undefined>([]);

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

	function getTeamInviteToken() {
		teamApi
			.getTeamInviteToken({
				id: Number(team?.id),
			})
			.then((res) => {
				const r = res.data;
				setInviteToken(r.invite_token);
			});
	}

	function updateTeamInviteToken() {
		teamApi
			.updateTeamInviteToken({
				id: Number(team?.id),
			})
			.then((res) => {
				const r = res.data;
				setInviteToken(r.invite_token);
				showSuccessNotification({
					message: `团队 ${team?.name} 邀请码更新成功`,
				});
			});
	}

	function updateTeam(request: TeamUpdateRequest) {
		teamApi
			.updateTeam({
				id: Number(team?.id),
				name: request?.name,
				description: request?.description,
				email: request?.email,
				captain_id: request?.captain_id || Number(authStore.user?.id),
			})
			.then((_) => {
				showSuccessNotification({
					message: `团队 ${form.values.name} 更新成功`,
				});
				setRefresh();
			})
			.catch((e) => {
				showErrNotification({
					message: e.response.data.error || "更新团队失败",
				});
			})
			.finally(() => {
				form.reset();
				modalProps.onClose();
			});
	}

	function deleteTeamUser(user?: User) {
		teamApi
			.deleteTeamUser({
				id: Number(team?.id),
				user_id: Number(user?.id),
			})
			.then((_) => {
				showSuccessNotification({
					message: `用户 ${user?.nickname} 已被踢出`,
				});
				setRefresh();
				setUsers((prevUsers) =>
					prevUsers?.filter((u) => u?.id !== user?.id)
				);
			});
	}

	function deleteTeam() {
		teamApi
			.deleteTeam({
				id: Number(team?.id),
			})
			.then((_) => {
				showSuccessNotification({
					message: `团队 ${team?.name} 已被解散`,
				});
			})
			.finally(() => {
				setRefresh();
				modalProps.onClose();
			});
	}

	const openDeleteTeamUserModal = (user?: User) =>
		modals.openConfirmModal({
			centered: true,
			children: (
				<>
					<Flex gap={10} align={"center"}>
						<ThemeIcon variant="transparent">
							<MDIcon>person_remove</MDIcon>
						</ThemeIcon>
						<Text fw={600}>踢出成员</Text>
					</Flex>
					<Divider my={10} />
					<Text>你确定要踢出成员 {user?.nickname} 吗？</Text>
				</>
			),
			withCloseButton: false,
			labels: {
				confirm: "确定",
				cancel: "取消",
			},
			confirmProps: {
				color: "red",
			},
			cancelProps: {
				display: "none",
			},
			onConfirm: () => {
				deleteTeamUser(user);
			},
		});

	const openTransferCaptainModal = (user?: User) =>
		modals.openConfirmModal({
			centered: true,
			children: (
				<>
					<Flex gap={10} align={"center"}>
						<ThemeIcon variant="transparent" color="orange">
							<MDIcon>star</MDIcon>
						</ThemeIcon>
						<Text fw={600}>转让队长</Text>
					</Flex>
					<Divider my={10} />
					<Text>你确定要将队长转让给 {user?.nickname} 吗？</Text>
				</>
			),
			withCloseButton: false,
			labels: {
				confirm: "确定",
				cancel: "取消",
			},
			cancelProps: {
				display: "none",
			},
			onConfirm: () => {
				updateTeam({
					captain_id: user?.id,
				});
			},
		});

	const openDeleteTeamModal = () =>
		modals.openConfirmModal({
			centered: true,
			children: (
				<>
					<Flex gap={10} align={"center"}>
						<ThemeIcon variant="transparent" color="red">
							<MDIcon>swap_horiz</MDIcon>
						</ThemeIcon>
						<Text fw={600}>解散团队</Text>
					</Flex>
					<Divider my={10} />
					<Text>你确定要解散团队 {team?.name} 吗？</Text>
				</>
			),
			withCloseButton: false,
			labels: {
				confirm: "确定",
				cancel: "取消",
			},
			cancelProps: {
				display: "none",
			},
			confirmProps: {
				color: "red",
			},
			onConfirm: () => {
				deleteTeam();
			},
		});

	useEffect(() => {
		setIsCaptain(authStore?.user?.id === team?.captain_id);
		if (authStore?.user?.id === team?.captain_id) {
			getTeamInviteToken();
		}
		setUsers(team?.users);
		form.setValues({
			name: team?.name,
			description: team?.description,
			email: team?.email,
		});
	}, [team]);

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
								<MDIcon>group_add</MDIcon>
							</ThemeIcon>
							<Text fw={600}>团队详情</Text>
						</Flex>
						<Divider my={10} />
						<Box p={10}>
							<form
								onSubmit={form.onSubmit((values) =>
									updateTeam({
										name: values.name,
										description: values.description,
										email: values.email,
									})
								)}
							>
								<Stack gap={10}>
									<TextInput
										label="团队名称"
										size="md"
										leftSection={<MDIcon>people</MDIcon>}
										key={form.key("name")}
										{...form.getInputProps("name")}
										readOnly={!isCaptain}
									/>
									{isCaptain && (
										<TextInput
											label="邀请码"
											size="md"
											readOnly
											rightSection={
												<ActionIcon
													variant="transparent"
													onClick={
														updateTeamInviteToken
													}
												>
													<MDIcon>refresh</MDIcon>
												</ActionIcon>
											}
											value={`${team?.id}:${inviteToken}`}
										/>
									)}
									<Textarea
										label="团队简介"
										size="md"
										key={form.key("description")}
										{...form.getInputProps("description")}
										readOnly={!isCaptain}
									/>
									<TextInput
										label="邮箱"
										size="md"
										leftSection={<MDIcon>email</MDIcon>}
										key={form.key("email")}
										{...form.getInputProps("email")}
										readOnly={!isCaptain}
									/>
								</Stack>
								<Stack mt={10}>
									<Text>成员</Text>
									<Group gap={20}>
										{users?.map((user) => (
											<Flex
												key={user?.id}
												align={"center"}
												gap={15}
											>
												<Flex align={"center"} gap={10}>
													<Avatar
														color="brand"
														src={`${import.meta.env.VITE_BASE_API}/media/users/${user?.id}/${user?.avatar?.name}`}
														radius="xl"
													>
														<MDIcon>person</MDIcon>
													</Avatar>
													<Text>
														{user?.nickname}
													</Text>
												</Flex>
												{user?.id ===
													team?.captain_id && (
													<Tooltip
														label="队长"
														withArrow
													>
														<ThemeIcon
															variant="transparent"
															color="yellow"
														>
															<MDIcon>
																star
															</MDIcon>
														</ThemeIcon>
													</Tooltip>
												)}
												{isCaptain &&
													user?.id !==
														authStore?.user?.id && (
														<Flex>
															<Tooltip
																label="转让队长"
																withArrow
															>
																<ActionIcon
																	variant="transparent"
																	color="grey"
																	onClick={() => {
																		openTransferCaptainModal(
																			user
																		);
																	}}
																>
																	<MDIcon>
																		star
																	</MDIcon>
																</ActionIcon>
															</Tooltip>
															<Tooltip
																label="踢出"
																withArrow
															>
																<ActionIcon
																	variant="transparent"
																	color="red"
																	onClick={() => {
																		openDeleteTeamUserModal(
																			user
																		);
																	}}
																>
																	<MDIcon>
																		close
																	</MDIcon>
																</ActionIcon>
															</Tooltip>
														</Flex>
													)}
											</Flex>
										))}
									</Group>
								</Stack>
								{isCaptain && (
									<Flex mt={20} justify={"end"} gap={10}>
										<Button
											leftSection={
												<MDIcon>swap_horiz</MDIcon>
											}
											onClick={openDeleteTeamModal}
											color="red"
										>
											解散
										</Button>
										<Button
											type="submit"
											leftSection={<MDIcon>check</MDIcon>}
										>
											更新
										</Button>
									</Flex>
								)}
							</form>
						</Box>
					</Card>
				</Modal.Content>
			</Modal.Root>
		</>
	);
}
