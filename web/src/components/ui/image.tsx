import { CircleOff, LoaderCircle } from "lucide-react";
import type React from "react";
import { useEffect, useRef, useState } from "react";

import { cn } from "@/utils";

type ImageProps = {
  src?: string | false;
  alt?: string;
  delay?: number;
  glass?: boolean;
  fallback?: React.ReactNode;
  className?: string;
};

function Image(props: ImageProps) {
  const {
    src,
    alt,
    delay = 500,
    glass = true,
    fallback = (
      <CircleOff
        className={cn(["w-1/5", "h-1/5", "text-secondary-foreground"])}
      />
    ),
    className,
  } = props;

  const [isLoading, setIsLoading] = useState<boolean>(true);
  const [hasError, setHasError] = useState<boolean>(false);
  const errorTimerRef = useRef<ReturnType<typeof setTimeout>>();

  useEffect(() => {
    if (errorTimerRef.current) {
      clearTimeout(errorTimerRef.current);
    }

    if (src) {
      setIsLoading(true);
      setHasError(false);
    } else {
      setIsLoading(true);
      errorTimerRef.current = setTimeout(() => {
        setIsLoading(false);
        setHasError(true);
      }, 2000);
    }

    return () => {
      if (errorTimerRef.current) {
        clearTimeout(errorTimerRef.current);
      }
    };
  }, [src]);

  function handleLoad() {
    if (errorTimerRef.current) {
      clearTimeout(errorTimerRef.current);
    }
    setTimeout(() => {
      setIsLoading(false);
    }, delay);
  }

  function handleError() {
    errorTimerRef.current = setTimeout(() => {
      setIsLoading(false);
      setHasError(true);
    }, 2000);
  }

  const imgHidden = glass ? hasError : isLoading || hasError;

  return (
    <div className={cn(["relative", "overflow-hidden"], className)}>
      <img
        src={src || undefined}
        alt={alt}
        decoding={"async"}
        onLoad={handleLoad}
        onError={handleError}
        draggable={false}
        className={cn([
          "w-full",
          "h-full",
          "object-cover",
          glass && ["transition-all", "duration-700", "ease-out"],
          glass && isLoading && !hasError && "scale-105",
          imgHidden && "hidden",
        ])}
      />

      {glass ? (
        !hasError && (
          <div
            className={cn([
              "absolute",
              "inset-0",
              "flex",
              "items-center",
              "justify-center",
              "bg-white/10",
              "dark:bg-black/10",
              "backdrop-blur-lg",
              "pointer-events-none",
              "transition-opacity",
              "duration-700",
              "ease-out",
              isLoading ? "opacity-100" : "opacity-0",
            ])}
          >
            {isLoading && (
              <LoaderCircle
                className={cn(["animate-spin", "text-foreground"])}
                size={24}
              />
            )}
          </div>
        )
      ) : (
        isLoading && (
          <div
            className={cn([
              "absolute",
              "inset-0",
              "flex",
              "items-center",
              "justify-center",
            ])}
          >
            <LoaderCircle
              className={cn(["animate-spin", "text-foreground"])}
              size={24}
            />
          </div>
        )
      )}

      {hasError && (
        <div
          className={cn([
            "absolute",
            "inset-0",
            "flex",
            "items-center",
            "justify-center",
            "text-foreground",
          ])}
        >
          {fallback}
        </div>
      )}
    </div>
  );
}

export { Image, type ImageProps };
