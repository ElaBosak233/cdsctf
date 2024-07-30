import MDIcon from "@/components/ui/MDIcon";
import { Button, Divider, Stack, StackProps } from "@mantine/core";
import { useLocation, useNavigate, useParams } from "react-router-dom";

interface GameEditSidebarProps extends StackProps {}

export default function GameEditSidebar(props: GameEditSidebarProps) {
    const { ...stackProps } = props;

    const { id } = useParams<{ id: string }>();
    const location = useLocation();
    const path = location.pathname.split(`/admin/games/${id}`)[1];
    const navigate = useNavigate();

    return (
        <Stack w={150} {...stackProps}>
            <Button
                size="md"
                leftSection={<MDIcon c={"white"}>arrow_back</MDIcon>}
                onClick={() => navigate(`/admin/games`)}
            >
                返回上级
            </Button>
            <Divider my={5} />
            <Stack gap={10}>
                <Button
                    variant={path === "" ? "filled" : "subtle"}
                    onClick={() => navigate(`/admin/games/${id}`)}
                    leftSection={
                        <MDIcon c={path === "" ? "white" : "brand"}>
                            info
                        </MDIcon>
                    }
                >
                    基本信息
                </Button>
                <Button
                    variant={path === "/challenges" ? "filled" : "subtle"}
                    onClick={() => navigate(`/admin/games/${id}/challenges`)}
                    leftSection={
                        <MDIcon c={path === "/challenges" ? "white" : "brand"}>
                            collections_bookmark
                        </MDIcon>
                    }
                >
                    题目
                </Button>
                <Button
                    variant={path === "/teams" ? "filled" : "subtle"}
                    onClick={() => navigate(`/admin/games/${id}/teams`)}
                    leftSection={
                        <MDIcon c={path === "/teams" ? "white" : "brand"}>
                            people
                        </MDIcon>
                    }
                >
                    参赛团队
                </Button>
                <Button
                    variant={path === "/submissions" ? "filled" : "subtle"}
                    onClick={() => navigate(`/admin/games/${id}/submissions`)}
                    leftSection={
                        <MDIcon c={path === "/submissions" ? "white" : "brand"}>
                            verified
                        </MDIcon>
                    }
                >
                    提交记录
                </Button>
                <Button
                    variant={path === "/notices" ? "filled" : "subtle"}
                    onClick={() => navigate(`/admin/games/${id}/notices`)}
                    leftSection={
                        <MDIcon c={path === "/notices" ? "white" : "brand"}>
                            campaign
                        </MDIcon>
                    }
                >
                    公告
                </Button>
            </Stack>
        </Stack>
    );
}
