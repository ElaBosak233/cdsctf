import { IdCardIcon, LinkIcon, UnplugIcon } from "lucide-react";
import { useCallback, useEffect, useState } from "react";
import { useTranslation } from "react-i18next";
import { toast } from "sonner";
import { getIdps } from "@/api/idps";
import { getMyIdps, unbindMyIdp } from "@/api/users/me/idp";
import { Avatar } from "@/components/ui/avatar";
import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";
import { Separator } from "@/components/ui/separator";
import type { Idp, UserIdp } from "@/models/idp";
import { useConfigStore } from "@/storages/config";
import { cn } from "@/utils";

export default function Index() {
  const { config } = useConfigStore();
  const { t } = useTranslation();
  const [idps, setIdps] = useState<Idp[]>([]);
  const [bound, setBound] = useState<UserIdp[]>([]);

  const refresh = useCallback(async () => {
    const [idpRes, boundRes] = await Promise.all([getIdps(), getMyIdps()]);
    setIdps(idpRes.idps ?? []);
    setBound(boundRes.idps ?? []);
  }, []);

  useEffect(() => {
    refresh();
  }, [refresh]);

  async function handleUnbind(id: number) {
    await unbindMyIdp(id);
    toast.success(t("user:idp.actions.unbound_toast"));
    refresh();
  }

  return (
    <>
      <title>{`${t("user:idp._")} - ${config?.meta?.title}`}</title>
      <div
        className={cn([
          "p-10",
          "flex",
          "flex-col",
          "gap-5",
          "xl:mx-50",
          "lg:mx-30",
        ])}
      >
        <div className={cn(["flex", "items-center", "gap-3"])}>
          <div
            className={cn([
              "flex items-center justify-center",
              "size-10 rounded-badge",
              "bg-primary/10",
              "shrink-0",
            ])}
          >
            <IdCardIcon className={cn(["size-5"])} />
          </div>
          <h2 className={cn(["text-base", "font-semibold"])}>
            {t("user:idp._")}
          </h2>
        </div>
        <Separator />
        <div className={cn(["grid", "gap-3"])}>
          {idps.map((idp) => {
            const item = bound.find((v) => v.idp_id === idp.id);
            return (
              <Card
                key={idp.id}
                className={cn(["p-4", "flex", "items-center", "gap-4"])}
              >
                <Avatar
                  square
                  className={cn(["size-11", "bg-transparent", "border-none"])}
                  src={idp.avatar_hash && `/api/media?hash=${idp.avatar_hash}`}
                  fallback={idp.name?.charAt(0)}
                />
                <div className={cn(["flex-1", "min-w-0"])}>
                  <div className={cn(["font-medium"])}>{idp.name}</div>
                  <div className={cn(["text-sm", "text-secondary-foreground"])}>
                    {item?.auth_key || `#${idp.id}`}
                  </div>
                </div>
                {item?.id ? (
                  <Button
                    variant="tonal"
                    level="error"
                    icon={<UnplugIcon />}
                    onClick={() => handleUnbind(item.id!)}
                  >
                    {t("user:idp.actions.unbind")}
                  </Button>
                ) : (
                  <Button asChild variant="solid" icon={<LinkIcon />}>
                    <a href={idp.portal || `/account/idps/${idp.id ?? ""}`}>
                      {t("user:idp.actions.bind")}
                    </a>
                  </Button>
                )}
              </Card>
            );
          })}
        </div>
      </div>
    </>
  );
}
