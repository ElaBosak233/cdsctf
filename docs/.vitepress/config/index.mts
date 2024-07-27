import { defineConfig } from "vitepress";
import en from "./en.mts";
import zh from "./zh.mts";

// https://vitepress.dev/reference/site-config
export default defineConfig({
	title: "Cloudsdale",
	description:
		"The Cloudsdale project is an open-source, high-performance, Jeopardy-style's CTF platform. ",
	head: [
		["link", { rel: "icon", href: "/favicon.webp", type: "image/webp" }],
	],
	rewrites: {
		"en/:rest*": ":rest*",
	},
	themeConfig: {
		logo: {
			light: "/favicon.webp",
			dark: "/favicon.webp",
		},
		socialLinks: [
			{
				icon: "github",
				link: "https://github.com/elabosak233/cloudsdale",
			},
		],
	},
	locales: {
		root: { label: "English", ...en },
		zh: { label: "简体中文", ...zh },
	},
});
