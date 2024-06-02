import { useUserApi } from "@/api/user";
import MDIcon from "@/components/ui/MDIcon";
import { useAuthStore } from "@/stores/auth";
import { useConfigStore } from "@/stores/config";
import { Box, Button, Flex, Group, Stack, TextInput } from "@mantine/core";
import { useForm } from "@mantine/form";
import { useEffect, useState } from "react";
import { useNavigate } from "react-router-dom";
import { User } from "@/types/user";
import {
	showErrNotification,
	showSuccessNotification,
} from "@/utils/notification";
import Turnstile from "react-turnstile";
import ReCAPTCHA from "react-google-recaptcha";

export default function Page() {
	const configStore = useConfigStore();
	const navigate = useNavigate();
	const userApi = useUserApi();
	const authStore = useAuthStore();

	useEffect(() => {
		document.title = `注册 - ${configStore?.pltCfg?.site?.title}`;
	}, []);

	const [registerLoading, setRegisterLoading] = useState(false);

	const form = useForm({
		mode: "controlled",
		initialValues: {
			username: "",
			nickname: "",
			password: "",
			email: "",
			token: "",
		},

		validate: {
			username: (value) => {
				if (value === "") {
					return "用户名不能为空";
				}
				return null;
			},
			password: (value) => {
				if (value === "") {
					return "密码不能为空";
				}
				return null;
			},
		},
	});

	function register() {
		if (
			configStore?.pltCfg?.user?.register?.captcha?.enabled &&
			!form.getValues().token
		) {
			showErrNotification({
				title: "注册失败",
				message: "请完成验证码验证",
			});
			return;
		}
		setRegisterLoading(true);
		userApi
			.register({
				username: form.getValues().username,
				nickname: form.getValues().nickname,
				password: form.getValues().password,
				email: form.getValues().email,
				token: form.getValues().token,
			})
			.then((res) => {
				const r = res.data;
				authStore.setPgsToken(r.token as string);
				authStore.setUser(r.data as User);
				showSuccessNotification({
					title: "注册成功",
					message: "请登录",
				});
				navigate("/login");
			})
			.catch((err) => {
				switch (err.response?.status) {
					case 400:
						showErrNotification({
							title: "注册失败",
							message: "用户名或邮箱已被注册",
						});
						break;
				}
			})
			.finally(() => {
				setRegisterLoading(false);
			});
	}

	return (
		<>
			<Box
				sx={{
					position: "fixed",
					top: "50%",
					left: "50%",
					zIndex: -1,
					transform: "translate(-50%, -50%)",
				}}
				className={"no-select"}
			>
				<Stack>
					<form onSubmit={form.onSubmit((_) => register())}>
						<Stack>
							<Group>
								<TextInput
									label="用户名"
									size="lg"
									leftSection={<MDIcon>person</MDIcon>}
									key={form.key("username")}
									{...form.getInputProps("username")}
								/>
								<TextInput
									label="昵称"
									size="lg"
									leftSection={<MDIcon>person</MDIcon>}
									key={form.key("nickname")}
									{...form.getInputProps("nickname")}
								/>
							</Group>
							<TextInput
								label="邮箱"
								size="lg"
								leftSection={<MDIcon>email</MDIcon>}
								key={form.key("email")}
								{...form.getInputProps("email")}
							/>
							<TextInput
								label="密码"
								type="password"
								size="lg"
								leftSection={<MDIcon>lock</MDIcon>}
								key={form.key("password")}
								{...form.getInputProps("password")}
							/>
							<Flex justify={"center"}>
								{configStore?.pltCfg?.user?.register?.captcha
									?.enabled && (
									<>
										{configStore?.captchaCfg?.provider ===
											"turnstile" && (
											<Turnstile
												sitekey={String(
													configStore?.captchaCfg
														?.turnstile?.site_key
												)}
												onVerify={(token) => {
													form.setValues({
														...form.values,
														token: token,
													});
												}}
											/>
										)}
										{configStore?.captchaCfg?.provider ===
											"recaptcha" && (
											<ReCAPTCHA
												sitekey={String(
													configStore?.captchaCfg
														?.recaptcha?.site_key
												)}
												onChange={(token) => {
													form.setValues({
														...form.values,
														token: String(token),
													});
												}}
											/>
										)}
									</>
								)}
							</Flex>
							<Button
								loading={registerLoading}
								size={"lg"}
								fullWidth
								sx={{ bgcolor: "primary.700" }}
								type="submit"
							>
								注册
							</Button>
						</Stack>
					</form>
					<Box
						sx={{
							display: "flex",
							marginTop: "1rem",
							justifyContent: "end",
						}}
					>
						已有帐号？
						<Box
							onClick={() => navigate("/login")}
							sx={{
								fontStyle: "italic",
								":hover": {
									cursor: "pointer",
								},
							}}
						>
							登录
						</Box>
					</Box>
				</Stack>
			</Box>
		</>
	);
}
