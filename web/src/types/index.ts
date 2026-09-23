import { z } from "zod";

/** JSON body for API error responses; use `response.status` for the HTTP status code. */
export type ErrorResponse = {
  msg?: unknown;
};

/** Branded RFC 3339 timestamp string. */
export const timestampSchema = z
  .string()
  .datetime({ offset: true })
  .brand("Timestamp");

/** Raw strings must be decoded through the schema before becoming a Timestamp. */
export type Timestamp = z.infer<typeof timestampSchema>;

/** Decode API JSON and brand timestamp fields at the transport boundary. */
export function decodeApiJson(value: unknown): unknown {
  if (Array.isArray(value)) return value.map(decodeApiJson);
  if (value == null || typeof value !== "object") return value;

  return Object.fromEntries(
    Object.entries(value).map(([key, entry]) => {
      if (key.endsWith("_at") && typeof entry === "string") {
        return [key, timestampSchema.parse(entry)];
      }
      return [key, decodeApiJson(entry)];
    })
  );
}
