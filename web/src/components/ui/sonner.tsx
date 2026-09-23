import {
  CircleAlertIcon,
  CircleCheckIcon,
  InfoIcon,
  LoaderCircleIcon,
  TriangleAlertIcon,
} from "lucide-react";
import { Toaster as Sonner } from "sonner";

import { useApperanceStore } from "@/storages/appearance";
import { cn } from "@/utils";

type ToasterProps = React.ComponentProps<typeof Sonner>;

function Toaster(props: ToasterProps) {
  const { ...rest } = props;
  const { computedTheme } = useApperanceStore();

  return (
    <Sonner
      richColors
      theme={computedTheme as ToasterProps["theme"]}
      position="bottom-right"
      closeButton
      expand={false}
      visibleToasts={4}
      gap={10}
      offset={{ bottom: 24, right: 24 }}
      mobileOffset={{ bottom: 16, left: 16, right: 16 }}
      className={cn(["toaster", "group"])}
      icons={{
        info: <InfoIcon />,
        loading: <LoaderCircleIcon className={cn(["animate-spin"])} />,
        error: <CircleAlertIcon />,
        warning: <TriangleAlertIcon />,
        success: <CircleCheckIcon />,
      }}
      toastOptions={{
        classNames: {
          toast: cn([
            "group",
            "toast",
            "group-[.toaster]:!gap-2",
            "group-[.toaster]:!px-3.5",
            "group-[.toaster]:!pl-4",
            "group-[.toaster]:!py-2.5",
            "group-[.toaster]:!pr-9",
            "group-[.toaster]:!bg-card",
            "group-[.toaster]:!text-foreground",
            "group-[.toaster]:!border-border",
            "group-[.toaster]:!border-l-4",
            "data-[type=success]:!border-l-success",
            "data-[type=error]:!border-l-error",
            "data-[type=warning]:!border-l-warning",
            "data-[type=info]:!border-l-info",
            "data-[type=loading]:!border-l-primary",
            "group-[.toaster]:!rounded-lg",
            "group-[.toaster]:!shadow-md",
            "group-[.toaster]:!duration-200",
          ]),
          icon: cn([
            "group-[.toast]:!size-4",
            "group-[.toast]:!self-center",
            "group-[.toast]:!ml-0",
            "group-[.toast]:!mr-0",
            "[&_svg]:size-4",
          ]),
          title: cn([
            "group-[.toast]:!text-sm",
            "group-[.toast]:!font-semibold",
            "group-[.toast]:!leading-5",
          ]),
          description: cn([
            "group-[.toast]:!mt-0.5",
            "group-[.toast]:!text-xs",
            "group-[.toast]:!leading-4",
            "group-[.toast]:!text-muted-foreground",
          ]),
          actionButton: cn([
            "group-[.toast]:!bg-primary",
            "group-[.toast]:!text-primary-foreground",
          ]),
          cancelButton: cn([
            "group-[.toast]:!bg-secondary",
            "group-[.toast]:!text-secondary-foreground",
          ]),
          closeButton: cn([
            "group-[.toast]:!left-auto",
            "group-[.toast]:!right-2",
            "group-[.toast]:!top-1/2",
            "group-[.toast]:!transform-none",
            "group-[.toast]:!-translate-y-1/2",
            "group-[.toast]:!size-6",
            "group-[.toast]:!rounded-md",
            "group-[.toast]:!border-0",
            "group-[.toast]:!bg-transparent",
            "group-[.toast]:!text-muted-foreground",
            "group-[.toast]:hover:!bg-muted",
            "group-[.toast]:hover:!text-foreground",
          ]),
        },
        duration: 4000,
      }}
      {...rest}
    />
  );
}

export { Toaster };
