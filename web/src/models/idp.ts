export interface Idp {
  id?: number;
  name?: string;
  enabled?: boolean;
  has_avatar?: boolean;
  portal?: string | null;
  script?: string;
  created_at?: number;
  updated_at?: number;
}

export interface UserIdp {
  id?: number;
  user_id?: number;
  idp_id?: number;
  auth_key?: string;
  data?: Record<string, string>;
  created_at?: number;
  updated_at?: number;
}
