import {
    CheckIcon,
    CircleHelpIcon,
    UserRoundIcon,
    LockIcon,
} from "lucide-react";
import { cn } from "@/utils";
import { Field, FieldIcon } from "@/components/ui/field";
import { TextField } from "@/components/ui/text-field";
import { z } from "zod";
import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import {
    Form,
    FormControl,
    FormField,
    FormItem,
    FormLabel,
    FormMessage,
} from "@/components/ui/form";
import { Button } from "@/components/ui/button";
import { Captcha } from "@/components/widgets/captcha";
import { useState } from "react";
import { login } from "@/api/users";
import { toast } from "sonner";
import { useAuthStore } from "@/storages/auth";
import { Link, useNavigate } from "react-router";
import { useConfigStore } from "@/storages/config";
import { StatusCodes } from "http-status-codes";
import { useTranslation } from "react-i18next";

function LoginForm() {
    const configStore = useConfigStore();
    const authStore = useAuthStore();
    const navigate = useNavigate();
    const { t } = useTranslation();

    const [loading, setLoading] = useState<boolean>(false);

    const formSchema = z.object({
        account: z.string({
            message: t("account:login.form.message.please_input_username"),
        }),
        password: z.string({
            message: t("account:login.form.message.please_input_password"),
        }),
        captcha: z
            .object({
                id: z.string(),
                content: z.string(),
            })
            .nullish(),
    });

    const form = useForm<z.infer<typeof formSchema>>({
        resolver: zodResolver(formSchema),
    });

    function onSubmit(values: z.infer<typeof formSchema>) {
        setLoading(true);
        login({
            ...values,
        })
            .then((res) => {
                if (res.code === StatusCodes.OK) {
                    authStore.setUser(res.data);
                    toast.success(t("account:login.success._"), {
                        id: "login",
                        description: t("account:login.success.welcome", {
                            name: res.data?.name,
                        }),
                    });
                    navigate("/");
                }

                if (res.code === StatusCodes.BAD_REQUEST) {
                    toast.error(t("account:login.error._"), {
                        id: "login",
                        description: t("account:login.error.invalid"),
                    });
                }

                if (res.code === StatusCodes.GONE) {
                    toast.error(t("account:captcha.expired"), {
                        id: "login",
                        description: t("account:captcha.please_refresh"),
                    });
                }
            })
            .finally(() => {
                setLoading(false);
            });
    }

    return (
        <Form {...form}>
            <form
                onSubmit={form.handleSubmit(onSubmit)}
                autoComplete={"off"}
                className={cn(["flex", "flex-col", "h-full", "gap-8"])}
            >
                <div className={cn("space-y-3", "flex-1")}>
                    <FormField
                        control={form.control}
                        name={"account"}
                        render={({ field }) => (
                            <FormItem>
                                <FormLabel>{`${t("user:username")} / ${t("user:email")}`}</FormLabel>
                                <FormControl>
                                    <Field>
                                        <FieldIcon>
                                            <UserRoundIcon />
                                        </FieldIcon>
                                        <TextField
                                            placeholder={"Account"}
                                            {...field}
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
                                <FormLabel>{t("user:password")}</FormLabel>
                                <FormControl>
                                    <Field>
                                        <FieldIcon>
                                            <LockIcon />
                                        </FieldIcon>
                                        <TextField
                                            placeholder={"Password"}
                                            type={"password"}
                                            {...field}
                                        />
                                    </Field>
                                </FormControl>
                                <FormMessage />
                            </FormItem>
                        )}
                    />
                    {configStore?.config?.email?.is_enabled && (
                        <div className={cn(["flex", "justify-end"])}>
                            <Link
                                to={"/account/forget"}
                                className={cn([
                                    "hover:underline",
                                    "underline-offset-3",
                                    "items-center",
                                    "text-sm",
                                    "flex",
                                    "gap-1",
                                ])}
                            >
                                <CircleHelpIcon className={cn(["size-4"])} />
                                {t("account:forgot")}
                            </Link>
                        </div>
                    )}
                    {configStore?.config?.captcha?.provider !== "none" && (
                        <FormField
                            name={"captcha"}
                            render={({ field }) => (
                                <FormItem>
                                    <FormLabel>
                                        {t("account:captcha._")}
                                    </FormLabel>
                                    <Captcha onChange={field.onChange} />
                                </FormItem>
                            )}
                        />
                    )}
                </div>
                <Button
                    variant={"solid"}
                    level={"success"}
                    type={"submit"}
                    size={"lg"}
                    className={cn(["w-full"])}
                    icon={CheckIcon}
                    loading={loading}
                >
                    {t("account:login._")}
                </Button>
            </form>
        </Form>
    );
}

export { LoginForm };
