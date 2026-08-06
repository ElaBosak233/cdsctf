export type UserAccountView = {
  id: number;
  name: string;
  username: string;
  verified: boolean | null;
  group: Group;
  description: string | null;
  avatar_hash: string | null;
  created_at: number;
  updated_at: number;
};

export enum Group {
  Guest = 0,
  Banned = 1,
  User = 2,
  Admin = 3,
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
  created_at: number;
};
