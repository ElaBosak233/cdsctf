import { useChallengeApi } from "@/api/challenge";
import withChallengeEdit from "@/components/layouts/admin/withChallengeEdit";
import MDIcon from "@/components/ui/MDIcon";
import { Challenge } from "@/types/challenge";
import { Env } from "@/types/env";
import { Port } from "@/types/port";
import { showSuccessNotification } from "@/utils/notification";
import {
	ActionIcon,
	Button,
	Divider,
	Flex,
	Group,
	NumberInput,
	SimpleGrid,
	Stack,
	Text,
	TextInput,
	Tooltip,
} from "@mantine/core";
import { useForm } from "@mantine/form";
import { useState, useEffect } from "react";
import { useParams } from "react-router-dom";

function Page() {
	const { id } = useParams<{ id: string }>();
	const challengeApi = useChallengeApi();

	const [refresh, setRefresh] = useState<number>(0);
	const [challenge, setChallenge] = useState<Challenge>();

	const [envs, setEnvs] = useState<Array<Env>>([]);
	const [ports, setPorts] = useState<Array<Port>>([]);

	const form = useForm({
		initialValues: {
			image_name: "",
			cpu_limit: 1,
			memory_limit: 256,
		},
	});

	function getChallenge() {
		challengeApi
			.getChallenges({
				id: Number(id),
				is_detailed: true,
			})
			.then((res) => {
				const r = res.data;
				setChallenge(r.data?.[0]);
				setEnvs(r.data?.[0]?.envs || []);
				setPorts(r.data?.[0]?.ports || []);
			});
	}

	function updateChallengeImage() {
		challengeApi
			.updateChallenge({
				id: Number(id),
				image_name: form.getValues().image_name,
				cpu_limit: form.getValues().cpu_limit,
				memory_limit: form.getValues().memory_limit,
				envs: envs,
				ports: ports,
			})
			.then((_) => {
				showSuccessNotification({
					message: "镜像更新成功",
				});
				setRefresh((prev) => prev + 1);
			});
	}

	useEffect(() => {
		form.setValues({
			image_name: challenge?.image_name || "",
			cpu_limit: challenge?.cpu_limit || 1,
			memory_limit: challenge?.memory_limit || 256,
		});
	}, [challenge]);

	useEffect(() => {
		getChallenge();
	}, [refresh]);

	useEffect(() => {
		document.title = `镜像 - ${challenge?.title}`;
	}, [challenge]);

	return (
		<Stack m={36}>
			<form onSubmit={form.onSubmit((_) => updateChallengeImage())}>
				<Stack>
					<Stack gap={10}>
						<Group>
							<MDIcon>deployed_code_update</MDIcon>
							<Text fw={700} size="xl">
								镜像参数
							</Text>
						</Group>
						<Divider />
					</Stack>
					<Stack mx={20}>
						<SimpleGrid cols={3}>
							<TextInput
								label="镜像名"
								description="例如 nginx:latest"
								key={form.key("image_name")}
								{...form.getInputProps("image_name")}
							/>
							<NumberInput
								label="CPU 限制"
								description="CPU 核心限制（核）"
								key={form.key("cpu_limit")}
								{...form.getInputProps("cpu_limit")}
							/>
							<NumberInput
								label="内存限制"
								description="内存大小限制（MB）"
								key={form.key("memory_limit")}
								{...form.getInputProps("memory_limit")}
							/>
						</SimpleGrid>
					</Stack>
					<Stack gap={10}>
						<Flex justify={"space-between"} align="center">
							<Group>
								<MDIcon>upgrade</MDIcon>
								<Text fw={700} size="xl">
									端口映射
								</Text>
							</Group>
							<Tooltip label="添加端口映射" withArrow>
								<ActionIcon
									onClick={() =>
										setPorts([
											...ports,
											{ value: 0, description: "" },
										])
									}
								>
									<MDIcon>add</MDIcon>
								</ActionIcon>
							</Tooltip>
						</Flex>
						<Divider />
					</Stack>
					<Stack mx={20}>
						{ports?.map((port, index) => (
							<Flex
								key={index}
								align={"center"}
								justify={"space-between"}
							>
								<Group gap={10}>
									<NumberInput
										label="端口"
										value={port?.value}
										onChange={(e) =>
											setPorts(
												ports.map((p, i) =>
													i === index
														? {
																...p,
																value: Number(
																	e
																),
															}
														: p
												)
											)
										}
									/>
									<TextInput
										label="描述"
										value={port?.description}
										onChange={(e) =>
											setPorts(
												ports.map((p, i) =>
													i === index
														? {
																...p,
																description:
																	e.target
																		.value,
															}
														: p
												)
											)
										}
									/>
								</Group>
								<Tooltip
									label="删除端口映射"
									withArrow
									onClick={() => {
										const newPorts = [...ports];
										newPorts.splice(index, 1);
										setPorts(newPorts);
									}}
								>
									<ActionIcon>
										<MDIcon color={"red"}>delete</MDIcon>
									</ActionIcon>
								</Tooltip>
							</Flex>
						))}
					</Stack>
					<Stack gap={10}>
						<Flex justify={"space-between"} align="center">
							<Group>
								<MDIcon>language</MDIcon>
								<Text fw={700} size="xl">
									环境变量
								</Text>
							</Group>
							<Tooltip label="添加环境变量" withArrow>
								<ActionIcon
									onClick={() => {
										setEnvs([
											...envs,
											{ key: "", value: "" },
										]);
									}}
								>
									<MDIcon>add</MDIcon>
								</ActionIcon>
							</Tooltip>
						</Flex>
						<Divider />
					</Stack>
					<Stack mx={20}>
						{envs?.map((env, index) => (
							<Flex
								key={index}
								align={"center"}
								justify={"space-between"}
							>
								<Group gap={10}>
									<TextInput
										label="键"
										value={env?.key}
										onChange={(e) =>
											setEnvs(
												envs.map((p, i) =>
													i === index
														? {
																...p,
																key: e.target
																	.value,
															}
														: p
												)
											)
										}
									/>
									<TextInput
										label="值"
										value={env?.value}
										onChange={(e) =>
											setEnvs(
												envs.map((p, i) =>
													i === index
														? {
																...p,
																value: e.target
																	.value,
															}
														: p
												)
											)
										}
									/>
								</Group>
								<Tooltip
									label="删除端口映射"
									withArrow
									onClick={() => {
										const newEnvs = [...envs];
										newEnvs.splice(index, 1);
										setEnvs(newEnvs);
									}}
								>
									<ActionIcon>
										<MDIcon color={"red"}>delete</MDIcon>
									</ActionIcon>
								</Tooltip>
							</Flex>
						))}
					</Stack>
					<Flex justify="end">
						<Button
							type="submit"
							leftSection={<MDIcon c={"white"}>check</MDIcon>}
						>
							保存
						</Button>
					</Flex>
				</Stack>
			</form>
		</Stack>
	);
}

export default withChallengeEdit(Page);
