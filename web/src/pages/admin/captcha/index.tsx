import { zodResolver } from "@hookform/resolvers/zod";
import { useQuery } from "@tanstack/react-query";
import {
  BotIcon,
  ClockIcon,
  KeyIcon,
  LinkIcon,
  LockIcon,
  LockKeyholeIcon,
  SaveIcon,
} from "lucide-react";
import { useEffect } from "react";
import { useForm } from "react-hook-form";
import { useTranslation } from "react-i18next";
import { toast } from "sonner";
import { z } from "zod";
import { getConfigs, updateConfig } from "@/api/admin/configs";
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
import { TextField } from "@/components/ui/text-field";
import { useConfigStore } from "@/storages/config";
import { cn } from "@/utils";
import { notifyApiError } from "@/utils/query";

export default function Index() {
  const { t } = useTranslation();

  const { config: globalConfig } = useConfigStore();
  const { data: config } = useQuery({
    queryKey: ["admin", "config"],
    queryFn: getConfigs,
    select: (response) => response.config,
  });

  const formSchema = z.object({
    provider: z.enum(["none", "pow", "image", "turnstile"]).optional(),
    difficulty: z.number().default(1).optional(),
    turnstile: z
      .object({
        url: z.string().default("").optional(),
        site_key: z.string().default("").optional(),
        secret_key: z.string().default("").optional(),
      })

      .optional(),
  });

  const form = useForm<z.infer<typeof formSchema>>({
    resolver: zodResolver(formSchema),
    defaultValues: config?.captcha,
  });

  useEffect(() => {
    form.reset(config?.captcha, {
      keepDefaultValues: false,
    });
  }, [config?.captcha, form]);

  async function onSubmit(values: z.infer<typeof formSchema>) {
    try {
      await updateConfig({ ...config, captcha: { ...values } });
      toast.success(t("admin:captcha.actions.update.success"));
    } catch (error) {
      await notifyApiError(error);
    }
  }

  return (
    <>
      <title>{`${t("admin:captcha._")} - ${globalConfig?.meta?.title}`}</title>
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
            <BotIcon />
            {t("admin:captcha._")}
          </h2>
          <Separator />
          <div className={cn(["flex", "flex-col", "gap-3", "sm:flex-row"])}>
            <FormField
              control={form.control}
              name={"provider"}
              render={({ field }) => (
                <FormItem className={cn(["w-full"])}>
                  <FormLabel>{t("admin:captcha.form.provider._")}</FormLabel>
                  <FormDescription>
                    {t("admin:captcha.form.provider.description")}
                  </FormDescription>
                  <FormControl>
                    <Field>
                      <FieldIcon>
                        <LockIcon />
                      </FieldIcon>
                      <Select
                        {...field}
                        onValueChange={(value) => field.onChange(value)}
                        value={String(field.value)}
                      >
                        <SelectTrigger>
                          <SelectValue />
                        </SelectTrigger>
                        <SelectContent>
                          <SelectItem value="none">
                            {t("admin:captcha.provider.none")}
                          </SelectItem>
                          <SelectItem value="pow">
                            {t("admin:captcha.provider.pow")}
                          </SelectItem>
                          <SelectItem value="image">
                            {t("admin:captcha.provider.image")}
                          </SelectItem>
                          <SelectItem value="turnstile">
                            {t("admin:captcha.provider.turnstile")}
                          </SelectItem>
                        </SelectContent>
                      </Select>
                    </Field>
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />
            {["image", "pow"].includes(form.watch("provider") ?? "") && (
              <FormField
                control={form.control}
                name={"difficulty"}
                render={({ field }) => (
                  <FormItem className={cn(["w-full"])}>
                    <FormLabel>
                      {t("admin:captcha.form.difficulty._")}
                    </FormLabel>
                    <FormDescription>
                      {t("admin:captcha.form.difficulty.description")}
                    </FormDescription>
                    <FormControl>
                      <Field>
                        <FieldIcon>
                          <ClockIcon />
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
            )}
          </div>
          {form.watch("provider") === "turnstile" && (
            <>
              <FormField
                control={form.control}
                name={"turnstile.url"}
                render={({ field }) => (
                  <FormItem className={cn(["w-full"])}>
                    <FormLabel>
                      {t("admin:captcha.form.turnstile.url._")}
                    </FormLabel>
                    <FormControl>
                      <Field>
                        <FieldIcon>
                          <LinkIcon />
                        </FieldIcon>
                        <TextField
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
              <div className={cn(["flex", "flex-col", "gap-3", "sm:flex-row"])}>
                <FormField
                  control={form.control}
                  name={"turnstile.site_key"}
                  render={({ field }) => (
                    <FormItem className={cn(["w-full"])}>
                      <FormLabel>
                        {t("admin:captcha.form.turnstile.site_key._")}
                      </FormLabel>
                      <FormControl>
                        <Field>
                          <FieldIcon>
                            <KeyIcon />
                          </FieldIcon>
                          <TextField
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
                  name={"turnstile.secret_key"}
                  render={({ field }) => (
                    <FormItem className={cn(["w-full"])}>
                      <FormLabel>
                        {t("admin:captcha.form.turnstile.secret_key._")}
                      </FormLabel>
                      <FormControl>
                        <Field>
                          <FieldIcon>
                            <LockKeyholeIcon />
                          </FieldIcon>
                          <TextField
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
            </>
          )}
          <div className={cn(["flex-1"])} />
          <Button
            type={"submit"}
            variant={"solid"}
            size={"lg"}
            icon={<SaveIcon />}
            className={cn(["mt-2"])}
          >
            {t("common:actions.save")}
          </Button>
        </form>
      </Form>
    </>
  );
}
