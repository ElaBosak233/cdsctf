import { Button } from "@/components/ui/button";
import {
    DropdownMenu,
    DropdownMenuContent,
    DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { Separator } from "@/components/ui/separator";
import { useThemeStore } from "@/storages/theme";
import { cn } from "@/utils";
import { Brush, Sun, Moon, Eclipse } from "lucide-react";

function AppearanceDropdown() {
    const { setTheme } = useThemeStore();

    return (
        <DropdownMenu>
            <DropdownMenuTrigger asChild>
                <Button variant={"ghost"} square size={"sm"} icon={Brush} />
            </DropdownMenuTrigger>
            <DropdownMenuContent className={"w-36"}>
                <div className={cn(["flex", "h-9", "justify-between"])}>
                    <Button
                        size={"sm"}
                        square
                        icon={Sun}
                        onClick={() => setTheme("light")}
                    />
                    <Separator orientation="vertical" />
                    <Button
                        size={"sm"}
                        icon={Moon}
                        square
                        onClick={() => setTheme("dark")}
                    />
                    <Separator orientation="vertical" />
                    <Button
                        size={"sm"}
                        square
                        icon={Eclipse}
                        onClick={() => setTheme("system")}
                    />
                </div>
            </DropdownMenuContent>
        </DropdownMenu>
    );
}

export { AppearanceDropdown };
