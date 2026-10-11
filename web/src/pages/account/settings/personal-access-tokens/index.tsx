import { zodResolver } from "@hookform/resolvers/zod";
import { useQuery, useQueryClient } from "@tanstack/react-query";
import {
  CalendarIcon,
  CheckIcon,
  ClipboardIcon,
  KeyRoundIcon,
  PlusIcon,
  ShieldCheckIcon,
  Trash2Icon,
} from "lucide-react";
import { Fragment, useMemo, useRef, useState } from "react";
import { useForm } from "react-hook-form";
import { useTranslation } from "react-i18next";
import { toast } from "sonner";
import { z } from "zod";
import {
  createPersonalAccessToken,
  getPersonalAccessTokens,
  revokePersonalAccessToken,
} from "@/api/users/me/personal-access-tokens";
import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";
import { DateTimePicker } from "@/components/ui/datetime-picker";
import {
  Dialog,
  DialogBody,
  DialogContent,
  DialogFooter,
  DialogHeader,
} from "@/components/ui/dialog";
import { Field, FieldIcon } from "@/components/ui/field";
import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "@/components/ui/form";
import {
  Item,
  ItemActions,
  ItemContent,
  ItemDescription,
  ItemGroup,
  ItemSeparator,
  ItemTitle,
} from "@/components/ui/item";
import { Separator } from "@/components/ui/separator";
import { TextField } from "@/components/ui/text-field";
import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from "@/components/ui/tooltip";
import { Captcha, type CaptchaRef } from "@/components/widgets/captcha";
import { useConfigStore } from "@/storages/config";
import { cn } from "@/utils";
import { notifyApiError } from "@/utils/query";

function defaultExpiry() {
  const value = new Date();
  value.setDate(value.getDate() + 90);
  value.setSeconds(0, 0);
  return value;
}

function formatDate(value: string) {
  return new Date(value).toLocaleString();
}

export default function Index() {
  const { t } = useTranslation();
  const configStore = useConfigStore();
  const queryClient = useQueryClient();
  const [loading, setLoading] = useState(false);
  const [revoking, setRevoking] = useState<number | null>(null);
  const [createdToken, setCreatedToken] = useState<string | null>(null);
  const [createDialogOpen, setCreateDialogOpen] = useState(false);
  const captchaRef = useRef<CaptchaRef>(null);

  const formSchema = z.object({
    name: z
      .string()
      .min(1, t("user:personal_access_tokens.errors.name_required")),
    expiresAt: z.date(),
    captcha: z
      .object({
        id: z.string(),
        content: z.string(),
      })
      .nullish(),
  });

  const form = useForm<z.infer<typeof formSchema>>({
    resolver: zodResolver(formSchema),
    defaultValues: {
      name: "",
      expiresAt: defaultExpiry(),
      captcha: null,
    },
  });

  const tokensQuery = useQuery({
    queryKey: ["account", "personal-access-tokens"],
    queryFn: getPersonalAccessTokens,
  });

  const tokens = tokensQuery.data?.personal_access_tokens ?? [];
  const activeTokens = useMemo(
    () =>
      tokens.filter(
        (token) => !token.revoked_at && new Date(token.expires_at) > new Date()
      ),
    [tokens]
  );
  const inactiveTokens = useMemo(
    () =>
      tokens.filter(
        (token) =>
          Boolean(token.revoked_at) || new Date(token.expires_at) <= new Date()
      ),
    [tokens]
  );

  async function handleCreate(values: z.infer<typeof formSchema>) {
    setLoading(true);
    try {
      const response = await createPersonalAccessToken({
        name: values.name.trim(),
        expires_at: values.expiresAt.toISOString(),
        captcha: values.captcha,
      });
      setCreatedToken(response.token);
      form.reset({ name: "", expiresAt: defaultExpiry(), captcha: null });
      setCreateDialogOpen(false);
      await queryClient.invalidateQueries({
        queryKey: ["account", "personal-access-tokens"],
      });
      toast.success(t("user:personal_access_tokens.create.success"));
    } catch (error) {
      await notifyApiError(error, {
        title: t("user:personal_access_tokens.create.error"),
      });
      captchaRef.current?.refresh();
    } finally {
      setLoading(false);
    }
  }

  async function handleRevoke(id: number) {
    setRevoking(id);
    try {
      await revokePersonalAccessToken(id);
      await queryClient.invalidateQueries({
        queryKey: ["account", "personal-access-tokens"],
      });
      toast.success(t("user:personal_access_tokens.revoke.success"));
    } catch (error) {
      await notifyApiError(error, {
        title: t("user:personal_access_tokens.revoke.error"),
      });
    } finally {
      setRevoking(null);
    }
  }

  async function copyToken() {
    if (!createdToken) return;
    await navigator.clipboard.writeText(createdToken);
    toast.success(t("user:personal_access_tokens.copy.success"));
  }

  return (
    <>
      <title>{`${t("user:settings.personal_access_tokens")} - ${configStore?.config?.meta?.title}`}</title>
      <div
        className={cn([
          "flex",
          "flex-col",
          "flex-1",
          "p-4",
          "sm:p-6",
          "lg:p-10",
          "xl:mx-50",
          "lg:mx-30",
          "gap-5",
          "min-w-0",
        ])}
      >
        <div className={cn(["flex", "justify-between", "items-center"])}>
          <div className={cn(["flex", "items-center", "gap-3"])}>
            <div
              className={cn([
                "flex items-center justify-center",
                "size-10 rounded-badge",
                "bg-primary/10",
                "shrink-0",
              ])}
            >
              <KeyRoundIcon className={cn(["size-5"])} />
            </div>
            <div className={cn(["min-w-0"])}>
              <h2 className={cn(["text-base", "font-semibold"])}>
                {t("user:settings.personal_access_tokens")}
              </h2>
              <p className={cn(["text-sm", "text-muted-foreground"])}>
                {t("user:personal_access_tokens.description")}
              </p>
            </div>
          </div>
          <Button
            variant={"solid"}
            icon={<PlusIcon />}
            onClick={() => setCreateDialogOpen(true)}
          >
            {t("user:personal_access_tokens.create._")}
          </Button>
        </div>
        <Separator />

        {createdToken && (
          <Card
            className={cn([
              "border-success/30",
              "bg-success/5",
              "p-5",
              "space-y-4",
            ])}
          >
            <div className={cn(["flex", "items-start", "gap-3"])}>
              <ShieldCheckIcon
                className={cn(["size-5", "text-success", "mt-0.5", "shrink-0"])}
              />
              <div className={cn(["min-w-0", "space-y-1"])}>
                <h3 className={cn(["font-medium"])}>
                  {t("user:personal_access_tokens.created.title")}
                </h3>
                <p className={cn(["text-sm", "text-muted-foreground"])}>
                  {t("user:personal_access_tokens.created.description")}
                </p>
              </div>
            </div>
            <div className={cn(["flex", "gap-2", "items-stretch", "min-w-0"])}>
              <Field size={"sm"} className={"flex-1"}>
                <FieldIcon>
                  <KeyRoundIcon />
                </FieldIcon>
                <TextField value={createdToken} />
              </Field>
              <Button
                icon={<ClipboardIcon />}
                variant="tonal"
                onClick={copyToken}
                aria-label={t("user:personal_access_tokens.copy._")}
              >
                {t("user:personal_access_tokens.copy._")}
              </Button>
            </div>
          </Card>
        )}

        <div className={cn(["space-y-3"])}>
          <h3 className={cn(["text-base", "font-semibold"])}>
            {t("user:personal_access_tokens.active")}
          </h3>
          {activeTokens.length === 0 ? (
            <div
              className={cn([
                "rounded-lg",
                "border",
                "border-dashed",
                "p-8",
                "text-center",
                "text-sm",
                "text-muted-foreground",
              ])}
            >
              {t("user:personal_access_tokens.empty")}
            </div>
          ) : (
            <ItemGroup
              className={cn([
                "overflow-hidden",
                "rounded-xl",
                "border",
                "bg-card",
                "text-card-foreground",
              ])}
            >
              {activeTokens.map((token, index) => (
                <Fragment key={token.id}>
                  <Item className={cn(["bg-card", "text-card-foreground"])}>
                    <ItemContent className="gap-1">
                      <ItemTitle>{token.name}</ItemTitle>
                      <ItemDescription
                        className={cn([
                          "flex",
                          "flex-wrap",
                          "gap-x-3",
                          "gap-y-1",
                        ])}
                      >
                        <span>
                          {t("user:personal_access_tokens.created_at", {
                            date: formatDate(token.created_at),
                          })}
                        </span>
                        <span>
                          {t("user:personal_access_tokens.expires_at", {
                            date: formatDate(token.expires_at),
                          })}
                        </span>
                        {token.last_used_at && (
                          <span>
                            {t("user:personal_access_tokens.last_used_at", {
                              date: formatDate(token.last_used_at),
                            })}
                          </span>
                        )}
                      </ItemDescription>
                    </ItemContent>
                    <ItemActions>
                      <Tooltip>
                        <TooltipTrigger
                          render={
                            <Button
                              level="error"
                              variant="ghost"
                              square
                              size="sm"
                              icon={<Trash2Icon />}
                              loading={revoking === token.id}
                              onClick={() => handleRevoke(token.id)}
                            />
                          }
                        />
                        <TooltipContent sideOffset={8}>
                          {t("user:personal_access_tokens.revoke._")}
                        </TooltipContent>
                      </Tooltip>
                    </ItemActions>
                  </Item>
                  {index !== activeTokens.length - 1 && <ItemSeparator />}
                </Fragment>
              ))}
            </ItemGroup>
          )}
        </div>

        {inactiveTokens.length > 0 && (
          <div className={cn(["space-y-3"])}>
            <h3
              className={cn([
                "text-base",
                "font-semibold",
                "text-muted-foreground",
              ])}
            >
              {t("user:personal_access_tokens.inactive")}
            </h3>
            <ItemGroup
              className={cn([
                "overflow-hidden",
                "rounded-xl",
                "border",
                "bg-card",
                "text-card-foreground",
              ])}
            >
              {inactiveTokens.map((token, index) => (
                <Fragment key={token.id}>
                  <Item
                    className={cn([
                      "bg-card",
                      "text-card-foreground",
                      "opacity-70",
                    ])}
                  >
                    <ItemContent className="gap-1">
                      <ItemTitle>{token.name}</ItemTitle>
                      <ItemDescription>
                        {token.revoked_at
                          ? t("user:personal_access_tokens.revoked_at", {
                              date: formatDate(token.revoked_at),
                            })
                          : t("user:personal_access_tokens.expired_at", {
                              date: formatDate(token.expires_at),
                            })}
                      </ItemDescription>
                    </ItemContent>
                  </Item>
                  {index !== inactiveTokens.length - 1 && <ItemSeparator />}
                </Fragment>
              ))}
            </ItemGroup>
          </div>
        )}
      </div>
      <Dialog open={createDialogOpen} onOpenChange={setCreateDialogOpen}>
        <DialogContent>
          <Card
            className={cn([
              "w-full",
              "max-w-xl",
              "rounded-elevated",
              "shadow-lg",
              "overflow-hidden",
              "flex",
              "flex-col",
            ])}
          >
            <DialogHeader
              className="p-5 pb-0"
              icon={<KeyRoundIcon />}
              title={t("user:personal_access_tokens.create._")}
              description={t("user:personal_access_tokens.create.description")}
            />
            <DialogBody className="p-5 pt-4">
              <Form {...form}>
                <form
                  onSubmit={form.handleSubmit(handleCreate)}
                  autoComplete="off"
                  className={cn(["flex", "flex-col", "flex-1", "gap-5"])}
                >
                  <FormField
                    control={form.control}
                    name="name"
                    render={({ field }) => (
                      <FormItem>
                        <FormLabel>
                          {t("user:personal_access_tokens.create.name")}
                        </FormLabel>
                        <FormControl>
                          <Field size="sm">
                            <FieldIcon>
                              <KeyRoundIcon />
                            </FieldIcon>
                            <TextField
                              {...field}
                              maxLength={64}
                              placeholder={t(
                                "user:personal_access_tokens.create.name_placeholder"
                              )}
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
                    name="expiresAt"
                    render={({ field }) => (
                      <FormItem>
                        <FormLabel>
                          {t("user:personal_access_tokens.create.expires_at")}
                        </FormLabel>
                        <FormControl>
                          <Field size="sm">
                            <FieldIcon>
                              <CalendarIcon />
                            </FieldIcon>
                            <DateTimePicker
                              {...field}
                              granularity="minute"
                              placeholder={t(
                                "user:personal_access_tokens.create.expires_at"
                              )}
                            />
                          </Field>
                        </FormControl>
                        <FormMessage />
                      </FormItem>
                    )}
                  />
                  {configStore?.config?.captcha?.provider !== "none" && (
                    <FormField
                      control={form.control}
                      name="captcha"
                      render={({ field }) => (
                        <FormItem>
                          <FormLabel>{t("account:captcha._")}</FormLabel>
                          <Captcha
                            ref={captchaRef}
                            onChange={field.onChange}
                            size="sm"
                          />
                        </FormItem>
                      )}
                    />
                  )}
                  <DialogFooter>
                    <Button
                      type="submit"
                      variant="solid"
                      icon={<CheckIcon />}
                      level="success"
                      loading={loading}
                    >
                      {t("common:actions.confirm")}
                    </Button>
                  </DialogFooter>
                </form>
              </Form>
            </DialogBody>
          </Card>
        </DialogContent>
      </Dialog>
    </>
  );
}
