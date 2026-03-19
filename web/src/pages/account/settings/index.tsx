import { zodResolver } from "@hookform/resolvers/zod";
import { StatusCodes } from "http-status-codes";
import {
  SaveIcon,
  TrashIcon,
  TypeIcon,
  UploadCloudIcon,
  UserRoundIcon,
} from "lucide-react";
import { useRef, useState } from "react";
import { useForm } from "react-hook-form";
import { useTranslation } from "react-i18next";
import { toast } from "sonner";
import { z } from "zod";
import { updateUserProfile } from "@/api/users/me";
import { deleteUserAvatar } from "@/api/users/me/avatar";
import { Avatar } from "@/components/ui/avatar";
import { Button } from "@/components/ui/button";
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
import { MarkdownEditor } from "@/components/ui/markdown-editor";
import { TextField } from "@/components/ui/text-field";
import { useAuthStore } from "@/storages/auth";
import { useConfigStore } from "@/storages/config";
import { useSharedStore } from "@/storages/shared";
import { cn } from "@/utils";
import { uploadFile } from "@/utils/file";

export default function Index() {
  const { t } = useTranslation();

  const authStore = useAuthStore();
  const sharedStore = useSharedStore();
  const configStore = useConfigStore();
  const [loading, setLoading] = useState<boolean>(false);

  const avatarInput = useRef<HTMLInputElement>(null);
  const [hasAvatar, setHasAvatar] = useState<boolean>(false);

  const formSchema = z.object({
    username: z.string().nullish(),
    name: z.string({
      message: t("user:form.name.messages._"),
    }),
    description: z.string().nullish(),
  });

  const form = useForm<z.infer<typeof formSchema>>({
    resolver: zodResolver(formSchema),
    defaultValues: authStore?.user,
  });

  async function onSubmit(values: z.infer<typeof formSchema>) {
    setLoading(true);
    try {
      const res = await updateUserProfile({
        ...values,
      });
      if (res.code === StatusCodes.OK) {
        authStore?.setUser(res.data);
        toast.success(t("user:settings.profile_update_success"));
      }
    } finally {
      setLoading(false);
    }
  }

  async function handleAvatarUpload(
    event: React.ChangeEvent<HTMLInputElement>
  ) {
    const file = event.target.files?.[0];

    if (!file) return;

    try {
      const res = await uploadFile(
        "/api/users/me/avatar",
        [file],
        ({ percent }) => {
          toast.loading(
            t("user:settings.avatar_upload.progress", {
              percent: percent.toFixed(0),
            }),
            {
              id: "user-avatar-upload",
            }
          );
        }
      );
      if (res.code === StatusCodes.OK) {
        toast.success(t("user:settings.avatar_upload.success"), {
          id: "user-avatar-upload",
        });
      }
    } catch {
      toast.error(t("user:settings.avatar_upload.error"));
    }

    event.target.value = "";
  }

  async function handleAvatarDelete() {
    if (!authStore?.user) return;

    const res = await deleteUserAvatar();
    if (res.code === StatusCodes.OK) {
      toast.success(t("user:settings.avatar_delete_success"));
    }
    sharedStore.setRefresh();
  }

  return (
    <>
      <title>{`${t("user:settings.info")} - ${configStore?.config?.meta?.title}`}</title>
      <div
        className={cn([
          "flex",
          "flex-col",
          "flex-1",
          "p-10",
          "xl:mx-50",
          "lg:mx-30",
        ])}
      >
        <Form {...form}>
          <form
            onSubmit={form.handleSubmit(onSubmit)}
            autoComplete={"off"}
            className={cn(["flex", "flex-col", "flex-1", "gap-8"])}
          >
            <div
              className={cn([
                "flex",
                "flex-wrap-reverse",
                "gap-5",
                "items-center",
                "justify-center",
              ])}
            >
              <div
                className={cn([
                  "flex",
                  "flex-col",
                  "gap-8",
                  "flex-1",
                  "min-w-2xs",
                ])}
              >
                <FormField
                  control={form.control}
                  name={"username"}
                  render={({ field }) => (
                    <FormItem>
                      <FormLabel>{t("user:form.username._")}</FormLabel>
                      <FormControl>
                        <Field>
                          <FieldIcon>
                            <UserRoundIcon />
                          </FieldIcon>
                          <TextField
                            {...field}
                            disabled
                            placeholder={"Username"}
                            value={field.value || ""}
                            onChange={field.onChange}
                          />
                        </Field>
                      </FormControl>
                      <FormMessage />
                    </FormItem>
                  )}
                />
                <FormField
                  control={form.control}
                  name={"name"}
                  render={({ field }) => (
                    <FormItem>
                      <FormLabel>{t("user:form.name._")}</FormLabel>
                      <FormControl>
                        <Field>
                          <FieldIcon>
                            <TypeIcon />
                          </FieldIcon>
                          <TextField
                            {...field}
                            placeholder={"Name"}
                            value={field.value || ""}
                            onChange={field.onChange}
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
                  <Label>{t("user:form.avatar")}</Label>
                </div>
                <Avatar
                  className={cn([
                    "h-30",
                    "w-30",
                    "transition-all",
                    "duration-300",
                    "border",
                  ])}
                  src={
                    authStore?.user?.has_avatar &&
                    `/api/users/${authStore?.user?.id}/avatar`
                  }
                  onLoadingStatusChange={(status) =>
                    setHasAvatar(status === "loaded")
                  }
                  fallback={authStore?.user?.name?.charAt(0)}
                >
                  <Button
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
                        avatarInput?.current?.click();
                      }
                    }}
                  >
                    <input
                      type={"file"}
                      className={"hidden"}
                      ref={avatarInput}
                      accept={".png,.jpg,.jpeg,.webp"}
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
              name={"description"}
              render={({ field }) => (
                <FormItem className={cn(["flex-1", "flex", "flex-col"])}>
                  <FormLabel>{t("user:form.description._")}</FormLabel>
                  <FormControl>
                    <MarkdownEditor
                      {...field}
                      placeholder={"Once upon a time..."}
                      className={cn(["h-full", "min-h-64"])}
                      value={field.value || ""}
                    />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />
            <Button
              size={"lg"}
              type={"submit"}
              level={"primary"}
              variant={"solid"}
              icon={<SaveIcon />}
              loading={loading}
            >
              {t("common:actions.save")}
            </Button>
          </form>
        </Form>
      </div>
    </>
  );
}
