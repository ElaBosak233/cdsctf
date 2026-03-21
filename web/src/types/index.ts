/** Error JSON body from the API (HTTP status is separate from ky). */
export interface ApiJsonError {
  code: number;
  msg?: unknown;
  data?: unknown;
  total?: number;
  ts: number;
}
