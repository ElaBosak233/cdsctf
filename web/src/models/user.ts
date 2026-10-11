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

export const MANAGEMENT_PERMISSIONS = [
  "management:config",
  "management:challenge",
  "management:game",
  "management:idp",
  "management:instance",
  "management:submission",
  "management:user",
] as const;

export function hasPermission(
  permissions: readonly string[] | undefined,
  required: string,
) {
  return (permissions ?? []).some(
    (granted) => granted === required,
  );
}

export function hasPermissions(
  permissions: readonly string[] | undefined,
  required: readonly string[],
) {
  return required.every((permission) => hasPermission(permissions, permission));
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
