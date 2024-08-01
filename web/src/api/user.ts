import {
    UserCreateRequest,
    UserDeleteRequest,
    UserFindRequest,
    UserLoginRequest,
    UserRegisterRequest,
    UserUpdateRequest,
} from "@/types/user";
import { useApi, useAuth } from "@/utils/axios";
import { AxiosRequestConfig } from "axios";

export function useUserApi() {
    const api = useApi();
    const auth = useAuth();

    const login = (request: UserLoginRequest) => {
        return api.post("/users/login", request);
    };

    const register = (request: UserRegisterRequest) => {
        return api.post("/users/register", request);
    };

    const getUsers = (request: UserFindRequest) => {
        return auth.get("/users", { params: request });
    };

    const getUserTeams = (id: number) => {
        return auth.get(`/users/${id}/teams`);
    };

    const updateUser = (request: UserUpdateRequest) => {
        return auth.put(`/users/${request?.id}`, request);
    };

    const createUser = (request: UserCreateRequest) => {
        return auth.post(`/users`, request);
    };

    const deleteUser = (request: UserDeleteRequest) => {
        return auth.delete(`/users/${request?.id}`);
    };

    const getUserAvatarMetadata = (id: number) => {
        return auth.get(`/users/${id}/avatar/metadata`);
    };

    const saveUserAvatar = (
        id: number,
        file: File,
        config: AxiosRequestConfig<FormData>
    ) => {
        const formData = new FormData();
        formData.append("file", file);
        return auth.post(`/users/${id}/avatar`, formData, config);
    };

    const deleteUserAvatar = (id: number) => {
        return auth.delete(`/users/${id}/avatar`);
    };

    return {
        login,
        register,
        getUsers,
        getUserTeams,
        updateUser,
        createUser,
        deleteUser,
        getUserAvatarMetadata,
        saveUserAvatar,
        deleteUserAvatar,
    };
}
