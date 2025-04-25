import { Button } from "@/components/ui/button";
import { Separator } from "@/components/ui/separator";
import { useConfigStore } from "@/storages/config";
import { cn } from "@/utils";
import { CodeXmlIcon, HandshakeIcon, HeartIcon, StarIcon } from "lucide-react";

export default function Index() {
    const { version } = useConfigStore();

    return (
        <div
            className={cn([
                "flex-1",
                "flex",
                "flex-col",
                "items-center",
                "justify-center",
                "select-none",
            ])}
        >
            <div
                className={cn([
                    "max-w-3xl",
                    "w-full",
                    "flex",
                    "flex-col",
                    "items-center",
                    "justify-center",
                    "select-none",
                    "gap-5",
                ])}
            >
                <div className={cn(["flex", "gap-3", "items-center"])}>
                    <img
                        alt="logo"
                        decoding={"async"}
                        src={"/logo.svg"}
                        draggable={false}
                        className={cn([
                            "drop-shadow-md",
                            "aspect-square",
                            "h-17.5",
                        ])}
                    />
                    <div className={cn(["flex", "flex-col", "gap-1"])}>
                        <h1
                            className={cn([
                                "text-2xl",
                                "lg:text-3xl",
                                "font-extrabold",
                            ])}
                        >
                            CdsCTF
                        </h1>
                        <div
                            className={cn([
                                "font-mono",
                                "text-secondary-foreground",
                                "text-md",
                            ])}
                        >
                            {`v${version?.tag} / ${version?.commit?.slice(0, 7)}`}
                        </div>
                    </div>
                </div>
                <Separator className={cn(["w-full"])} />
                <h3 className={cn(["flex", "gap-2", "items-center"])}>
                    <CodeXmlIcon className={cn(["size-5"])} />
                    Developers
                </h3>
                <div
                    className={cn([
                        "flex",
                        "flex-wrap",
                        "justify-center",
                        "gap-3",
                    ])}
                >
                    <Button
                        className={cn(["rounded-full"])}
                        icon={StarIcon}
                        variant={"tonal"}
                        size={"sm"}
                        asChild
                    >
                        <a
                            href={"https://github.com/ElaBosak233"}
                            target={"_blank"}
                        >
                            ElaBosak233
                        </a>
                    </Button>
                    <Button
                        className={cn(["rounded-full"])}
                        icon={StarIcon}
                        variant={"tonal"}
                        size={"sm"}
                        asChild
                    >
                        <a href={"https://github.com/Ec3o"} target={"_blank"}>
                            Ec3o
                        </a>
                    </Button>
                </div>
                <h3 className={cn(["flex", "gap-2", "items-center"])}>
                    <HandshakeIcon className={cn(["size-5"])} />
                    Additional Supports
                </h3>
                <div
                    className={cn([
                        "flex",
                        "flex-wrap",
                        "justify-center",
                        "gap-3",
                    ])}
                >
                    <Button
                        className={cn(["rounded-full"])}
                        icon={StarIcon}
                        variant={"tonal"}
                        size={"sm"}
                        asChild
                    >
                        <a
                            href={"https://github.com/Albertknight2023"}
                            target={"_blank"}
                        >
                            AlbertKnight - Art
                        </a>
                    </Button>
                    <Button
                        className={cn(["rounded-full"])}
                        icon={StarIcon}
                        variant={"tonal"}
                        asChild
                    >
                        <a
                            href={"https://github.com/skyhaibara"}
                            target={"_blank"}
                        >
                            Skyhaibara - Paper works
                        </a>
                    </Button>
                </div>
                <h3 className={cn(["flex", "gap-2", "items-center"])}>
                    <HeartIcon className={cn(["size-5"])} />
                    Special Thanks
                </h3>
                <div
                    className={cn([
                        "flex",
                        "flex-wrap",
                        "justify-center",
                        "gap-3",
                    ])}
                >
                    <Button
                        className={cn(["rounded-full"])}
                        icon={StarIcon}
                        variant={"tonal"}
                        size={"sm"}
                        asChild
                    >
                        <a
                            href={"https://github.com/Reverier-Xu"}
                            target={"_blank"}
                        >
                            Reverier-Xu & Ret 2 Shell
                        </a>
                    </Button>
                </div>
            </div>
        </div>
    );
}
