import ChallengeEditSidebar from "@/components/navigations/admin/ChallengeEditSidebar";
import { Flex, Paper } from "@mantine/core";

export default function withChallengeEdit(
	WrappedComponent: React.ComponentType<any>
) {
	return function withChallengeEdit(props: any) {
		return (
			<>
				<Flex my={56} mx={"10%"}>
					<ChallengeEditSidebar />
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
