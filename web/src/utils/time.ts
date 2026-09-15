export function timestamp(value: string | null | undefined): number {
  return value == null ? NaN : Date.parse(value);
}

export function date(value: string | null | undefined): Date {
  return new Date(timestamp(value));
}
