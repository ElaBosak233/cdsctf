import { Button } from "@/components/ui/button";
import {
    DropdownMenu,
    DropdownMenuContent,
    DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { Separator } from "@/components/ui/separator";
import { useApperanceStore } from "@/storages/appearance";
import { cn } from "@/utils";
import { MoonIcon, BrushIcon, SunIcon, EclipseIcon } from "lucide-react";
import { useTranslation } from "react-i18next";

function Appearance() {
    const { setTheme } = useApperanceStore();
    const { i18n } = useTranslation();

    return (
        <DropdownMenu>
            <DropdownMenuTrigger asChild>
                <Button
                    variant={"ghost"}
                    square
                    size={"sm"}
                    icon={<BrushIcon />}
                />
            </DropdownMenuTrigger>
            <DropdownMenuContent className={cn(["space-y-1"])}>
                <div className={cn(["flex", "h-9", "justify-evenly"])}>
                    <Button
                        size={"sm"}
                        square
                        icon={<SunIcon />}
                        onClick={() => setTheme("light")}
                    />
                    <Separator orientation="vertical" />
                    <Button
                        size={"sm"}
                        icon={<MoonIcon />}
                        square
                        onClick={() => setTheme("dark")}
                    />
                    <Separator orientation="vertical" />
                    <Button
                        size={"sm"}
                        square
                        icon={<EclipseIcon />}
                        onClick={() => setTheme("system")}
                    />
                </div>
                <Separator />
                <div className={cn(["flex", "h-9", "justify-evenly", "gap-1"])}>
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
            </DropdownMenuContent>
        </DropdownMenu>
    );
}

export { Appearance };
