import CryptoJS from "crypto-js";
import { create } from "zustand";
import { createJSONStorage, persist } from "zustand/middleware";

import type { GameNotice } from "@/models/game_notice";

const MAX_SEEN_FINGERPRINTS = 200;

export type NoticeScope = {
  datasetEpoch: number;
  seen: Record<string, number>;
  maxId?: number;
  newestCreatedAt?: number;
  snapshot?: Array<string>;
};

type NoticeReadState = {
  scopes: Record<string, NoticeScope>;
  syncNotices: (scopeKey: string, notices: Array<GameNotice>) => void;
  markAsRead: (scopeKey: string, fingerprints: Array<string>) => void;
  reset: (scopeKey?: string) => void;
};

function getNoticeScopeKey(userId: number, gameId: number) {
  return `${userId}:${gameId}`;
}

function getNoticeFingerprint(notice: GameNotice) {
  return CryptoJS.SHA256(
    JSON.stringify([
      notice.game_id ?? null,
      notice.id ?? null,
      notice.created_at ?? null,
      notice.title ?? "",
      notice.content ?? "",
    ])
  ).toString();
}

function getNoticeFingerprints(notices: Array<GameNotice>) {
  return Array.from(new Set(notices.map(getNoticeFingerprint)));
}

function getMaxNumber(values: Array<number | undefined>): number | undefined {
  const numbers = values.filter(
    (value): value is number => value != null && Number.isFinite(value)
  );

  return numbers.length > 0 ? Math.max(...numbers) : undefined;
}

function trimSeen(seen: Record<string, number>) {
  return Object.fromEntries(
    Object.entries(seen)
      .sort(([, left], [, right]) => right - left)
      .slice(0, MAX_SEEN_FINGERPRINTS)
  );
}

function shouldResetDataset(
  previous: NoticeScope | undefined,
  fingerprints: Array<string>,
  maxId: number | undefined,
  newestCreatedAt: number | undefined
) {
  if (!previous?.snapshot?.length) return false;

  const idRegressed =
    previous.maxId != null && maxId != null && maxId < previous.maxId;
  const timeRegressed =
    previous.newestCreatedAt != null &&
    newestCreatedAt != null &&
    newestCreatedAt < previous.newestCreatedAt;

  if (!idRegressed && !timeRegressed) return false;

  return !fingerprints.some((fingerprint) =>
    previous.snapshot?.includes(fingerprint)
  );
}

export const useNoticeReadStore = create<NoticeReadState>()(
  persist(
    (set) => ({
      scopes: {},
      syncNotices: (scopeKey, notices) =>
        set((state) => {
          const previous = state.scopes[scopeKey];
          const fingerprints = getNoticeFingerprints(notices);
          const maxId = getMaxNumber(notices.map((notice) => notice.id));
          const newestCreatedAt = getMaxNumber(
            notices.map((notice) => notice.created_at)
          );
          const datasetReset = shouldResetDataset(
            previous,
            fingerprints,
            maxId,
            newestCreatedAt
          );

          return {
            scopes: {
              ...state.scopes,
              [scopeKey]: {
                datasetEpoch:
                  (previous?.datasetEpoch ?? 0) + (datasetReset ? 1 : 0),
                seen: datasetReset ? {} : (previous?.seen ?? {}),
                maxId,
                newestCreatedAt,
                snapshot: fingerprints,
              },
            },
          };
        }),
      markAsRead: (scopeKey, fingerprints) =>
        set((state) => {
          const previous = state.scopes[scopeKey];
          const seen = { ...(previous?.seen ?? {}) };
          const seenAt = Date.now();

          for (const fingerprint of fingerprints) {
            seen[fingerprint] = seenAt;
          }

          return {
            scopes: {
              ...state.scopes,
              [scopeKey]: {
                datasetEpoch: previous?.datasetEpoch ?? 0,
                seen: trimSeen(seen),
                maxId: previous?.maxId,
                newestCreatedAt: previous?.newestCreatedAt,
                snapshot: previous?.snapshot,
              },
            },
          };
        }),
      reset: (scopeKey) =>
        set((state) => {
          if (!scopeKey) return { scopes: {} };

          const scopes = { ...state.scopes };
          delete scopes[scopeKey];
          return { scopes };
        }),
    }),
    {
      name: "notice-read",
      version: 1,
      storage: createJSONStorage(() => localStorage),
      partialize: (state) => ({ scopes: state.scopes }),
    }
  )
);

export { getNoticeFingerprint, getNoticeFingerprints, getNoticeScopeKey };
