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
      <DropdownMenuTrigger
        className={cn(["lg:hidden", "mr-1", "sm:mr-3"])}
        render={
          <Button
            square
            size={"sm"}
            icon={<ListIcon />}
            aria-label="Open navigation"
          />
        }
      />
      <DropdownMenuContent sideOffset={20} className={cn(["space-y-1"])}>
        {options?.map((option, index) => {
          const Comp = option?.disabled ? DropdownMenuItem : Link;

          return (
            <DropdownMenuItem
              key={index}
              disabled={option?.disabled}
              render={<Comp to={option.link} />}
            >
              {option?.icon}
              {option.name}
            </DropdownMenuItem>
          );
        })}
      </DropdownMenuContent>
    </DropdownMenu>
  );
}

export { MobileTab };
