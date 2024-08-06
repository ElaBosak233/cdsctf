import { Team } from "@/types/team";
import {
    User,
    UserCreateRequest,
    UserDeleteRequest,
    UserFindRequest,
    UserLoginRequest,
    UserRegisterRequest,
    UserUpdateRequest,
} from "@/types/user";
import { api } from "@/utils/axios";
import { AxiosRequestConfig } from "axios";

export async function login(request: UserLoginRequest) {
    return await api().post<{
        code: number;
        data: User;
        token: string;
    }>("/users/login", request);
}

export async function register(request: UserRegisterRequest) {
    return await api().post<{
        code: number;
        data: User;
    }>("/users/register", request);
}

export async function getUsers(request: UserFindRequest) {
    return await api().get<{
        code: number;
        data: Array<User>;
        total: number;
    }>("/users", { params: request });
}

export async function getUserTeams(id: number) {
    return await api().get<{
        code: number;
        data: Array<Team>;
    }>(`/users/${id}/teams`);
}

export async function updateUser(request: UserUpdateRequest) {
    return await api().put<{
        code: number;
        data: User;
    }>(`/users/${request?.id}`, request);
}

export async function createUser(request: UserCreateRequest) {
    return await api().post<{
        code: number;
        data: User;
    }>("/users", request);
}

export async function deleteUser(request: UserDeleteRequest) {
    return await api().delete<{
        code: number;
    }>(`/users/${request?.id}`);
}

export async function getUserAvatarMetadata(id: number) {
    return await api().get<{
        code: number;
        data: {
            filename: string;
            size: number;
        };
    }>(`/users/${id}/avatar/metadata`);
}

export async function saveUserAvatar(
    id: number,
    file: File,
    config: AxiosRequestConfig<FormData>
) {
    const formData = new FormData();
    formData.append("file", file);
    return await api().post<{
        code: number;
        data: {
            filename: string;
            size: number;
        };
    }>(`/users/${id}/avatar`, formData, config);
}

export async function deleteUserAvatar(id: number) {
    return await api().delete<{
        code: number;
    }>(`/users/${id}/avatar`);
}
