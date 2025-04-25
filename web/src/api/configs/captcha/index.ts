import { alova } from "@/utils/alova";
import { WebResponse } from "@/types";

export async function generateCaptcha() {
    return alova.Get<
        WebResponse<{
            id?: string;
            challenge?: string;
        }>
    >("/configs/captcha/generate");
}
