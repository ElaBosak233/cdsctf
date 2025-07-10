import { CodeXmlIcon, HeartIcon, MoonIcon } from "lucide-react";

import { Button } from "@/components/ui/button";
import { Separator } from "@/components/ui/separator";
import { useConfigStore } from "@/storages/config";
import { cn } from "@/utils";

export default function Index() {
  const { config, version } = useConfigStore();

  return (
    <>
      <title>{`关于 - ${config?.meta?.title}`}</title>
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
              className={cn(["aspect-square", "h-17.5"])}
            />
            <div className={cn(["flex", "flex-col", "gap-1"])}>
              <h1 className={cn(["text-2xl", "lg:text-3xl", "font-extrabold"])}>
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
            <MoonIcon className={cn(["size-5"])} />
            Founder
          </h3>
          <div className={cn(["flex", "flex-wrap", "justify-center", "gap-3"])}>
            <Button
              className={cn(["rounded-full"])}
              variant={"tonal"}
              size={"sm"}
              asChild
            >
              <a href={"https://github.com/ElaBosak233"} target={"_blank"}>
                <span
                  className={cn(["size-1.5", "rounded-full", "bg-info"])}
                  aria-hidden="true"
                />
                ElaBosak233
              </a>
            </Button>
          </div>
          <h3 className={cn(["flex", "gap-2", "items-center"])}>
            <CodeXmlIcon className={cn(["size-5"])} />
            Contributors
          </h3>
          <div className={cn(["flex", "flex-wrap", "justify-center", "gap-3"])}>
            <Button
              className={cn(["rounded-full"])}
              variant={"tonal"}
              size={"sm"}
              asChild
            >
              <a href={"https://github.com/Ec3o"} target={"_blank"}>
                <span
                  className={cn(["size-1.5", "rounded-full", "bg-success"])}
                  aria-hidden="true"
                />
                Ec3o
              </a>
            </Button>
            <Button
              className={cn(["rounded-full"])}
              variant={"tonal"}
              size={"sm"}
              asChild
            >
              <a href={"https://github.com/Albertknight2023"} target={"_blank"}>
                <span
                  className={cn(["size-1.5", "rounded-full", "bg-success"])}
                  aria-hidden="true"
                />
                AlbertKnight
              </a>
            </Button>
            <Button
              className={cn(["rounded-full"])}
              variant={"tonal"}
              size={"sm"}
              asChild
            >
              <a href={"https://github.com/skyhaibara"} target={"_blank"}>
                <span
                  className={cn(["size-1.5", "rounded-full", "bg-success"])}
                  aria-hidden="true"
                />
                Skyhaibara
              </a>
            </Button>
          </div>
          <h3 className={cn(["flex", "gap-2", "items-center"])}>
            <HeartIcon className={cn(["size-5"])} />
            Special Thanks
          </h3>
          <div className={cn(["flex", "flex-wrap", "justify-center", "gap-3"])}>
            <Button
              className={cn(["rounded-full"])}
              variant={"tonal"}
              size={"sm"}
              asChild
            >
              <a href={"https://github.com/Reverier-Xu"} target={"_blank"}>
                <span
                  className={cn(["size-1.5", "rounded-full", "bg-pink-400"])}
                  aria-hidden="true"
                />
                Reverier-Xu & Ret 2 Shell
              </a>
            </Button>
          </div>
        </div>
      </div>
    </>
  );
}
