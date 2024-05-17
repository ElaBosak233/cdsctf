import { Box, Text } from "@mantine/core";

export default function Page() {
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
					欢迎来到管理面板
				</Text>
			</Box>
		</>
	);
}
