import { InfoIcon } from "lucide-react";
import { Link } from "react-router";

import { Button } from "@/components/ui/button";
import { Image } from "@/components/ui/image";
import { Separator } from "@/components/ui/separator";
import { Typography } from "@/components/ui/typography";
import { MarkdownRender } from "@/components/utils/markdown-render";
import { useDecryptedText } from "@/hooks/use-decrypted-text";
import { useConfigStore } from "@/storages/config";
import { cn } from "@/utils";

export default function () {
  const configStore = useConfigStore();
  const title = useDecryptedText({
    text: configStore?.config?.meta?.title || "",
    speed: 100,
  });
  const description = useDecryptedText({
    text: configStore?.config?.meta?.description || "",
    speed: 25,
  });

  return (
    <>
      <title>{configStore?.config?.meta?.title}</title>
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
            className={cn(["drop-shadow-md", "aspect-square", "h-[8rem]"])}
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
          <span className={cn(["text-secondary-foreground"])}>
            {description}
          </span>
        </div>
        <div className={cn(["hidden", "sm:flex", "items-center", "gap-3"])}>
          <Button>
            <Typography className={cn(["text-secondary-foreground"])}>
              <MarkdownRender src={configStore?.config?.meta?.footer} />
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
