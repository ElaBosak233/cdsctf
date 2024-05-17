import { Box, Loader } from "@mantine/core";

export default function Loading() {
	return (
		<Box
			sx={{
				width: "100%",
				height: "100vh",
				display: "flex",
				justifyContent: "center",
				alignItems: "center",
			}}
		>
			<Loader color="blue" size="lg" />
		</Box>
	);
}
