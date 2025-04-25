import { updateUserProfile } from "@/api/users/profile";
import { deleteUserAvatar } from "@/api/users/profile/avatar";
import { Alert } from "@/components/ui/alert";
import { Avatar } from "@/components/ui/avatar";
import { Button } from "@/components/ui/button";
import { Dialog, DialogContent } from "@/components/ui/dialog";
import {
    Dropzone,
    DropZoneArea,
    DropzoneTrigger,
    useDropzone,
} from "@/components/ui/dropzone";
import { Editor } from "@/components/ui/editor";
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
import { Label } from "@/components/ui/label";
import { useAuthStore } from "@/storages/auth";
import { useSharedStore } from "@/storages/shared";
import { cn } from "@/utils";
import { zodResolver } from "@hookform/resolvers/zod";
import {
    MailIcon,
    SaveIcon,
    TrashIcon,
    TypeIcon,
    UserRoundIcon,
} from "lucide-react";
import { useState } from "react";
import { useForm } from "react-hook-form";
import { toast } from "sonner";
import { z } from "zod";
import { EmailVerifyDialog } from "./email-verify-dialog";

export default function Index() {
    const authStore = useAuthStore();
    const sharedStore = useSharedStore();
    const [loading, setLoading] = useState<boolean>(false);

    const [emailVerifyDialogOpen, setEmailVerifyDialogOpen] =
        useState<boolean>(false);

    const formSchema = z.object({
        username: z.string().nullish(),
        nickname: z.string({
            message: "请输入昵称",
        }),
        email: z.string().email({
            message: "请输入合法的邮箱",
        }),
        description: z.string().nullish(),
    });

    const form = useForm<z.infer<typeof formSchema>>({
        resolver: zodResolver(formSchema),
        defaultValues: authStore?.user,
    });

    function onSubmit(values: z.infer<typeof formSchema>) {
        setLoading(true);
        updateUserProfile({
            ...values,
        })
            .then((res) => {
                if (res.code === 200) {
                    authStore?.setUser(res.data);
                    toast.success("个人资料更新成功");
                }
            })
            .finally(() => {
                setLoading(false);
            });
    }

    const dropzone = useDropzone({
        validation: {
            accept: {
                "image/*": [".png", ".jpg", ".jpeg", ".webp"],
            },
            maxSize: 3 * 1024 * 1024,
            maxFiles: 1,
        },
        onDropFile: async (file) => {
            const formData = new FormData();
            formData.append("file", file);
            const xhr = new XMLHttpRequest();
            xhr.open("POST", `/api/users/profile/avatar`, true);
            xhr.upload.onprogress = (event) => {
                if (event.lengthComputable) {
                    const percentComplete = (event.loaded / event.total) * 100;
                    toast.loading(`上传进度 ${percentComplete}%`, {
                        id: "user-avatar-upload",
                    });
                }
            };
            xhr.onload = () => {
                if (xhr.status === 200) {
                    toast.success("头像上传成功", {
                        id: "user-avatar-upload",
                    });
                    sharedStore?.setRefresh();
                } else {
                    toast.error("头像上传失败", {
                        id: "user-avatar-upload",
                        description: xhr.responseText,
                    });
                }
            };
            xhr.onerror = () => {
                return {
                    status: "error",
                };
            };

            xhr.send(formData);

            return {
                status: "success",
                result: "",
            };
        },
    });

    function handleAvatarDelete() {
        deleteUserAvatar({
            user_id: authStore?.user?.id!,
        })
            .then((res) => {
                if (res.code === 200) {
                    toast.success(`头像删除成功`);
                }
            })
            .finally(() => {
                sharedStore.setRefresh();
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
            ])}
        >
            <Form {...form}>
                <form
                    onSubmit={form.handleSubmit(onSubmit)}
                    autoComplete={"off"}
                    className={cn(["flex", "flex-col", "flex-1", "gap-8"])}
                >
                    <div className={cn(["flex", "gap-5", "items-center"])}>
                        <div
                            className={cn([
                                "flex",
                                "flex-col",
                                "gap-8",
                                "flex-1",
                            ])}
                        >
                            <FormField
                                control={form.control}
                                name={"username"}
                                render={({ field }) => (
                                    <FormItem>
                                        <FormLabel>用户名</FormLabel>
                                        <FormControl>
                                            <Field>
                                                <FieldIcon>
                                                    <UserRoundIcon />
                                                </FieldIcon>
                                                <TextField
                                                    {...field}
                                                    disabled
                                                    placeholder={"用户名"}
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
                                            <Field>
                                                <FieldIcon>
                                                    <TypeIcon />
                                                </FieldIcon>
                                                <TextField
                                                    {...field}
                                                    placeholder={"昵称"}
                                                    value={field.value || ""}
                                                    onChange={field.onChange}
                                                />
                                            </Field>
                                        </FormControl>
                                        <FormMessage />
                                    </FormItem>
                                )}
                            />
                        </div>
                        <div>
                            <div
                                className={cn([
                                    "flex",
                                    "gap-3",
                                    "items-center",
                                    "justify-between",
                                ])}
                            >
                                <Label>头像</Label>
                                <Button
                                    type={"button"}
                                    icon={TrashIcon}
                                    size={"sm"}
                                    level={"error"}
                                    square
                                    onClick={handleAvatarDelete}
                                />
                            </div>
                            <Dropzone {...dropzone}>
                                <DropZoneArea
                                    className={cn([
                                        "relative",
                                        "aspect-square",
                                        "h-36",
                                        "p-0",
                                        "rounded-full",
                                        "overflow-hidden",
                                    ])}
                                >
                                    <DropzoneTrigger
                                        className={cn([
                                            "bg-transparent",
                                            "text-center",
                                            "h-full",
                                            "aspect-square",
                                        ])}
                                    >
                                        <Avatar
                                            className={cn(["h-30", "w-30"])}
                                            src={`/api/users/${authStore?.user?.id}/avatar?refresh=${sharedStore?.refresh}`}
                                            fallback={authStore?.user?.username?.charAt(
                                                0
                                            )}
                                        />
                                    </DropzoneTrigger>
                                </DropZoneArea>
                            </Dropzone>
                        </div>
                    </div>
                    <FormField
                        control={form.control}
                        name={"email"}
                        render={({ field }) => (
                            <FormItem>
                                <FormLabel>电子邮箱</FormLabel>
                                <FormControl>
                                    <Field>
                                        <FieldIcon>
                                            <MailIcon />
                                        </FieldIcon>
                                        <TextField
                                            {...field}
                                            placeholder={"电子邮箱"}
                                            value={field.value || ""}
                                            onChange={field.onChange}
                                        />
                                    </Field>
                                </FormControl>
                                <FormMessage />
                            </FormItem>
                        )}
                    />
                    {!authStore?.user?.is_verified && (
                        <Alert
                            className={cn([
                                "flex",
                                "items-center",
                                "justify-between",
                            ])}
                        >
                            <p>你的邮箱 {authStore?.user?.email} 尚未验证！</p>
                            <Button
                                variant={"solid"}
                                size={"sm"}
                                className={cn(["h-8"])}
                                onClick={() => setEmailVerifyDialogOpen(true)}
                            >
                                去验证
                            </Button>
                            <Dialog
                                open={emailVerifyDialogOpen}
                                onOpenChange={setEmailVerifyDialogOpen}
                            >
                                <DialogContent>
                                    <EmailVerifyDialog
                                        onClose={() =>
                                            setEmailVerifyDialogOpen(false)
                                        }
                                    />
                                </DialogContent>
                            </Dialog>
                        </Alert>
                    )}
                    <FormField
                        control={form.control}
                        name={"description"}
                        render={({ field }) => (
                            <FormItem
                                className={cn(["flex-1", "flex", "flex-col"])}
                            >
                                <FormLabel>简介</FormLabel>
                                <FormControl>
                                    <Editor
                                        {...field}
                                        lang={"markdown"}
                                        tabSize={4}
                                        showLineNumbers
                                        className={cn(["h-full", "min-h-64"])}
                                        value={field.value || ""}
                                    />
                                </FormControl>
                                <FormMessage />
                            </FormItem>
                        )}
                    />
                    <Button
                        size={"lg"}
                        type={"submit"}
                        level={"primary"}
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
