import { BrushIcon, MonitorDotIcon, SunMoonIcon } from "lucide-react";
import { useTranslation } from "react-i18next";

import { Button } from "@/components/ui/button";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { Separator } from "@/components/ui/separator";
import { ThemeSwitch } from "@/pages/_blocks/navbar/theme-switch";
import { useApperanceStore } from "@/storages/appearance";
import { cn } from "@/utils";

function Appearance() {
  const { theme, computedTheme, setTheme } = useApperanceStore();
  const { i18n } = useTranslation();

  return (
    <DropdownMenu>
      <DropdownMenuTrigger asChild>
        <Button variant={"ghost"} square size={"sm"} icon={<BrushIcon />} />
      </DropdownMenuTrigger>
      <DropdownMenuContent
        sideOffset={20}
        className={cn([
          "space-y-1",
          "bg-transparent",
          "border-none",
          "shadow-none",
          "rounded-none",
          "overflow-visible",
          "p-0",
        ])}
      >
        <div
          className={cn([
            "flex",
            "justify-evenly",
            "gap-1",
            "bg-popover",
            "p-1",
            "border",
            "rounded-md",
            "shadow-md",
          ])}
        >
          <Button
            size={"sm"}
            square
            onClick={() => i18n.changeLanguage("en-US")}
          >
            EN
          </Button>
          <Separator orientation="vertical" />
          <Button
            size={"sm"}
            square
            onClick={() => i18n.changeLanguage("zh-CN")}
          >
            简
          </Button>
          <Separator orientation="vertical" />
          <Button
            size={"sm"}
            square
            onClick={() => i18n.changeLanguage("zh-TW")}
          >
            繁
          </Button>
          <Separator orientation="vertical" />
          <Button
            size={"sm"}
            square
            onClick={() => i18n.changeLanguage("ja-JP")}
          >
            な
          </Button>
        </div>
        <div
          className={cn([
            "flex",
            "justify-evenly",
            "gap-1",
            "bg-popover",
            "p-1",
            "border",
            "rounded-md",
            "shadow-md",
          ])}
        >
          <Button
            size={"sm"}
            icon={theme === "system" ? <SunMoonIcon /> : <MonitorDotIcon />}
            square
            onClick={() =>
              setTheme(theme === "system" ? computedTheme : "system")
            }
          />
          <ThemeSwitch />
        </div>
      </DropdownMenuContent>
    </DropdownMenu>
  );
}

export { Appearance };
