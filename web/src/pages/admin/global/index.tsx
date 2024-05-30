import { useConfigApi } from "@/api/config";
import MDIcon from "@/components/ui/MDIcon";
import { useConfigStore } from "@/stores/config";
import { Config } from "@/types/config";
import { showSuccessNotification } from "@/utils/notification";
import {
	Box,
	Button,
	Divider,
	Flex,
	Group,
	NumberInput,
	Paper,
	SimpleGrid,
	Stack,
	Switch,
	Text,
	TextInput,
} from "@mantine/core";
import { useForm } from "@mantine/form";
import { useEffect } from "react";

export default function Page() {
	const configStore = useConfigStore();
	const configApi = useConfigApi();

	const form = useForm<Config>({
		initialValues: {
			site: {
				title: "",
				description: "",
				color: "",
			},
			container: {
				parallel_limit: 0,
				request_limit: 0,
			},
			user: {
				register: {
					enabled: false,
					captcha: {
						enabled: false,
					},
					email: {
						domains: [],
						enabled: false,
					},
				},
			},
		},
	});

	function updateConfig() {
		configApi.updatePltCfg(form.getValues()).then((_) => {
			showSuccessNotification({
				message: "全局设置更新成功",
			});
		});
	}

	useEffect(() => {
		if (configStore?.pltCfg) {
			form.setValues(configStore?.pltCfg);
		}
	}, [configStore?.pltCfg]);

	useEffect(() => {
		document.title = `全局设置 - ${configStore?.pltCfg?.site?.title}`;
	}, []);

	return (
		<Paper my={36} mx={"10%"} mih={"85vh"} p={36}>
			<form onSubmit={form.onSubmit((_) => updateConfig())}>
				<Stack gap={20} mih={"calc(70vh)"}>
					<Stack gap={10}>
						<Group gap={10}>
							<MDIcon>language</MDIcon>
							<Text fw={700}>站点</Text>
						</Group>
						<Divider />
					</Stack>
					<SimpleGrid cols={2}>
						<TextInput
							label={"标题"}
							size="lg"
							key={form.key("site.title")}
							{...form.getInputProps("site.title")}
						/>
						<TextInput
							label={"描述"}
							size="lg"
							key={form.key("site.description")}
							{...form.getInputProps("site.description")}
						/>
					</SimpleGrid>
					<Stack gap={10}>
						<Group gap={10}>
							<MDIcon>package_2</MDIcon>
							<Text fw={700}>容器</Text>
						</Group>
						<Divider />
					</Stack>
					<SimpleGrid cols={2}>
						<NumberInput
							label={"并行限制（个）"}
							size="md"
							key={form.key("container.parallel_limit")}
							{...form.getInputProps("container.parallel_limit")}
						/>
						<NumberInput
							label={"请求限制（秒）"}
							size="md"
							key={form.key("container.request_limit")}
							{...form.getInputProps("container.request_limit")}
						/>
					</SimpleGrid>
					<Stack gap={10}>
						<Group gap={10}>
							<MDIcon>person</MDIcon>
							<Text fw={700}>用户</Text>
						</Group>
						<Divider />
					</Stack>
					<SimpleGrid cols={3}>
						<Switch
							label={"允许新用户注册"}
							checked={form.getValues()?.user?.register?.enabled}
							key={form.key("user.register.enabled")}
							{...form.getInputProps("user.register.enabled")}
						/>
						<Switch
							label={"使用 Captcha 验证"}
							checked={
								form.getValues()?.user?.register?.captcha
									?.enabled
							}
							key={form.key("user.register.captcha.enabled")}
							{...form.getInputProps(
								"user.register.captcha.enabled"
							)}
						/>
					</SimpleGrid>
				</Stack>
				<Flex justify={"end"}>
					<Button
						size={"md"}
						type="submit"
						leftSection={<MDIcon c={"white"}>check</MDIcon>}
					>
						保存
					</Button>
				</Flex>
			</form>
		</Paper>
	);
}
