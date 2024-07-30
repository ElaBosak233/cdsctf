import React from "react";
import ReactDOM from "react-dom/client";
import { BrowserRouter as Router } from "react-router-dom";
import App from "./App.tsx";
import "@mantine/core/styles.css";
import "@mantine/notifications/styles.css";
import "@mantine/dates/styles.css";
import "@mantine/dropzone/styles.css";
import "./main.css";
import { MantineProvider } from "@mantine/core";
import { DatesProvider } from "@mantine/dates";
import { emotionTransform, MantineEmotionProvider } from "@mantine/emotion";
import { ModalsProvider } from "@mantine/modals";
import { useTheme } from "@/utils/theme.ts";
import { Notifications } from "@mantine/notifications";

const { theme } = useTheme();

ReactDOM.createRoot(document.getElementById("root")!).render(
    <React.StrictMode>
        <Router>
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
                            <App />
                            <Notifications zIndex={5000} />
                        </DatesProvider>
                    </ModalsProvider>
                </MantineEmotionProvider>
            </MantineProvider>
        </Router>
    </React.StrictMode>
);
