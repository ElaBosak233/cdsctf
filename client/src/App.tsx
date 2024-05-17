import { useRoutes } from "react-router";
import Navbar from "@/components/navigations/Navbar";
import routes from "~react-pages";
import { Box, MantineProvider } from "@mantine/core";
import { emotionTransform, MantineEmotionProvider } from "@mantine/emotion";
import { Suspense, useEffect } from "react";
import Loading from "@/components/ui/Loading";
import useTheme from "@/composables/useTheme";
import { useCategoryApi } from "@/api/category";
import { useCategoryStore } from "./stores/category";
import { useConfigApi } from "./api/config";
import { useConfigStore } from "./stores/config";
import { Notifications } from "@mantine/notifications";
import { ModalsProvider } from "@mantine/modals";

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
						<Navbar />
						<Box pt={64}>
							<Suspense fallback={<Loading />}>
								{useRoutes(routes)}
							</Suspense>
						</Box>
						<Notifications zIndex={5000} />
					</ModalsProvider>
				</MantineEmotionProvider>
			</MantineProvider>
		</>
	);
}

export default App;
