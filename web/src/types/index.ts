export interface WebResponse<T> {
    code: number;
    data?: T;
    msg?: string;
    ts: number;
    total?: number;
}
