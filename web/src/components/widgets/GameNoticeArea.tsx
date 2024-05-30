import { useGameApi } from "@/api/game";
import { Notice } from "@/types/notice";
import { Box, Flex, ScrollArea, ThemeIcon } from "@mantine/core";
import { useEffect, useState } from "react";
import { useParams } from "react-router-dom";
import MDIcon from "@/components/ui/MDIcon";
import dayjs from "dayjs";
import FirstBloodIcon from "@/components/icons/hexagons/FirstBloodIcon";
import SecondBloodIcon from "@/components/icons/hexagons/SecondBloodIcon";
import ThirdBloodIcon from "@/components/icons/hexagons/ThirdBloodIcon";

export default function GameNoticeArea() {
	const { id } = useParams<{ id: string }>();
	const gameApi = useGameApi();

	const [notices, setNotices] = useState<Array<Notice>>([]);

	function getGameNotices() {
		gameApi
			.getGameNotices({
				game_id: Number(id),
			})
			.then((res) => {
				const r = res.data;
				setNotices(r.data);
			});
	}

	useEffect(() => {
		getGameNotices();
	}, []);

	useEffect(() => {
		let wsURL = "";

		if (import.meta.env.VITE_BASE_API === "/api") {
			const protocol = window.location.protocol;
			const host = window.location.host;
			wsURL = `${protocol === "https:" ? "wss" : "ws"}://${host}/api/games/${id}/broadcast`;
		} else {
			const regex = /^(https?:\/\/)([\w\.-]+:\d+)/;
			const match = import.meta.env.VITE_BASE_API.match(regex);
			wsURL = `${match?.[1] === "https" ? "wss" : "ws"}://${match?.[2]}/api/games/${id}/broadcast`;
		}

		const socket = new WebSocket(wsURL);

		socket.onmessage = (event) => {
			const n = JSON.parse(event.data);
			setNotices((notices) => {
				return [...notices, n];
			});
		};

		socket.onerror = (event) => {
			console.error(event);
		};

		return () => {
			socket.close();
		};
	}, []);

	return (
		<>
			<ScrollArea>
				{notices?.map((notice) => (
					<Flex key={notice?.id}>
						<Box
							sx={{
								width: "10%",
							}}
						>
							{notice?.type === "normal" && (
								<MDIcon>campaign</MDIcon>
							)}
							{notice?.type === "first_blood" && (
								<FirstBloodIcon />
							)}
							{notice?.type === "second_blood" && (
								<SecondBloodIcon />
							)}
							{notice?.type === "third_blood" && (
								<ThirdBloodIcon />
							)}
							{notice?.type === "new_challenge" && (
								<MDIcon color={"green"}>add</MDIcon>
							)}
						</Box>
						<Box
							sx={{
								width: "90%",
								fontSize: "0.85rem",
								marginLeft: "0.5rem",
							}}
						>
							<Box
								sx={{
									fontWeight: "bold",
								}}
							>
								{dayjs(notice?.created_at).format(
									"YYYY/MM/DD HH:mm:ss"
								)}
							</Box>
							<Box>
								{notice?.type === "normal" && (
									<Box>{notice?.content}</Box>
								)}
								{notice?.type === "first_blood" && (
									<Box>
										恭喜 {notice?.team?.name} 斩获{" "}
										{notice?.challenge?.title} 一血
									</Box>
								)}
								{notice?.type === "second_blood" && (
									<Box>
										恭喜 {notice?.team?.name} 斩获{" "}
										{notice?.challenge?.title} 二血
									</Box>
								)}
								{notice?.type === "third_blood" && (
									<Box>
										恭喜 {notice?.team?.name} 斩获{" "}
										{notice?.challenge?.title} 三血
									</Box>
								)}
								{notice?.type === "new_challenge" && (
									<Box>
										新增题目 {notice?.challenge?.title}
									</Box>
								)}
							</Box>
						</Box>
					</Flex>
				))}
			</ScrollArea>
		</>
	);
}
