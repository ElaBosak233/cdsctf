export function timestamp(value: string | null | undefined): number {
  return value == null ? NaN : Date.parse(value);
}

export function date(value: string | null | undefined): Date | undefined {
  const milliseconds = timestamp(value);
  return Number.isFinite(milliseconds) ? new Date(milliseconds) : undefined;
}

/** Formats elapsed time for compact operational tables. */
export function formatDuration(milliseconds: number): string {
  if (!Number.isFinite(milliseconds)) return "-";

  const duration = Math.max(0, milliseconds);
  if (duration < 1_000) return `${Math.round(duration)} ms`;

  const seconds = duration / 1_000;
  if (seconds < 60) return `${seconds.toFixed(1)} s`;

  const minutes = Math.floor(seconds / 60);
  const remainingSeconds = seconds - minutes * 60;
  if (minutes < 60) {
    return `${minutes}m ${remainingSeconds.toFixed(1).padStart(4, "0")}s`;
  }

  const hours = Math.floor(minutes / 60);
  const remainingMinutes = minutes - hours * 60;
  return `${hours}h ${String(remainingMinutes).padStart(2, "0")}m`;
}
