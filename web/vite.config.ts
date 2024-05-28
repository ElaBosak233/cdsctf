import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import Pages from "vite-plugin-pages";
import path from "path";

export default defineConfig({
	plugins: [
		react(),
		Pages({
			dirs: [
				{
					dir: "./src/pages",
					baseRoute: "",
				},
			],
		}),
	],
	resolve: {
		alias: {
			"@": path.resolve(__dirname, "src"),
			"#": path.resolve(__dirname, "."),
		},
	},
});
