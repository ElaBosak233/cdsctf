import { WebResponse } from "@/types";
import { alova } from "@/utils/alova";

export interface UserVerifyRequest {
    code?: string;
}

export async function verify(request: UserVerifyRequest) {
    return alova.Post<WebResponse<never>>(`/users/profile/verify`, request);
}

export async function sendVerifyEmail() {
    return alova.Post<WebResponse<never>>(`/users/profile/verify/send`);
}
