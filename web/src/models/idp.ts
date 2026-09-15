export type IdpView = {
  id: number;
  name: string;
  enabled: boolean;
  registration_enabled: boolean;
  avatar_hash: string | null;
  portal: string | null;
  script: string;
  created_at: string;
  updated_at: string;
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
  source: "registration" | "binding";
};
