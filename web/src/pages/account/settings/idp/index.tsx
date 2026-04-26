import { IdCardIcon, LinkIcon, TrashIcon } from "lucide-react";
import { useCallback, useEffect, useState } from "react";
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
    toast.success("IdP unbound");
    refresh();
  }

  return (
    <>
      <title>{`IdP - ${config?.meta?.title}`}</title>
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
        <h2 className={cn(["flex", "items-center", "gap-2", "text-xl"])}>
          <IdCardIcon />
          IdP
        </h2>
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
                  className={cn(["size-11"])}
                  src={idp.has_avatar && `/api/idps/${idp.id}/avatar`}
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
                    icon={<TrashIcon />}
                    onClick={() => handleUnbind(item.id!)}
                  >
                    Unbind
                  </Button>
                ) : (
                  <Button asChild variant="solid" icon={<LinkIcon />}>
                    <a
                      href={
                        idp.portal || `/account/idp/${idp.id ?? ""}`
                      }
                    >
                      Bind
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
