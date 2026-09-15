export function timestamp(value: string | null | undefined): number {
  return value == null ? NaN : Date.parse(value);
}

export function date(value: string | null | undefined): Date | undefined {
  const milliseconds = timestamp(value);
  return Number.isFinite(milliseconds) ? new Date(milliseconds) : undefined;
}
