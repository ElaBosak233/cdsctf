import { useThemeStore } from "@/storages/theme";
import { cn } from "@/utils";
import {
    CircleAlertIcon,
    CircleCheckIcon,
    InfoIcon,
    LoaderCircleIcon,
    TriangleAlertIcon,
} from "lucide-react";
import { Toaster as Sonner } from "sonner";

type ToasterProps = React.ComponentProps<typeof Sonner>;

function Toaster(props: ToasterProps) {
    const { ...rest } = props;
    const { theme } = useThemeStore();

    return (
        <Sonner
            richColors
            theme={theme as ToasterProps["theme"]}
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
                        "group-[.toaster]:!px-5",
                        "group-[.toaster]:!bg-card",
                        "group-[.toaster]:!text-foreground",
                        "group-[.toaster]:!border-border",
                        "group-[.toaster]:!rounded-lg",
                        "group-[.toaster]:!shadow-lg",
                    ]),
                    icon: cn(["group-[.toast]:!size-5", "[&_svg]:size-5"]),
                    description: cn([
                        "group-[.toast]:!text-secondary-foreground",
                    ]),
                    actionButton: cn([
                        "group-[.toast]:!bg-primary",
                        "group-[.toast]:!text-primary-foreground",
                    ]),
                    cancelButton: cn([
                        "group-[.toast]:!bg-secondary",
                        "group-[.toast]:!text-secondary-foreground",
                    ]),
                },
            }}
            {...rest}
        />
    );
}

export { Toaster };
