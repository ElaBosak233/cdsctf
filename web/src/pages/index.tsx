import { FlagIcon, InfoIcon } from "lucide-react";
import { Link } from "react-router";

import { Button } from "@/components/ui/button";
import { Image } from "@/components/ui/image";
import { MarkdownRender } from "@/components/ui/markdown-render";
import { Separator } from "@/components/ui/separator";
import { Typography } from "@/components/ui/typography";
import { useDecryptedText } from "@/hooks/use-decrypted-text";
import { useConfigStore } from "@/storages/config";
import { cn } from "@/utils";

export default function Index() {
  const { config } = useConfigStore();
  const title = useDecryptedText({
    text: config?.meta?.title || "",
    speed: 100,
  });
  const description = useDecryptedText({
    text: config?.meta?.description || "",
    speed: 25,
  });

  return (
    <>
      <title>{config?.meta?.title}</title>
      <div
        className={cn([
          "flex-1",
          "flex",
          "flex-col",
          "items-center",
          "justify-between",
          "select-none",
          "my-5",
        ])}
      >
        <div />
        <div
          className={cn([
            "flex",
            "flex-col",
            "items-center",
            "flex-1",
            "justify-center",
          ])}
        >
          <Image
            src={"/api/configs/logo"}
            fallback={<FlagIcon className={cn("size-16", "rotate-15")} />}
            className={cn(["aspect-square", "h-32"])}
            alt={"logo"}
          />
          <h1
            className={cn([
              "text-3xl",
              "lg:text-4xl",
              "font-extrabold",
              "mt-5",
            ])}
          >
            {title}
          </h1>
          <span className={cn(["text-secondary-foreground", "mt-2"])}>
            {description}
          </span>
        </div>
        <div className={cn(["hidden", "sm:flex", "items-center", "gap-3"])}>
          <Button>
            <Typography className={cn(["text-secondary-foreground"])}>
              <MarkdownRender src={config?.meta?.footer} />
            </Typography>
          </Button>
          <Separator orientation={"vertical"} className={cn(["h-5"])} />
          <Button
            square
            asChild
            icon={<InfoIcon />}
            className={cn(["text-secondary-foreground"])}
          >
            <Link to={"/about"} />
          </Button>
        </div>
      </div>
    </>
  );
}
