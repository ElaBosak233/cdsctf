export interface Flag {
    type: Type;
    banned: boolean;
    value: string;
    env: string;
}

export enum Type {
    Static = 0,
    Pattern = 1,
    Dynamic = 2,
}
