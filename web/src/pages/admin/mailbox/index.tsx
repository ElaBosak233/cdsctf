import { zodResolver } from "@hookform/resolvers/zod";
import { useQuery, useQueryClient } from "@tanstack/react-query";
import {
  EthernetPortIcon,
  KeyRoundIcon,
  ListFilterIcon,
  MailCheckIcon,
  PowerIcon,
  SaveIcon,
  ServerIcon,
  ShieldCheckIcon,
  UserRoundIcon,
} from "lucide-react";
import { useEffect } from "react";
import { useForm } from "react-hook-form";
import { useTranslation } from "react-i18next";
import { toast } from "sonner";
import { z } from "zod";
import { getConfigs, updateConfig } from "@/api/management/configs";
import { Button } from "@/components/ui/button";
import { Field, FieldIcon } from "@/components/ui/field";
import {
  Form,
  FormControl,
  FormDescription,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "@/components/ui/form";
import { LoadingOverlay } from "@/components/ui/loading-overlay";
import { NumberField } from "@/components/ui/number-field";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { Separator } from "@/components/ui/separator";
import { TagsField } from "@/components/ui/tags-field";
import { TextField } from "@/components/ui/text-field";
import type { AdminConfig } from "@/models/config";
import { useConfigStore } from "@/storages/config";
import { cn } from "@/utils";
import { isEmailDomainRule, splitEmailDomainRules } from "@/utils/email-domain";
import { notifyApiError } from "@/utils/query";

function mailboxFormValues(email: AdminConfig["email"]) {
  return {
    ...email,
    password: "",
    whitelist: [...new Set(email?.whitelist)],
  };
}

export default function Index() {
  const { t } = useTranslation();
  const queryClient = useQueryClient();

  const { config: globalConfig } = useConfigStore();
  const { data: config } = useQuery({
    queryKey: ["admin", "config"],
    queryFn: getConfigs,
    select: (response) => response.config,
  });

  const formSchema = z.object({
    enabled: z.boolean(),
    host: z.string().default("").optional(),
    port: z.number().min(0).max(65535),
    tls: z.enum(["starttls", "tls", "none"]).optional(),
    username: z.string().default("").optional(),
    password: z.string().optional(),
    whitelist: z
      .array(z.string())
      .refine((value) => value.every(isEmailDomainRule), {
        message: t("admin:mailbox.form.whitelist.invalid"),
      }),
  });

  const form = useForm<z.infer<typeof formSchema>>({
    resolver: zodResolver(formSchema),
    defaultValues: mailboxFormValues(config?.email),
  });

  useEffect(() => {
    form.reset(mailboxFormValues(config?.email));
  }, [config?.email, form]);

  async function onSubmit(values: z.infer<typeof formSchema>) {
    if (!config) return;
    const { password, ...email } = values;
    try {
      const response = await updateConfig({
        ...config,
        email: {
          ...email,
          password: password || undefined,
        },
      });
      queryClient.setQueryData(["admin", "config"], response);
      form.reset(mailboxFormValues(response.config.email));
      void queryClient.invalidateQueries({ queryKey: ["configs"] });
      toast.success(t("admin:mailbox.actions.update.success"));
    } catch (error) {
      await notifyApiError(error);
    }
  }

  return (
    <>
      <title>{`${t("admin:mailbox._")} - ${globalConfig?.meta?.title}`}</title>
      <Form {...form}>
        <form
          onSubmit={form.handleSubmit(onSubmit)}
          autoComplete={"off"}
          className={cn([
            "flex",
            "flex-col",
            "gap-3",
            "p-4",
            "sm:p-6",
            "lg:p-10",
            "xl:mx-60",
            "lg:mx-30",
            "min-h-(--app-content-height)",
            "relative",
          ])}
        >
          <LoadingOverlay loading={!config} />
          <h2
            className={cn(["flex", "gap-2", "items-center", "text-xl", "mt-2"])}
          >
            <MailCheckIcon />
            {t("admin:mailbox._")}
          </h2>
          <Separator />

          <FormField
            control={form.control}
            name={"enabled"}
            render={({ field }) => (
              <FormItem className={cn(["w-full"])}>
                <FormLabel>{t("admin:mailbox.form.enabled._")}</FormLabel>
                <Field>
                  <FieldIcon>
                    <PowerIcon />
                  </FieldIcon>
                  <Select
                    onValueChange={(value) => field.onChange(value === "true")}
                    value={String(field.value)}
                  >
                    <FormControl>
                      <SelectTrigger>
                        <SelectValue />
                      </SelectTrigger>
                    </FormControl>
                    <SelectContent>
                      <SelectItem value="true">
                        {t("admin:mailbox.enabled.true")}
                      </SelectItem>
                      <SelectItem value="false">
                        {t("admin:mailbox.enabled.false")}
                      </SelectItem>
                    </SelectContent>
                  </Select>
                </Field>
                <FormDescription>
                  {t(
                    field.value
                      ? "admin:mailbox.form.enabled.description.enabled"
                      : "admin:mailbox.form.enabled.description.disabled"
                  )}
                </FormDescription>
                <FormMessage />
              </FormItem>
            )}
          />

          {form.watch("enabled") && (
            <>
              <div className={cn(["flex", "flex-col", "gap-3", "sm:flex-row"])}>
                <FormField
                  control={form.control}
                  name={"host"}
                  render={({ field }) => (
                    <FormItem className={cn(["w-full"])}>
                      <FormLabel>{t("admin:mailbox.form.host._")}</FormLabel>
                      <FormControl>
                        <Field>
                          <FieldIcon>
                            <ServerIcon />
                          </FieldIcon>
                          <TextField
                            {...field}
                            placeholder={t(
                              "admin:mailbox.form.host.placeholder"
                            )}
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
                  name={"port"}
                  render={({ field }) => (
                    <FormItem className={cn(["w-full"])}>
                      <FormLabel>{t("admin:mailbox.form.port._")}</FormLabel>
                      <FormControl>
                        <Field>
                          <FieldIcon>
                            <EthernetPortIcon />
                          </FieldIcon>
                          <NumberField
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
                  name={"tls"}
                  render={({ field }) => (
                    <FormItem className={cn(["w-full"])}>
                      <FormLabel>TLS</FormLabel>
                      <FormControl>
                        <Field>
                          <FieldIcon>
                            <ShieldCheckIcon />
                          </FieldIcon>
                          <Select
                            onValueChange={field.onChange}
                            value={String(field.value)}
                          >
                            <SelectTrigger>
                              <SelectValue />
                            </SelectTrigger>
                            <SelectContent>
                              <SelectItem value="tls">
                                {t("admin:mailbox.tls.tls")}
                              </SelectItem>
                              <SelectItem value="starttls">
                                {t("admin:mailbox.tls.starttls")}
                              </SelectItem>
                              <SelectItem value="none">
                                {t("admin:mailbox.tls.none")}
                              </SelectItem>
                            </SelectContent>
                          </Select>
                        </Field>
                      </FormControl>
                      <FormMessage />
                    </FormItem>
                  )}
                />
              </div>

              <FormField
                control={form.control}
                name={"username"}
                render={({ field }) => (
                  <FormItem className={cn(["w-full"])}>
                    <FormLabel>{t("admin:mailbox.form.username._")}</FormLabel>
                    <FormControl>
                      <Field>
                        <FieldIcon>
                          <UserRoundIcon />
                        </FieldIcon>
                        <TextField
                          {...field}
                          placeholder={t(
                            "admin:mailbox.form.username.placeholder"
                          )}
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
                name={"password"}
                render={({ field }) => (
                  <FormItem className={cn(["w-full"])}>
                    <FormLabel>{t("admin:mailbox.form.password._")}</FormLabel>
                    <Field>
                      <FieldIcon>
                        <KeyRoundIcon />
                      </FieldIcon>
                      <FormControl>
                        <TextField
                          {...field}
                          type="password"
                          autoComplete="new-password"
                        />
                      </FormControl>
                    </Field>
                    <FormDescription>
                      {t("admin:mailbox.form.password.description")}
                    </FormDescription>
                    <FormMessage />
                  </FormItem>
                )}
              />
            </>
          )}

          <FormField
            control={form.control}
            name="whitelist"
            render={({ field }) => (
              <FormItem className="w-full">
                <FormLabel>{t("admin:mailbox.form.whitelist._")}</FormLabel>
                <Field>
                  <FieldIcon className="h-auto self-stretch">
                    <ListFilterIcon aria-hidden="true" />
                  </FieldIcon>
                  <FormControl>
                    <TagsField
                      value={field.value}
                      onValueChange={(rules) =>
                        field.onChange([
                          ...new Set(
                            rules.map((rule) => rule.trim().toLowerCase())
                          ),
                        ])
                      }
                      onBlur={field.onBlur}
                      inputRef={field.ref}
                      parsePaste={splitEmailDomainRules}
                      commitOnBlur
                      placeholder="example.edu"
                    />
                  </FormControl>
                </Field>
                <FormDescription>
                  {t("admin:mailbox.form.whitelist.description")}
                </FormDescription>
                <FormMessage className="static block" />
              </FormItem>
            )}
          />

          <div className={cn(["flex-1"])} />
          <Button
            type={"submit"}
            variant={"solid"}
            size={"lg"}
            icon={<SaveIcon />}
            loading={form.formState.isSubmitting}
            className={cn(["mt-2"])}
          >
            {t("common:actions.save")}
          </Button>
        </form>
      </Form>
    </>
  );
}
