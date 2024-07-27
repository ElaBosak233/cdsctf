import { useChallengeApi } from "@/api/challenge";
import ChallengeModal from "@/components/modals/ChallengeModal";
import MDIcon from "@/components/ui/MDIcon";
import ChallengeCard from "@/components/widgets/ChallengeCard";
import { useAuthStore } from "@/stores/auth";
import { useCategoryStore } from "@/stores/category";
import { useConfigStore } from "@/stores/config";
import { Challenge, ChallengeStatus } from "@/types/challenge";
import { showErrNotification } from "@/utils/notification";
import {
	ActionIcon,
	Box,
	Button,
	Flex,
	Group,
	Input,
	LoadingOverlay,
	Pagination,
	Select,
	Stack,
	UnstyledButton,
} from "@mantine/core";
import { useDisclosure } from "@mantine/hooks";
import { useEffect, useState } from "react";

export default function Page() {
	const authStore = useAuthStore();
	const configStore = useConfigStore();
	const categoryStore = useCategoryStore();
	const challengeApi = useChallengeApi();

	const [refresh, setRefresh] = useState<number>(0);

	const [challenges, setChallenges] = useState<Array<Challenge>>([]);
	const [challengeStatus, setChallengeStatus] = useState<
		Record<number, ChallengeStatus>
	>([]);

	const [search, setSearch] = useState<string>("");
	const [searchInput, setSearchInput] = useState<string>("");
	const [rowsPerPage, setRowsPerPage] = useState<number>(20);
	const [page, setPage] = useState<number>(1);
	const [total, setTotal] = useState<number>(0);
	const [selectedCategory, setSelectedCategory] = useState<number>(0);
	const [sort, setSort] = useState<string>("id_desc");

	const [loading, setLoading] = useState<boolean>(true);

	const [opened, { open, close }] = useDisclosure(false);
	const [selectedChallenge, setSelectedChallenge] = useState<Challenge>();

	useEffect(() => {
		document.title = `题库 - ${configStore?.pltCfg?.site?.title}`;
	}, []);

	function getChallenges() {
		setLoading(true);
		challengeApi
			.getChallenges({
				is_practicable: true,
				is_detailed: false,
				page: page,
				size: rowsPerPage,
				title: search,
				category_id:
					selectedCategory === 0 ? undefined : selectedCategory,
				sort_key: sort.split("_")[0],
				sort_order: sort.split("_")[1],
			})
			.then((res) => {
				const r = res.data;
				setChallenges(r?.data);
				setTotal(r?.total);
			})
			.catch((err) => {
				if (err?.response?.status === 400) {
					showErrNotification({
						message: `获取题目失败 ${err}`,
					});
				}
			})
			.finally(() => {
				setLoading(false);
			});
	}

	function getChallengeStatus() {
		challengeApi
			.getChallengeStatus({
				cids: challenges.map((c) => Number(c?.id)),
				user_id: authStore.user?.id,
			})
			.then((res) => {
				const r = res.data;
				setChallengeStatus(r?.data);
			});
	}

	useEffect(() => {
		getChallenges();
	}, [page, rowsPerPage, search, selectedCategory, sort, refresh]);

	useEffect(() => {
		getChallengeStatus();
	}, [challenges]);
	return (
		<>
			<Stack m={56}>
				<Flex align={"start"}>
					<Flex w={360} mx={36} visibleFrom={"md"}>
						<Box
							sx={{
								flexGrow: 1,
							}}
						>
							<Flex justify={"space-between"} align={"center"}>
								<Input
									variant="filled"
									placeholder="搜索"
									mr={5}
									value={searchInput}
									onChange={(e) =>
										setSearchInput(e.target.value)
									}
									sx={{
										flexGrow: 1,
									}}
								/>
								<ActionIcon
									variant={"filled"}
									onClick={() => setSearch(searchInput)}
								>
									<MDIcon size={15} c={"white"}>
										search
									</MDIcon>
								</ActionIcon>
							</Flex>
							<Select
								label="每页显示"
								description="选择每页显示的题目数量"
								value={String(rowsPerPage)}
								allowDeselect={false}
								data={["20", "25", "50", "100"]}
								onChange={(_, option) =>
									setRowsPerPage(Number(option.value))
								}
								mt={15}
							/>
							<Box my={15}>
								<Button
									mt={5}
									size="md"
									h={49}
									fullWidth
									justify="center"
									variant={
										selectedCategory === 0
											? "filled"
											: "subtle"
									}
									leftSection={
										<MDIcon
											c={
												selectedCategory === 0
													? "white"
													: "brand"
											}
										>
											extension
										</MDIcon>
									}
									onClick={() => setSelectedCategory(0)}
									color="brand"
								>
									ALL
								</Button>
								{categoryStore?.categories?.map((category) => (
									<Button
										key={category?.id}
										mt={5}
										size="md"
										h={49}
										fullWidth
										justify="center"
										variant={
											selectedCategory === category?.id
												? "filled"
												: "subtle"
										}
										leftSection={
											<MDIcon
												c={
													selectedCategory ===
													category?.id
														? "white"
														: category?.color
												}
											>
												{category?.icon}
											</MDIcon>
										}
										onClick={() =>
											setSelectedCategory(category?.id!)
										}
										color={category?.color}
									>
										{category?.name?.toUpperCase()}
									</Button>
								))}
							</Box>
							<Select
								label="排序"
								description="选择题目排序方式"
								value={sort}
								allowDeselect={false}
								data={[
									{
										label: "ID 降序",
										value: "id_desc",
									},
									{
										label: "ID 升序",
										value: "id_asc",
									},
									{
										label: "难度降序",
										value: "difficulty_desc",
									},
									{
										label: "难度升序",
										value: "difficulty_asc",
									},
								]}
								onChange={(_, option) => setSort(option.value)}
								mt={15}
							/>
						</Box>
					</Flex>
					<Stack w={"120%"}>
						<Box mih={"calc(100vh - 260px)"} pos={"relative"}>
							<LoadingOverlay visible={loading} zIndex={2} />
							<Group
								gap={"lg"}
								sx={{
									flexGrow: 1,
								}}
							>
								{challenges?.map((challenge) => (
									<UnstyledButton
										onClick={() => {
											open();
											setSelectedChallenge(challenge);
										}}
										key={challenge?.id}
									>
										<ChallengeCard
											challenge={challenge}
											status={
												challengeStatus?.[
													Number(challenge?.id)
												]
											}
										/>
									</UnstyledButton>
								))}
							</Group>
						</Box>
						<Flex justify={"center"} mt={30}>
							<Pagination
								total={Math.ceil(total / rowsPerPage)}
								value={page}
								onChange={setPage}
								size="md"
								withEdges
							/>
						</Flex>
					</Stack>
				</Flex>
			</Stack>
			<ChallengeModal
				opened={opened}
				onClose={close}
				centered
				challenge={selectedChallenge}
				setRefresh={() => setRefresh((prev) => prev + 1)}
				mode="practice"
			/>
		</>
	);
}
