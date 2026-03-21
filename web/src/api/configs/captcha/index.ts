import { api } from "@/utils/query";

export async function generateCaptcha() {
  return api.get("configs/captcha/generate").json<{
    id: string;
    challenge: string;
    criteria?: string;
  }>();
}
