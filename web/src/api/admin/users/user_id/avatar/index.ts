import { WebResponse } from "@/types";
import { alova } from "@/utils/alova";

interface DeleteUserAvatarRequest {
    user_id: number;
}

export async function deleteUserAvatar(request: DeleteUserAvatarRequest) {
    return alova.Delete<WebResponse<never>>(
        `/admin/users/${request.user_id}/avatar`
    );
}
