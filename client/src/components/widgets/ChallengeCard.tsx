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
} from "@mantine/core";
import MDIcon from "@/components/ui/MDIcon";
import FirstBloodIcon from "@/components/icons/hexagons/FirstBloodIcon";
import SecondBloodIcon from "@/components/icons/hexagons/SecondBloodIcon";
import ThirdBloodIcon from "@/components/icons/hexagons/ThirdBloodIcon";
import { useEffect, useState } from "react";

export default function ChallengeCard({
	challenge,
}: {
	challenge?: Challenge;
}) {
	const { colorScheme } = useMantineColorScheme();
	const theme = useMantineTheme();

	const [color, setColor] = useState<string>(theme.colors.brand[6]);

	const [cardBgColor, setCardBgColor] = useState<string>();
	const [cardHoverBgColor, setCardHoverBgColor] = useState<string>();
	const [cardTextColor, setCardTextColor] = useState<string>();

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
				right={-15}
				bottom={-22.5}
				sx={{
					opacity: 0.2,
					color: cardTextColor,
				}}
			>
				<MDIcon size={180}>{challenge?.category?.icon}</MDIcon>
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
						<ThemeIcon variant="transparent" c={"#FFF"}>
							<MDIcon size={30}>done</MDIcon>
						</ThemeIcon>
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
					{challenge?.pts || challenge?.practice_pts || "?"} pts
				</Text>
				<Flex align={"center"}>
					{challenge?.submissions && (
						<>
							{challenge?.submissions?.length > 0 && (
								<Tooltip
									label={`一血 ${challenge?.submissions?.[0]?.user?.nickname}`}
									withArrow
									position="bottom"
								>
									<ThemeIcon variant="transparent">
										<FirstBloodIcon size={24} />
									</ThemeIcon>
								</Tooltip>
							)}
							{challenge?.submissions?.length > 1 && (
								<Tooltip
									label={`二血 ${challenge?.submissions?.[1]?.user?.nickname}`}
									withArrow
									position="bottom"
								>
									<ThemeIcon variant="transparent">
										<SecondBloodIcon size={24} />
									</ThemeIcon>
								</Tooltip>
							)}
							{challenge?.submissions?.length > 2 && (
								<Tooltip
									label={`三血 ${challenge?.submissions?.[2]?.user?.nickname}`}
									withArrow
									position="bottom"
								>
									<ThemeIcon variant="transparent">
										<ThirdBloodIcon size={24} />
									</ThemeIcon>
								</Tooltip>
							)}
						</>
					)}
				</Flex>
			</Flex>
		</Card>
	);
}
