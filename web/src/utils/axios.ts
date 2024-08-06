import axios, { AxiosInstance } from "axios";
import { useAuthStore } from "@/stores/auth";
import { showInfoNotification } from "./notification";

/**
 * Remove undefined key-value pairs from a JSON object
 * @param object A JSON object
 * @returns object without undefined key-value pairs
 */
export function c(object: Object) {
    return Object.fromEntries(
        Object.entries(object).filter(([_, value]) => value !== undefined)
    );
}

export function api(): AxiosInstance {
    const instance = axios.create({
        baseURL: "/api",
        headers: {
            Authorization: useAuthStore.getState()?.pgsToken,
        },
    });
    instance.interceptors.request.use((config) => {
        if (config?.params) {
            config.params = c(config.params);
        }
        return config;
    });
    instance.interceptors.response.use(
        (response) => {
            return response;
        },
        (error) => {
            if (error.response?.status === 401) {
                useAuthStore.getState().logout();
                showInfoNotification({
                    id: "auth-expired",
                    message: "登录凭据已过期，请重新登录",
                });
            }
            return Promise.reject(error);
        }
    );
    return instance;
}
