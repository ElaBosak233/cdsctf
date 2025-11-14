import { LogInIcon, UserRoundPlusIcon } from "lucide-react";
import { useEffect } from "react";
import { useTranslation } from "react-i18next";
import { Link, useNavigate } from "react-router";
import { toast } from "sonner";
import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";
import { Separator } from "@/components/ui/separator";
import { useAuthStore } from "@/storages/auth";
import { useConfigStore } from "@/storages/config";
import { cn } from "@/utils";
import { LoginForm } from "./login-form";

export default function Index() {
  const { config } = useConfigStore();
  const navigate = useNavigate();
  const { t } = useTranslation();

  useEffect(() => {
    if (!useAuthStore.getState().user) return;

    navigate("/");
    toast.warning("你已经登录了", {
      id: "login-already",
    });
  }, [navigate]);

  return (
    <>
      <title>{`登录 - ${config?.meta?.title}`}</title>
      <div className={cn(["flex-1", "flex", "items-center", "justify-center"])}>
        <Card className={cn(["p-2", "w-[50rem]", "flex", "justify-between"])}>
          <div className={cn(["flex-1/2", "flex", "flex-col"])}>
            <div className={cn(["flex", "flex-col", "space-y-1.5", "p-6"])}>
              <div
                className={cn([
                  "text-2xl",
                  "font-semibold",
                  "leading-none",
                  "tracking-tight",
                  "flex",
                  "gap-2",
                  "items-center",
                ])}
              >
                <LogInIcon />
                {t("account.login._")}
              </div>
              <div className={cn(["text-sm", "text-secondary-foreground"])}>
                {`${t("account.login.continue")} ${config?.meta?.title}`}
              </div>
              <div className={cn(["pt-6"])}>
                <LoginForm />
              </div>
            </div>
          </div>
          <Separator
            orientation={"vertical"}
            className={cn(["hidden", "md:block", "h-81", "my-auto"])}
          />
          <div
            className={cn(["hidden", "md:flex", "flex-col", "flex-1/2", "p-6"])}
          >
            <div
              className={cn([
                "flex",
                "flex-col",
                "flex-1",
                "items-center",
                "justify-center",
                "select-none",
              ])}
            >
              <img
                alt="logo"
                decoding={"async"}
                src={"/api/configs/logo"}
                draggable={false}
                className={cn(["drop-shadow-md", "aspect-square", "h-[8rem]"])}
              />
              <span className={cn(["mt-4", "text-2xl", "font-semibold"])}>
                {config?.meta?.title}
              </span>
              <span className={cn(["text-sm", "text-secondary-foreground"])}>
                {config?.meta?.description}
              </span>
            </div>
            {config?.auth?.is_registration_enabled && (
              <Button
                asChild
                className={cn("w-full")}
                size={"lg"}
                variant={"tonal"}
                icon={<UserRoundPlusIcon />}
              >
                <Link to={"/account/register"}>
                  {t("account.register.not_yet")}
                </Link>
              </Button>
            )}
          </div>
        </Card>
      </div>
    </>
  );
}
