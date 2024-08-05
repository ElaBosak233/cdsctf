import axios from "axios";
import { useAuthStore } from "@/stores/auth";
import { useMemo } from "react";
import { useNavigate } from "react-router";
import { showInfoNotification } from "./notification";

export const useApi = () => {
    return useMemo(() => {
        const api = axios.create({
            baseURL: "/api",
            headers: {
                "Accept-Language": "zh-Hans",
            },
        });
        return api;
    }, []);
};

export const useAuth = () => {
    const { pgsToken } = useAuthStore();
    const navigate = useNavigate();

    return useMemo(() => {
        const auth = axios.create({
            baseURL: "/api",
            headers: {
                Authorization: pgsToken ? `${pgsToken}` : undefined,
                "Accept-Language": "zh-Hans",
            },
        });

        auth.interceptors.response.use(
            (response) => {
                return response;
            },
            (error) => {
                if (error.response?.status === 401) {
                    useAuthStore.setState({ user: undefined, pgsToken: "" });
                    navigate("/login");
                    showInfoNotification({
                        id: "auth-expired",
                        message: "登录凭据已过期，请重新登录",
                    });
                }
                return Promise.reject(error);
            }
        );

        return auth;
    }, [pgsToken, navigate]);
};
