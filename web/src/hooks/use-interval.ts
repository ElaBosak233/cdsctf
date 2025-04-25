import { useEffect, useRef } from "react";

function useInterval(
    callback: () => void,
    delay: number,
    deps: React.DependencyList,
    options?: {
        immediate?: boolean;
    }
) {
    const savedCallback = useRef<() => void | null>(null);

    useEffect(() => {
        savedCallback.current = callback;
    }, [callback]);

    useEffect(() => {
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
    }, [delay, ...deps]);
}

export { useInterval };
