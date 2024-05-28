import GameEditSidebar from "@/components/navigations/admin/GameEditSidebar";
import { Flex, Paper } from "@mantine/core";

export default function withGameEdit(
	WrappedComponent: React.ComponentType<any>
) {
	return function withGameEdit(props: any) {
		return (
			<>
				<Flex my={56} mx={"10%"}>
					<GameEditSidebar />
					<Paper
						mx={36}
						mih={"calc(100vh - 180px)"}
						sx={{
							flexGrow: 1,
						}}
					>
						<WrappedComponent {...props} />
					</Paper>
				</Flex>
			</>
		);
	};
}
