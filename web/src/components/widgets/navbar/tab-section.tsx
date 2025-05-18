import { Link, useLocation } from "react-router";

import { useOptions } from "./context";

import { Button } from "@/components/ui/button";

function TabSection() {
  const location = useLocation();
  const pathname = location.pathname;
  const options = useOptions();

  return (
    <>
      {options?.map((option, index) => {
        const Comp = option?.disabled ? Button : Link;

        return (
          <Button
            key={index}
            asChild
            variant={pathname === option?.link ? "tonal" : "ghost"}
            size={"sm"}
            className={"font-semibold"}
            disabled={option?.disabled}
            icon={option.icon}
            level={option?.warning ? "warning" : "primary"}
          >
            <Comp to={option.link}>{option?.name}</Comp>
          </Button>
        );
      })}
    </>
  );
}

export { TabSection };
