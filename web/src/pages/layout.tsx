import { Outlet, useNavigate } from "react-router";

import { Navbar } from "@/components/widgets/navbar";
import { cn } from "@/utils";
import globalRouter from "@/utils/global-router";

export default function () {
  const navigate = useNavigate();
  globalRouter.navigate = navigate;

  return (
    <>
      <Navbar />
      <main className={cn(["flex", "flex-col", "min-h-[calc(100vh-64px)]"])}>
        <Outlet />
      </main>
    </>
  );
}
