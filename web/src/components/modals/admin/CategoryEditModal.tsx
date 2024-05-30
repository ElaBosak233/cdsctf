import { useCategoryApi } from "@/api/category";
import MDIcon from "@/components/ui/MDIcon";
import { Category } from "@/types/category";
import { showSuccessNotification } from "@/utils/notification";
import {
	Box,
	Button,
	Card,
	Divider,
	Flex,
	Modal,
	ModalProps,
	Stack,
	TextInput,
	Text,
	ColorInput,
} from "@mantine/core";
import { useForm, zodResolver } from "@mantine/form";
import { useEffect, useState } from "react";
import { z } from "zod";

interface CategoryEditModalProps extends ModalProps {
	setRefresh: () => void;
	categoryID: number;
}

export default function CategoryEditModal(props: CategoryEditModalProps) {
	const { setRefresh, categoryID, ...modalProps } = props;
	const categoryApi = useCategoryApi();

	const [category, setCategory] = useState<Category>();

	const form = useForm({
		mode: "controlled",
		initialValues: {
			name: "",
			color: "",
			icon: "",
		},
		validate: zodResolver(
			z.object({
				name: z.string(),
			})
		),
	});

	function getCategory() {
		categoryApi.getCategories().then((res) => {
			const r = res.data;
			setCategory(r?.data?.find((c: Category) => c.id === categoryID));
		});
	}

	function updateCategory() {
		categoryApi
			.updateCategory({
				id: categoryID,
				name: form.getValues().name,
				icon: form.getValues().icon,
				color: form.getValues().color,
			})
			.then((_) => {
				showSuccessNotification({
					message: `分类 ${form.getValues().name} 更新成功`,
				});
				setRefresh();
				modalProps.onClose();
			});
	}

	useEffect(() => {
		form.reset();
		if (modalProps.opened) {
			getCategory();
		}
	}, [modalProps.opened]);

	useEffect(() => {
		if (category) {
			form.setValues({
				name: category.name,
				color: category.color,
				icon: category.icon,
			});
		}
	}, [category]);

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
							<MDIcon>collections_bookmark</MDIcon>
							<Text fw={600}>更新分类</Text>
						</Flex>
						<Divider my={10} />
						<Box p={10}>
							<form
								onSubmit={form.onSubmit((_) =>
									updateCategory()
								)}
							>
								<Stack gap={10}>
									<TextInput
										label="分类名"
										withAsterisk
										key={form.key("name")}
										{...form.getInputProps("name")}
									/>
									<ColorInput
										label="颜色"
										key={form.key("color")}
										{...form.getInputProps("color")}
									/>
									<TextInput
										label="图标"
										withAsterisk
										leftSection={
											<MDIcon>
												{form.getValues().icon}
											</MDIcon>
										}
										key={form.key("icon")}
										{...form.getInputProps("icon")}
									/>
								</Stack>
								<Flex mt={20} justify={"end"}>
									<Button
										type="submit"
										leftSection={
											<MDIcon c={"white"}>check</MDIcon>
										}
									>
										保存
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
