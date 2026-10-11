import type { Timestamp } from "@/types";

export type UserAccountView = {
  id: number;
  name: string;
  username: string;
  verified: boolean | null;
  banned: boolean;
  permissions: string[];
  description: string | null;
  avatar_hash: string | null;
  created_at: Timestamp;
  updated_at: Timestamp;
};

export function hasPermission(
  permissions: readonly string[] | undefined,
  required: string,
) {
  return (permissions ?? []).some(
    (granted) =>
      granted === "*" ||
      granted === required ||
      (granted.endsWith("*") && required.startsWith(granted.slice(0, -1))),
  );
}

export type UserSummary = {
  id: number;
  name: string;
  username: string;
  avatar_hash: string | null;
};

export type UserProfile = {
  id: number;
  name: string;
  username: string;
  description: string | null;
  avatar_hash: string | null;
  created_at: Timestamp;
};
