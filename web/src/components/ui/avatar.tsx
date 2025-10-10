import * as RadixAvatar from "@radix-ui/react-avatar";
import { cva, type VariantProps } from "class-variance-authority";
import { LoaderCircleIcon } from "lucide-react";
import type * as React from "react";
import { useState } from "react";
import { cn } from "@/utils";

const avatarVariants = cva(
  [
    "relative",
    "flex",
    "h-10",
    "w-10",
    "shrink-0",
    "overflow-hidden",
    "bg-input",
  ],
  {
    variants: {
      square: {
        false: "rounded-full",
        true: "rounded-sm",
      },
    },
    defaultVariants: {
      square: false,
    },
  }
);

type AvatarProps = React.ComponentProps<typeof RadixAvatar.Root> &
  VariantProps<typeof avatarVariants> & {
    src: string;
    fallback?: React.ReactNode;
    fit?: "cover" | "contain" | "fill" | "none" | "scale-down";
    onLoadingStatusChange?: (
      status: "idle" | "loading" | "loaded" | "error"
    ) => void;
  };

function Avatar(props: AvatarProps) {
  const {
    src,
    fallback,
    fit = "cover",
    square,
    className,
    ref,
    children,
    onLoadingStatusChange,
    ...rest
  } = props;
  const [loading, setLoading] = useState<boolean>(true);

  return (
    <RadixAvatar.Root
      ref={ref}
      className={cn(avatarVariants({ square, className }))}
      {...rest}
    >
      <AvatarImage
        src={src}
        fit={fit}
        onLoadingStatusChange={(status) => {
          setLoading(status === "loading");
          onLoadingStatusChange?.(status);
        }}
      />
      <AvatarFallback>{!loading && fallback}</AvatarFallback>
      {children}
      <div
        className={cn([
          "absolute",
          "top-0",
          "left-0",
          "w-full",
          "h-full",
          "flex",
          "items-center",
          "justify-center",
          !loading && "hidden",
        ])}
      >
        <LoaderCircleIcon
          className={cn(["h-5", "w-5", "animate-spin", "text-primary"])}
        />
      </div>
    </RadixAvatar.Root>
  );
}

function AvatarImage({
  className,
  ref,
  fit = "cover",
  ...rest
}: React.ComponentProps<typeof RadixAvatar.Image> & { fit?: string }) {
  return (
    <RadixAvatar.Image
      ref={ref}
      className={cn(
        ["aspect-square", "h-full", "w-full"],
        fit === "cover" && "object-cover",
        fit === "contain" && "object-contain",
        fit === "fill" && "object-fill",
        fit === "none" && "object-none",
        fit === "scale-down" && "object-scale-down",
        className
      )}
      draggable={false}
      {...rest}
    />
  );
}

function AvatarFallback({
  className,
  ref,
  ...rest
}: React.ComponentProps<typeof RadixAvatar.Fallback>) {
  return (
    <RadixAvatar.Fallback
      ref={ref}
      className={cn(
        ["flex", "h-full", "w-full", "items-center", "justify-center"],
        className
      )}
      {...rest}
    />
  );
}

export { Avatar };
