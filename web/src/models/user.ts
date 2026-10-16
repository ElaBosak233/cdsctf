import type { Timestamp } from "@/types";

export type UserAccountView = {
  id: number;
  name: string;
  username: string;
  verified: boolean | null;
  banned: boolean;
  permissions: Permission[];
  description: string | null;
  avatar_hash: string | null;
  created_at: Timestamp;
  updated_at: Timestamp;
};

export const BASIC_PERMISSION = "basic" as const;
export const MANAGEMENT_CONFIG_PERMISSION = "management:config" as const;
export const MANAGEMENT_CHALLENGE_PERMISSION = "management:challenge" as const;
export const MANAGEMENT_GAME_PERMISSION = "management:game" as const;
export const MANAGEMENT_IDP_PERMISSION = "management:idp" as const;
export const MANAGEMENT_USER_PERMISSION = "management:user" as const;

export const MANAGEMENT_PERMISSIONS = [
  MANAGEMENT_CONFIG_PERMISSION,
  MANAGEMENT_CHALLENGE_PERMISSION,
  MANAGEMENT_GAME_PERMISSION,
  MANAGEMENT_IDP_PERMISSION,
  MANAGEMENT_USER_PERMISSION,
] as const;

export const PERMISSIONS = [
  BASIC_PERMISSION,
  ...MANAGEMENT_PERMISSIONS,
] as const;

export type Permission = (typeof PERMISSIONS)[number];

export const PERMISSION_OPTIONS = [
  { value: BASIC_PERMISSION, key: "basic" },
  { value: MANAGEMENT_CONFIG_PERMISSION, key: "config" },
  { value: MANAGEMENT_CHALLENGE_PERMISSION, key: "challenge" },
  { value: MANAGEMENT_GAME_PERMISSION, key: "game" },
  { value: MANAGEMENT_IDP_PERMISSION, key: "idp" },
  { value: MANAGEMENT_USER_PERMISSION, key: "user" },
] as const satisfies ReadonlyArray<{ value: Permission; key: string }>;

export function hasPermission(
  permissions: readonly Permission[] | undefined,
  required: Permission
) {
  return (permissions ?? []).some((granted) => granted === required);
}

export function hasAnyPermission(
  permissions: readonly Permission[] | undefined,
  required: readonly Permission[]
) {
  return (permissions ?? []).some((granted) => required.includes(granted));
}

export function hasPermissions(
  permissions: readonly Permission[] | undefined,
  required: readonly Permission[]
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

export type UserStatistics = {
  solved: number;
  solved_by_category: Array<{
    category: number;
    solved: number;
  }>;
  games: Array<{
    game_id: number;
    title: string;
    solved: number;
    total: number;
  }>;
};
