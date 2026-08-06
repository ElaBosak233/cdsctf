export type PublicConfig = {
  meta?: {
    title?: string;
    description?: string;
    keywords?: Array<string>;
    footer?: string;
  };
  auth?: {
    registration_enabled?: boolean;
  };
  captcha?: {
    provider?: "none" | "pow" | "image" | "turnstile" | "hcaptcha";
    difficulty?: number;
    turnstile?: {
      site_key?: string;
    };
    hcaptcha?: {
      site_key?: string;
    };
  };
  email?: {
    enabled?: boolean;
  };
  logo_hash?: string | null;
};

export type AdminConfig = Omit<PublicConfig, "captcha" | "email"> & {
  captcha?: {
    provider?: "none" | "pow" | "image" | "turnstile" | "hcaptcha";
    difficulty?: number;
    turnstile?: {
      url?: string;
      site_key?: string;
      secret_key?: string;
    };
    hcaptcha?: {
      url?: string;
      site_key?: string;
      secret_key?: string;
    };
  };
  email?: {
    enabled?: boolean;
    host?: string;
    port?: number;
    tls?: "starttls" | "tls" | "none";
    username?: string;
    password?: string;
    whitelist?: Array<string>;
  };
};

export type Version = {
  tag?: string;
  commit?: string;
};

export type Statistics = {
  users?: number;
  games?: number;
  challenges?: {
    total?: number;
    in_game?: number;
  };
  submissions?: {
    total?: number;
    solved?: number;
  };
};
