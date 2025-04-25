import { WebResponse } from "@/types";
import { alova } from "@/utils/alova";

export interface UserForgetRequest {
    email?: string;
    code?: string;
    password?: string;
}

export async function forget(request: UserForgetRequest) {
    return alova.Post<WebResponse<never>>(`/users/forget`, request);
}

export interface UserSendForgetEmailRequest {
    email?: string;
}

export async function sendForgetEmail(request: UserSendForgetEmailRequest) {
    return alova.Post<WebResponse<never>>(`/users/forget/send`, request);
}
