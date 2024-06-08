import { useUserApi } from "@/api/user";
import MDIcon from "@/components/ui/MDIcon";
import { useAuthStore } from "@/stores/auth";
import { useConfigStore } from "@/stores/config";
import { Box, Button, TextInput } from "@mantine/core";
import { useForm } from "@mantine/form";
import { useEffect, useState } from "react";
import { useNavigate } from "react-router-dom";
import { User } from "@/types/user";
import {
	showErrNotification,
	showSuccessNotification,
} from "@/utils/notification";

export default function Page() {
	const configStore = useConfigStore();
	const navigate = useNavigate();
	const userApi = useUserApi();
	const authStore = useAuthStore();

	useEffect(() => {
		document.title = `登录 - ${configStore?.pltCfg?.site?.title}`;
	}, []);

	const [loginLoading, setLoginLoading] = useState(false);

	const form = useForm({
		mode: "controlled",
		initialValues: {
			username: "",
			password: "",
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

	function login() {
		setLoginLoading(true);
		userApi
			.login({
				username: form.getValues().username?.toLocaleLowerCase(),
				password: form.getValues().password,
			})
			.then((res) => {
				const r = res.data;
				authStore.setPgsToken(r.token as string);
				authStore.setUser(r.data as User);
				showSuccessNotification({
					title: "登录成功",
					message: `欢迎进入 ${configStore?.pltCfg?.site?.title}`,
				});
				navigate("/");
			})
			.catch((err) => {
				showErrNotification({
					title: "发生了错误",
					message: `登录失败 ${err.response?.data?.msg}`,
				});
			})
			.finally(() => {
				setLoginLoading(false);
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
				<Box
					sx={{
						display: "flex",
						flexDirection: "column",
						marginTop: "2rem",
					}}
				>
					<form onSubmit={form.onSubmit((_) => login())}>
						<TextInput
							label="用户名"
							size="lg"
							leftSection={<MDIcon>person</MDIcon>}
							key={form.key("username")}
							{...form.getInputProps("username")}
						/>
						<TextInput
							label="密码"
							type="password"
							size="lg"
							leftSection={<MDIcon>lock</MDIcon>}
							mt={10}
							key={form.key("password")}
							{...form.getInputProps("password")}
						/>
						<Button
							loading={loginLoading}
							size={"lg"}
							fullWidth
							sx={{ marginTop: "2rem", bgcolor: "primary.700" }}
							type="submit"
						>
							登录
						</Button>
					</form>
					{configStore?.pltCfg?.user?.register?.enabled && (
						<Box
							sx={{
								display: "flex",
								marginTop: "1rem",
								justifyContent: "end",
							}}
						>
							没有帐号？
							<Box
								onClick={() => navigate("/register")}
								sx={{
									fontStyle: "italic",
									":hover": {
										cursor: "pointer",
									},
								}}
							>
								注册
							</Box>
						</Box>
					)}
				</Box>
			</Box>
		</>
	);
}
