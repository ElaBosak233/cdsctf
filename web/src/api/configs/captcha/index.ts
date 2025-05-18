import { WebResponse } from "@/types";
import { alova } from "@/utils/alova";

export async function generateCaptcha() {
  return alova.Get<
    WebResponse<{
      id?: string;
      challenge?: string;
    }>
  >("/configs/captcha/generate");
}
