import { WebResponse } from "@/types";
import { alova } from "@/utils/alova";

export async function deleteChallengeAttachment(id: string) {
    return alova.Delete<WebResponse<never>>(
        `/admin/challenges/${id}/attachment`
    );
}
