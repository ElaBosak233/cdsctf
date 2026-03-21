/** Error JSON body from the API (HTTP status is separate from ky). */
export interface ApiJsonError {
  code: number;
  msg?: string;
  data?: unknown;
  total?: number;
  ts: number;
}
