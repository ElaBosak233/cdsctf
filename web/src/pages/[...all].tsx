import { useEffect, useState } from "react";
import { useConfigStore } from "@/stores/config";
import { Box, Text } from "@mantine/core";

export default function Page() {
	const configStore = useConfigStore();

	useEffect(() => {
		document.title = `${configStore?.pltCfg?.site?.title}`;
	}, []);

	return (
		<>
			<Box
				sx={{
					position: "fixed",
					top: "50%",
					left: "50%",
					zIndex: -1,
					transform: "translate(-50%, -50%)",
				}}
				className={"no-select"}
			>
				<Text
					sx={{ fontSize: "5rem", opacity: 0.2, textAlign: "center" }}
				>
					404 Not Found
				</Text>
			</Box>
		</>
	);
}
