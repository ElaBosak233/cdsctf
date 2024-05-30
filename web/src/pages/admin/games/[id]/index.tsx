import { useGameApi } from "@/api/game";
import withGameEdit from "@/components/layouts/admin/withGameEdit";
import MDIcon from "@/components/ui/MDIcon";
import { useConfigStore } from "@/stores/config";
import { Game } from "@/types/game";
import {
	showErrNotification,
	showSuccessNotification,
} from "@/utils/notification";
import {
	Center,
	Flex,
	NumberInput,
	SimpleGrid,
	Stack,
	Switch,
	TextInput,
	Textarea,
	Image,
	Text,
	Button,
	Group,
	Divider,
} from "@mantine/core";
import { DateTimePicker } from "@mantine/dates";
import { Dropzone } from "@mantine/dropzone";
import { useForm } from "@mantine/form";
import { AxiosRequestConfig } from "axios";
import { useEffect, useState } from "react";
import { useParams } from "react-router-dom";

function Page() {
	const { id } = useParams<{ id: string }>();
	const configStore = useConfigStore();
	const gameApi = useGameApi();

	const [game, setGame] = useState<Game>();

	const form = useForm({
		mode: "controlled",
		initialValues: {
			title: "",
			bio: "",
			description: "",
			is_public: false,
			started_at: 0,
			ended_at: 0,
			member_limit_min: 0,
			member_limit_max: 0,
			parallel_container_limit: 0,
			first_blood_reward_ratio: 0,
			second_blood_reward_ratio: 0,
			third_blood_reward_ratio: 0,
			is_need_write_up: false,
		},
	});

	function getGame() {
		gameApi
			.getGames({
				id: Number(id),
			})
			.then((res) => {
				const r = res.data;
				setGame(r.data[0]);
			});
	}

	function updateGame() {
		gameApi
			.updateGame({
				id: Number(id),
				title: form.getValues().title,
				bio: form.getValues().bio,
				description: form.getValues().description,
				is_public: form.getValues().is_public,
				started_at: Math.ceil(form.getValues().started_at),
				ended_at: Math.ceil(form.getValues().ended_at),
				member_limit_min: form.getValues().member_limit_min,
				member_limit_max: form.getValues().member_limit_max,
				parallel_container_limit:
					form.getValues().parallel_container_limit,
				first_blood_reward_ratio:
					form.getValues().first_blood_reward_ratio,
				second_blood_reward_ratio:
					form.getValues().second_blood_reward_ratio,
				third_blood_reward_ratio:
					form.getValues().third_blood_reward_ratio,
				is_need_write_up: form.getValues().is_need_write_up,
			})
			.then((_) => {
				showSuccessNotification({
					message: `比赛 ${form.getValues().title} 更新成功`,
				});
			});
	}

	function saveGamePoster(file?: File) {
		const config: AxiosRequestConfig<FormData> = {};
		gameApi
			.saveGamePoster(Number(game?.id), file!, config)
			.then((_) => {
				showSuccessNotification({
					message: `比赛 ${form.getValues().title} 头图更新成功`,
				});
			})
			.finally(() => {
				getGame();
			});
	}

	useEffect(() => {
		getGame();
	}, []);

	useEffect(() => {
		if (game) {
			form.setValues({
				title: game?.title,
				bio: game?.bio,
				description: game?.description,
				is_public: game?.is_public,
				started_at: game?.started_at,
				ended_at: game?.ended_at,
				member_limit_min: game?.member_limit_min,
				member_limit_max: game?.member_limit_max,
				parallel_container_limit: game?.parallel_container_limit,
				first_blood_reward_ratio: game?.first_blood_reward_ratio,
				second_blood_reward_ratio: game?.second_blood_reward_ratio,
				third_blood_reward_ratio: game?.third_blood_reward_ratio,
				is_need_write_up: game?.is_need_write_up,
			});
		}
	}, [game]);

	useEffect(() => {
		document.title = `${game?.title} - ${configStore?.pltCfg?.site?.title}`;
	}, [game]);

	return (
		<>
			<Stack m={36}>
				<Stack gap={10}>
					<Group>
						<MDIcon>info</MDIcon>
						<Text fw={700} size="xl">
							基本信息
						</Text>
					</Group>
					<Divider />
				</Stack>
				<form onSubmit={form.onSubmit((_) => updateGame())}>
					<Stack mx={20}>
						<Flex gap={20}>
							<SimpleGrid
								cols={4}
								sx={{
									alignItems: "center",
								}}
							>
								<TextInput
									label="标题"
									description="展示在最醒目位置的比赛大标题"
									withAsterisk
									key={form.key("title")}
									{...form.getInputProps("title")}
								/>
								<NumberInput
									label="最小人数"
									description="一个团队所需的最少的人数"
									withAsterisk
									key={form.key("member_limit_min")}
									{...form.getInputProps("member_limit_min")}
								/>
								<NumberInput
									label="最大人数"
									description="一个团队所需的最多的人数"
									withAsterisk
									key={form.key("member_limit_max")}
									{...form.getInputProps("member_limit_max")}
								/>
								<NumberInput
									label="容器限制"
									description="一个团队最多可启用的容器数量"
									withAsterisk
									key={form.key("parallel_container_limit")}
									{...form.getInputProps(
										"parallel_container_limit"
									)}
								/>
								<DateTimePicker
									withSeconds
									withAsterisk
									label="开始时间"
									description="比赛开始的时间"
									placeholder="请选择比赛开始的时间"
									valueFormat="YYYY/MM/DD HH:mm:ss"
									value={
										new Date(
											form.getValues().started_at * 1000
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
									description="比赛结束的时间"
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
								<Switch
									label="是否公开"
									description="若为是，则队伍在报名参赛后自动审批"
									labelPosition="left"
									checked={form.getValues().is_public}
									onChange={(value) => {
										form.setFieldValue(
											"is_public",
											value.target.checked
										);
									}}
								/>
								<Switch
									label="是否需要 WP"
									description="若为是，则比赛页面中将显示提交 WP 入口"
									labelPosition="left"
									checked={form.getValues().is_need_write_up}
									onChange={(value) => {
										form.setFieldValue(
											"is_need_write_up",
											value.target.checked
										);
									}}
								/>
							</SimpleGrid>
							<Dropzone
								onDrop={(files: any) =>
									saveGamePoster(files[0])
								}
								onReject={() => {
									showErrNotification({
										message: "文件上传失败",
									});
								}}
								w={350}
								mah={256}
								accept={[
									"image/png",
									"image/gif",
									"image/jpeg",
									"image/webp",
									"image/avif",
									"image/heic",
								]}
								styles={{
									root: {
										height: "200px",
										padding: 20,
									},
								}}
							>
								<Center
									style={{
										pointerEvents: "none",
									}}
								>
									{game?.poster?.name ? (
										<Center>
											<Image
												mah={160}
												w={310}
												fit="contain"
												src={`${import.meta.env.VITE_BASE_API}/media/games/${game?.id}/poster/${game?.poster?.name}`}
											/>
										</Center>
									) : (
										<Center h={200}>
											<Stack gap={0}>
												<Text size="xl" inline>
													拖拽或点击上传头像
												</Text>
												<Text
													size="sm"
													c="dimmed"
													inline
													mt={7}
												>
													图片大小不超过 3MB
												</Text>
											</Stack>
										</Center>
									)}
								</Center>
							</Dropzone>
						</Flex>
						<SimpleGrid cols={3}>
							<NumberInput
								label="一血奖励（%）"
								description="每道题目做出来的第一支队伍，奖励此时分值的百分比"
								key={form.key("first_blood_reward_ratio")}
								{...form.getInputProps(
									"first_blood_reward_ratio"
								)}
							/>
							<NumberInput
								label="二血奖励（%）"
								key={form.key("second_blood_reward_ratio")}
								description="每道题目做出来的第二支队伍，奖励此时分值的百分比"
								{...form.getInputProps(
									"second_blood_reward_ratio"
								)}
							/>
							<NumberInput
								label="三血奖励（%）"
								key={form.key("third_blood_reward_ratio")}
								description="每道题目做出来的第三支队伍，奖励此时分值的百分比"
								{...form.getInputProps(
									"third_blood_reward_ratio"
								)}
							/>
						</SimpleGrid>
						<Textarea
							label="简介"
							description="展示在比赛页面的简短介绍"
							key={form.key("bio")}
							{...form.getInputProps("bio")}
						/>
						<Textarea
							label="详情"
							description="展示在比赛详情页的介绍"
							minRows={12}
							maxRows={12}
							resize="vertical"
							autosize
							key={form.key("description")}
							{...form.getInputProps("description")}
						/>
						<Flex justify={"end"}>
							<Button
								type="submit"
								size="md"
								leftSection={<MDIcon c={"white"}>check</MDIcon>}
							>
								保存
							</Button>
						</Flex>
					</Stack>
				</form>
			</Stack>
		</>
	);
}

export default withGameEdit(Page);
