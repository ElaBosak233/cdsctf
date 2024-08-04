import {
    showErrNotification,
    showLoadingNotification,
    showSuccessNotification,
} from "@/utils/notification";
import axios from "axios";
import { create } from "zustand";
import { createJSONStorage, persist } from "zustand/middleware";

interface WsrxState {
    isEnabled?: boolean;
    setIsEnabled: (isEnabled: boolean) => void;

    url?: string;
    setUrl: (url: string) => void;

    status?: "online" | "offline" | "connecting";
    setStatus: (status: "online" | "offline" | "connecting") => void;

    connect: () => void;
    ping: () => void;
}

export const useWsrxStore = create<WsrxState>()(
    persist(
        (set, get) => ({
            setIsEnabled: (isEnabled) => set(() => ({ isEnabled })),
            setUrl: (url) => set(() => ({ url })),
            setStatus: (status) => set(() => ({ status })),
            connect: async () => {
                set({ status: "offline" });
                await axios.post(
                    `${get()?.url!}/connect`,
                    `"${window.location.protocol}//${window.location.host}"`,
                    {
                        headers: {
                            "content-type": "application/json",
                        },
                    }
                );
                get().ping();
            },
            ping: () => {
                axios
                    .get(`${get()?.url!}/connect`)
                    .then((res) => {
                        if (res.status === 201) {
                            showLoadingNotification({
                                id: "wsrx",
                                title: "WSRX",
                                message: "等待本地授权",
                            });
                            set({ status: "connecting" });
                        }

                        if (res.status === 202 && get()?.status !== "online") {
                            showSuccessNotification({
                                id: "wsrx",
                                title: "WSRX",
                                message: "连接成功",
                                update: get()?.status === "connecting",
                            });
                            set({ status: "online" });
                        }
                    })
                    .catch((_) => {
                        if (get()?.status !== "offline") {
                            showErrNotification({
                                id: "wsrx",
                                title: "WSRX",
                                message: "连接已断开",
                            });
                            set({ status: "offline" });
                        }
                    });
            },
        }),
        {
            name: "wsrx_storage",
            storage: createJSONStorage(() => localStorage),
        }
    )
);
