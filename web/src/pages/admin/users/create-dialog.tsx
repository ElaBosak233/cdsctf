import { createUser } from "@/api/admin/users";
import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";
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
import { Select } from "@/components/ui/select";
import { Group } from "@/models/user";
import { useSharedStore } from "@/storages/shared";
import { cn } from "@/utils";
import { zodResolver } from "@hookform/resolvers/zod";
import {
    CheckIcon,
    UserRoundIcon,
    MailIcon,
    UserRoundCheckIcon,
    KeyIcon,
    UserRoundPlusIcon,
    ShieldIcon,
} from "lucide-react";
import { useState } from "react";
import { useForm } from "react-hook-form";
import { toast } from "sonner";
import { z } from "zod";

interface CreateUserDialogProps {
    onClose: () => void;
}

function CreateUserDialog(props: CreateUserDialogProps) {
    const { onClose } = props;

    const sharedStore = useSharedStore();

    const [loading, setLoading] = useState<boolean>(false);

    const formSchema = z.object({
        username: z.string().min(3, { message: "用户名至少3个字符!" }),
        nickname: z.string().min(2, { message: "昵称至少2个字符!" }),
        email: z.string().email({ message: "请输入有效的邮箱地址!" }),
        password: z.string().min(6, { message: "密码至少6个字符!" }),
        group: z.number({
            message: "请选择用户组!",
        }),
    });

    const form = useForm<z.infer<typeof formSchema>>({
        resolver: zodResolver(formSchema),
        defaultValues: {
            group: Group.User,
            username: "",
            nickname: "",
            email: "",
            password: "",
        },
    });

    function onSubmit(values: z.infer<typeof formSchema>) {
        setLoading(true);
        createUser({
            ...values,
        })
            .then((res) => {
                if (res.code === 200) {
                    toast.success(`用户 ${values.username} 创建成功`);
                    onClose();
                }

                if (res.code === 409) {
                    toast.error("发生错误", {
                        description: res.msg,
                    });
                }
            })
            .finally(() => {
                sharedStore.setRefresh();
                setLoading(false);
            });
    }

    const groupOptions = [
        { id: Group.User, name: "用户", icon: UserRoundCheckIcon },
        { id: Group.Admin, name: "管理员", icon: ShieldIcon },
    ];

    return (
        <Card
            className={cn([
                "w-128",
                "min-h-64",
                "p-5",
                "flex",
                "flex-col",
                "gap-5",
            ])}
        >
            <h3 className={cn(["flex", "gap-3", "items-center", "text-md"])}>
                <UserRoundPlusIcon className={cn(["size-4"])} />
                创建用户
            </h3>
            <Form {...form}>
                <form
                    onSubmit={form.handleSubmit(onSubmit)}
                    autoComplete={"off"}
                    className={cn(["flex", "flex-col", "flex-1", "gap-5"])}
                >
                    <FormField
                        control={form.control}
                        name={"username"}
                        render={({ field }) => (
                            <FormItem>
                                <FormLabel>用户名</FormLabel>
                                <FormControl>
                                    <Field size={"sm"}>
                                        <FieldIcon>
                                            <UserRoundIcon />
                                        </FieldIcon>
                                        <TextField
                                            {...field}
                                            placeholder={"请输入用户名"}
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
                        name={"nickname"}
                        render={({ field }) => (
                            <FormItem>
                                <FormLabel>昵称</FormLabel>
                                <FormControl>
                                    <Field size={"sm"}>
                                        <FieldIcon>
                                            <UserRoundCheckIcon />
                                        </FieldIcon>
                                        <TextField
                                            {...field}
                                            placeholder={"请输入昵称"}
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
                        name={"email"}
                        render={({ field }) => (
                            <FormItem>
                                <FormLabel>邮箱</FormLabel>
                                <FormControl>
                                    <Field size={"sm"}>
                                        <FieldIcon>
                                            <MailIcon />
                                        </FieldIcon>
                                        <TextField
                                            {...field}
                                            type="email"
                                            placeholder={"请输入邮箱地址"}
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
                            <FormItem>
                                <FormLabel>密码</FormLabel>
                                <FormControl>
                                    <Field size={"sm"}>
                                        <FieldIcon>
                                            <KeyIcon />
                                        </FieldIcon>
                                        <TextField
                                            {...field}
                                            type="password"
                                            placeholder={"请输入密码"}
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
                        name={"group"}
                        render={({ field }) => (
                            <FormItem>
                                <FormLabel>用户组</FormLabel>
                                <FormControl>
                                    <Field size={"sm"}>
                                        <FieldIcon>
                                            <UserRoundCheckIcon />
                                        </FieldIcon>
                                        <Select
                                            {...field}
                                            options={groupOptions.map(
                                                (group) => {
                                                    const Icon = group.icon;
                                                    return {
                                                        value: String(group.id),
                                                        content: (
                                                            <div
                                                                className={cn([
                                                                    "flex",
                                                                    "gap-2",
                                                                    "items-center",
                                                                ])}
                                                            >
                                                                <Icon className="size-4" />
                                                                {group.name}
                                                            </div>
                                                        ),
                                                    };
                                                }
                                            )}
                                            onValueChange={(value) => {
                                                field.onChange(Number(value));
                                            }}
                                            value={String(field.value)}
                                        />
                                    </Field>
                                </FormControl>
                                <FormMessage />
                            </FormItem>
                        )}
                    />
                    <Button
                        type={"submit"}
                        variant={"solid"}
                        icon={CheckIcon}
                        level={"success"}
                        loading={loading}
                    >
                        确定
                    </Button>
                </form>
            </Form>
        </Card>
    );
}

export { CreateUserDialog };
