export interface Challenge {
  id?: number;
  title?: string;
  tags?: Array<string>;
  description?: string;
  category?: number;
  has_attachment?: boolean;
  is_public?: boolean;
  is_dynamic?: boolean;
  env?: Env;
  checker?: string;
  updated_at?: number;
  created_at?: number;
}

export interface Env {
  duration?: number;
  internet?: boolean;
  containers?: Array<Container>;
}

export interface Container {
  image: string;
  cpu_limit: number;
  memory_limit: number;
  ports: Array<Port>;
  envs: Array<EnvVar>;
}

export interface Port {
  port: number;
  protocol: "TCP" | "UDP";
}

export interface EnvVar {
  key: string;
  value: string;
}

export interface ChallengeMini {
  id?: number;
  title?: string;
  category?: number;
  tags?: Array<string>;
}
