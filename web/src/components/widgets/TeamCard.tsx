import { Team } from "@/types/team";
import {
	Avatar,
	Box,
	Card,
	Flex,
	ThemeIcon,
	alpha,
	Text,
	Stack,
	Tooltip,
	Divider,
} from "@mantine/core";
import MDIcon from "@/components/ui/MDIcon";

export default function TeamCard({ team }: { team?: Team }) {
	return (
		<Card
			shadow="md"
			h={200}
			w={380}
			p={25}
			radius={"md"}
			sx={{
				position: "relative",
			}}
			className="no-select"
		>
			<Flex gap={15} align={"center"}>
				<Avatar
					src={`${import.meta.env.VITE_BASE_API}/media/teams/${team?.id}/${team?.avatar?.name}`}
					size={"xl"}
					color="brand"
				>
					<MDIcon size={40}>people</MDIcon>
				</Avatar>
				<Stack gap={5}>
					<Text size="2rem" fw={600}>
						{team?.name}
					</Text>
					<Text>{team?.description}</Text>
				</Stack>
			</Flex>
			<Divider my={15} />
			<Flex
				justify={"end"}
				sx={{
					zIndex: 2,
				}}
			>
				<Tooltip.Group>
					<Avatar.Group spacing="sm">
						{team?.users?.map((user) => (
							<Tooltip
								key={user?.id}
								label={user?.nickname}
								withArrow
							>
								<Avatar
									color="brand"
									src={`${import.meta.env.VITE_BASE_API}/media/users/${user?.id}/${user?.avatar?.name}`}
									radius="xl"
								>
									<MDIcon>person</MDIcon>
								</Avatar>
							</Tooltip>
						))}
					</Avatar.Group>
				</Tooltip.Group>
			</Flex>
			<Box
				sx={{
					position: "absolute",
					right: 50,
					bottom: 10,
				}}
			>
				<MDIcon
					c={alpha("var(--mantine-color-gray-1)", 0.2)}
					size={120}
				>
					people
				</MDIcon>
			</Box>
		</Card>
	);
}
