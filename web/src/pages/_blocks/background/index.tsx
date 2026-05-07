import { useMemo, useState } from "react";
import { cn } from "@/utils";
import "./styles.css";

const ANIMATIONS = ["bg-float-1", "bg-float-2", "bg-float-3"] as const;
const CIRCLE_COUNT = 10;

function generateCircles(w: number, h: number) {
  const pad = Math.max(w, h) * 0.2;
  return Array.from({ length: CIRCLE_COUNT }, () => ({
    cx: Math.random() * (w + pad * 2) - pad,
    cy: Math.random() * (h + pad * 2) - pad,
    r: Math.min(w, h) * (0.08 + Math.random() * 0.15),
    animation: ANIMATIONS[Math.floor(Math.random() * ANIMATIONS.length)],
  }));
}

function Background() {
  const [size] = useState(() => ({
    w: window.innerWidth,
    h: window.innerHeight,
  }));

  const circles = useMemo(() => generateCircles(size.w, size.h), [size]);

  return (
    <div
      className={cn([
        "fixed",
        "inset-0",
        "overflow-hidden",
        "-z-10",
        "print:hidden",
      ])}
    >
      <div
        className={cn([
          "absolute",
          "inset-0",
          "transition-colors",
          "duration-700",
        ])}
      />

      <svg
        viewBox={`0 0 ${size.w} ${size.h}`}
        xmlns="http://www.w3.org/2000/svg"
        aria-hidden="true"
        preserveAspectRatio="xMidYMid slice"
        className={cn([
          "absolute",
          "left-1/2",
          "top-1/2",
          "-translate-x-1/2",
          "-translate-y-1/2",
          "min-w-full",
          "min-h-full",
          "print:hidden",
        ])}
      >
        <g className={cn(["[&_circle]:fill-primary/1"])}>
          {circles.map((circle, i) => (
            <circle
              key={i}
              cx={circle.cx}
              cy={circle.cy}
              r={circle.r}
              className={cn([circle.animation])}
            />
          ))}
        </g>
      </svg>
    </div>
  );
}

export { Background };
