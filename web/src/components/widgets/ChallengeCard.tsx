import { Challenge } from "@/types/challenge";
import {
	Badge,
	Box,
	Text,
	Card,
	rgba,
	Divider,
	Flex,
	useMantineColorScheme,
	Tooltip,
	ThemeIcon,
	useMantineTheme,
	Stack,
} from "@mantine/core";
import MDIcon from "@/components/ui/MDIcon";
import FirstBloodIcon from "@/components/icons/hexagons/FirstBloodIcon";
import SecondBloodIcon from "@/components/icons/hexagons/SecondBloodIcon";
import ThirdBloodIcon from "@/components/icons/hexagons/ThirdBloodIcon";
import { useEffect, useState } from "react";
import dayjs from "dayjs";

export default function ChallengeCard({
	challenge,
	pts,
}: {
	challenge?: Challenge;
	pts?: number;
}) {
	const { colorScheme } = useMantineColorScheme();
	const theme = useMantineTheme();

	const [color, setColor] = useState<string>(theme.colors.brand[6]);

	const [cardBgColor, setCardBgColor] = useState<string>();
	const [cardHoverBgColor, setCardHoverBgColor] = useState<string>();
	const [cardTextColor, setCardTextColor] = useState<string>();

	const bloodMap = [
		{
			name: "一血",
			icon: <FirstBloodIcon size={24} />,
		},
		{
			name: "二血",
			icon: <SecondBloodIcon size={24} />,
		},
		{
			name: "三血",
			icon: <ThirdBloodIcon size={24} />,
		},
	];

	useEffect(() => {
		setColor(challenge?.category?.color || theme.colors.brand[6]);
	}, []);

	useEffect(() => {
		switch (colorScheme) {
			case "dark":
				if (challenge?.solved) {
					setCardBgColor(rgba(color, 0.65));
					setCardHoverBgColor(rgba(color, 0.7));
					setCardTextColor("#FFF");
				} else {
					setCardBgColor(rgba(color, 0.3));
					setCardHoverBgColor(rgba(color, 0.35));
					setCardTextColor("#FFF");
				}
				break;
			case "light":
				if (challenge?.solved) {
					setCardBgColor(rgba(color, 1));
					setCardHoverBgColor(rgba(color, 0.95));
					setCardTextColor("#FFF");
				} else {
					setCardBgColor(rgba(color, 0.15));
					setCardHoverBgColor(rgba(color, 0.2));
					setCardTextColor(rgba(color, 1));
				}
				break;
		}
	}, [challenge?.solved, colorScheme, color]);

	return (
		<Card
			shadow="md"
			h={150}
			w={275}
			pos={"relative"}
			sx={{
				backgroundColor: cardBgColor,
				"&:hover": {
					backgroundColor: cardHoverBgColor,
				},
				transitionTimingFunction: "ease-out",
				transitionProperty: "background",
				transitionDuration: "0.3s",
			}}
			className="no-select"
		>
			<Box
				pos={"absolute"}
				right={0}
				bottom={-20}
				sx={{
					opacity: 0.2,
					color: cardTextColor,
				}}
			>
				<MDIcon color={cardTextColor} size={180}>
					{challenge?.category?.icon}
				</MDIcon>
			</Box>
			{challenge?.solved && (
				<Box
					pos={"absolute"}
					right={20}
					top={20}
					sx={{
						color: "#FFF",
					}}
				>
					<Tooltip label="已解决">
						<MDIcon size={30} color={"#FFF"}>
							done
						</MDIcon>
					</Tooltip>
				</Box>
			)}
			<Box>
				<Badge variant="light" color={cardTextColor}>
					{challenge?.category?.name}
				</Badge>
			</Box>
			<Box py={10}>
				<Text
					size="lg"
					c={cardTextColor}
					fw={700}
					sx={{
						width: 200,
						overflow: "hidden",
						whiteSpace: "nowrap",
						textOverflow: "ellipsis",
					}}
				>
					{challenge?.title}
				</Text>
			</Box>
			<Divider
				py={5}
				sx={{
					borderColor: cardTextColor,
				}}
			/>
			<Flex justify={"space-between"} align={"center"} px={5}>
				<Text size="lg" c={cardTextColor} fw={700}>
					{pts || challenge?.practice_pts || "?"} pts
				</Text>
				<Flex align={"center"}>
					{challenge?.submissions?.map((submission, index) => (
						<Tooltip
							multiline
							label={
								<Stack gap={0}>
									<Text size={"sm"} fw={600}>
										{submission?.team?.name ||
											submission?.user?.nickname}
									</Text>
									<Text size={"xs"}>
										{dayjs(submission?.created_at).format(
											"YYYY/MM/DD HH:mm:ss"
										)}
									</Text>
								</Stack>
							}
							withArrow
							position="bottom"
						>
							<ThemeIcon variant="transparent">
								{bloodMap[index]?.icon}
							</ThemeIcon>
						</Tooltip>
					))}
				</Flex>
			</Flex>
		</Card>
	);
}
