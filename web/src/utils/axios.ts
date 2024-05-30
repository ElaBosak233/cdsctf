import axios from "axios";
import { useAuthStore } from "@/stores/auth";
import { useMemo } from "react";
import { useNavigate } from "react-router";
import { showInfoNotification } from "./notification";

export const useApi = () => {
	return useMemo(() => {
		const api = axios.create({
			baseURL: import.meta.env.VITE_BASE_API as string,
		});
		return api;
	}, []);
};

export const useAuth = () => {
	const authStore = useAuthStore();
	const { pgsToken } = useAuthStore();
	const navigate = useNavigate();

	return useMemo(() => {
		const auth = axios.create({
			baseURL: import.meta.env.VITE_BASE_API as string,
			headers: pgsToken
				? {
						Authorization: `${pgsToken}`,
					}
				: {},
		});

		auth.interceptors.response.use(
			(response) => {
				return response;
			},
			(error) => {
				if (error.response?.status === 401) {
					authStore.setPgsToken("");
					authStore.setUser(undefined);
					navigate("/login");
					showInfoNotification({
						message: "登录凭据已过期，请重新登录",
					});
				}
				return Promise.reject(error);
			}
		);

		return auth;
	}, [pgsToken, navigate]);
};
