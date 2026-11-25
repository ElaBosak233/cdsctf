import type { AdminGameNoticeView } from "@/models/game_notice";
import { api } from "@/utils/query";

export type GetManagementGameNoticesRequest = {
  game_id: number;
};

export async function getManagementGameNotices(
  request: GetManagementGameNoticesRequest
) {
  return api
    .get(`management/games/${request.game_id}/notices`)
    .json<{ notices: AdminGameNoticeView[]; total: number }>();
}

export type CreateGameNoticeRequest = {
  game_id: number;
  title: string;
  content: string;
  challenge_id?: number | null;
};

export async function createGameNotice(request: CreateGameNoticeRequest) {
  const { game_id, title, content, challenge_id } = request;
  return api
    .post(`management/games/${game_id}/notices`, {
      json: { title, content, challenge_id: challenge_id ?? null },
    })
    .json<{ notice: AdminGameNoticeView }>();
}

export type UpdateGameNoticeRequest = {
  id: number;
  game_id: number;
  title?: string;
  content?: string;
  challenge_id?: number | null;
};

export async function updateGameNotice(request: UpdateGameNoticeRequest) {
  const { id, game_id, title, content, challenge_id } = request;
  return api
    .patch(`management/games/${game_id}/notices/${id}`, {
      json: {
        ...(title !== undefined ? { title } : {}),
        ...(content !== undefined ? { content } : {}),
        ...(challenge_id !== undefined ? { challenge_id } : {}),
      },
    })
    .json<{ notice: AdminGameNoticeView }>();
}

export type DeleteGameNoticeRequest = {
  id?: number;
  game_id?: number;
};

export async function deleteGameNotice(request: DeleteGameNoticeRequest) {
  return api
    .delete(`management/games/${request.game_id}/notices/${request.id}`, {
      json: request,
    })
    .json<Record<string, never>>();
}
