import { Navbar } from "@/components/widgets/navbar";
import { Outlet, useNavigate } from "react-router";
import globalRouter from "@/utils/global-router";
import { cn } from "@/utils";

export default function () {
    const navigate = useNavigate();
    globalRouter.navigate = navigate;

    return (
        <div>
            <Navbar />
            <main
                className={cn(["flex", "flex-col", "min-h-[calc(100vh-64px)]"])}
            >
                <Outlet />
            </main>
        </div>
    );
}
