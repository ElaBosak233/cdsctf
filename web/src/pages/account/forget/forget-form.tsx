import { Lock, Check, MailIcon, SendIcon, LockIcon } from "lucide-react";
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
import { toast } from "sonner";
import { useAuthStore } from "@/storages/auth";
import { useNavigate } from "react-router";
import { useConfigStore } from "@/storages/config";
import { forget, sendForgetEmail } from "@/api/users/forget";

function ForgetForm() {
    const configStore = useConfigStore();
    const authStore = useAuthStore();
    const navigate = useNavigate();

    const [loading, setLoading] = useState<boolean>(false);

    const formSchema = z.object({
        email: z
            .string({
                message: "请输入邮箱",
            })
            .email(),
        code: z.string({
            message: "请输入验证码",
        }),
        password: z.string({
            message: "请输入新密码",
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
        forget({
            ...values,
        })
            .then((res) => {
                if (res.code === 200) {
                    authStore.setUser(res.data);
                    toast.success("密码重置成功", {
                        description: "请登录",
                    });
                    navigate("/account/login");
                }

                if (res.code === 400) {
                    toast.error("发生错误", {
                        description: res.msg,
                    });
                }
            })
            .finally(() => {
                setLoading(false);
            });
    }

    function handleSendForgetEmail() {
        sendForgetEmail({
            email: form.getValues().email,
        }).then((res) => {
            if (res.code === 200) {
                toast.success("验证码已发送，请查收");
            }

            if (res.code === 400) {
                toast.error("发生错误", {
                    description: res.msg,
                });
            }

            if (res.code === 404) {
                toast.error("邮箱不存在");
            }
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
                        name={"email"}
                        render={({ field }) => (
                            <FormItem>
                                <FormLabel>邮箱</FormLabel>
                                <FormControl>
                                    <Field>
                                        <FieldIcon>
                                            <MailIcon />
                                        </FieldIcon>
                                        <TextField
                                            placeholder={"Email"}
                                            {...field}
                                        />
                                    </Field>
                                </FormControl>
                                <FormMessage />
                            </FormItem>
                        )}
                    />
                    <div className={cn(["flex", "gap-2", "items-center"])}>
                        <FormField
                            control={form.control}
                            name={"code"}
                            render={({ field }) => (
                                <FormItem className={cn(["flex-1"])}>
                                    <FormLabel>验证码</FormLabel>
                                    <FormControl>
                                        <Field>
                                            <FieldIcon>
                                                <MailIcon />
                                            </FieldIcon>
                                            <TextField
                                                placeholder={"Code"}
                                                {...field}
                                            />
                                        </Field>
                                    </FormControl>
                                    <FormMessage />
                                </FormItem>
                            )}
                        />
                        <Button
                            variant={"solid"}
                            icon={SendIcon}
                            className={cn(["mt-6"])}
                            onClick={handleSendForgetEmail}
                        >
                            请求
                        </Button>
                    </div>
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
                    level={"info"}
                    type={"submit"}
                    size={"lg"}
                    className={cn(["w-full"])}
                    icon={Check}
                    loading={loading}
                >
                    重置密码
                </Button>
            </form>
        </Form>
    );
}

export { ForgetForm };
