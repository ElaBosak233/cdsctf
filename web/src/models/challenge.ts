export interface Challenge {
    id?: string;
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
    envs: Record<string, string>;
    ports: Array<number>;
}
