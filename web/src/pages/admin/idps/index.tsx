import { keepPreviousData, useQuery } from "@tanstack/react-query";
import { IdCardIcon, PlusCircleIcon, UserRoundPlusIcon } from "lucide-react";
import { useEffect, useState } from "react";
import { useTranslation } from "react-i18next";
import { toast } from "sonner";
import { getConfigs, updateConfig } from "@/api/admin/configs";
import { getAdminIdps } from "@/api/admin/idps";
import { Button, buttonVariants } from "@/components/ui/button";
import { Card } from "@/components/ui/card";
import { Dialog, DialogContent } from "@/components/ui/dialog";
import { LoadingOverlay } from "@/components/ui/loading-overlay";
import { ScrollArea } from "@/components/ui/scroll-area";
import { ScrollableNav } from "@/components/ui/scrollable-nav";
import { Switch } from "@/components/ui/switch";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import type { AdminConfig } from "@/models/config";
import type { IdpView } from "@/models/idp";
import { useConfigStore } from "@/storages/config";
import { useSharedStore } from "@/storages/shared";
import { cn } from "@/utils";
import {
  flexRender,
  type SortingState,
  useDataTable,
} from "@/utils/data-table";
import { RowProvider, useColumns } from "./_blocks/columns";
import { CreateDialog } from "./_blocks/create-dialog";

export default function Index() {
  const { t } = useTranslation();
  const { config, setConfig } = useConfigStore();
  const sharedStore = useSharedStore();
  const [createDialogOpen, setCreateDialogOpen] = useState(false);
  const [adminConfig, setAdminConfig] = useState<AdminConfig>();
  const [configSaving, setConfigSaving] = useState(false);
  const [sorting, setSorting] = useState<SortingState>([
    { id: "updated_at", desc: true },
  ]);

  useEffect(() => {
    getConfigs().then((response) => setAdminConfig(response.config));
  }, []);

  async function setLocalRegistrationEnabled(enabled: boolean) {
    if (!adminConfig) return;
    setConfigSaving(true);
    try {
      const response = await updateConfig({
        ...adminConfig,
        auth: {
          ...adminConfig.auth,
          local_registration_enabled: enabled,
        },
      });
      setAdminConfig(response.config);
      setConfig({
        ...config,
        auth: response.config.auth,
      });
      toast.success(t("admin:idp.local_registration.updated"));
    } catch {
      toast.error(t("common:errors.default"));
    } finally {
      setConfigSaving(false);
    }
  }

  const renderLocalRegistrationControl = (compact = false) => (
    <div
      className={cn(
        buttonVariants({ variant: "ghost", size: compact ? "sm" : "md" }),
        "justify-start",
        "text-muted-foreground",
        compact ? "shrink-0" : "w-full"
      )}
    >
      <UserRoundPlusIcon className="size-4" />
      <span className="flex-1 whitespace-nowrap">
        {t("admin:idp.local_registration._")}
      </span>
      <Switch
        checked={adminConfig?.auth?.local_registration_enabled ?? false}
        disabled={!adminConfig || configSaving}
        onCheckedChange={setLocalRegistrationEnabled}
        aria-label={t("admin:idp.local_registration._")}
      />
    </div>
  );

  const { data, isLoading } = useQuery({
    queryKey: ["admin", "idps", sharedStore.refresh],
    queryFn: getAdminIdps,
    select: (response) => response.idps ?? [],
    placeholderData: keepPreviousData,
  });

  const columns = useColumns();
  const table = useDataTable<IdpView>({
    data: data ?? [],
    columns,
    onSortingChange: setSorting,
    state: { sorting },
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
          "flex-1",
          "min-h-0",
          "min-w-0",
          "flex-col",
          "xl:flex-row",
          "xl:min-h-(--app-content-height)",
          "xl:pl-64",
        ])}
      >
        <ScrollableNav className="xl:hidden">
          <Button
            icon={<PlusCircleIcon className="size-4" />}
            variant="solid"
            size="sm"
            className="shrink-0"
            onClick={() => setCreateDialogOpen(true)}
          >
            {t("common:actions.add")}
          </Button>
          {renderLocalRegistrationControl(true)}
        </ScrollableNav>
        <aside
          className={cn([
            "hidden",
            "xl:fixed",
            "xl:left-16",
            "xl:top-16",
            "xl:z-10",
            "xl:flex",
            "xl:h-(--app-content-height)",
            "xl:w-64",
            "xl:flex-col",
            "xl:gap-4",
            "xl:border-r",
            "xl:bg-card/30",
            "xl:px-4",
            "xl:py-5",
            "xl:backdrop-blur-sm",
            "xl:overflow-y-auto",
          ])}
        >
          <div
            className={cn([
              "flex",
              "shrink-0",
              "items-center",
              "gap-2",
              "px-2",
              "text-sm",
              "font-medium",
              "text-muted-foreground",
            ])}
          >
            <IdCardIcon className="size-4" />
            {t("admin:idp._")}
          </div>
          <Button
            icon={<PlusCircleIcon className="size-4" />}
            variant="solid"
            className={cn(["w-full", "shrink-0", "justify-start"])}
            onClick={() => setCreateDialogOpen(true)}
          >
            {t("common:actions.add")}
          </Button>
          <div className={cn(["mt-auto", "border-t", "pt-4"])}>
            {renderLocalRegistrationControl()}
          </div>
        </aside>
        <Card
          className={cn([
            "flex",
            "flex-1",
            "min-h-0",
            "min-w-0",
            "flex-col",
            "rounded-none",
            "border-y-0",
            "xl:h-(--app-content-height)",
            "xl:rounded-l-none",
          ])}
        >
          <div
            className={cn([
              "flex",
              "h-full",
              "min-h-0",
              "flex-col",
              "gap-4",
              "overflow-hidden",
              "px-4",
              "py-4",
              "sm:px-6",
              "sm:py-6",
              "lg:px-8",
              "lg:py-8",
            ])}
          >
            <ScrollArea
              className={cn([
                "w-full",
                "max-w-full",
                "flex-1",
                "min-h-0",
                "overflow-hidden",
                "rounded-lg",
                "border",
                "ring-1",
                "ring-border/50",
                "shadow-sm",
              ])}
            >
              <LoadingOverlay loading={isLoading} />
              <Table
                className={cn([
                  "w-full",
                  "min-w-192",
                  "table-fixed",
                  "text-foreground",
                ])}
              >
                <TableHeader
                  className={cn([
                    "sticky",
                    "top-0",
                    "z-2",
                    "border-b",
                    "bg-muted/80",
                    "backdrop-blur-sm",
                  ])}
                >
                  {table.getHeaderGroups().map((headerGroup) => (
                    <TableRow key={headerGroup.id}>
                      {headerGroup.headers.map((header) => (
                        <TableHead
                          key={header.id}
                          className={cn([
                            "bg-muted/95",
                            header.column.id === "status" && ["w-64"],
                            header.column.id === "updated_at" && ["w-48"],
                            header.column.id === "actions" && [
                              "sticky",
                              "right-0",
                              "z-3",
                              "w-24",
                            ],
                          ])}
                        >
                          {!header.isPlaceholder &&
                            flexRender(
                              header.column.columnDef.header,
                              header.getContext()
                            )}
                        </TableHead>
                      ))}
                    </TableRow>
                  ))}
                </TableHeader>
                <TableBody>
                  {table.getRowModel().rows.length ? (
                    table.getRowModel().rows.map((row) => (
                      <RowProvider key={row.original.id} idp={row.original}>
                        <TableRow
                          data-state={
                            row.getIsSelected() ? "selected" : undefined
                          }
                          className={cn([
                            "group",
                            "transition-colors",
                            "hover:bg-transparent",
                          ])}
                        >
                          {row.getVisibleCells().map((cell) => (
                            <TableCell
                              key={cell.id}
                              className={cn([
                                "py-3",
                                "transition-colors",
                                "group-hover:bg-muted/50",
                                cell.column.id === "actions" && [
                                  "sticky",
                                  "right-0",
                                  "z-1",
                                  "bg-card",
                                ],
                              ])}
                            >
                              {flexRender(
                                cell.column.columnDef.cell,
                                cell.getContext()
                              )}
                            </TableCell>
                          ))}
                        </TableRow>
                      </RowProvider>
                    ))
                  ) : !isLoading ? (
                    <TableRow>
                      <TableCell
                        colSpan={columns.length}
                        className={cn([
                          "h-40",
                          "text-center",
                          "text-muted-foreground",
                        ])}
                      >
                        {t("admin:idp.empty")}
                      </TableCell>
                    </TableRow>
                  ) : null}
                </TableBody>
              </Table>
            </ScrollArea>
            <p className={cn(["shrink-0", "text-sm", "text-muted-foreground"])}>
              {t("admin:idp.result_count", { count: data?.length ?? 0 })}
            </p>
          </div>
        </Card>
      </div>
    </>
  );
}
