import { zodResolver } from "@hookform/resolvers/zod";
import {
  ClockAlertIcon,
  ClockFadingIcon,
  ClockIcon,
  FileCheck2Icon,
  LockOpenIcon,
  SaveIcon,
  TrashIcon,
  TypeIcon,
  UploadCloudIcon,
  UsersRoundIcon,
} from "lucide-react";
import { useContext, useEffect, useRef, useState } from "react";
import { useForm } from "react-hook-form";
import { useTranslation } from "react-i18next";
import { useParams } from "react-router";
import { toast } from "sonner";
import { z } from "zod";
import { updateGame } from "@/api/admin/games/game_id";
import { deleteGameIcon } from "@/api/admin/games/game_id/icon";
import { deleteGamePoster } from "@/api/admin/games/game_id/poster";
import { Avatar } from "@/components/ui/avatar";
import { Button } from "@/components/ui/button";
import { DateTimePicker } from "@/components/ui/datetime-picker";
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
import { MarkdownEditor } from "@/components/ui/markdown-editor";
import { NumberField } from "@/components/ui/number-field";
import { Select } from "@/components/ui/select";
import { TextField } from "@/components/ui/text-field";
import { useRefresh } from "@/hooks/use-refresh";
import { useSharedStore } from "@/storages/shared";
import { cn } from "@/utils";
import { uploadFile } from "@/utils/file";
import { parseRouteNumericId } from "@/utils/query";
import { Context } from "./context";

export default function Index() {
  const { t } = useTranslation();

  const { game_id } = useParams<{ game_id: string }>();
  const routeGameId = parseRouteNumericId(game_id);
  const { game } = useContext(Context);
  const sharedStore = useSharedStore();

  const resolvedGameId = routeGameId ?? game?.id;

  const [loading, setLoading] = useState<boolean>(false);

  const iconInput = useRef<HTMLInputElement>(null);
  const [hasIcon, setHasIcon] = useState<boolean>(false);
  const { bump: iconBump } = useRefresh();

  const posterInput = useRef<HTMLInputElement>(null);
  const [hasPoster, setHasPoster] = useState<boolean>(false);
  const { bump: posterBump } = useRefresh();

  const formSchema = z.object({
    title: z.string({
      message: t("game:form.title.message"),
    }),
    sketch: z.string().nullable(),
    description: z.string().nullable(),
    public: z.boolean(),
    writeup_required: z.boolean(),
    member_limit_min: z.number({
      message: t("game:form.member_limit_min.message"),
    }),
    member_limit_max: z.number({
      message: t("game:form.member_limit_max.message"),
    }),
    started_at: z.date({
      message: t("game:form.started_at.message"),
    }),
    frozen_at: z.date({
      message: t("game:form.frozen_at.message"),
    }),
    ended_at: z.date({
      message: t("game:form.ended_at.message"),
    }),
  });

  const form = useForm<z.infer<typeof formSchema>>({
    resolver: zodResolver(formSchema),
    defaultValues: {
      ...game,
      started_at: undefined,
      frozen_at: undefined,
      ended_at: undefined,
    },
  });

  useEffect(() => {
    form.reset(
      {
        ...game,
        started_at: new Date(Number(game?.started_at || 0) * 1000),
        frozen_at: new Date(Number(game?.frozen_at || 0) * 1000),
        ended_at: new Date(Number(game?.ended_at || 0) * 1000),
      },
      {
        keepDefaultValues: false,
      }
    );
  }, [game, form]);

  useEffect(() => {
    if (!game) return;

    setHasIcon(game.icon_hash != null);
    setHasPoster(game.poster_hash != null);
  }, [game]);

  function onSubmit(values: z.infer<typeof formSchema>) {
    if (resolvedGameId == null) return;

    setLoading(true);
    updateGame({
      ...values,
      id: resolvedGameId,
      started_at: Math.floor(values.started_at?.getTime() / 1000),
      frozen_at: Math.floor(values.frozen_at?.getTime() / 1000),
      ended_at: Math.floor(values.ended_at?.getTime() / 1000),
    })
      .then((res) => {
        toast.success(
          t("game:actions.update.success", { title: res?.game?.title })
        );
      })
      .finally(() => {
        sharedStore.setRefresh();
        setLoading(false);
      });
  }

  async function handlePosterUpload(
    event: React.ChangeEvent<HTMLInputElement>
  ) {
    const file = event.target.files?.[0];
    if (!file || resolvedGameId == null) return;

    try {
      await uploadFile(`/api/admin/games/${resolvedGameId}/poster`, [file]);
      toast.success(t("game:form.poster_upload.success"));
    } catch (_) {
      toast.error(t("game:form.poster_upload.error"), {
        description: t("common:errors.network"),
      });
      return;
    }

    event.target.value = "";
    posterBump();
  }

  async function handlePosterDelete() {
    if (resolvedGameId == null) return;

    try {
      await deleteGamePoster({
        game_id: resolvedGameId,
      });

      toast.success(t("game:form.poster_delete.success"));
    } finally {
      posterBump();
    }
  }

  async function handleIconUpload(event: React.ChangeEvent<HTMLInputElement>) {
    const file = event.target.files?.[0];
    if (!file || resolvedGameId == null) return;

    try {
      await uploadFile(`/api/admin/games/${resolvedGameId}/icon`, [file]);
      toast.success(t("game:form.icon_upload.success"));
    } catch (_) {
      toast.error(t("game:form.icon_upload.error"), {
        description: t("common:errors.network"),
      });
      return;
    }

    event.target.value = "";
    iconBump();
  }

  async function handleIconDelete() {
    if (resolvedGameId == null) return;

    try {
      await deleteGameIcon({
        game_id: resolvedGameId,
      });

      toast.success(t("game:form.icon_delete.success"));
    } finally {
      iconBump();
    }
  }

  return (
    <Form {...form}>
      <form
        onSubmit={form.handleSubmit(onSubmit)}
        autoComplete={"off"}
        className={cn(["flex", "flex-col", "flex-1", "gap-8"])}
      >
        <div className={cn(["flex", "gap-8", "flex-wrap", "items-center"])}>
          <div
            className={cn(["flex", "flex-col", "flex-1", "gap-8", "w-full"])}
          >
            <FormField
              control={form.control}
              name={"title"}
              render={({ field }) => (
                <FormItem className={cn(["w-full"])}>
                  <FormLabel>{t("game:form.title._")}</FormLabel>
                  <FormControl>
                    <Field>
                      <FieldIcon>
                        <TypeIcon />
                      </FieldIcon>
                      <TextField
                        {...field}
                        placeholder={"My CTF Game"}
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
              name={"sketch"}
              render={({ field }) => (
                <FormItem className={cn(["w-full"])}>
                  <FormLabel>{t("game:form.sketch")}</FormLabel>
                  <FormControl>
                    <Editor
                      {...field}
                      placeholder={"Once upon a time..."}
                      value={field.value || ""}
                      className={cn(["h-32"])}
                    />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />
          </div>
          <div className={cn(["flex", "gap-8"])}>
            <div className={cn(["flex", "flex-col", "gap-3"])}>
              <div
                className={cn([
                  "flex",
                  "gap-3",
                  "items-center",
                  "justify-between",
                ])}
              >
                <Label className="py-1.5">{t("game:form.poster")}</Label>
              </div>
              <div className={cn(["h-36", "aspect-video"])}>
                <Avatar
                  className={cn([
                    "h-full",
                    "w-full",
                    "rounded-lg",
                    "border",
                    "select-none",
                  ])}
                  src={
                    game?.poster_hash
                      ? `/api/media?hash=${game?.poster_hash}`
                      : undefined
                  }
                  onLoadingStatusChange={(status) =>
                    setHasPoster(status === "loaded")
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
                      if (hasPoster) {
                        handlePosterDelete();
                      } else {
                        posterInput?.current?.click();
                      }
                    }}
                  >
                    <input
                      type={"file"}
                      className={"hidden"}
                      ref={posterInput}
                      accept={".png,.jpg,.jpeg,.webp"}
                      onChange={handlePosterUpload}
                    />

                    {hasPoster ? (
                      <TrashIcon className={cn(["shrink-0", "text-error"])} />
                    ) : (
                      <UploadCloudIcon className="shrink-0" />
                    )}
                  </Button>
                </Avatar>
              </div>
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
                <Label className="py-1.5">{t("game:form.icon")}</Label>
              </div>
              <div className={cn(["h-36", "aspect-square"])}>
                <Avatar
                  fit="contain"
                  className={cn([
                    "h-full",
                    "w-full",
                    "rounded-lg",
                    "border",
                    "p-5",
                    "select-none",
                  ])}
                  src={
                    game?.icon_hash
                      ? `/api/media?hash=${game?.icon_hash}`
                      : undefined
                  }
                  onLoadingStatusChange={(status) =>
                    setHasIcon(status === "loaded")
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
                      if (hasIcon) {
                        handleIconDelete();
                      } else {
                        iconInput?.current?.click();
                      }
                    }}
                  >
                    <input
                      type={"file"}
                      className={"hidden"}
                      ref={iconInput}
                      accept={".png,.jpg,.jpeg,.webp"}
                      onChange={handleIconUpload}
                    />

                    {hasIcon ? (
                      <TrashIcon className={cn(["shrink-0", "text-error"])} />
                    ) : (
                      <UploadCloudIcon className="shrink-0" />
                    )}
                  </Button>
                </Avatar>
              </div>
            </div>
          </div>
        </div>
        <div className={cn(["grid", "grid-cols-4", "gap-5"])}>
          <FormField
            control={form.control}
            name={"public"}
            render={({ field }) => (
              <FormItem className={cn(["w-full"])}>
                <FormLabel>{t("game:form.public._")}</FormLabel>
                <FormControl>
                  <Field>
                    <FieldIcon>
                      <LockOpenIcon />
                    </FieldIcon>
                    <Select
                      {...field}
                      options={[
                        {
                          value: String(true),
                          content: t("game:form.public.true"),
                        },
                        {
                          value: String(false),
                          content: t("game:form.public.false"),
                        },
                      ]}
                      onValueChange={(value) => {
                        field.onChange(value === "true");
                      }}
                      value={String(field.value)}
                    />
                  </Field>
                </FormControl>
                <FormMessage />
              </FormItem>
            )}
          />
          <FormField
            control={form.control}
            name={"writeup_required"}
            render={({ field }) => (
              <FormItem className={cn(["w-full"])}>
                <FormLabel>{t("game:form.writeup_required._")}</FormLabel>
                <FormControl>
                  <Field>
                    <FieldIcon>
                      <FileCheck2Icon />
                    </FieldIcon>
                    <Select
                      {...field}
                      options={[
                        {
                          value: String(true),
                          content: t("game:form.writeup_required.true"),
                        },
                        {
                          value: String(false),
                          content: t("game:form.writeup_required.false"),
                        },
                      ]}
                      onValueChange={(value) =>
                        field.onChange(value === "true")
                      }
                      value={String(field.value)}
                    />
                  </Field>
                </FormControl>
                <FormMessage />
              </FormItem>
            )}
          />
          <FormField
            control={form.control}
            name={"member_limit_min"}
            render={({ field }) => (
              <FormItem className={cn(["w-full"])}>
                <FormLabel>{t("game:form.member_limit_min._")}</FormLabel>
                <FormControl>
                  <Field>
                    <FieldIcon>
                      <UsersRoundIcon />
                    </FieldIcon>
                    <NumberField
                      placeholder={"3"}
                      value={field.value}
                      onValueChange={(value) => field.onChange(value)}
                    />
                  </Field>
                </FormControl>
                <FormMessage />
              </FormItem>
            )}
          />
          <FormField
            control={form.control}
            name={"member_limit_max"}
            render={({ field }) => (
              <FormItem className={cn(["w-full"])}>
                <FormLabel>{t("game:form.member_limit_max._")}</FormLabel>
                <FormControl>
                  <Field>
                    <FieldIcon>
                      <UsersRoundIcon />
                    </FieldIcon>
                    <NumberField
                      placeholder={"3"}
                      value={field.value}
                      onValueChange={(value) => field.onChange(value)}
                    />
                  </Field>
                </FormControl>
                <FormMessage />
              </FormItem>
            )}
          />
        </div>
        <div className={cn(["grid", "grid-cols-3", "gap-5"])}>
          <FormField
            control={form.control}
            name={"started_at"}
            render={({ field }) => (
              <FormItem>
                <FormLabel>{t("game:form.started_at._")}</FormLabel>
                <FormControl>
                  <Field>
                    <FieldIcon>
                      <ClockIcon />
                    </FieldIcon>
                    <DateTimePicker {...field} />
                  </Field>
                </FormControl>
                <FormMessage />
              </FormItem>
            )}
          />
          <FormField
            control={form.control}
            name={"frozen_at"}
            render={({ field }) => (
              <FormItem>
                <FormLabel>{t("game:form.frozen_at._")}</FormLabel>
                <FormControl>
                  <Field>
                    <FieldIcon>
                      <ClockFadingIcon />
                    </FieldIcon>
                    <DateTimePicker {...field} />
                  </Field>
                </FormControl>
                <FormMessage />
              </FormItem>
            )}
          />
          <FormField
            control={form.control}
            name={"ended_at"}
            render={({ field }) => (
              <FormItem>
                <FormLabel>{t("game:form.ended_at._")}</FormLabel>
                <FormControl>
                  <Field>
                    <FieldIcon>
                      <ClockAlertIcon />
                    </FieldIcon>
                    <DateTimePicker {...field} />
                  </Field>
                </FormControl>
                <FormMessage />
              </FormItem>
            )}
          />
        </div>
        <FormField
          control={form.control}
          name={"description"}
          render={({ field }) => (
            <FormItem className={cn(["flex-1", "flex", "flex-col"])}>
              <FormLabel>{t("game:form.description")}</FormLabel>
              <FormControl>
                <MarkdownEditor
                  {...field}
                  placeholder={"Once upon a time..."}
                  value={field.value || ""}
                  className={cn(["h-full", "min-h-128"])}
                />
              </FormControl>
              <FormMessage />
            </FormItem>
          )}
        />
        <Button
          variant={"solid"}
          level={"primary"}
          type={"submit"}
          size={"lg"}
          className={cn(["w-full"])}
          icon={<SaveIcon />}
          loading={loading}
        >
          {t("common:actions.save")}
        </Button>
      </form>
    </Form>
  );
}
