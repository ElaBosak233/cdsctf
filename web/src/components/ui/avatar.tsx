import { Avatar as BaseAvatar } from "@base-ui/react/avatar";
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

type AvatarProps = React.ComponentProps<typeof BaseAvatar.Root> &
  VariantProps<typeof avatarVariants> & {
    src?: string | false | null;
    fallback?: React.ReactNode;
    fit?: "cover" | "contain" | "fill" | "none" | "scale-down";
    aspect?: "square" | "video";
    onLoadingStatusChange?: (
      status: "idle" | "loading" | "loaded" | "error"
    ) => void;
  };

function Avatar(props: AvatarProps) {
  const {
    src,
    fallback,
    fit = "cover",
    aspect = "square",
    square,
    className,
    ref,
    children,
    onLoadingStatusChange,
    ...rest
  } = props;
  const [loading, setLoading] = useState<boolean>(Boolean(src));

  return (
    <BaseAvatar.Root
      ref={ref}
      className={cn(avatarVariants({ square, className }))}
      {...rest}
    >
      <AvatarImage
        src={src || undefined}
        fit={fit}
        aspect={aspect}
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
    </BaseAvatar.Root>
  );
}

function AvatarImage({
  className,
  ref,
  fit = "cover",
  aspect = "square",
  ...rest
}: React.ComponentProps<typeof BaseAvatar.Image> & {
  fit?: string;
  aspect?: "square" | "video";
}) {
  return (
    <BaseAvatar.Image
      ref={ref}
      className={cn(
        [
          aspect === "video" ? "aspect-video" : "aspect-square",
          "h-full",
          "w-full",
        ],
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
}: React.ComponentProps<typeof BaseAvatar.Fallback>) {
  return (
    <BaseAvatar.Fallback
      ref={ref}
      className={cn(
        ["flex", "h-full", "w-full", "items-center", "justify-center"],
        className
      )}
      {...rest}
    />
  );
}

function AvatarGroup({ className, ...props }: React.ComponentProps<"div">) {
  return (
    <div
      data-slot="avatar-group"
      className={cn(
        "group/avatar-group flex -space-x-2",
        // "*:data-[slot=avatar]:ring-2 *:data-[slot=avatar]:ring-background",
        className
      )}
      {...props}
    />
  );
}
function AvatarGroupCount({
  className,
  ...props
}: React.ComponentProps<"div">) {
  return (
    <div
      data-slot="avatar-group-count"
      className={cn(
        "relative flex size-9 shrink-0 items-center justify-center rounded-full bg-input text-sm text-muted-foreground group-has-data-[size=lg]/avatar-group:size-10 group-has-data-[size=sm]/avatar-group:size-6 [&>svg]:size-4 group-has-data-[size=lg]/avatar-group:[&>svg]:size-5 group-has-data-[size=sm]/avatar-group:[&>svg]:size-3",
        className
      )}
      {...props}
    />
  );
}

export { Avatar, AvatarGroup, AvatarGroupCount };
