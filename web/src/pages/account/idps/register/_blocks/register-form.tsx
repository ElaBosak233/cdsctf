import { zodResolver } from "@hookform/resolvers/zod";
import { useQuery } from "@tanstack/react-query";
import { StatusCodes } from "http-status-codes";
import { HTTPError } from "ky";
import {
  CheckIcon,
  LockIcon,
  MailIcon,
  TypeIcon,
  UserRoundIcon,
} from "lucide-react";
import { useState } from "react";
import { useForm } from "react-hook-form";
import { useTranslation } from "react-i18next";
import { useNavigate, useParams } from "react-router";
import { toast } from "sonner";
import { z } from "zod";
import { getIdp, registerWithIdp } from "@/api/idps";
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
import { TextField } from "@/components/ui/text-field";
import { useAuthStore } from "@/storages/auth";
import { cn } from "@/utils";
import { formatApiMsg, parseErrorResponse } from "@/utils/query";

type PendingIdentity = {
  token: string;
  idp_id: number;
  data: Record<string, string>;
};

function IdpRegisterForm() {
  const navigate = useNavigate();
  const { idp_id } = useParams();
  const idpId = Number(idp_id);

  const { t } = useTranslation();
  const [loading, setLoading] = useState<boolean>(false);

  const { data: idp } = useQuery({
    queryKey: ["idp", idpId],
    queryFn: () => getIdp(idpId).then((res) => res.idp),
    enabled: idpId != null,
  });

  const pendingRaw = sessionStorage.getItem("idp_pending_identity");
  let pending: PendingIdentity | null = null;
  try {
    pending = pendingRaw ? JSON.parse(pendingRaw) : null;
  } catch {
    sessionStorage.removeItem("idp_pending_identity");
  }

  const formSchema = z
    .object({
      username: z
        .string()
        .regex(/^[a-z]/, t("account:register.form.username.start_lower"))
        .regex(/^[a-z0-9]*$/, t("account:register.form.username.chars")),
      name: z.string(),
      email: z.email(t("account:register.form.email.invalid")),
      password: z.string().min(6, t("account:register.form.password.min")),
      confirm_password: z.string(),
    })
    .refine((data) => data.password === data.confirm_password, {
      message: t("account:register.form.confirm_password.mismatch"),
      path: ["confirm_password"],
    });

  const form = useForm<z.infer<typeof formSchema>>({
    resolver: zodResolver(formSchema),
    defaultValues: {
      username: pending?.data?.username?.toLowerCase() || "",
      name: pending?.data?.name || "",
      email: pending?.data?.email || "",
    },
  });

  if (!pending) {
    return (
      <div
        className={cn(["flex", "flex-col", "items-center", "gap-4", "py-8"])}
      >
        <p className={cn(["text-muted-foreground", "text-sm"])}>
          {t("account:idp.register.no_pending")}
        </p>
        <Button
          variant={"solid"}
          level={"info"}
          onClick={() => navigate("/account/login")}
        >
          {t("account:idp.register.go_login")}
        </Button>
      </div>
    );
  }

  async function onSubmit(values: z.infer<typeof formSchema>) {
    setLoading(true);
    try {
      const res = await registerWithIdp(idpId, {
        token: pending!.token,
        username: values.username,
        name: values.name,
        email: values.email,
        password: values.password,
      });

      sessionStorage.removeItem("idp_pending_identity");
      useAuthStore.getState().setUser(res.user);
      toast.success(t("account:idp.register.success"), {
        id: "idp-register-success",
        description: t("account:idp.register.success_desc"),
      });
      navigate("/", { replace: true });
    } catch (error) {
      if (!(error instanceof HTTPError)) throw error;
      const status = error.response.status;
      const body = await parseErrorResponse(error);

      if (status === StatusCodes.BAD_REQUEST) {
        toast.error(t("account:idp.register.failed"), {
          id: "idp-register-error",
          description: formatApiMsg(body.msg),
        });
      }

      if (status === StatusCodes.CONFLICT) {
        toast.error(t("account:idp.register.failed"), {
          id: "idp-register-error",
          description: t("account:idp.register.conflict"),
        });
      }
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
        {idp && (
          <div
            className={cn(["flex", "items-center", "justify-center", "gap-3"])}
          >
            <Avatar
              square
              className={cn(["size-12", "bg-transparent"])}
              src={idp.avatar_hash && `/api/media?hash=${idp.avatar_hash}`}
              fallback={idp.name?.charAt(0)}
            />
            <span
              className={cn([
                "text-lg",
                "font-medium",
                "text-muted-foreground",
              ])}
            >
              {idp.name}
            </span>
          </div>
        )}
        <div className={cn("space-y-3", "flex-1")}>
          <div className={cn(["flex", "gap-3", "items-center"])}>
            <FormField
              control={form.control}
              name={"username"}
              render={({ field }) => (
                <FormItem className={cn(["flex-1"])}>
                  <FormLabel>{t("account:register.form.username._")}</FormLabel>
                  <FormControl>
                    <Field>
                      <FieldIcon>
                        <UserRoundIcon />
                      </FieldIcon>
                      <TextField
                        {...field}
                        placeholder={t("account:register.form.username._")}
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
                  <FormLabel>{t("account:register.form.name._")}</FormLabel>
                  <FormControl>
                    <Field>
                      <FieldIcon>
                        <TypeIcon />
                      </FieldIcon>
                      <TextField
                        {...field}
                        placeholder={t("account:register.form.name._")}
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
                <FormLabel>{t("account:register.form.email._")}</FormLabel>
                <FormControl>
                  <Field>
                    <FieldIcon>
                      <MailIcon />
                    </FieldIcon>
                    <TextField
                      {...field}
                      placeholder={t("account:register.form.email._")}
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
                <FormLabel>{t("account:register.form.password._")}</FormLabel>
                <FormControl>
                  <Field>
                    <FieldIcon>
                      <LockIcon />
                    </FieldIcon>
                    <TextField
                      type={"password"}
                      {...field}
                      placeholder={t("account:register.form.password._")}
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
                  {t("account:register.form.confirm_password._")}
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
                        "account:register.form.confirm_password._"
                      )}
                    />
                  </Field>
                </FormControl>
                <FormMessage />
              </FormItem>
            )}
          />
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
          {t("account:idp.register.submit")}
        </Button>
      </form>
    </Form>
  );
}

export { IdpRegisterForm };
