export interface WebResponse<T, S = string> {
  code: number;
  data?: T;
  msg?: S;
  ts: number;
  total?: number;
}
