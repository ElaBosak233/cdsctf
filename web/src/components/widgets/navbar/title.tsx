import { FlagIcon } from "lucide-react";
import { useContext } from "react";
import { Link } from "react-router";
import { Image } from "@/components/ui/image";
import { useConfigStore } from "@/storages/config";
import { useGameStore } from "@/storages/game";
import { cn } from "@/utils";
import { Context } from "./context";

type TitleProps = Omit<React.ComponentProps<typeof Link>, "to"> & {};

function Title(props: TitleProps) {
  const { className, ...rest } = props;
  const { mode } = useContext(Context);
  const { currentGame } = useGameStore();
  const configStore = useConfigStore();

  const modeConfig = {
    game: {
      to: `/games/${currentGame?.id}`,
      src: currentGame?.has_icon
        ? `/api/games/${currentGame?.id}/icon`
        : "/api/configs/logo",
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
        fallback={<FlagIcon className={cn("size-6", "rotate-15")} />}
        delay={0}
        className={cn(["h-8", "min-w-8"])}
      />
      <h1
        className={cn([
          "text-xl",
          "font-semibold",
          "overflow-hidden",
          "text-ellipsis",
          "text-nowrap",
          "max-w-24",
          "sm:max-w-full",
        ])}
      >
        {title}
      </h1>
    </Link>
  );
}

export { Title };
