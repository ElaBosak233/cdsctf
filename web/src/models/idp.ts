export type IdpView = {
  id: number;
  name: string;
  enabled: boolean;
  avatar_hash: string | null;
  portal: string | null;
  script: string;
  created_at: number;
  updated_at: number;
};

export type IdpSummary = {
  id: number;
  name: string;
  avatar_hash: string | null;
  portal: string | null;
};

export type UserIdpSummary = {
  id: number;
  idp_id: number;
  auth_key: string;
};
