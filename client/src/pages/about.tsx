import ChallengeDialog from "@/components/modals/ChallengeModal";
import ChallengeCard from "@/components/widgets/ChallengeCard";
import GameCard from "@/components/widgets/GameCard";
import { Box, Divider, Flex, Group } from "@mantine/core";

export default function Page() {
	return (
		<Flex justify={"center"}>
			<Box
				my={56}
				sx={{
					width: "80%",
				}}
			>
				<Group gap={"lg"}>
					<ChallengeCard
						challenge={{
							category: {
								color: "#3F51B5",
								icon: "fingerprint",
								name: "misc",
							},
							title: "测试题目",
							description: "测试描述",
							submissions: [],
							is_enabled: true,
							pts: 200,
							solved: true,
						}}
					/>
					<ChallengeCard
						challenge={{
							category: {
								color: "#3F51B5",
								icon: "fingerprint",
								name: "misc",
							},
							title: "测试题目",
							description: "测试描述",
							submissions: [],
							is_enabled: true,
							pts: 200,
							solved: false,
						}}
					/>
				</Group>
				<Divider my={20} />
				<ChallengeDialog
					challenge={{
						id: 1,
						category: {
							color: "#3F51B5",
							icon: "fingerprint",
							name: "misc",
						},
						title: "测试题目",
						description: `# Heading1 \n ## Heading2 \n ### Heading3 \n \`code here\`\n \`\`\`python\nprint('p')\n\`\`\`\n 222`,
						is_enabled: true,
						pts: 200,
						is_dynamic: false,
						submissions: [
							{
								id: 1,
								user: {
									id: 1,
									username: "test",
									email: "111",
									nickname: "test",
								},
								team: {
									id: 1,
									name: "test",
								},
							},
						],
					}}
				/>
				<Divider my={20} />
				<GameCard
					game={{
						title: "THUCTF 2023",
						description: "草草草",
					}}
				/>
			</Box>
		</Flex>
	);
}
