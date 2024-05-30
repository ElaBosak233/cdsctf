import { useUserApi } from "@/api/user";
import MDIcon from "@/components/ui/MDIcon";
import { User } from "@/types/user";
import {
	Card,
	Divider,
	Flex,
	Modal,
	ModalProps,
	TextInput,
	ThemeIcon,
	Text,
	Stack,
	Avatar,
	Group,
	ActionIcon,
	Pagination,
} from "@mantine/core";
import { useEffect, useState } from "react";

interface UserSelectModalProps extends ModalProps {
	setUser: (user: User) => void;
}

export default function UserSelectModal(props: UserSelectModalProps) {
	const { setUser, ...modalProps } = props;

	const userApi = useUserApi();
	const [users, setUsers] = useState<Array<User>>([]);
	const [search, setSearch] = useState<string>("");
	const [page, setPage] = useState<number>(1);
	const [total, setTotal] = useState<number>(0);
	const [rowsPerPage, _] = useState<number>(10);

	function getUsers() {
		userApi
			.getUsers({
				size: 10,
				page: page,
				name: search,
			})
			.then((res) => {
				const r = res.data;
				setUsers(r.data);
				setTotal(r.total);
			});
	}

	useEffect(() => {
		getUsers();
	}, [search, page]);

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
							<MDIcon>person</MDIcon>
							<Text fw={600}>选择用户</Text>
						</Flex>
						<Divider my={10} />
						<Stack p={10} gap={20} align="center">
							<TextInput
								label="搜索"
								value={search}
								onChange={(e) => setSearch(e.target.value)}
								w={"100%"}
							/>
							<Stack w={"100%"}>
								{users?.map((user) => (
									<Flex
										key={user?.id}
										justify={"space-between"}
										align={"center"}
									>
										<Group gap={15}>
											<Avatar
												color="brand"
												src={`${import.meta.env.VITE_BASE_API}/media/users/${user?.id}/${user?.avatar?.name}`}
												radius="xl"
											>
												<MDIcon>person</MDIcon>
											</Avatar>
											<Text fw={700} size="1rem">
												{user?.username}
											</Text>
											<Text fw={500} size="1rem">
												{user?.nickname}
											</Text>
										</Group>
										<ActionIcon
											variant="transparent"
											onClick={() => {
												setUser(user);
												modalProps.onClose();
											}}
										>
											<MDIcon>check</MDIcon>
										</ActionIcon>
									</Flex>
								))}
							</Stack>
							<Pagination
								total={Math.ceil(total / rowsPerPage)}
								value={page}
								onChange={setPage}
							/>
						</Stack>
					</Card>
				</Modal.Content>
			</Modal.Root>
		</>
	);
}
