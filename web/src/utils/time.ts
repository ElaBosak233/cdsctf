import {
  addSeconds,
  differenceInMilliseconds,
  differenceInSeconds,
  isAfter as isAfterDate,
  isBefore as isBeforeDate,
  isValid,
  parseISO,
} from "date-fns";
import { type Timestamp, timestampSchema } from "@/types";

export type GamePhase = "upcoming" | "ongoing" | "frozen" | "ended";

export type GameSchedule = {
  started_at: Timestamp;
  frozen_at: Timestamp;
  ended_at: Timestamp;
};

export type GameScheduleInput = Partial<GameSchedule>;

export type TimestampInput = Date | string | null | undefined;

export function parseTimestamp(value: TimestampInput): Date | undefined {
  return toDate(value);
}

export function getGamePhase(
  schedule: GameScheduleInput,
  currentTime = new Date()
): GamePhase | undefined {
  if (!isValid(currentTime)) return undefined;
  if (!schedule.started_at || !schedule.frozen_at || !schedule.ended_at) {
    return undefined;
  }
  const startedAt = parseTimestamp(schedule.started_at);
  const frozenAt = parseTimestamp(schedule.frozen_at);
  const endedAt = parseTimestamp(schedule.ended_at);
  if (!startedAt || !frozenAt || !endedAt) return undefined;
  if (isAfterDate(startedAt, frozenAt) || isAfterDate(frozenAt, endedAt)) {
    return undefined;
  }
  if (!isBeforeDate(currentTime, endedAt)) return "ended";
  if (isBeforeDate(currentTime, startedAt)) return "upcoming";
  if (!isBeforeDate(currentTime, frozenAt)) return "frozen";
  return "ongoing";
}

export function isGameActive(
  schedule: GameScheduleInput,
  currentTime = new Date()
): boolean {
  const phase = getGamePhase(schedule, currentTime);
  return phase === "ongoing" || phase === "frozen";
}

export function secondsUntil(
  value: TimestampInput,
  currentTime = new Date()
): number | undefined {
  const target = toDate(value);
  return target == null ? undefined : differenceInSeconds(target, currentTime);
}

export function millisecondsBetween(
  start: TimestampInput,
  end: TimestampInput
): number | undefined {
  const startDate = toDate(start);
  const endDate = toDate(end);
  return startDate == null || endDate == null
    ? undefined
    : differenceInMilliseconds(endDate, startDate);
}

export function toEpochMilliseconds(value: TimestampInput): number | undefined {
  return toDate(value)?.getTime();
}

export function instanceExpiresAt(
  startedAt: Timestamp | null | undefined,
  durationSeconds: number | null | undefined,
  renewCount: number | null | undefined
): Date | undefined {
  const start = parseTimestamp(startedAt);
  if (
    start == null ||
    durationSeconds == null ||
    renewCount == null ||
    !Number.isFinite(durationSeconds) ||
    !Number.isFinite(renewCount)
  )
    return undefined;
  return addSeconds(start, (renewCount + 1) * durationSeconds);
}

export function formatDuration(milliseconds: number | undefined): string {
  if (milliseconds == null || !Number.isFinite(milliseconds)) return "-";
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

function toDate(value: TimestampInput): Date | undefined {
  if (value instanceof Date) return isValid(value) ? value : undefined;
  const decoded = timestampSchema.safeParse(value);
  if (!decoded.success) return undefined;
  const parsed = parseISO(decoded.data);
  return isValid(parsed) ? parsed : undefined;
}
