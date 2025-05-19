import { LoaderCircleIcon } from "lucide-react";

import { cn } from "@/utils";

type LoadingOverlayProps = {
  loading: boolean;
  className?: string;
};

function LoadingOverlay(props: LoadingOverlayProps) {
  const { loading, className } = props;

  if (!loading) return null;

  return (
    <div
      className={cn(
        [
          "absolute",
          "inset-0",
          "z-1",
          "flex",
          "items-center",
          "justify-center",
          "backdrop-blur-sm",
          "rounded",
        ],
        className
      )}
    >
      <LoaderCircleIcon
        className={cn([["h-8", "w-8", "animate-spin", "text-primary"]])}
      />
    </div>
  );
}

export { LoadingOverlay };
