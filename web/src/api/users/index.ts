import { User } from "@/models/user";
import { WebResponse } from "@/types";
import { alova } from "@/utils/alova";

export interface UserLoginRequest {
    account: string;
    password: string;
    captcha?: {
        id?: string;
        content?: string;
    } | null;
}

export async function login(request: UserLoginRequest) {
    return alova.Post<WebResponse<User>>("/users/login", request);
}

export async function logout() {
    return alova.Post<WebResponse<never>>("/users/logout");
}

export interface UserRegisterRequest {
    username: string;
    name?: string;
    email: string;
    password: string;
    captcha?: {
        id?: string;
        content?: string;
    };
}

export async function register(request: UserRegisterRequest) {
    return alova.Post<WebResponse<User>>("/users/register", request);
}
