import { useRoutes } from "react-router";
import Navbar from "@/components/navigations/Navbar";
import routes from "~react-pages";
import { Box, LoadingOverlay, MantineProvider } from "@mantine/core";
import { emotionTransform, MantineEmotionProvider } from "@mantine/emotion";
import { Suspense, useEffect } from "react";
import { useTheme } from "@/utils/theme";
import { useCategoryApi } from "@/api/category";
import { useCategoryStore } from "@/stores/category";
import { useConfigApi } from "@/api/config";
import { useConfigStore } from "@/stores/config";
import { Notifications } from "@mantine/notifications";
import { ModalsProvider } from "@mantine/modals";
import { DatesProvider } from "@mantine/dates";
import "dayjs/locale/zh-cn";

function App() {
	const { theme } = useTheme();
	const categoryApi = useCategoryApi();
	const categoryStore = useCategoryStore();
	const configApi = useConfigApi();
	const configStore = useConfigStore();

	// Get platform config
	useEffect(() => {
		configApi
			.getPltCfg()
			.then((res) => {
				const r = res.data;
				configStore.setPltCfg(r.data);
			})
			.catch((err) => {
				console.log(err);
			});
	}, [configStore.refresh]);

	// Get captcha config
	useEffect(() => {
		if (configStore?.pltCfg?.user?.register?.captcha?.enabled) {
			configApi.getCaptchaCfg().then((res) => {
				const r = res.data;
				configStore.setCaptchaCfg(r.data);
			});
		}
	}, [configStore?.pltCfg]);

	// Get exists categories
	useEffect(() => {
		categoryApi.getCategories().then((res) => {
			const r = res.data;
			if (r.code === 200) {
				categoryStore.setCategories(r.data);
			}
		});
	}, [categoryStore.refresh]);

	return (
		<>
			<MantineProvider
				stylesTransform={emotionTransform}
				theme={theme}
				defaultColorScheme="light"
			>
				<MantineEmotionProvider>
					<ModalsProvider>
						<DatesProvider
							settings={{
								locale: "zh-cn",
								firstDayOfWeek: 0,
								weekendDays: [0, 6],
								timezone: "UTC",
								consistentWeeks: true,
							}}
						>
							<Navbar />
							<Box pt={64}>
								<Suspense fallback={<LoadingOverlay />}>
									{useRoutes(routes)}
								</Suspense>
							</Box>
							<Notifications zIndex={5000} />
						</DatesProvider>
					</ModalsProvider>
				</MantineEmotionProvider>
			</MantineProvider>
		</>
	);
}

export default App;
