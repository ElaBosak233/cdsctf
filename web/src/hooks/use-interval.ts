import { useEffect, useRef } from "react";

function useInterval(
  callback: () => void,
  delay: number,
  deps: React.DependencyList,
  options?: {
    immediate?: boolean;
    /** When `false`, the timer is not started and no ticks run. Defaults to `true`. */
    enabled?: boolean;
  }
) {
  const savedCallback = useRef<() => void | null>(null);
  const enabled = options?.enabled !== false;

  useEffect(() => {
    savedCallback.current = callback;
  }, [callback]);

  useEffect(() => {
    if (!enabled) return;

    function tick() {
      if (savedCallback.current) {
        savedCallback.current();
      }
    }
    if (options?.immediate) {
      tick();
    }
    const id = setInterval(tick, delay);
    return () => clearInterval(id);
  }, [delay, enabled, options?.immediate, ...deps]);
}

export { useInterval };
