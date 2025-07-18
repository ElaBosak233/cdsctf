import { zodResolver } from "@hookform/resolvers/zod";
import { StatusCodes } from "http-status-codes";
import { BotIcon, ClockIcon, LockIcon, SaveIcon, SendIcon } from "lucide-react";
import { useEffect, useState } from "react";
import { useForm } from "react-hook-form";
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
import { Select } from "@/components/ui/select";
import { Separator } from "@/components/ui/separator";
import { TextField } from "@/components/ui/text-field";
import { Config } from "@/models/config";
import { cn } from "@/utils";

export default function Index() {
  const [config, setConfig] = useState<Config>();

  useEffect(() => {
    getConfigs().then((res) => {
      setConfig(res.data);
    });
  }, []);

  const formSchema = z.object({
    provider: z
      .enum(["none", "pow", "image", "turnstile", "hcaptcha"])
      .optional(),
    difficulty: z.number().default(1).optional(),
    turnstile: z
      .object({
        url: z.string().default("").optional(),
        site_key: z.string().default("").optional(),
        secret_key: z.string().default("").optional(),
      })

      .optional(),
    hcaptcha: z
      .object({
        url: z.string().default("").optional(),
        site_key: z.string().default("").optional(),
        secret_key: z.string().default("").optional(),
        score: z.number().default(0).optional(),
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
  }, [config?.captcha, form.reset]);

  function onSubmit(values: z.infer<typeof formSchema>) {
    updateConfig({
      ...config,
      captcha: { ...values },
    }).then((res) => {
      if (res.code === StatusCodes.OK) {
        toast.success("人机验证配置更新成功");
      }
    });
  }

  return (
    <Form {...form}>
      <form
        onSubmit={form.handleSubmit(onSubmit)}
        autoComplete={"off"}
        className={cn([
          "flex",
          "flex-col",
          "gap-3",
          "p-10",
          "xl:mx-60",
          "lg:mx-30",
          "min-h-[calc(100vh-64px)]",
          "relative",
        ])}
      >
        <LoadingOverlay loading={!config} />
        <h2
          className={cn(["flex", "gap-2", "items-center", "text-xl", "mt-2"])}
        >
          <BotIcon />
          人机验证
        </h2>
        <Separator />
        <div className={cn(["flex", "gap-3"])}>
          <FormField
            control={form.control}
            name={"provider"}
            render={({ field }) => (
              <FormItem className={cn(["w-full"])}>
                <FormLabel>提供方</FormLabel>
                <FormDescription>
                  若启用，则在必要的界面中启用人机验证。
                </FormDescription>
                <FormControl>
                  <Field>
                    <FieldIcon>
                      <LockIcon />
                    </FieldIcon>
                    <Select
                      {...field}
                      options={[
                        {
                          value: "none",
                          content: "不启用",
                        },
                        {
                          value: "pow",
                          content: "工作量验证",
                        },
                        {
                          value: "image",
                          content: "图形验证",
                        },
                        {
                          value: "turnstile",
                          content: "Cloudflare Trunstile",
                        },
                        {
                          value: "hcaptcha",
                          content: "HCaptcha",
                        },
                      ]}
                      onValueChange={(value) => field.onChange(value)}
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
            name={"difficulty"}
            render={({ field }) => (
              <FormItem className={cn(["w-full"])}>
                <FormLabel>难度</FormLabel>
                <FormDescription>适用于图形验证和工作量验证。</FormDescription>
                <FormControl>
                  <Field>
                    <FieldIcon>
                      <ClockIcon />
                    </FieldIcon>
                    <NumberField
                      placeholder="请输入难度"
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
        {form.watch("provider") === "turnstile" && (
          <>
            <FormField
              control={form.control}
              name={"turnstile.url"}
              render={({ field }) => (
                <FormItem className={cn(["w-full"])}>
                  <FormLabel>API URL</FormLabel>
                  <FormControl>
                    <Field>
                      <FieldIcon>
                        <SendIcon />
                      </FieldIcon>
                      <TextField
                        {...field}
                        placeholder="请输入 API URL"
                        value={field.value || ""}
                        onChange={field.onChange}
                      />
                    </Field>
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />
            <div className={cn(["flex", "gap-3"])}>
              <FormField
                control={form.control}
                name={"turnstile.site_key"}
                render={({ field }) => (
                  <FormItem className={cn(["w-full"])}>
                    <FormLabel>SITE_KEY</FormLabel>
                    <FormControl>
                      <Field>
                        <FieldIcon>
                          <SendIcon />
                        </FieldIcon>
                        <TextField
                          {...field}
                          placeholder="请输入 SITE_KEY"
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
                    <FormLabel>SECRET_KEY</FormLabel>
                    <FormControl>
                      <Field>
                        <FieldIcon>
                          <SendIcon />
                        </FieldIcon>
                        <TextField
                          {...field}
                          placeholder="请输入 SECRET_KEY"
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
        {form.watch("provider") === "hcaptcha" && (
          <>
            <div className={cn(["flex", "gap-3"])}>
              <FormField
                control={form.control}
                name={"hcaptcha.url"}
                render={({ field }) => (
                  <FormItem className={cn(["w-full"])}>
                    <FormLabel>API URL</FormLabel>
                    <FormControl>
                      <Field>
                        <FieldIcon>
                          <SendIcon />
                        </FieldIcon>
                        <TextField
                          {...field}
                          placeholder="请输入 API URL"
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
                name={"hcaptcha.score"}
                render={({ field }) => (
                  <FormItem className={cn(["w-full"])}>
                    <FormLabel>分数要求</FormLabel>
                    <FormControl>
                      <Field>
                        <FieldIcon>
                          <ClockIcon />
                        </FieldIcon>
                        <TextField
                          {...field}
                          type={"number"}
                          placeholder="请输入分数要求"
                          value={field.value || ""}
                          onChange={(e) =>
                            field.onChange(e.target.valueAsNumber)
                          }
                        />
                      </Field>
                    </FormControl>
                    <FormMessage />
                  </FormItem>
                )}
              />
            </div>
            <div className={cn(["flex", "gap-3"])}>
              <FormField
                control={form.control}
                name={"hcaptcha.site_key"}
                render={({ field }) => (
                  <FormItem className={cn(["w-full"])}>
                    <FormLabel>SITE_KEY</FormLabel>
                    <FormControl>
                      <Field>
                        <FieldIcon>
                          <SendIcon />
                        </FieldIcon>
                        <TextField
                          {...field}
                          placeholder="请输入 SITE_KEY"
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
                name={"hcaptcha.secret_key"}
                render={({ field }) => (
                  <FormItem className={cn(["w-full"])}>
                    <FormLabel>SECRET_KEY</FormLabel>
                    <FormControl>
                      <Field>
                        <FieldIcon>
                          <SendIcon />
                        </FieldIcon>
                        <TextField
                          {...field}
                          placeholder={"请输入 SECRET_KEY"}
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
          保存
        </Button>
      </form>
    </Form>
  );
}
