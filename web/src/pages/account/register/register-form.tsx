import { zodResolver } from "@hookform/resolvers/zod";
import { StatusCodes } from "http-status-codes";
import { HTTPError } from "ky";
import {
  CheckIcon,
  LockIcon,
  MailIcon,
  TypeIcon,
  UserRoundIcon,
} from "lucide-react";
import { useRef, useState } from "react";
import { useForm } from "react-hook-form";
import { useTranslation } from "react-i18next";
import { useNavigate } from "react-router";
import { toast } from "sonner";
import { z } from "zod";
import { register } from "@/api/users";
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
import { TextField } from "@/components/ui/text-field";
import { Captcha, type CaptchaRef } from "@/components/widgets/captcha";
import { useConfigStore } from "@/storages/config";
import { cn } from "@/utils";
import { parseErrorResponse } from "@/utils/query";

function RegisterForm() {
  const configStore = useConfigStore();
  const navigate = useNavigate();
  const { t } = useTranslation();

  const captchaRef = useRef<CaptchaRef>(null);

  const [loading, setLoading] = useState<boolean>(false);

  const formSchema = z
    .object({
      username: z
        .string({
          message: t("account.register.form.username.message"),
        })
        .regex(/^[a-z]/, t("account.register.form.username.start_lower"))
        .regex(/^[a-z0-9]*$/, t("account.register.form.username.chars")),
      name: z.string({
        message: t("account.register.form.name.message"),
      }),
      email: z.email(t("account.register.form.email.invalid")),
      password: z
        .string({
          message: t("account.register.form.password.message"),
        })
        .min(6, t("account.register.form.password.min")),
      confirm_password: z.string({
        message: t("account.register.form.confirm_password.message"),
      }),
      captcha: z
        .object({
          id: z.string(),
          content: z.string(),
        })
        .optional(),
    })
    .refine((data) => data.password === data.confirm_password, {
      message: t("account.register.form.confirm_password.mismatch"),
      path: ["confirm_password"],
    });

  const form = useForm<z.infer<typeof formSchema>>({
    resolver: zodResolver(formSchema),
  });

  async function onSubmit(values: z.infer<typeof formSchema>) {
    setLoading(true);
    try {
      const res = await register({
        ...values,
      });

      if (res.code === StatusCodes.OK) {
        toast.success(t("account.register.toast.success._"), {
          id: "register-success",
          description: t("account.register.toast.success.desc"),
        });
        navigate("/account/login");
      }
    } catch (error) {
      if (!(error instanceof HTTPError)) throw error;
      const res = await parseErrorResponse(error);

      if (res.code === StatusCodes.BAD_REQUEST) {
        toast.success(t("account.register.toast.failure._"), {
          id: "register-error",
          description: res.msg,
        });
      }

      if (res.code === StatusCodes.CONFLICT) {
        toast.success(t("account.register.toast.failure._"), {
          id: "register-error",
          description: t("account.register.toast.failure.conflict"),
        });
      }

      captchaRef.current?.refresh();
    } finally {
      setLoading(false);
    }
  }

  return (
    <Form {...form}>
      <form
        onSubmit={form.handleSubmit(onSubmit)}
        autoComplete={"off"}
        className={cn(["flex", "flex-col", "h-full", "gap-8"])}
      >
        <div className={cn("space-y-3", "flex-1")}>
          <div className={cn(["flex", "gap-3", "items-center"])}>
            <FormField
              control={form.control}
              name={"username"}
              render={({ field }) => (
                <FormItem className={cn(["flex-1"])}>
                  <FormLabel>{t("account.register.form.username._")}</FormLabel>
                  <FormControl>
                    <Field>
                      <FieldIcon>
                        <UserRoundIcon />
                      </FieldIcon>
                      <TextField
                        {...field}
                        placeholder={t("account.register.form.username._")}
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
                <FormItem className={cn(["flex-1"])}>
                  <FormLabel>{t("account.register.form.name._")}</FormLabel>
                  <FormControl>
                    <Field>
                      <FieldIcon>
                        <TypeIcon />
                      </FieldIcon>
                      <TextField
                        {...field}
                        placeholder={t("account.register.form.name._")}
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
            name={"email"}
            render={({ field }) => (
              <FormItem>
                <FormLabel>{t("account.register.form.email._")}</FormLabel>
                <FormControl>
                  <Field>
                    <FieldIcon>
                      <MailIcon />
                    </FieldIcon>
                    <TextField
                      {...field}
                      placeholder={t("account.register.form.email._")}
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
              <FormItem>
                <FormLabel>{t("account.register.form.password._")}</FormLabel>
                <FormControl>
                  <Field>
                    <FieldIcon>
                      <LockIcon />
                    </FieldIcon>
                    <TextField
                      type={"password"}
                      {...field}
                      placeholder={t("account.register.form.password._")}
                    />
                  </Field>
                </FormControl>
                <FormMessage />
              </FormItem>
            )}
          />
          <FormField
            control={form.control}
            name={"confirm_password"}
            render={({ field }) => (
              <FormItem>
                <FormLabel>
                  {t("account.register.form.confirm_password._")}
                </FormLabel>
                <FormControl>
                  <Field>
                    <FieldIcon>
                      <LockIcon />
                    </FieldIcon>
                    <TextField
                      type={"password"}
                      {...field}
                      placeholder={t(
                        "account.register.form.confirm_password._"
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
              name={"captcha"}
              render={({ field }) => (
                <FormItem>
                  <FormLabel>{t("account.register.form.captcha._")}</FormLabel>
                  <Captcha ref={captchaRef} onChange={field.onChange} />
                </FormItem>
              )}
            />
          )}
        </div>
        <Button
          variant={"solid"}
          level={"info"}
          type={"submit"}
          size={"lg"}
          className={cn(["w-full"])}
          icon={<CheckIcon />}
          loading={loading}
        >
          {t("account.register.submit")}
        </Button>
      </form>
    </Form>
  );
}

export { RegisterForm };
