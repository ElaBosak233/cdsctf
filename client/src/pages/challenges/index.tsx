import { useChallengeApi } from "@/api/challenge";
import ChallengeModal from "@/components/modals/ChallengeModal";
import MDIcon from "@/components/ui/MDIcon";
import ChallengeCard from "@/components/widgets/ChallengeCard";
import { useCategoryStore } from "@/stores/category";
import { useConfigStore } from "@/stores/config";
import { Challenge } from "@/types/challenge";
import {
	ActionIcon,
	Box,
	Button,
	Flex,
	Group,
	Input,
	Pagination,
	Select,
	Stack,
	UnstyledButton,
	rgba,
} from "@mantine/core";
import { useDisclosure } from "@mantine/hooks";
import { showNotification } from "@mantine/notifications";
import { useEffect, useState } from "react";

export default function Page() {
	const configStore = useConfigStore();
	const categoryStore = useCategoryStore();
	const challengeApi = useChallengeApi();

	const [challenges, setChallenges] = useState<Array<Challenge>>([]);
	const [search, setSearch] = useState<string>("");
	const [searchInput, setSearchInput] = useState<string>("");
	const [rowsPerPage, setRowsPerPage] = useState<number>(20);
	const [page, setPage] = useState<number>(1);
	const [total, setTotal] = useState<number>(0);
	const [selectedCategory, setSelectedCategory] = useState<number>(0);
	const [sort, setSort] = useState<string>("id_desc");

	const [opened, { open, close }] = useDisclosure(false);
	const [selectedChallenge, setSelectedChallenge] = useState<Challenge>();

	useEffect(() => {
		document.title = `题库 - ${configStore?.pltCfg?.site?.title}`;
	}, []);

	function getChallenges() {
		challengeApi
			.getChallenges({
				is_practicable: true,
				is_detailed: false,
				page: page,
				size: rowsPerPage,
				submission_qty: 3,
				title: search,
				category_id:
					selectedCategory === 0 ? undefined : selectedCategory,
				sort_key: sort.split("_")[0],
				sort_order: sort.split("_")[1],
			})
			.then((res) => {
				const r = res.data;
				if (r.code === 200) {
					setChallenges(r.data as Array<Challenge>);
					setTotal(r.total as number);
				}
			})
			.catch((err) => {
				if (err?.response?.status === 400) {
					showNotification({
						title: "发生了错误",
						message: `获取题目失败 ${err}`,
						color: "red",
					});
				}
			});
	}

	useEffect(() => {
		getChallenges();
	}, [page, rowsPerPage, search, selectedCategory, sort]);

	return (
		<>
			<Stack m={56}>
				<Flex align={"start"}>
					<Flex w={360} mx={36}>
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
									onClick={() => setSearch(searchInput)}
								>
									<MDIcon size={15}>search</MDIcon>
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
									leftSection={<MDIcon>extension</MDIcon>}
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
											<MDIcon>{category?.icon}</MDIcon>
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
					<Flex
						justify={"space-between"}
						direction={"column"}
						w={"150%"}
						sx={{
							minHeight: "80vh",
						}}
					>
						<Box>
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
										<ChallengeCard challenge={challenge} />
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
					</Flex>
				</Flex>
			</Stack>
			<ChallengeModal
				opened={opened}
				onClose={close}
				centered
				challenge={selectedChallenge}
				setSolved={(solved) => {
					setChallenges(
						challenges.map((c) =>
							c.id === selectedChallenge?.id
								? { ...c, solved: solved }
								: c
						)
					);
				}}
				mode="practice"
			/>
		</>
	);
}
