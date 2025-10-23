import type { Team } from "./team";

export interface User {
  id?: number;
  name?: string;
  username?: string;
  is_verified?: boolean;
  group?: Group;
  description?: string;
  has_avatar?: boolean;
  teams?: Array<Team>;
  created_at?: string;
  updated_at?: string;
}

export enum Group {
  Guest = 0,
  Banned = 1,
  User = 2,
  Admin = 3,
}

export interface UserMini {
  id?: number;
  name?: string;
  username?: string;
  has_avatar?: boolean;
}
