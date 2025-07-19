import { useCallback, useState } from "react";

export const useRefresh = () => {
  const [tick, setTick] = useState(0);
  const bump = useCallback(() => {
    setTick((t) => t + 1);
  }, []);
  return { tick, bump };
};
