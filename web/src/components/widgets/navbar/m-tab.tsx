import { Button } from "@/components/ui/button";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { cn } from "@/utils";
import { ListIcon } from "lucide-react";
import { useOptions } from "./context";
import { Link } from "react-router";

function MobileTab() {
  const options = useOptions();

  return (
    <DropdownMenu>
      <DropdownMenuTrigger asChild className={cn(["lg:hidden", "mr-3"])}>
        <Button square size={"sm"} icon={<ListIcon />} />
      </DropdownMenuTrigger>
      <DropdownMenuContent className={cn(["space-y-1"])}>
        {options?.map((option, index) => {
          const Comp = option?.disabled ? DropdownMenuItem : Link;

          return (
            <DropdownMenuItem
              key={index}
              disabled={option?.disabled}
              icon={option?.icon}
              asChild
            >
              <Comp to={option.link}>{option.name}</Comp>
            </DropdownMenuItem>
          );
        })}
      </DropdownMenuContent>
    </DropdownMenu>
  );
}

export { MobileTab };
