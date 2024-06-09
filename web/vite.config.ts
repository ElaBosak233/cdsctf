import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import Pages from "vite-plugin-pages";
import { prismjsPlugin } from "vite-plugin-prismjs";
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
		prismjsPlugin({
			languages: "all",
			css: true,
		}),
	],
	resolve: {
		alias: {
			"@": path.resolve(__dirname, "src"),
			"#": path.resolve(__dirname, "."),
		},
	},
});
