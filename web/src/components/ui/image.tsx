import { CircleOff, LoaderCircle } from "lucide-react";
import type React from "react";
import { useEffect, useState } from "react";

import { cn } from "@/utils";

type ImageProps = {
  src: string;
  alt?: string;
  delay?: number;
  fallback?: React.ReactNode;
  className?: string;
};

function Image(props: ImageProps) {
  const {
    src,
    alt,
    delay = 500,
    fallback = (
      <CircleOff
        className={cn(["w-1/5", "h-1/5", "text-secondary-foreground"])}
      />
    ),
    className,
  } = props;

  const [isLoading, setIsLoading] = useState<boolean>(true);
  const [hasError, setHasError] = useState<boolean>(false);

  useEffect(() => {
    void src;

    setIsLoading(true);
    setHasError(false);
  }, [src]);

  function handleLoad() {
    setTimeout(() => {
      setIsLoading(false);
    }, delay);
  }

  function handleError() {
    setIsLoading(false);
    setHasError(true);
  }

  return (
    <div className={cn(["relative"], className)}>
      {(isLoading || hasError) && (
        <div
          className={cn([
            "absolute",
            "inset-0",
            "flex",
            "items-center",
            "justify-center",
            "bg-opacity-50",
            "text-foreground",
            "bg-transparent",
          ])}
        >
          {isLoading ? (
            <LoaderCircle
              className={cn(["animate-spin", "text-foreground"])}
              size={24}
            />
          ) : (
            fallback
          )}
        </div>
      )}

      <img
        src={src}
        alt={alt}
        decoding={"async"}
        onLoad={handleLoad}
        onError={handleError}
        draggable={false}
        className={cn([
          "w-full",
          "h-full",
          "object-cover",
          isLoading || hasError ? "hidden" : "block",
        ])}
      />
    </div>
  );
}

export { Image, type ImageProps };
