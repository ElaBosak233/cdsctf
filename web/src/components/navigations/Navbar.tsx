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
    Stack,
    Indicator,
    Tooltip,
} from "@mantine/core";
import { Link, useNavigate } from "react-router-dom";
import MDIcon from "@/components/ui/MDIcon";
import { useAuthStore } from "@/stores/auth";
import { useConfigStore } from "@/stores/config";
import { useWsrxStore } from "@/stores/wsrx";

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
    const navigate = useNavigate();
    const { colorScheme, setColorScheme } = useMantineColorScheme({
        keepTransitions: true,
    });

    function logout() {
        authStore.setPgsToken("");
        authStore.setUser(undefined);
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
                            src={"/api/configs/favicon"}
                            fallbackSrc={"/favicon.ico"}
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
                    <Menu
                        shadow="md"
                        width={250}
                        offset={20}
                        withArrow
                        radius={"md"}
                    >
                        <Menu.Target>
                            <Avatar
                                src={`/api/users/${authStore?.user?.id}/avatar`}
                                color="white"
                                sx={{
                                    "&:hover": {
                                        cursor: "pointer",
                                    },
                                }}
                            >
                                <MDIcon color={"white"}>person</MDIcon>
                            </Avatar>
                        </Menu.Target>
                        <Menu.Dropdown>
                            <Menu.Item
                                h={75}
                                leftSection={
                                    <Avatar
                                        color={"brand"}
                                        size={50}
                                        src={`/api/users/${authStore?.user?.id}/avatar`}
                                    >
                                        <MDIcon size={30}>person</MDIcon>
                                    </Avatar>
                                }
                                onClick={() => {}}
                            >
                                <Stack gap={0}>
                                    <Text fw={600}>
                                        {authStore?.user?.nickname}
                                    </Text>
                                    <Text fz={"xs"} c={"gray"}>
                                        {authStore?.user?.username} #
                                        {authStore?.user?.id}
                                    </Text>
                                </Stack>
                            </Menu.Item>
                            <Menu.Divider />
                            <Menu.Item
                                leftSection={<MDIcon>link</MDIcon>}
                                onClick={() => navigate("/wsrx")}
                                pos={"relative"}
                            >
                                连接器设置
                                <Tooltip label={"离线"} withArrow offset={10}>
                                    <Indicator
                                        color={
                                            useWsrxStore.getState().status ===
                                            "online"
                                                ? "green"
                                                : useWsrxStore.getState()
                                                        .status === "offline"
                                                  ? "red"
                                                  : "orange"
                                        }
                                        processing
                                        sx={{
                                            position: "absolute",
                                            right: 20,
                                            top: "50%",
                                        }}
                                    />
                                </Tooltip>
                            </Menu.Item>
                            <Menu.Divider />
                            <Flex>
                                <Menu.Item
                                    leftSection={
                                        <MDIcon>manage_accounts</MDIcon>
                                    }
                                    onClick={() => navigate("/profile")}
                                >
                                    个人设置
                                </Menu.Item>
                                <Menu.Item
                                    color={"red"}
                                    leftSection={
                                        <MDIcon color={"red"}>logout</MDIcon>
                                    }
                                    onClick={logout}
                                >
                                    退出登录
                                </Menu.Item>
                            </Flex>
                        </Menu.Dropdown>
                    </Menu>
                )}
            </Flex>
        </Flex>
    );
}
