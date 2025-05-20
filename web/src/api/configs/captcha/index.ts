import { WebResponse } from "@/types";
import { api } from "@/utils/ky";

export async function generateCaptcha() {
  return api.get("configs/captcha/generate").json<
    WebResponse<{
      id?: string;
      challenge?: string;
    }>
  >();
}
