import MDIcon from "@/components/ui/MDIcon";
import { Button, Divider, Stack, StackProps } from "@mantine/core";
import { useLocation, useNavigate, useParams } from "react-router-dom";

interface ChallengeEditSidebarProps extends StackProps {}

export default function ChallengeEditSidebar(props: ChallengeEditSidebarProps) {
	const { ...stackProps } = props;

	const { id } = useParams<{ id: string }>();
	const location = useLocation();
	const path = location.pathname.split(`/admin/challenges/${id}`)[1];
	const navigate = useNavigate();

	return (
		<Stack w={150} {...stackProps}>
			<Button
				size="md"
				leftSection={<MDIcon>arrow_back</MDIcon>}
				onClick={() => navigate(`/admin/challenges`)}
			>
				返回上级
			</Button>
			<Divider my={5} />
			<Stack gap={10}>
				<Button
					variant={path === "" ? "filled" : "subtle"}
					onClick={() => navigate(`/admin/challenges/${id}`)}
					leftSection={<MDIcon>info</MDIcon>}
				>
					基本信息
				</Button>
				<Button
					variant={path === "/flags" ? "filled" : "subtle"}
					onClick={() => navigate(`/admin/challenges/${id}/flags`)}
					leftSection={<MDIcon>flag</MDIcon>}
				>
					Flags
				</Button>
				<Button
					variant={path === "/images" ? "filled" : "subtle"}
					onClick={() => navigate(`/admin/challenges/${id}/images`)}
					leftSection={<MDIcon>package_2</MDIcon>}
				>
					镜像
				</Button>
				<Button
					variant={path === "/submissions" ? "filled" : "subtle"}
					onClick={() =>
						navigate(`/admin/challenges/${id}/submissions`)
					}
					leftSection={<MDIcon>verified</MDIcon>}
				>
					提交记录
				</Button>
			</Stack>
		</Stack>
	);
}
