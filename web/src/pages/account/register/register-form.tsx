import {
    UserRound,
    Lock,
    Check,
    TypeIcon,
    MailIcon,
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
import { register } from "@/api/users";
import { toast } from "sonner";
import { useNavigate } from "react-router";
import { useConfigStore } from "@/storages/config";

function RegisterForm() {
    const configStore = useConfigStore();
    const navigate = useNavigate();

    const [loading, setLoading] = useState<boolean>(false);

    const formSchema = z
        .object({
            username: z
                .string({
                    message: "请输入用户名",
                })
                .regex(/^[a-z]/, "用户名必须以小写字母开头")
                .regex(/^[a-z0-9]*$/, "用户名只能包含小写字母和数字"),
            nickname: z.string({
                message: "请输入昵称",
            }),
            email: z
                .string({
                    message: "请输入邮箱",
                })
                .email("邮箱格式不合法"),
            password: z
                .string({
                    message: "请输入密码",
                })
                .min(6, "密码最少需要 6 个字符"),
            confirm_password: z.string({
                message: "请重新输入新密码",
            }),
        })
        .refine((data) => data.password === data.confirm_password, {
            message: "新密码与确认密码不一致",
            path: ["confirm_password"],
        });

    const [captcha, setCaptcha] = useState<{
        id?: string;
        content?: string;
    }>();

    const form = useForm<z.infer<typeof formSchema>>({
        resolver: zodResolver(formSchema),
    });

    function onSubmit(values: z.infer<typeof formSchema>) {
        setLoading(true);
        register({
            captcha,
            ...values,
        })
            .then((res) => {
                if (res.code === 200) {
                    toast.success("注册成功", {
                        id: "register-success",
                        description: "注册成功，请登录",
                    });
                    navigate("/account/login");
                }

                if (res.code === 400) {
                    toast.success("注册失败", {
                        id: "register-error",
                        description: res.msg,
                    });
                }

                if (res.code === 409) {
                    toast.success("注册失败", {
                        id: "register-error",
                        description: "用户名或邮箱重复",
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
                    <div className={cn(["flex", "gap-3", "items-center"])}>
                        <FormField
                            control={form.control}
                            name={"username"}
                            render={({ field }) => (
                                <FormItem className={cn(["flex-1"])}>
                                    <FormLabel>用户名</FormLabel>
                                    <FormControl>
                                        <Field>
                                            <FieldIcon>
                                                <UserRoundIcon />
                                            </FieldIcon>
                                            <TextField {...field} />
                                        </Field>
                                    </FormControl>
                                    <FormMessage />
                                </FormItem>
                            )}
                        />
                        <FormField
                            control={form.control}
                            name={"nickname"}
                            render={({ field }) => (
                                <FormItem className={cn(["flex-1"])}>
                                    <FormLabel>昵称</FormLabel>
                                    <FormControl>
                                        <Field>
                                            <FieldIcon>
                                                <TypeIcon />
                                            </FieldIcon>
                                            <TextField {...field} />
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
                                <FormLabel>邮箱</FormLabel>
                                <FormControl>
                                    <Field>
                                        <FieldIcon>
                                            <MailIcon />
                                        </FieldIcon>
                                        <TextField {...field} />
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
                                            type={"password"}
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
                        name={"confirm_password"}
                        render={({ field }) => (
                            <FormItem>
                                <FormLabel>确认密码</FormLabel>
                                <FormControl>
                                    <Field>
                                        <FieldIcon>
                                            <LockIcon />
                                        </FieldIcon>
                                        <TextField
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
                            render={() => (
                                <FormItem>
                                    <FormLabel>验证码</FormLabel>
                                    <Captcha onChange={setCaptcha} />
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
                    注册
                </Button>
            </form>
        </Form>
    );
}

export { RegisterForm };
