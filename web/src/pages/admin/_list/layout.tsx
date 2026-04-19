import type { ReactNode } from "react";
import { Outlet } from "react-router";
import { Card } from "@/components/ui/card";
import { cn } from "@/utils";
import { AdminListContext, type AdminListContextValue } from "./context";

export type AdminListLayoutProps = {
  value: AdminListContextValue;
  sidebar: ReactNode;
  children?: ReactNode;
};

export function AdminListLayout({
  value,
  sidebar,
  children,
}: AdminListLayoutProps) {
  return (
    <AdminListContext.Provider value={value}>
      <div
        className={cn(
          "flex flex-col xl:flex-row xl:min-h-(--app-content-height) flex-1 min-h-0 xl:pl-64"
        )}
      >
        <aside
          className={cn(
            "hidden xl:flex xl:fixed xl:left-16 xl:top-16 xl:z-10 xl:h-(--app-content-height) xl:w-64 xl:flex-col xl:border-r xl:bg-card/30 xl:backdrop-blur-sm",
            "py-5 px-4 gap-4 overflow-y-auto"
          )}
        >
          {sidebar}
        </aside>

        <Card
          className={cn(
            "h-(--app-content-height) flex-1 min-h-0 min-w-0 border-y-0 rounded-none flex flex-col xl:rounded-l-none"
          )}
        >
          {children ?? <Outlet />}
        </Card>
      </div>
    </AdminListContext.Provider>
  );
}
