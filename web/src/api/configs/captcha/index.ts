import { WebResponse } from "@/types";
import { api } from "@/utils/query";

export async function generateCaptcha() {
  return api.get("configs/captcha/generate").json<
    WebResponse<{
      id?: string;
      challenge?: string;
    }>
  >();
}
