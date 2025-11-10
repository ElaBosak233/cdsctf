import { ListIcon } from "lucide-react";
import { Link } from "react-router";
import { Button } from "@/components/ui/button";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { cn } from "@/utils";
import { useOptions } from "./context";

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
            <DropdownMenuItem key={index} disabled={option?.disabled} asChild>
              <Comp to={option.link}>
                {option?.icon}
                {option.name}
              </Comp>
            </DropdownMenuItem>
          );
        })}
      </DropdownMenuContent>
    </DropdownMenu>
  );
}

export { MobileTab };
