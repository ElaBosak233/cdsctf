export type Challenge = {
  id?: number;
  title?: string;
  tags?: Array<string>;
  description?: string;
  category?: number;
  has_attachment?: boolean;
  public?: boolean;
  has_writeup?: boolean;
  has_instance?: boolean;
  instance?: Instance;
  checker?: string;
  writeup?: string;
  updated_at?: number;
  created_at?: number;
}

export type Instance = {
  duration?: number;
  internet?: boolean;
  containers?: Array<Container>;
}

export type Container = {
  image: string;
  cpu_limit: number;
  memory_limit: number;
  ports: Array<Port>;
  envs: Array<EnvVar>;
  image_pull_policy: string;
}

export type Port = {
  port: number;
  protocol: "TCP" | "UDP";
}

export type EnvVar = {
  key: string;
  value: string;
}

export type ChallengeMini = {
  id?: number;
  title?: string;
  category?: number;
  tags?: Array<string>;
}
