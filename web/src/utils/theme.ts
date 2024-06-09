import { ActionIcon, Avatar, ThemeIcon, createTheme } from "@mantine/core";

export function useTheme() {
	const theme = createTheme({
		colors: {
			brand: [
				"#B4CFF9",
				"#8EB7F6",
				"#68A0F3",
				"#4288F0",
				"#1D70ED",
				"#115DD0",
				"#0D47A1",
				"#0C4497",
				"#0B3B84",
				"#093371",
			],
			light: [
				"#FFFFFF",
				"#F8F8F8",
				"#EFEFEF",
				"#E0E0E0",
				"#DFDFDF",
				"#D0D0D0",
				"#CFCFCF",
				"#C0C0C0",
				"#BFBFBF",
				"#B0B0B0",
			],
			dark: [
				"#d5d7d7",
				"#acaeae",
				"#8c8f8f",
				"#666969",
				"#4d4f4f",
				"#343535",
				"#2b2c2c",
				"#1d1e1e",
				"#0c0d0d",
				"#010101",
			],
			gray: [
				"#EBEBEB",
				"#CFCFCF",
				"#B3B3B3",
				"#969696",
				"#7A7A7A",
				"#5E5E5E",
				"#414141",
				"#252525",
				"#202020",
				"#141414",
			],
		},
		primaryColor: "brand",
		components: {
			LoadingOverlay: {
				defaultProps: {
					transitionProps: {
						exitDuration: 250,
					},
					overlayProps: {
						backgroundOpacity: 0,
					},
				},
			},
			ActionIcon: {
				defaultProps: {
					variant: "transparent",
				},
			},
			ThemeIcon: {
				defaultProps: {
					variant: "transparent",
				},
			},
			Avatar: {
				defaultProps: {
					imageProps: {
						draggable: false,
					},
				},
			},
		},
	});
	return { theme: theme };
}
