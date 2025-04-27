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

function LoginForm() {
    const configStore = useConfigStore();
    const authStore = useAuthStore();
    const navigate = useNavigate();

    const [loading, setLoading] = useState<boolean>(false);

    const formSchema = z.object({
        account: z.string({
            message: "请输入用户名",
        }),
        password: z.string({
            message: "请输入密码",
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
                    toast.success("登录成功", {
                        id: "login",
                        description: `欢迎回来，${res.data?.nickname}！`,
                    });
                    navigate("/");
                }

                if (res.code === StatusCodes.BAD_REQUEST) {
                    toast.error("登录失败", {
                        id: "login",
                        description: "用户名或密码错误",
                    });
                }

                if (res.code === StatusCodes.GONE) {
                    toast.error("Captcha 已失效", {
                        id: "login",
                        description: "请刷新 Captcha",
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
                                <FormLabel>用户名/邮箱</FormLabel>
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
                                <FormLabel>密码</FormLabel>
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
                                忘记密码
                            </Link>
                        </div>
                    )}
                    {configStore?.config?.captcha?.provider !== "none" && (
                        <FormField
                            name={"captcha"}
                            render={({ field }) => (
                                <FormItem>
                                    <FormLabel>验证码</FormLabel>
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
                    登录
                </Button>
            </form>
        </Form>
    );
}

export { LoginForm };
