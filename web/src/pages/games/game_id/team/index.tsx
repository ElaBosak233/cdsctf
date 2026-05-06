import { zodResolver } from "@hookform/resolvers/zod";
import {
  MailIcon,
  MessageCircleIcon,
  SaveIcon,
  TrashIcon,
  TypeIcon,
  UploadCloudIcon,
} from "lucide-react";
import { useEffect, useRef, useState } from "react";
import { useForm } from "react-hook-form";
import { useTranslation } from "react-i18next";
import { toast } from "sonner";
import { z } from "zod";
import { updateTeam } from "@/api/games/game_id/teams/us";
import { deleteTeamAvatar } from "@/api/games/game_id/teams/us/avatar";
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
import { TextField } from "@/components/ui/text-field";
import { useGameStore } from "@/storages/game";
import { useSharedStore } from "@/storages/shared";
import { cn } from "@/utils";
import { uploadFile } from "@/utils/file";

export default function Index() {
  const { currentGame, selfTeam } = useGameStore();
  const { setRefresh } = useSharedStore();
  const { t } = useTranslation();

  const [loading, setLoading] = useState<boolean>(false);
  const disabled = Date.now() / 1000 > Number(currentGame?.ended_at);

  const avatarInput = useRef<HTMLInputElement>(null);
  const [hasAvatar, setHasAvatar] = useState<boolean>(false);

  const formSchema = z.object({
    name: z.string({
      message: t("team:form.name.required"),
    }),
    email: z.string().nullish(),
    description: z.string().nullish(),
    slogan: z.string().nullish(),
  });

  const form = useForm<z.infer<typeof formSchema>>({
    resolver: zodResolver(formSchema),
    defaultValues: selfTeam,
  });

  useEffect(() => {
    form.reset(selfTeam, {
      keepDefaultValues: false,
    });
  }, [selfTeam, form]);

  async function onSubmit(values: z.infer<typeof formSchema>) {
    if (!selfTeam || !currentGame) return;
    setLoading(true);
    try {
      const res = await updateTeam({
        id: selfTeam.id!,
        game_id: currentGame.id!,
        ...values,
      });

      toast.success(
        t("team:actions.update.success", { name: res?.team?.name })
      );
    } finally {
      setRefresh();
      setLoading(false);
    }
  }

  async function handleAvatarUpload(
    event: React.ChangeEvent<HTMLInputElement>
  ) {
    const file = event.target.files?.[0];

    if (!file) return;

    try {
      await uploadFile(
        `/api/games/${currentGame?.id}/teams/us/avatar`,
        [file],
        ({ percent }) => {
          toast.loading(
            t("team:avatar.upload.progress", { percent: percent.toFixed(0) }),
            {
              id: "team-avatar-upload",
            }
          );
        }
      );
      toast.success(t("team:avatar.upload.success"), {
        id: "team-avatar-upload",
      });
      setRefresh();
    } catch {
      toast.error(t("team:avatar.upload.error"));
    }

    event.target.value = "";
  }

  async function handleAvatarDelete() {
    if (!selfTeam || !currentGame) return;
    try {
      await deleteTeamAvatar({
        game_id: currentGame.id!,
        team_id: selfTeam.id!,
      });

      toast.success(
        t("team:avatar.team_delete_success", { name: selfTeam?.name })
      );
    } finally {
      setRefresh();
    }
  }

  return (
    <>
      <title>{`${t("team:_")} - ${currentGame?.title}`}</title>
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
            <div className={cn(["flex", "gap-5", "items-center"])}>
              <div className={cn(["flex", "flex-col", "gap-8", "flex-1"])}>
                <FormField
                  control={form.control}
                  name={"name"}
                  render={({ field }) => (
                    <FormItem>
                      <FormLabel>{t("team:name")}</FormLabel>
                      <FormControl>
                        <Field>
                          <FieldIcon>
                            <TypeIcon />
                          </FieldIcon>
                          <TextField
                            disabled={disabled}
                            placeholder={t("team:name")}
                            {...field}
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
                  name={"email"}
                  render={({ field }) => (
                    <FormItem>
                      <FormLabel>{t("team:email")}</FormLabel>
                      <FormControl>
                        <Field>
                          <FieldIcon>
                            <MailIcon />
                          </FieldIcon>
                          <TextField
                            disabled={disabled}
                            placeholder={t("team:email")}
                            {...field}
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
                  <Label>{t("team:avatar._")}</Label>
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
                    selfTeam?.avatar_hash &&
                    `/api/media?hash=${selfTeam?.avatar_hash}`
                  }
                  fallback={selfTeam?.name?.charAt(0)}
                  onLoadingStatusChange={(status) =>
                    setHasAvatar(status === "loaded")
                  }
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
              name={"slogan"}
              render={({ field }) => (
                <FormItem>
                  <FormLabel>{t("team:slogan._")}</FormLabel>
                  <FormControl>
                    <Field>
                      <FieldIcon>
                        <MessageCircleIcon />
                      </FieldIcon>
                      <TextField
                        disabled={disabled}
                        placeholder={t("team:slogan._")}
                        {...field}
                        value={field.value || ""}
                        onChange={field.onChange}
                      />
                    </Field>
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />
            <div className={cn(["flex-1"])} />
            <Button
              size={"lg"}
              type={"submit"}
              variant={"solid"}
              icon={<SaveIcon />}
              loading={loading}
              disabled={disabled}
            >
              {t("common:actions.save")}
            </Button>
          </form>
        </Form>
      </div>
    </>
  );
}
