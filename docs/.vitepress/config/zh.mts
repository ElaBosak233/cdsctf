import { defineConfig } from "vitepress";

export default defineConfig({
	lang: "zh-Hans",
	description: "Cloudsdale 是一个开源、高性能的解题模式 CTF 平台",
	themeConfig: {
		nav: [{ text: "指南", link: "/zh/guide" }],

		sidebar: [
			{
				text: "简介",
				items: [{ text: "什么是 Cloudsdale？", link: "/zh/guide/" }],
			},
		],

		docFooter: {
			prev: "上一页",
			next: "下一页",
		},

		outline: {
			label: "页面导航",
		},

		lightModeSwitchTitle: "切换到浅色模式",
		darkModeSwitchTitle: "切换到深色模式",
	},
});
