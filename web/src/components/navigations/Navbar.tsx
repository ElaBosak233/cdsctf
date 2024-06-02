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
} from "@mantine/core";
import { useLocation, useNavigate } from "react-router-dom";
import MDIcon from "@/components/ui/MDIcon";
import { useAuthStore } from "@/stores/auth";
import { useConfigStore } from "@/stores/config";
import { useEffect, useState } from "react";
import { useTeamStore } from "@/stores/team";

export default function Navbar() {
	const authStore = useAuthStore();
	const configStore = useConfigStore();
	const teamStore = useTeamStore();
	const navigate = useNavigate();
	const { colorScheme, setColorScheme } = useMantineColorScheme({
		keepTransitions: true,
	});

	const [adminMode, setAdminMode] = useState<boolean>(false);
	const location = useLocation();

	function logout() {
		authStore.setPgsToken("");
		authStore.setUser(undefined);
		teamStore.setSelectedTeamID(0);
		navigate("/login");
	}

	useEffect(() => {
		setAdminMode(false);
		if (location.pathname.startsWith("/admin")) {
			setAdminMode(true);
		}
	}, [location.pathname]);

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
			<Box
				w={"50%"}
				sx={{
					display: "flex",
					justifyContent: "start",
				}}
			>
				<Button
					h={48}
					sx={{
						backgroundColor: "transparent",
						"&:hover": {
							backgroundColor: "transparent",
						},
					}}
					onClick={() => navigate("/")}
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
						>
							{configStore?.pltCfg?.site?.title}
						</Title>
					</Flex>
				</Button>
			</Box>
			<Group
				sx={{
					flexShrink: 0,
				}}
			>
				{!adminMode && (
					<>
						<Button
							sx={{
								backgroundColor: "transparent",
								"&:hover": {
									backgroundColor: "transparent",
								},
							}}
							leftSection={
								<MDIcon color={"white"}>
									collections_bookmark
								</MDIcon>
							}
							onClick={() => navigate("/challenges")}
						>
							题库
						</Button>
						<Button
							sx={{
								backgroundColor: "transparent",
								"&:hover": {
									backgroundColor: "transparent",
								},
							}}
							leftSection={<MDIcon color={"white"}>flag</MDIcon>}
							onClick={() => navigate("/games")}
						>
							比赛
						</Button>
						<Button
							sx={{
								backgroundColor: "transparent",
								"&:hover": {
									backgroundColor: "transparent",
								},
							}}
							leftSection={
								<MDIcon color={"white"}>people</MDIcon>
							}
							onClick={() => navigate("/teams")}
						>
							团队
						</Button>
						{authStore?.user?.group === "admin" && (
							<Button
								sx={{
									backgroundColor: "transparent",
									"&:hover": {
										backgroundColor: "transparent",
									},
								}}
								leftSection={
									<MDIcon color={"white"}>settings</MDIcon>
								}
								onClick={() => navigate("/admin")}
							>
								管理
							</Button>
						)}
					</>
				)}
				{adminMode && (
					<>
						<Button
							sx={{
								backgroundColor: "transparent",
								"&:hover": {
									backgroundColor: "transparent",
								},
							}}
							leftSection={
								<MDIcon color={"white"}>keyboard_return</MDIcon>
							}
							onClick={() => navigate("/")}
						>
							返回
						</Button>
						<Button
							sx={{
								backgroundColor: "transparent",
								"&:hover": {
									backgroundColor: "transparent",
								},
							}}
							leftSection={
								<MDIcon color={"white"}>settings</MDIcon>
							}
							onClick={() => navigate("/admin/global")}
						>
							全局
						</Button>
						<Button
							sx={{
								backgroundColor: "transparent",
								"&:hover": {
									backgroundColor: "transparent",
								},
							}}
							leftSection={
								<MDIcon color={"white"}>
									collections_bookmark
								</MDIcon>
							}
							onClick={() => navigate("/admin/challenges")}
						>
							题库
						</Button>
						<Button
							sx={{
								backgroundColor: "transparent",
								"&:hover": {
									backgroundColor: "transparent",
								},
							}}
							leftSection={<MDIcon color={"white"}>flag</MDIcon>}
							onClick={() => navigate("/admin/games")}
						>
							比赛
						</Button>
						<Button
							sx={{
								backgroundColor: "transparent",
								"&:hover": {
									backgroundColor: "transparent",
								},
							}}
							leftSection={
								<MDIcon color={"white"}>people</MDIcon>
							}
							onClick={() => navigate("/admin/teams")}
						>
							团队
						</Button>
						<Button
							sx={{
								backgroundColor: "transparent",
								"&:hover": {
									backgroundColor: "transparent",
								},
							}}
							leftSection={
								<MDIcon color={"white"}>person</MDIcon>
							}
							onClick={() => navigate("/admin/users")}
						>
							用户
						</Button>
					</>
				)}
			</Group>
			<Flex w={"50%"} justify={"end"} align={"center"}>
				<ActionIcon
					variant="transparent"
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
								c={"brand"}
								leftSection={<MDIcon>person</MDIcon>}
								onClick={() => navigate("/profile")}
							>
								<Text fw={600}>
									{authStore?.user?.nickname}
								</Text>
							</Menu.Item>
							<Menu.Divider />
							<Menu.Item
								c={"red"}
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
