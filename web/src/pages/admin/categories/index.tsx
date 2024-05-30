import MDIcon from "@/components/ui/MDIcon";
import {
	Flex,
	Stack,
	ActionIcon,
	Paper,
	Table,
	Badge,
	Group,
	Text,
	ColorSwatch,
} from "@mantine/core";
import { useEffect, useState } from "react";
import { Category } from "@/types/category";
import { useCategoryApi } from "@/api/category";

export default function Page() {
	const categoryApi = useCategoryApi();
	const [categories, setCategories] = useState<Array<Category>>([]);

	function getCategories() {
		categoryApi.getCategories().then((res) => {
			const r = res.data;
			setCategories(r?.data);
		});
	}

	useEffect(() => {
		getCategories();
	}, []);

	return (
		<>
			<Flex my={36} mx={"10%"} justify={"center"}>
				<Stack
					align="center"
					gap={36}
					component={Paper}
					mih={"calc(100vh - 10rem)"}
					sx={{
						flexGrow: 1,
					}}
				>
					<Table stickyHeader horizontalSpacing={"md"} striped>
						<Table.Thead>
							<Table.Tr
								sx={{
									lineHeight: 3,
								}}
							>
								<Table.Th>#</Table.Th>
								<Table.Th>分类名</Table.Th>
								<Table.Th>颜色（Hex）</Table.Th>
								<Table.Th>图标</Table.Th>
								<Table.Th>
									<Flex justify={"center"}>
										<ActionIcon variant="transparent">
											<MDIcon>add</MDIcon>
										</ActionIcon>
									</Flex>
								</Table.Th>
							</Table.Tr>
						</Table.Thead>
						<Table.Tbody>
							{categories?.map((category) => (
								<Table.Tr key={category?.id}>
									<Table.Th>
										<Badge>{category?.id}</Badge>
									</Table.Th>
									<Table.Th>
										{category?.name?.toUpperCase()}
									</Table.Th>
									<Table.Th>
										<Group>
											<ColorSwatch
												color={
													category?.color || "brand"
												}
											/>
											<Text c={category?.color} fw={600}>
												{category?.color?.toUpperCase()}
											</Text>
										</Group>
									</Table.Th>
									<Table.Th>
										<Group>
											<MDIcon>{category?.icon}</MDIcon>
											<Text>{category?.icon}</Text>
										</Group>
									</Table.Th>
									<Table.Th>
										<Group justify="center">
											<ActionIcon variant="transparent">
												<MDIcon>edit</MDIcon>
											</ActionIcon>
											<ActionIcon variant="transparent">
												<MDIcon color={"red"}>
													delete
												</MDIcon>
											</ActionIcon>
										</Group>
									</Table.Th>
								</Table.Tr>
							))}
						</Table.Tbody>
					</Table>
				</Stack>
			</Flex>
		</>
	);
}
