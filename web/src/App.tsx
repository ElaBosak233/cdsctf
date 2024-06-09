import { useRoutes } from "react-router";
import Navbar, {
	NavItems,
	AdminNavItems,
} from "@/components/navigations/Navbar";
import routes from "~react-pages";
import {
	AppShell,
	Button,
	LoadingOverlay,
	UnstyledButton,
} from "@mantine/core";
import { Suspense, useEffect, useState } from "react";
import { useCategoryApi } from "@/api/category";
import { useCategoryStore } from "@/stores/category";
import { useConfigApi } from "@/api/config";
import { useConfigStore } from "@/stores/config";
import "dayjs/locale/zh-cn";
import { useDisclosure } from "@mantine/hooks";
import { Link, useLocation } from "react-router-dom";
import MDIcon from "./components/ui/MDIcon";

function App() {
	const categoryApi = useCategoryApi();
	const categoryStore = useCategoryStore();
	const configApi = useConfigApi();
	const configStore = useConfigStore();

	const [opened, { toggle }] = useDisclosure();
	const [adminMode, setAdminMode] = useState<boolean>(false);
	const location = useLocation();

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
			categoryStore.setCategories(r.data);
		});
	}, [categoryStore.refresh]);

	useEffect(() => {
		setAdminMode(false);
		if (location.pathname.startsWith("/admin")) {
			setAdminMode(true);
		}
	}, [location.pathname]);

	return (
		<>
			<AppShell
				header={{ height: 64 }}
				navbar={{
					width: 300,
					breakpoint: "md",
					collapsed: { desktop: true, mobile: !opened },
				}}
			>
				<AppShell.Header>
					<Navbar
						burger={{
							opened: opened,
							toggle: toggle,
						}}
						adminMode={adminMode}
					/>
				</AppShell.Header>
				<AppShell.Navbar py={"md"}>
					{!adminMode && (
						<>
							{NavItems?.map((item) => (
								<Button
									key={item.path}
									variant={"subtle"}
									h={50}
									px={20}
									radius={0}
									justify={"start"}
									component={Link}
									to={item.path}
									leftSection={<MDIcon>{item?.icon}</MDIcon>}
									onClick={toggle}
								>
									{item.name}
								</Button>
							))}
						</>
					)}
				</AppShell.Navbar>
				<AppShell.Main>
					<Suspense fallback={<LoadingOverlay />}>
						{useRoutes(routes)}
					</Suspense>
				</AppShell.Main>
			</AppShell>
		</>
	);
}

export default App;
