import { defineConfig } from "vitepress";

export default defineConfig({
	lang: "en-US",
	description:
		"The Cloudsdale project is an open-source, high-performance, Jeopardy-style's CTF platform. ",
	themeConfig: {
		nav: [{ text: "Guide", link: "/guide/" }],

		sidebar: [
			{
				text: "Introduction",
				items: [{ text: "What is Cloudsdale?", link: "/guide/" }],
			},
		],
	},
});
