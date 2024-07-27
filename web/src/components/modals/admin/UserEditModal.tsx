import {
	Card,
	Flex,
	Modal,
	ModalProps,
	Text,
	Divider,
	TextInput,
	Stack,
	Button,
	Box,
	Select,
	Center,
	Image,
} from "@mantine/core";
import MDIcon from "@/components/ui/MDIcon";
import { useForm } from "@mantine/form";
import { zodResolver } from "mantine-form-zod-resolver";
import { z } from "zod";
import {
	showErrNotification,
	showSuccessNotification,
} from "@/utils/notification";
import { useUserApi } from "@/api/user";
import { useEffect, useState } from "react";
import { User } from "@/types/user";
import { Dropzone } from "@mantine/dropzone";
import { AxiosRequestConfig } from "axios";

interface UserEditModalProps extends ModalProps {
	setRefresh: () => void;
	userID?: number;
}

export default function UserEditModal(props: UserEditModalProps) {
	const { userID, setRefresh, ...modalProps } = props;

	const userApi = useUserApi();

	const [user, setUser] = useState<User>();

	const form = useForm({
		mode: "controlled",
		initialValues: {
			username: "",
			nickname: "",
			email: "",
			password: "",
			group: "user",
		},
		validate: zodResolver(
			z.object({
				nickname: z.string().min(1, { message: "昵称不能为空" }),
				email: z.string().email({ message: "邮箱格式不正确" }),
				password: z
					.string()
					.optional()
					.refine((val) => val === "" || Number(val?.length) >= 6, {
						message: "密码长度至少为 6 位",
					}),
				group: z.string().regex(/^(user|admin)$/, {
					message: "用户组只能为 user 或 admin",
				}),
			})
		),
	});

	function getUser() {
		userApi
			.getUsers({
				id: userID,
			})
			.then((res) => {
				const r = res.data;
				setUser(r.data?.[0]);
			});
	}

	function updateUser() {
		userApi
			.updateUser({
				id: Number(user?.id),
				nickname: form.getValues().nickname,
				email: form.getValues().email,
				password: form.getValues().password
					? form.getValues().password
					: undefined,
				group: form.getValues().group,
			})
			.then((_) => {
				showSuccessNotification({
					message: `用户 ${form.getValues().nickname} 更新成功`,
				});
				setRefresh();
			})
			.catch((e) => {
				showErrNotification({
					message: e.response.data.error || "更新用户失败",
				});
			})
			.finally(() => {
				form.reset();
				modalProps.onClose();
			});
	}

	function saveUserAvatar(file?: File) {
		const config: AxiosRequestConfig<FormData> = {};
		userApi
			.saveUserAvatar(Number(user?.id), file!, config)
			.then((_) => {
				showSuccessNotification({
					message: `用户 ${form.getValues().nickname} 头像更新成功`,
				});
			})
			.finally(() => {
				setRefresh();
				getUser();
			});
	}

	useEffect(() => {
		if (!modalProps.opened) {
			setTimeout(() => {
				setUser(undefined);
				form.reset();
			}, 100); // wait for modal to close
			return;
		}
		if (userID) {
			getUser();
		}
	}, [modalProps.opened]);

	useEffect(() => {
		if (user) {
			form.setValues({
				username: user.username,
				nickname: user.nickname,
				email: user.email,
				group: user.group,
			});
		}
	}, [user]);

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
						pos={"relative"}
					>
						<Flex gap={10} align={"center"}>
							<MDIcon>person_add</MDIcon>
							<Text fw={600}>更新用户</Text>
						</Flex>
						<Divider my={10} />
						<Box p={10}>
							<form onSubmit={form.onSubmit((_) => updateUser())}>
								<Stack gap={10}>
									<Flex gap={10}>
										<Stack
											sx={{
												flexGrow: 1,
											}}
										>
											<Flex gap={10} w={"100%"}>
												<TextInput
													label="用户名"
													size="md"
													w={"40%"}
													disabled
													leftSection={
														<MDIcon>person</MDIcon>
													}
													key={form.key("username")}
													{...form.getInputProps(
														"username"
													)}
												/>
												<TextInput
													label="昵称"
													size="md"
													w={"60%"}
													key={form.key("nickname")}
													{...form.getInputProps(
														"nickname"
													)}
												/>
											</Flex>
											<Select
												label="权限组"
												data={["user", "admin"]}
												allowDeselect={false}
												key={form.key("group")}
												{...form.getInputProps("group")}
											/>
										</Stack>
										<Dropzone
											onDrop={(files: any) =>
												saveUserAvatar(files[0])
											}
											onReject={() => {
												showErrNotification({
													message: "文件上传失败",
												});
											}}
											h={150}
											w={150}
											accept={[
												"image/png",
												"image/gif",
												"image/jpeg",
												"image/webp",
												"image/avif",
												"image/heic",
											]}
										>
											<Center
												style={{
													pointerEvents: "none",
												}}
											>
												{user?.avatar?.name ? (
													<Center>
														<Image
															w={120}
															h={120}
															fit="contain"
															src={`${import.meta.env.VITE_BASE_API}/users/${user?.id}/avatar`}
														/>
													</Center>
												) : (
													<Center>
														<Stack gap={0}>
															<Text
																size="xl"
																inline
															>
																拖拽或点击上传头像
															</Text>
															<Text
																size="sm"
																c="dimmed"
																inline
																mt={7}
															>
																图片大小不超过
																3MB
															</Text>
														</Stack>
													</Center>
												)}
											</Center>
										</Dropzone>
									</Flex>
									<TextInput
										label="邮箱"
										size="md"
										leftSection={<MDIcon>email</MDIcon>}
										key={form.key("email")}
										{...form.getInputProps("email")}
									/>
									<TextInput
										label="密码"
										size="md"
										leftSection={<MDIcon>lock</MDIcon>}
										key={form.key("password")}
										{...form.getInputProps("password")}
									/>
								</Stack>
								<Flex mt={20} justify={"end"}>
									<Button
										type="submit"
										leftSection={
											<MDIcon c={"white"}>check</MDIcon>
										}
									>
										更新
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
