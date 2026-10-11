import { api, toSearchParams } from "@/utils/query";

export function getEmail(type: "verify" | "forget") {
  return api
    .get("management/configs/mailbox", {
      searchParams: toSearchParams({ type }),
    })
    .json<{ content: string }>();
}

export type SaveEmailRequest = {
  type: "verify" | "forget";
  data: string;
};

export function saveEmail(request: SaveEmailRequest) {
  return api
    .post("management/configs/mailbox", { json: request })
    .json<Record<string, never>>();
}
