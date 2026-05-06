import { keepPreviousData, useQuery } from "@tanstack/react-query";
import { EditIcon, IdCardIcon, PlusCircleIcon, TrashIcon } from "lucide-react";
import { useOptimistic, useState, useTransition } from "react";
import { Trans, useTranslation } from "react-i18next";
import { Link } from "react-router";
import { toast } from "sonner";
import { deleteAdminIdp, getAdminIdps, updateAdminIdp } from "@/api/admin/idps";
import { Avatar } from "@/components/ui/avatar";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";
import { Dialog, DialogContent } from "@/components/ui/dialog";
import { LoadingOverlay } from "@/components/ui/loading-overlay";
import { ScrollArea } from "@/components/ui/scroll-area";
import { Switch } from "@/components/ui/switch";
import type { Idp } from "@/models/idp";
import { useConfigStore } from "@/storages/config";
import { useSharedStore } from "@/storages/shared";
import { cn } from "@/utils";
import { CreateDialog } from "./_blocks/create-dialog";

function IdpCard({ idp }: { idp: Idp }) {
  const { t } = useTranslation();
  const sharedStore = useSharedStore();

  const [enabled, setEnabled] = useState(idp.enabled);
  const [_, startTransition] = useTransition();
  const [optimisticEnabled, setOptimisticEnabled] = useOptimistic(enabled);

  const [deleteDialogOpen, setDeleteDialogOpen] = useState(false);

  async function handleToggleEnabled() {
    const newValue = !optimisticEnabled;
    setOptimisticEnabled(newValue);
    startTransition(async () => {
      await updateAdminIdp(idp.id!, { ...idp, enabled: newValue });
      setEnabled(newValue);
      toast.success(
        newValue
          ? t("admin:idp.actions.enable.success", { name: idp.name })
          : t("admin:idp.actions.disable.success", { name: idp.name })
      );
    });
  }

  async function handleDelete() {
    await deleteAdminIdp(idp.id!);
    toast.success(t("admin:idp.actions.delete.success", { name: idp.name }));
    setDeleteDialogOpen(false);
    sharedStore.setRefresh();
  }

  return (
    <Card className={cn(["p-4", "flex", "items-center", "gap-4"])}>
      <Avatar
        square
        className={cn(["size-11", "border", "bg-transparent", "border-none"])}
        src={idp.has_avatar && `/api/idps/${idp.id}/avatar`}
        fallback={idp.name?.charAt(0)}
      />
      <div className={cn(["flex-1", "min-w-0"])}>
        <div className={cn(["font-medium", "truncate"])}>{idp.name}</div>
        <div className={cn(["text-sm", "text-secondary-foreground"])}>
          #{idp.id}
        </div>
      </div>
      <Badge variant={optimisticEnabled ? "solid" : "tonal"}>
        {optimisticEnabled
          ? t("admin:idp.enabled.true")
          : t("admin:idp.enabled.false")}
      </Badge>
      <Switch checked={optimisticEnabled} onCheckedChange={handleToggleEnabled} />
      <Button variant="ghost" size="sm" square icon={<EditIcon />} asChild>
        <Link to={`/admin/idps/${idp.id}`} />
      </Button>
      <Button
        variant="ghost"
        level="error"
        size="sm"
        square
        icon={<TrashIcon />}
        onClick={() => setDeleteDialogOpen(true)}
      />
      <Dialog open={deleteDialogOpen} onOpenChange={setDeleteDialogOpen}>
        <DialogContent>
          <Card
            className={cn(["flex", "flex-col", "p-5", "min-h-32", "w-lg", "gap-5"])}
          >
            <div className={cn(["flex", "gap-2", "items-center", "text-sm"])}>
              <TrashIcon className={cn(["size-4", "text-error"])} />
              {t("admin:idp.actions.delete._")}
            </div>
            <p className={cn(["text-sm"])}>
              <Trans
                i18nKey={"admin:idp.actions.delete.message"}
                values={{ name: idp.name }}
                components={{
                  muted: <span className={cn(["text-muted-foreground"])} />,
                }}
              />
            </p>
            <div className={cn(["flex", "justify-end"])}>
              <Button
                level={"error"}
                variant={"solid"}
                size={"sm"}
                onClick={handleDelete}
              >
                {t("common:actions.confirm")}
              </Button>
            </div>
          </Card>
        </DialogContent>
      </Dialog>
    </Card>
  );
}

export default function Index() {
  const { t } = useTranslation();
  const { config } = useConfigStore();
  const sharedStore = useSharedStore();
  const [createDialogOpen, setCreateDialogOpen] = useState(false);

  const { data, isLoading } = useQuery({
    queryKey: ["admin", "idps", sharedStore.refresh],
    queryFn: getAdminIdps,
    select: (response) => response.idps ?? [],
    placeholderData: keepPreviousData,
  });

  return (
    <>
      <title>{`${t("admin:idp._")} - ${config?.meta?.title}`}</title>
      <Dialog open={createDialogOpen} onOpenChange={setCreateDialogOpen}>
        <DialogContent>
          <CreateDialog onClose={() => setCreateDialogOpen(false)} />
        </DialogContent>
      </Dialog>
      <div
        className={cn([
          "flex",
          "flex-col",
          "xl:flex-row",
          "xl:min-h-(--app-content-height)",
          "flex-1",
          "min-h-0",
          "xl:pl-64",
        ])}
      >
        <aside
          className={cn([
            "hidden",
            "xl:flex",
            "xl:fixed",
            "xl:left-16",
            "xl:top-16",
            "xl:z-10",
            "xl:h-(--app-content-height)",
            "xl:w-64",
            "xl:flex-col",
            "xl:border-r",
            "xl:bg-card/30",
            "xl:backdrop-blur-sm",
            "py-5",
            "px-4",
            "gap-4",
            "overflow-y-auto",
          ])}
        >
          <div
            className={cn([
              "flex",
              "items-center",
              "gap-2",
              "px-2",
              "text-sm",
              "font-medium",
              "text-muted-foreground",
              "shrink-0",
            ])}
          >
            <IdCardIcon className="size-4" />
            {t("admin:idp._")}
          </div>
          <Button
            icon={<PlusCircleIcon className="size-4" />}
            variant="solid"
            className={cn(["justify-start", "w-full", "shrink-0"])}
            onClick={() => setCreateDialogOpen(true)}
          >
            {t("common:actions.add")}
          </Button>
        </aside>
        <Card
          className={cn([
            "h-(--app-content-height)",
            "flex-1",
            "min-h-0",
            "min-w-0",
            "border-y-0",
            "rounded-none",
            "flex",
            "flex-col",
            "xl:rounded-l-none",
          ])}
        >
          <div
            className={cn([
              "xl:hidden",
              "flex",
              "items-center",
              "justify-between",
              "gap-3",
              "p-3",
              "border-b",
              "bg-card/30",
              "shrink-0",
            ])}
          >
            <div
              className={cn(["flex", "items-center", "gap-2", "font-medium"])}
            >
              <IdCardIcon className="size-4" />
              {t("admin:idp._")}
            </div>
            <Button
              icon={<PlusCircleIcon />}
              variant="solid"
              size="sm"
              onClick={() => setCreateDialogOpen(true)}
            >
              {t("common:actions.add")}
            </Button>
          </div>
          <ScrollArea className={cn(["flex-1", "min-h-0"])}>
            <LoadingOverlay loading={isLoading} />
            <div className={cn(["grid", "gap-3", "p-6"])}>
              {data?.map((idp) => (
                <IdpCard key={idp.id} idp={idp} />
              ))}
              {!isLoading && data?.length === 0 && (
                <div
                  className={cn([
                    "h-40",
                    "flex",
                    "items-center",
                    "justify-center",
                    "text-muted-foreground",
                  ])}
                >
                  {t("admin:idp.empty")}
                </div>
              )}
            </div>
          </ScrollArea>
        </Card>
      </div>
    </>
  );
}
