import {
  IdCardIcon,
  LinkIcon,
  LockKeyholeIcon,
  UnplugIcon,
} from "lucide-react";
import { useCallback, useEffect, useState } from "react";
import { useTranslation } from "react-i18next";
import { toast } from "sonner";
import { getIdps } from "@/api/idps";
import { getMyIdps, unbindMyIdp } from "@/api/users/me/idp";
import { Avatar } from "@/components/ui/avatar";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Separator } from "@/components/ui/separator";
import type { IdpSummary, UserIdpSummary } from "@/models/idp";
import { useConfigStore } from "@/storages/config";
import { cn } from "@/utils";

export default function Index() {
  const { config } = useConfigStore();
  const { t } = useTranslation();
  const [idps, setIdps] = useState<IdpSummary[]>([]);
  const [bound, setBound] = useState<UserIdpSummary[]>([]);

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
          "p-4",
          "sm:p-6",
          "lg:p-10",
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
        {idps.length > 0 && (
          <div className={cn(["grid", "gap-3"])}>
            <div
              className={cn([
                "overflow-hidden",
                "rounded-lg",
                "border",
                "ring-1",
                "ring-border/50",
                "shadow-sm",
              ])}
            >
              {[...idps]
                .sort((a, b) => {
                  const aBound = bound.some((item) => item.idp_id === a.id);
                  const bBound = bound.some((item) => item.idp_id === b.id);
                  return Number(bBound) - Number(aBound);
                })
                .map((idp) => {
                  const item = bound.find((v) => v.idp_id === idp.id);
                  return (
                    <div
                      key={idp.id}
                      className={cn([
                        "flex",
                        "flex-col",
                        "gap-4",
                        "min-w-0",
                        "border-b",
                        "p-4",
                        "last:border-b-0",
                        "sm:flex-row",
                        "sm:items-center",
                        "transition-colors",
                        "hover:bg-muted/50",
                      ])}
                    >
                      <div
                        className={cn([
                          "flex",
                          "min-w-0",
                          "flex-1",
                          "items-center",
                          "gap-3",
                        ])}
                      >
                        <Avatar
                          square
                          className={cn([
                            "size-11",
                            "shrink-0",
                            "border",
                            "bg-transparent",
                          ])}
                          src={
                            idp.avatar_hash &&
                            `/api/media?hash=${idp.avatar_hash}`
                          }
                          fallback={idp.name?.charAt(0)}
                        />
                        <div className={cn(["min-w-0", "flex-1"])}>
                          <div className={cn(["truncate", "font-semibold"])}>
                            {idp.name}
                          </div>
                          <div
                            className={cn([
                              "mt-0.5",
                              "truncate",
                              "font-mono",
                              "text-xs",
                              "text-muted-foreground",
                            ])}
                          >
                            {item?.auth_key || `#${idp.id}`}
                          </div>
                        </div>
                      </div>
                      <div
                        className={cn([
                          "flex",
                          "shrink-0",
                          "items-center",
                          "justify-end",
                          "gap-2",
                        ])}
                      >
                        {item?.source === "registration" ? (
                          <Badge
                            variant="outline"
                            className={cn([
                              "border-info/20",
                              "bg-info/10",
                              "text-info",
                            ])}
                          >
                            <LockKeyholeIcon />
                            {t("user:idp.source.registration")}
                          </Badge>
                        ) : item ? (
                          <>
                            <Badge variant="outline">
                              {t("user:idp.actions.bound")}
                            </Badge>
                            <Button
                              className={cn(["shrink-0"])}
                              variant="tonal"
                              level="error"
                              icon={<UnplugIcon />}
                              onClick={() => handleUnbind(item.id)}
                            >
                              {t("user:idp.actions.unbind")}
                            </Button>
                          </>
                        ) : (
                          <Button asChild variant="solid" icon={<LinkIcon />}>
                            <a href={idp.portal || `/account/idps/${idp.id}`}>
                              {t("user:idp.actions.bind")}
                            </a>
                          </Button>
                        )}
                      </div>
                    </div>
                  );
                })}
            </div>
          </div>
        )}
      </div>
    </>
  );
}
