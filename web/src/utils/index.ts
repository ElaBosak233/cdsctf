import { type ClassValue, clsx } from "clsx";
import { twMerge } from "tailwind-merge";

export function cn(...inputs: Array<ClassValue>) {
  return twMerge(clsx(inputs));
}

export function stripIndent(str: string): string {
  const lines = str.replace(/^\n/, "").split("\n");

  const minIndent = lines
    .filter((line) => line.trim().length > 0)
    .reduce(
      (min, line) => {
        const indent = line.match(/^(\s*)/)![1].length;
        return min === null ? indent : Math.min(min, indent);
      },
      null as number | null
    );

  return lines.map((line) => line.slice(minIndent!)).join("\n");
}
