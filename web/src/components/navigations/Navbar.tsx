import {
	Button,
	Box,
	Avatar,
	Menu,
	Flex,
	Image,
	Title,
	Group,
	Text,
	ActionIcon,
	useMantineColorScheme,
	Burger,
} from "@mantine/core";
import { Link, useNavigate } from "react-router-dom";
import MDIcon from "@/components/ui/MDIcon";
import { useAuthStore } from "@/stores/auth";
import { useConfigStore } from "@/stores/config";
import { useTeamStore } from "@/stores/team";

export const NavItems = [
	{
		name: "题库",
		path: "/challenges",
		icon: "collections_bookmark",
	},
	{
		name: "比赛",
		path: "/games",
		icon: "flag",
	},
	{
		name: "团队",
		path: "/teams",
		icon: "people",
	},
];

export const AdminNavItems = [
	{
		name: "全局",
		path: "/admin/global",
		icon: "settings",
	},
	{
		name: "题库",
		path: "/admin/challenges",
		icon: "collections_bookmark",
	},
	{
		name: "比赛",
		path: "/admin/games",
		icon: "flag",
	},
	{
		name: "团队",
		path: "/admin/teams",
		icon: "people",
	},
	{
		name: "用户",
		path: "/admin/users",
		icon: "person",
	},
];

interface NavbarProps {
	burger?: {
		opened: boolean;
		toggle: () => void;
	};
	adminMode?: boolean;
}

export default function Navbar(props: NavbarProps) {
	const { burger, adminMode } = props;
	const authStore = useAuthStore();
	const configStore = useConfigStore();
	const teamStore = useTeamStore();
	const navigate = useNavigate();
	const { colorScheme, setColorScheme } = useMantineColorScheme({
		keepTransitions: true,
	});

	function logout() {
		authStore.setPgsToken("");
		authStore.setUser(undefined);
		teamStore.setSelectedTeamID(0);
		navigate("/login");
	}

	return (
		<Flex
			h={64}
			w={"100%"}
			bg={"brand"}
			px={20}
			display={"flex"}
			justify={"space-between"}
			align={"center"}
			pos={"fixed"}
			sx={{
				top: 0,
				zIndex: 2,
			}}
		>
			<Group w={"50%"} wrap={"nowrap"} gap={0}>
				<Burger
					opened={burger?.opened}
					onClick={burger?.toggle}
					hiddenFrom={"md"}
					size={"sm"}
					color={"white"}
				/>
				<Button
					h={48}
					component={Link}
					variant={"transparent"}
					to={"/"}
					draggable={false}
				>
					<Flex align={"center"}>
						<Image
							src="/favicon.ico"
							alt=""
							w={36}
							h={36}
							draggable={false}
						/>
						<Title
							px={10}
							order={3}
							sx={{
								color: "white",
							}}
							visibleFrom={"xs"}
						>
							{configStore?.pltCfg?.site?.title}
						</Title>
					</Flex>
				</Button>
			</Group>
			<Box sx={{ flexShrink: 0 }}>
				<Group visibleFrom={"md"}>
					{!adminMode && (
						<>
							{NavItems?.map((item) => (
								<Button
									key={item?.name}
									component={Link}
									variant={"transparent"}
									c={"white"}
									leftSection={
										<MDIcon color={"white"}>
											{item?.icon}
										</MDIcon>
									}
									draggable={false}
									to={item?.path}
								>
									{item?.name}
								</Button>
							))}
							{authStore?.user?.group === "admin" && (
								<Button
									variant={"transparent"}
									c={"white"}
									component={Link}
									leftSection={
										<MDIcon color={"white"}>
											settings
										</MDIcon>
									}
									draggable={false}
									to={"/admin"}
								>
									管理
								</Button>
							)}
						</>
					)}
					{adminMode && (
						<>
							<Button
								component={Link}
								variant={"transparent"}
								c={"white"}
								leftSection={
									<MDIcon color={"white"}>
										keyboard_return
									</MDIcon>
								}
								draggable={false}
								to={"/"}
							>
								返回
							</Button>
							{AdminNavItems?.map((item) => (
								<Button
									key={item?.name}
									component={Link}
									variant={"transparent"}
									c={"white"}
									leftSection={
										<MDIcon color={"white"}>
											{item?.icon}
										</MDIcon>
									}
									draggable={false}
									to={item?.path}
								>
									{item?.name}
								</Button>
							))}
						</>
					)}
				</Group>
			</Box>
			<Flex w={"50%"} justify={"end"} align={"center"}>
				<ActionIcon
					aria-label="Settings"
					c={"white"}
					mx={10}
					onClick={() => {
						setColorScheme(
							colorScheme === "dark" ? "light" : "dark"
						);
					}}
				>
					<MDIcon color={"white"}>
						{colorScheme === "dark" ? "light_mode" : "dark_mode"}
					</MDIcon>
				</ActionIcon>
				{!authStore?.user && (
					<Avatar
						color="white"
						sx={{
							"&:hover": {
								cursor: "pointer",
							},
						}}
						onClick={() => navigate("/login")}
					>
						<span className="material-symbols-rounded">person</span>
					</Avatar>
				)}
				{authStore?.user && (
					<Menu shadow="md" width={200} offset={20} withArrow>
						<Menu.Target>
							<Avatar
								src={
									authStore.user?.avatar?.name
										? `${import.meta.env.VITE_BASE_API}/media/users/${authStore?.user?.id}/${authStore?.user?.avatar?.name}`
										: undefined
								}
								color="white"
								sx={{
									"&:hover": {
										cursor: "pointer",
									},
								}}
							>
								<span className="material-symbols-rounded">
									person
								</span>
							</Avatar>
						</Menu.Target>
						<Menu.Dropdown>
							<Menu.Item
								color={"brand"}
								leftSection={<MDIcon>person</MDIcon>}
								onClick={() => navigate("/profile")}
							>
								<Text fw={600}>
									{authStore?.user?.nickname}
								</Text>
							</Menu.Item>
							<Menu.Divider />
							<Menu.Item
								color={"red"}
								leftSection={
									<MDIcon color={"red"}>logout</MDIcon>
								}
								onClick={logout}
							>
								退出
							</Menu.Item>
						</Menu.Dropdown>
					</Menu>
				)}
			</Flex>
		</Flex>
	);
}
