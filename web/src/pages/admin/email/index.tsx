import { zodResolver } from "@hookform/resolvers/zod";
import { MailCheckIcon, SaveIcon, TypeIcon } from "lucide-react";
import { useEffect, useState } from "react";
import { useForm } from "react-hook-form";
import { z } from "zod";

import { getConfigs, updateConfig } from "@/api/admin/configs";
import { getEmail, saveEmail } from "@/api/admin/configs/email";
import { Button } from "@/components/ui/button";
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
import { LoadingOverlay } from "@/components/ui/loading-overlay";
import { NumberField } from "@/components/ui/number-field";
import { Select } from "@/components/ui/select";
import { Separator } from "@/components/ui/separator";
import { TextField } from "@/components/ui/text-field";
import { Config } from "@/models/config";
import { useConfigStore } from "@/storages/config";
import { cn } from "@/utils";
import { toast } from "sonner";

export default function Index() {
  const { config: globalConfig } = useConfigStore();
  const [config, setConfig] = useState<Config>();

  const [verifyBody, setVerifyBody] = useState<string>();
  const [forgetBody, setForgetBody] = useState<string>();

  useEffect(() => {
    getConfigs().then((res) => {
      setConfig(res.data);
    });

    getEmail("verify").then((res) => {
      setVerifyBody(res.data);
    });

    getEmail("forget").then((res) => {
      setForgetBody(res.data);
    });
  }, []);

  const formSchema = z.object({
    is_enabled: z.boolean(),
    host: z.string().default("").optional(),
    port: z.number().min(0).max(65535),
    tls: z.enum(["starttls", "tls", "none"]).optional(),
    username: z.string().default("").optional(),
    password: z.string().default("").optional(),
    whitelist: z.array(z.string()).default([]).optional(),

    verify_body: z.string().default("").optional(),
    forget_body: z.string().default("").optional(),
  });

  const form = useForm<z.infer<typeof formSchema>>({
    resolver: zodResolver(formSchema),
    defaultValues: {
      ...config?.email,
      verify_body: verifyBody,
      forget_body: forgetBody,
    },
  });

  useEffect(() => {
    form.reset(
      { ...config?.email, verify_body: verifyBody, forget_body: forgetBody },
      {
        keepDefaultValues: false,
      }
    );
  }, [config?.email, verifyBody, forgetBody, form.reset]);

  async function onSubmit(values: z.infer<typeof formSchema>) {
    try {
      await updateConfig({
        ...config,
        email: { ...values },
      });

      if (values.is_enabled) {
        await saveEmail({
          type: "verify",
          data: values.verify_body!,
        });

        await saveEmail({
          type: "forget",
          data: values.forget_body!,
        });
      }
    } finally {
      toast.success("邮箱配置更新成功");
    }
  }

  return (
    <>
      <title>{`邮箱 - ${globalConfig?.meta?.title}`}</title>
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
            <MailCheckIcon />
            邮箱
          </h2>
          <Separator />

          <FormField
            control={form.control}
            name={"is_enabled"}
            render={({ field }) => (
              <FormItem className={cn(["w-full"])}>
                <FormLabel>是否启用</FormLabel>
                <FormControl>
                  <Field>
                    <FieldIcon>
                      <TypeIcon />
                    </FieldIcon>
                    <Select
                      options={[
                        {
                          value: String(true),
                          content: "启用",
                        },
                        {
                          value: String(false),
                          content: "禁用",
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

          {form.watch("is_enabled") && (
            <>
              <div className={cn(["flex", "gap-3"])}>
                <FormField
                  control={form.control}
                  name={"host"}
                  render={({ field }) => (
                    <FormItem className={cn(["w-full"])}>
                      <FormLabel>SMTP 地址</FormLabel>
                      <FormControl>
                        <Field>
                          <FieldIcon>
                            <TypeIcon />
                          </FieldIcon>
                          <TextField
                            {...field}
                            placeholder="请输入 SMTP 地址"
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
                      <FormLabel>端口</FormLabel>
                      <FormControl>
                        <Field>
                          <FieldIcon>
                            <TypeIcon />
                          </FieldIcon>
                          <NumberField
                            placeholder="请输入端口"
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
                            <TypeIcon />
                          </FieldIcon>
                          <Select
                            options={[
                              {
                                value: "tls",
                                content: "TLS",
                              },
                              {
                                value: "starttls",
                                content: "StartTLS",
                              },
                              {
                                value: "none",
                                content: "禁用",
                              },
                            ]}
                            onValueChange={field.onChange}
                            value={String(field.value)}
                          />
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
                    <FormLabel>邮箱用户名</FormLabel>
                    <FormControl>
                      <Field>
                        <FieldIcon>
                          <TypeIcon />
                        </FieldIcon>
                        <TextField
                          {...field}
                          placeholder="请输入邮箱用户名"
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
                    <FormLabel>邮箱密码/授权码</FormLabel>
                    <FormControl>
                      <Field>
                        <FieldIcon>
                          <TypeIcon />
                        </FieldIcon>
                        <TextField
                          {...field}
                          placeholder="请输入邮箱密码"
                          value={field.value || ""}
                          onChange={field.onChange}
                        />
                      </Field>
                    </FormControl>
                    <FormMessage />
                  </FormItem>
                )}
              />

              <Separator />

              <FormField
                control={form.control}
                name={"verify_body"}
                render={({ field }) => (
                  <FormItem className={cn(["w-full"])}>
                    <FormLabel>账户验证邮件</FormLabel>
                    <FormControl>
                      <Field>
                        <Editor
                          {...field}
                          lang="html"
                          placeholder="xHTML (RFC 2557)"
                          value={field.value || ""}
                          onChange={field.onChange}
                          showLineNumbers
                          className={cn(["h-80"])}
                        />
                      </Field>
                    </FormControl>
                    <FormMessage />
                  </FormItem>
                )}
              />

              <FormField
                control={form.control}
                name={"forget_body"}
                render={({ field }) => (
                  <FormItem className={cn(["w-full"])}>
                    <FormLabel>忘记密码邮件</FormLabel>
                    <FormControl>
                      <Field>
                        <Editor
                          {...field}
                          lang="html"
                          placeholder="xHTML (RFC 2557)"
                          value={field.value || ""}
                          onChange={field.onChange}
                          showLineNumbers
                          className={cn(["h-80"])}
                        />
                      </Field>
                    </FormControl>
                    <FormMessage />
                  </FormItem>
                )}
              />
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
    </>
  );
}
