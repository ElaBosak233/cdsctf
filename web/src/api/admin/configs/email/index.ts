import type { WebResponse } from "@/types";
import { api, toSearchParams } from "@/utils/query";

export function getEmail(type: "verify" | "forget") {
  return api
    .get("admin/configs/email", { searchParams: toSearchParams({ type }) })
    .json<WebResponse<string>>();
}

export interface SaveEmailRequest {
  type: "verify" | "forget";
  data: string;
}

export function saveEmail(request: SaveEmailRequest) {
  return api
    .post("admin/configs/email", { json: request })
    .json<WebResponse<unknown>>();
}
