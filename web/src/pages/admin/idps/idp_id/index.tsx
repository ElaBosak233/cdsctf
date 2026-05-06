import { zodResolver } from "@hookform/resolvers/zod";
import { keepPreviousData, useQuery } from "@tanstack/react-query";
import {
  IdCardIcon,
  LayoutTemplateIcon,
  LinkIcon,
  SaveIcon,
  TrashIcon,
  TypeIcon,
  UploadCloudIcon,
} from "lucide-react";
import { useEffect, useRef, useState } from "react";
import { useForm } from "react-hook-form";
import { useTranslation } from "react-i18next";
import { Link, useParams } from "react-router";
import { toast } from "sonner";
import { z } from "zod";
import {
  type DiagnosticMarker,
  deleteAdminIdp,
  deleteAdminIdpAvatar,
  getAdminIdp,
  lintIdpScript,
  updateAdminIdp,
} from "@/api/admin/idps";
import { Avatar } from "@/components/ui/avatar";
import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";
import { Editor } from "@/components/ui/editor";
import { Field, FieldIcon } from "@/components/ui/field";
import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "@/components/ui/form";
import { Label } from "@/components/ui/label";
import { LoadingOverlay } from "@/components/ui/loading-overlay";
import { Select } from "@/components/ui/select";
import { TextField } from "@/components/ui/text-field";
import { useDebounce } from "@/hooks/use-debounce";
import { useConfigStore } from "@/storages/config";
import { useSharedStore } from "@/storages/shared";
import { cn } from "@/utils";
import { uploadFile } from "@/utils/file";
import { parseRouteNumericId } from "@/utils/query";

import casScript from "./_blocks/examples/cas.cdsx?raw";
import defaultScript from "./_blocks/examples/default.cdsx?raw";
import githubScript from "./_blocks/examples/github.cdsx?raw";

const scriptTemplates = {
  default: defaultScript,
  github: githubScript,
  cas: casScript,
};

const schema = z.object({
  name: z.string().min(1),
  portal: z.string().optional().nullable(),
  script: z.string().min(1),
});

type IdpForm = z.infer<typeof schema>;

export default function Index() {
  const { t } = useTranslation();
  const { config } = useConfigStore();
  const sharedStore = useSharedStore();
  const { idp_id } = useParams<{ idp_id: string }>();
  const idpId = parseRouteNumericId(idp_id);

  const [saving, setSaving] = useState(false);
  const [deleting, setDeleting] = useState(false);
  const [hasAvatar, setHasAvatar] = useState(false);
  const [lint, setLint] = useState<Array<DiagnosticMarker>>();
  const avatarInput = useRef<HTMLInputElement>(null);
  const { data: idp, isLoading } = useQuery({
    queryKey: ["admin", "idp", idpId, sharedStore.refresh],
    queryFn: async () => {
      const res = await getAdminIdp(idpId!);
      return res.idp;
    },
    enabled: idpId != null,
    placeholderData: keepPreviousData,
  });

  const form = useForm<IdpForm>({
    resolver: zodResolver(schema),
    defaultValues: {
      name: "",
      portal: "",
      script: defaultScript,
    },
  });

  useEffect(() => {
    form.reset(
      {
        name: idp?.name ?? "",
        portal: idp?.portal ?? "",
        script: idp?.script || defaultScript,
      },
      { keepDefaultValues: false }
    );
    setHasAvatar(false);
  }, [idp, form]);

  const debouncedScript = useDebounce(form.watch("script"), 500);

  useEffect(() => {
    if (debouncedScript) {
      lintIdpScript(debouncedScript).then((res) => {
        setLint(res.markers);
      });
    }
  }, [debouncedScript]);

  async function submit(values: IdpForm) {
    if (idpId == null) return;
    setSaving(true);
    try {
      const res = await updateAdminIdp(idpId, {
        ...values,
        enabled: idp?.enabled ?? true,
        portal: values.portal || null,
      });
      toast.success(
        t("admin:idp.actions.update.success", { name: res.idp?.name })
      );
      sharedStore.setRefresh();
    } finally {
      setSaving(false);
    }
  }

  async function handleAvatarUpload(
    event: React.ChangeEvent<HTMLInputElement>
  ) {
    const file = event.target.files?.[0];
    if (!file || idpId == null) return;
    try {
      await uploadFile(
        `/api/admin/idps/${idpId}/avatar`,
        [file],
        ({ percent }) => {
          toast.loading(
            t("admin:idp.avatar_upload.progress", {
              percent: percent.toFixed(0),
            }),
            { id: "idp-avatar-upload" }
          );
        }
      );
      toast.success(t("admin:idp.avatar_upload.success"), {
        id: "idp-avatar-upload",
      });
      sharedStore.setRefresh();
    } catch {
      toast.error(t("admin:idp.avatar_upload.error"), {
        id: "idp-avatar-upload",
      });
    }
    event.target.value = "";
  }

  async function handleAvatarDelete() {
    if (idpId == null) return;
    await deleteAdminIdpAvatar(idpId);
    toast.success(t("admin:idp.avatar_delete.success"));
    setHasAvatar(false);
    sharedStore.setRefresh();
  }

  return (
    <>
      <title>{`${idp?.name ?? t("admin:idp._")} - ${config?.meta?.title}`}</title>
      <Card
        className={cn([
          "h-(--app-content-height)",
          "flex-1",
          "min-h-0",
          "min-w-0",
          "p-10",
          "border-y-0",
          "rounded-none",
          "flex",
          "flex-col",
          "xl:rounded-l-none",
        ])}
      >
        <LoadingOverlay loading={isLoading} />
        <Form {...form}>
          <form
            onSubmit={form.handleSubmit(submit)}
            autoComplete="off"
            className={cn(["flex", "flex-col", "flex-1", "gap-8"])}
          >
            <div className={cn(["flex", "gap-8", "flex-wrap-reverse"])}>
              <div className={cn(["flex", "flex-col", "gap-8", "flex-1"])}>
                <FormField
                  control={form.control}
                  name="name"
                  render={({ field }) => (
                    <FormItem>
                      <FormLabel>{t("admin:idp.form.name._")}</FormLabel>
                      <FormControl>
                        <Field>
                          <FieldIcon>
                            <TypeIcon />
                          </FieldIcon>
                          <TextField
                            {...field}
                            placeholder={t("admin:idp.form.name.placeholder")}
                            value={field.value || ""}
                          />
                        </Field>
                      </FormControl>
                      <FormMessage />
                    </FormItem>
                  )}
                />
                <FormField
                  control={form.control}
                  name="portal"
                  render={({ field }) => (
                    <FormItem>
                      <FormLabel>{t("admin:idp.form.portal._")}</FormLabel>
                      <FormControl>
                        <Field>
                          <FieldIcon>
                            <LinkIcon />
                          </FieldIcon>
                          <TextField
                            {...field}
                            placeholder={t("admin:idp.form.portal.placeholder")}
                            value={field.value || ""}
                          />
                        </Field>
                      </FormControl>
                      <FormMessage />
                    </FormItem>
                  )}
                />
              </div>
              <div className={cn(["flex", "flex-col", "gap-3"])}>
                <div
                  className={cn([
                    "flex",
                    "gap-3",
                    "items-center",
                    "justify-between",
                  ])}
                >
                  <Label className="py-1.5">{t("admin:idp.form.avatar")}</Label>
                </div>
                <Avatar
                  square
                  className={cn([
                    "h-30",
                    "w-30",
                    "transition-all",
                    "duration-300",
                    "border",
                  ])}
                  src={
                    idp?.avatar_hash
                      ? `/api/media?hash=${idp?.avatar_hash}`
                      : undefined
                  }
                  onLoadingStatusChange={(status) =>
                    setHasAvatar(status === "loaded")
                  }
                  fallback={(idp?.name || form.watch("name") || "I").charAt(0)}
                >
                  <Button
                    type="button"
                    className={cn([
                      "absolute",
                      "top-0",
                      "left-0",
                      "w-full",
                      "h-full",
                      "opacity-0",
                      "backdrop-blur-xs",
                      "transition-all",
                      "hover:opacity-100",
                    ])}
                    onClick={() => {
                      if (hasAvatar) {
                        handleAvatarDelete();
                      } else {
                        avatarInput.current?.click();
                      }
                    }}
                  >
                    <input
                      type="file"
                      className="hidden"
                      ref={avatarInput}
                      accept=".png,.jpg,.jpeg,.webp"
                      onChange={handleAvatarUpload}
                    />
                    {hasAvatar ? (
                      <TrashIcon className={cn(["shrink-0", "text-error"])} />
                    ) : (
                      <UploadCloudIcon className="shrink-0" />
                    )}
                  </Button>
                </Avatar>
              </div>
            </div>

            <FormField
              control={form.control}
              name="script"
              render={({ field }) => (
                <FormItem
                  className={cn(["flex", "flex-1", "flex-col", "gap-3"])}
                >
                  <div className={cn(["flex", "flex-col", "gap-3"])}>
                    <FormLabel>{t("admin:idp.form.script._")}</FormLabel>
                    <Field size="sm" className={cn(["w-full"])}>
                      <FieldIcon>
                        <LayoutTemplateIcon />
                      </FieldIcon>
                      <Select
                        placeholder={t("admin:idp.form.script.templates._")}
                        options={[
                          {
                            value: "default",
                            content: t(
                              "admin:idp.form.script.templates.default"
                            ),
                          },
                          {
                            value: "github",
                            content: t("admin:idp.form.script.templates.oauth"),
                          },
                          {
                            value: "cas",
                            content: t("admin:idp.form.script.templates.cas"),
                          },
                        ]}
                        onValueChange={(
                          value: keyof typeof scriptTemplates
                        ) => {
                          form.setValue("script", scriptTemplates[value]);
                        }}
                      />
                    </Field>
                  </div>
                  <FormControl>
                    <Editor
                      lang="rune"
                      showLineNumbers
                      className={cn(["min-h-96", "flex-1"])}
                      value={field.value}
                      onChange={field.onChange}
                      diagnostics={lint}
                    />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />

            <div className={cn(["flex"])}>
              <Button
                type="submit"
                variant="solid"
                level="primary"
                size="lg"
                className={cn(["flex-1"])}
                icon={<SaveIcon />}
                loading={saving}
              >
                {t("common:actions.save")}
              </Button>
            </div>
          </form>
        </Form>
      </Card>
    </>
  );
}
