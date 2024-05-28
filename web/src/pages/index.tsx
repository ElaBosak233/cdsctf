import { useConfigStore } from "@/stores/config";
import { Box, Text } from "@mantine/core";
import { useEffect } from "react";

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
			>
				<Text size="5rem" opacity={0.2} className="no-select">
					{configStore?.pltCfg?.site?.description}
				</Text>
			</Box>
		</>
	);
}
