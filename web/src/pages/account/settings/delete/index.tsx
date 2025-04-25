import { deleteUserProfile } from "@/api/users/profile";
import { Button } from "@/components/ui/button";
import {
    Form,
    FormField,
    FormItem,
    FormLabel,
    FormMessage,
} from "@/components/ui/form";
import { Field, FieldIcon } from "@/components/ui/field";
import { TextField } from "@/components/ui/text-field";
import { Separator } from "@/components/ui/separator";
import { Typography } from "@/components/ui/typography";
import { Captcha } from "@/components/widgets/captcha";
import { useAuthStore } from "@/storages/auth";
import { useConfigStore } from "@/storages/config";
import { cn } from "@/utils";
import { zodResolver } from "@hookform/resolvers/zod";
import {
    CheckCheckIcon,
    LockIcon,
    TriangleAlertIcon,
    UserRoundIcon,
} from "lucide-react";
import { useForm } from "react-hook-form";
import { useNavigate } from "react-router";
import { toast } from "sonner";
import { z } from "zod";

export default function Index() {
    const configStore = useConfigStore();
    const authStore = useAuthStore();
    const navigate = useNavigate();

    const formSchema = z
        .object({
            username: z.string({
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
        })
        .refine((data) => data.username === authStore?.user?.username, {
            message: "用户名不正确",
            path: ["username"],
        });

    const form = useForm<z.infer<typeof formSchema>>({
        resolver: zodResolver(formSchema),
    });

    function onSubmit(values: z.infer<typeof formSchema>) {
        deleteUserProfile({
            ...values,
        }).then((res) => {
            if (res.code === 200) {
                toast.success("注销成功");
                authStore?.clear();
                navigate("/");
            }

            if (res.code === 400) {
                toast.error("注销失败", {
                    description: res.msg,
                });
            }
        });
    }

    return (
        <div
            className={cn([
                "flex",
                "flex-col",
                "flex-1",
                "p-10",
                "xl:mx-50",
                "lg:mx-30",
                "gap-8",
            ])}
        >
            <Typography className={cn(["space-y-5"])}>
                <div
                    className={cn([
                        "flex",
                        "justify-center",
                        "items-center",
                        "text-warning",
                        "gap-3",
                    ])}
                >
                    <TriangleAlertIcon className={cn(["size-12"])} />
                    <span className={cn(["text-xl", "font-semibold"])}>
                        最后警告
                    </span>
                </div>
                <Separator />
                <p className={cn(["font-bold"])}>
                    你正在注销你的账号。但是你的账号不会被真正地删除，而是将永久处于软删除状态。且无法恢复！
                </p>
                <ul>
                    <li>
                        你的用户名会以一种特别的形式修改并保存。你可以使用原先的用户名注册新的账号。
                    </li>
                    <li>
                        你的邮箱也会以一种特别的形式修改并保存。你可以使用原先的邮箱注册新的账号。
                    </li>
                    <li>
                        所有与你相关的比赛、团队、提交等数据仍然会被留存。但在注销账号后，没有任何信息可以证明这些数据属于你。
                    </li>
                </ul>
                <p className={cn(["font-bold"])}>
                    请再次注意，你接下来的操作是不可逆的！请三思而后行！
                </p>
                <p className={cn(["text-error"])}>
                    请认真阅读并确定上述内容！如果你真的确定要注销账号，请在下方输入你的完整用户名、密码并点击确认。
                </p>
            </Typography>
            <Form {...form}>
                <form
                    onSubmit={form.handleSubmit(onSubmit)}
                    autoComplete={"off"}
                    className={cn(["flex", "flex-col", "gap-5"])}
                >
                    <div className={cn(["flex", "gap-5"])}>
                        <FormField
                            name={"username"}
                            render={({ field }) => (
                                <FormItem className={cn(["w-full"])}>
                                    <FormLabel>用户名</FormLabel>
                                    <Field>
                                        <FieldIcon>
                                            <UserRoundIcon />
                                        </FieldIcon>
                                        <TextField
                                            {...field}
                                            placeholder={
                                                authStore?.user?.username
                                            }
                                        />
                                    </Field>
                                    <FormMessage />
                                </FormItem>
                            )}
                        />
                        <FormField
                            name={"password"}
                            render={({ field }) => (
                                <FormItem className={cn(["w-full"])}>
                                    <FormLabel>密码</FormLabel>
                                    <Field>
                                        <FieldIcon>
                                            <LockIcon />
                                        </FieldIcon>
                                        <TextField
                                            {...field}
                                            type={"password"}
                                        />
                                    </Field>
                                    <FormMessage />
                                </FormItem>
                            )}
                        />
                    </div>
                    {configStore?.config?.captcha?.provider !== "none" && (
                        <FormField
                            name={"captcha"}
                            render={({ field }) => (
                                <FormItem className={cn(["w-full"])}>
                                    <FormLabel>验证码</FormLabel>
                                    <Captcha onChange={field.onChange} />
                                </FormItem>
                            )}
                        />
                    )}
                    <Button
                        variant={"solid"}
                        size={"lg"}
                        level={"error"}
                        icon={CheckCheckIcon}
                        type={"submit"}
                    >
                        确定
                    </Button>
                </form>
            </Form>
        </div>
    );
}
