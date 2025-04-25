import { updateUser } from "@/api/admin/users/user_id";
import { Button } from "@/components/ui/button";
import {
    Form,
    FormControl,
    FormField,
    FormItem,
    FormLabel,
    FormMessage,
} from "@/components/ui/form";
import { Field, FieldIcon } from "@/components/ui/field";
import { TextField } from "@/components/ui/text-field";
import { cn } from "@/utils";
import { zodResolver } from "@hookform/resolvers/zod";
import { LockIcon, SaveIcon } from "lucide-react";
import { useContext, useState } from "react";
import { useForm } from "react-hook-form";
import { toast } from "sonner";
import { z } from "zod";
import { Context } from "../context";

export default function Index() {
    const { user } = useContext(Context);
    const [loading, setLoading] = useState<boolean>(false);

    const formSchema = z
        .object({
            new_password: z
                .string({
                    message: "请输入新密码",
                })
                .min(6, "密码最少需要 6 个字符"),
            confirm_password: z.string({
                message: "请重新输入新密码",
            }),
        })
        .refine((data) => data.new_password === data.confirm_password, {
            message: "新密码与确认密码不一致",
            path: ["confirm_password"],
        });

    const form = useForm<z.infer<typeof formSchema>>({
        resolver: zodResolver(formSchema),
        defaultValues: {
            new_password: "",
            confirm_password: "",
        },
    });

    function onSubmit(values: z.infer<typeof formSchema>) {
        setLoading(true);
        updateUser({
            id: user?.id!,
            password: values.new_password,
        })
            .then((res) => {
                if (res.code === 200) {
                    toast.success(`用户 ${user?.username} 密码更新成功`);
                    form.reset();
                }

                if (res.code === 400) {
                    toast.error("更新失败", {
                        description: res.msg,
                    });
                }
            })
            .finally(() => {
                setLoading(false);
            });
    }

    return (
        <div className={cn(["flex", "flex-col", "flex-1"])}>
            <Form {...form}>
                <form
                    onSubmit={form.handleSubmit(onSubmit)}
                    autoComplete={"off"}
                    className={cn(["flex", "flex-col", "flex-1", "gap-8"])}
                >
                    <FormField
                        control={form.control}
                        name={"new_password"}
                        render={({ field }) => (
                            <FormItem>
                                <FormLabel>新密码</FormLabel>
                                <FormControl>
                                    <Field>
                                        <FieldIcon>
                                            <LockIcon />
                                        </FieldIcon>
                                        <TextField
                                            {...field}
                                            type={"password"}
                                            placeholder={"新密码"}
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
                                            {...field}
                                            type={"password"}
                                            placeholder={"确认密码"}
                                            value={field.value || ""}
                                            onChange={field.onChange}
                                        />
                                    </Field>
                                </FormControl>
                                <FormMessage />
                            </FormItem>
                        )}
                    />

                    <Button
                        size={"lg"}
                        type={"submit"}
                        variant={"solid"}
                        icon={SaveIcon}
                        loading={loading}
                    >
                        保存
                    </Button>
                </form>
            </Form>
        </div>
    );
}
