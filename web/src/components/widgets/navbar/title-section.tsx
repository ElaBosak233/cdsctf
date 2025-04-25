import { cn } from "@/utils";
import { Link } from "react-router";
import { useConfigStore } from "@/storages/config";
import { useContext } from "react";
import { Context } from "./context";
import { useGameStore } from "@/storages/game";
import { Image } from "@/components/ui/image";
import { Flag } from "lucide-react";

interface TitleSectionProps
    extends Omit<React.ComponentProps<typeof Link>, "to"> {}

function TitleSection(props: TitleSectionProps) {
    const { className, ...rest } = props;
    const { mode } = useContext(Context);
    const { currentGame } = useGameStore();
    const configStore = useConfigStore();

    const modeConfig = {
        game: {
            to: `/games/${currentGame?.id}`,
            src: `/api/games/${currentGame?.id}/icon`,
            title: currentGame?.title,
        },
        admin: {
            to: "/admin/platform",
            src: "/api/configs/logo",
            title: configStore?.config?.meta?.title,
        },
        default: {
            to: "/",
            src: "/api/configs/logo",
            title: configStore?.config?.meta?.title,
        },
    };

    const { to, src, title } = modeConfig[mode] || modeConfig.default;

    return (
        <Link
            className={cn([
                "flex",
                "gap-3",
                "items-center",
                "text-foreground",
                className,
            ])}
            to={to}
            {...rest}
        >
            <Image
                src={src}
                fallback={<Flag />}
                className={cn([
                    "drop-shadow-md",
                    "h-8",
                    "w-8",
                    "rounded-lg",
                    "overflow-hidden",
                ])}
            />
            <h1 className={cn(["text-xl", "font-semibold"])}>{title}</h1>
        </Link>
    );
}

export { TitleSection };
