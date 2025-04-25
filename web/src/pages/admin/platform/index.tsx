import { getConfigs, updateConfig } from "@/api/admin/configs";
import { deleteLogo } from "@/api/admin/configs/logo";
import { Button } from "@/components/ui/button";
import {
    Dropzone,
    DropZoneArea,
    DropzoneTrigger,
    useDropzone,
} from "@/components/ui/dropzone";
import { Image } from "@/components/ui/image";
import {
    Form,
    FormControl,
    FormDescription,
    FormField,
    FormItem,
    FormLabel,
    FormMessage,
} from "@/components/ui/form";
import { Field, FieldIcon } from "@/components/ui/field";
import { TextField } from "@/components/ui/text-field";
import { Separator } from "@/components/ui/separator";
import { cn } from "@/utils";
import { zodResolver } from "@hookform/resolvers/zod";
import {
    BadgeCheckIcon,
    BotIcon,
    ClockIcon,
    InfoIcon,
    LockIcon,
    SaveIcon,
    SendIcon,
    TextIcon,
    TrashIcon,
    TypeIcon,
    UserRoundCheckIcon,
} from "lucide-react";
import { useEffect, useState } from "react";
import { useForm } from "react-hook-form";
import { toast } from "sonner";
import { z } from "zod";
import { Label } from "@/components/ui/label";
import { Config } from "@/models/config";
import { Select } from "@/components/ui/select";
import { NumberField } from "@/components/ui/number-field";

export default function Index() {
    const [refresh, setRefresh] = useState<number>(0);
    const [config, setConfig] = useState<Config>();

    useEffect(() => {
        getConfigs().then((res) => {
            setConfig(res.data);
        });
    }, []);

    const formSchema = z.object({
        meta: z
            .object({
                title: z.string().optional(),
                description: z.string().optional(),
                keywords: z.string().optional(),
            })
            .optional(),
        auth: z
            .object({
                is_registration_enabled: z.boolean().optional(),
            })
            .optional(),
        captcha: z
            .object({
                provider: z
                    .enum(["none", "pow", "image", "turnstile", "hcaptcha"])
                    .optional(),
                difficulty: z.number().default(1).optional(),
                turnstile: z
                    .object({
                        url: z.string().default("").optional(),
                        site_key: z.string().default("").optional(),
                        secret_key: z.string().default("").optional(),
                    })

                    .optional(),
                hcaptcha: z
                    .object({
                        url: z.string().default("").optional(),
                        site_key: z.string().default("").optional(),
                        secret_key: z.string().default("").optional(),
                        score: z.number().default(0).optional(),
                    })
                    .optional(),
            })
            .optional(),
        email: z
            .object({
                is_enabled: z.boolean().default(false).optional(),
                host: z.string().default("").optional(),
                port: z.number().default(587).optional(),
                tls: z.enum(["starttls", "tls", "none"]).optional(),
                username: z.string().default("").optional(),
                password: z.string().default("").optional(),
                whitelist: z.array(z.string()).default([]).optional(),
            })
            .default({})
            .optional(),
    });

    const form = useForm<z.infer<typeof formSchema>>({
        resolver: zodResolver(formSchema),
        defaultValues: config,
    });

    useEffect(() => {
        form.reset(config, {
            keepDefaultValues: false,
        });
    }, [config, form.reset]);

    function onSubmit(values: z.infer<typeof formSchema>) {
        updateConfig({
            ...config,
            ...values,
        }).then((res) => {
            if (res.code === 200) {
                toast.success("平台配置更新成功");
            }
        });
    }

    const iconDropzone = useDropzone({
        onDropFile: async (file) => {
            const formData = new FormData();
            formData.append("file", file);

            const xhr = new XMLHttpRequest();
            xhr.open("POST", "/api/admin/configs/logo", true);

            xhr.upload.onprogress = (event) => {
                if (event.lengthComputable) {
                    const percentComplete = (event.loaded / event.total) * 100;
                    toast.loading(`上传进度 ${percentComplete.toFixed(0)}%`, {
                        id: "logo-upload",
                    });
                }
            };

            xhr.onload = () => {
                if (xhr.status === 200) {
                    toast.success("Logo 上传成功", {
                        id: "logo-upload",
                    });
                    setRefresh((prev) => prev + 1);
                } else {
                    toast.error("Logo 上传失败", {
                        id: "logo-upload",
                        description: xhr.responseText,
                    });
                }
            };

            xhr.onerror = () => {
                toast.error("Logo 上传失败", {
                    id: "logo-upload",
                    description: "网络错误",
                });
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
        validation: {
            accept: {
                "image/*": [".png", ".jpg", ".jpeg", ".webp"],
            },
            maxSize: 3 * 1024 * 1024,
            maxFiles: 1,
        },
    });

    function handleLogoDelete() {
        deleteLogo()
            .then((res) => {
                if (res.code === 200) {
                    toast.success(`图标删除成功`);
                }
            })
            .finally(() => {
                setRefresh((prev) => prev + 1);
            });
    }

    return (
        <Form {...form}>
            <form
                onSubmit={form.handleSubmit(onSubmit)}
                autoComplete={"off"}
                className={cn([
                    "flex",
                    "flex-col",
                    "gap-3",
                    "p-10",
                    "xl:mx-60",
                    "lg:mx-30",
                    "min-h-[calc(100vh-64px)]",
                ])}
            >
                <h2
                    className={cn([
                        "flex",
                        "gap-2",
                        "items-center",
                        "text-xl",
                        "mt-2",
                    ])}
                >
                    <InfoIcon />
                    元信息
                </h2>
                <Separator />
                <div className={cn(["flex", "gap-3"])}>
                    <div
                        className={cn(["flex", "flex-col", "gap-3", "flex-1"])}
                    >
                        <FormField
                            control={form.control}
                            name={"meta.title"}
                            render={({ field }) => (
                                <FormItem className={cn(["w-full"])}>
                                    <FormLabel>标题</FormLabel>
                                    <FormControl>
                                        <Field>
                                            <FieldIcon>
                                                <TypeIcon />
                                            </FieldIcon>
                                            <TextField
                                                {...field}
                                                placeholder="请输入标题"
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
                            name={"meta.description"}
                            render={({ field }) => (
                                <FormItem className={cn(["w-full"])}>
                                    <FormLabel>描述</FormLabel>
                                    <FormControl>
                                        <Field>
                                            <FieldIcon>
                                                <TextIcon />
                                            </FieldIcon>
                                            <TextField
                                                {...field}
                                                placeholder="请输入描述"
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
                            <Label>图标</Label>
                            <Button
                                icon={TrashIcon}
                                size={"sm"}
                                level={"error"}
                                square
                                onClick={handleLogoDelete}
                            />
                        </div>
                        <Dropzone {...iconDropzone}>
                            <DropZoneArea
                                className={cn([
                                    "relative",
                                    "aspect-square",
                                    "h-28",
                                    "p-0",
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
                                    <Image
                                        src={`/api/configs/logo?r=${refresh}`}
                                        className={cn([
                                            "object-cover",
                                            "rounded-md",
                                            "overflow-hidden",
                                            "aspect-square",
                                            "w-full",
                                            "select-none",
                                        ])}
                                    />
                                </DropzoneTrigger>
                            </DropZoneArea>
                        </Dropzone>
                    </div>
                </div>
                <h2
                    className={cn([
                        "flex",
                        "gap-2",
                        "items-center",
                        "text-xl",
                        "mt-2",
                    ])}
                >
                    <BadgeCheckIcon />
                    用户策略
                </h2>
                <Separator />
                <FormField
                    control={form.control}
                    name={"auth.is_registration_enabled"}
                    render={({ field }) => (
                        <FormItem className={cn(["w-full"])}>
                            <FormLabel>是否允许新用户注册</FormLabel>
                            <FormControl>
                                <Field>
                                    <FieldIcon>
                                        <UserRoundCheckIcon />
                                    </FieldIcon>
                                    <Select
                                        {...field}
                                        options={[
                                            {
                                                value: String(true),
                                                content: "允许",
                                            },
                                            {
                                                value: String(false),
                                                content: "不允许",
                                            },
                                        ]}
                                        onValueChange={(value) =>
                                            field.onChange(value === "true")
                                        }
                                        value={String(field.value)}
                                    />
                                </Field>
                            </FormControl>
                            <FormMessage />
                        </FormItem>
                    )}
                />
                <h2
                    className={cn([
                        "flex",
                        "gap-2",
                        "items-center",
                        "text-xl",
                        "mt-2",
                    ])}
                >
                    <BotIcon />
                    人机验证
                </h2>
                <Separator />
                <div className={cn(["flex", "gap-3"])}>
                    <FormField
                        control={form.control}
                        name={"captcha.provider"}
                        render={({ field }) => (
                            <FormItem className={cn(["w-full"])}>
                                <FormLabel>提供方</FormLabel>
                                <FormDescription>
                                    若启用，则在必要的界面中启用人机验证。
                                </FormDescription>
                                <FormControl>
                                    <Field>
                                        <FieldIcon>
                                            <LockIcon />
                                        </FieldIcon>
                                        <Select
                                            {...field}
                                            options={[
                                                {
                                                    value: "none",
                                                    content: "不启用",
                                                },
                                                {
                                                    value: "pow",
                                                    content: "工作量验证",
                                                },
                                                {
                                                    value: "image",
                                                    content: "图形验证",
                                                },
                                                {
                                                    value: "turnstile",
                                                    content:
                                                        "Cloudflare Trunstile",
                                                },
                                                {
                                                    value: "hcaptcha",
                                                    content: "HCaptcha",
                                                },
                                            ]}
                                            onValueChange={(value) =>
                                                field.onChange(value)
                                            }
                                            value={String(field.value)}
                                        />
                                    </Field>
                                </FormControl>
                                <FormMessage />
                            </FormItem>
                        )}
                    />
                    <FormField
                        control={form.control}
                        name={"captcha.difficulty"}
                        render={({ field }) => (
                            <FormItem className={cn(["w-full"])}>
                                <FormLabel>难度</FormLabel>
                                <FormDescription>
                                    适用于图形验证和工作量验证。
                                </FormDescription>
                                <FormControl>
                                    <Field>
                                        <FieldIcon>
                                            <ClockIcon />
                                        </FieldIcon>
                                        <NumberField
                                            {...field}
                                            placeholder="请输入难度"
                                            value={field.value}
                                            onValueChange={(value) =>
                                                field.onChange(value)
                                            }
                                        />
                                    </Field>
                                </FormControl>
                                <FormMessage />
                            </FormItem>
                        )}
                    />
                </div>
                {form.watch("captcha.provider") === "turnstile" && (
                    <>
                        <FormField
                            control={form.control}
                            name={"captcha.turnstile.url"}
                            render={({ field }) => (
                                <FormItem className={cn(["w-full"])}>
                                    <FormLabel>API URL</FormLabel>
                                    <FormControl>
                                        <Field>
                                            <FieldIcon>
                                                <SendIcon />
                                            </FieldIcon>
                                            <TextField
                                                {...field}
                                                placeholder="请输入 API URL"
                                                value={field.value || ""}
                                                onChange={field.onChange}
                                            />
                                        </Field>
                                    </FormControl>
                                    <FormMessage />
                                </FormItem>
                            )}
                        />
                        <div className={cn(["flex", "gap-3"])}>
                            <FormField
                                control={form.control}
                                name={"captcha.turnstile.site_key"}
                                render={({ field }) => (
                                    <FormItem className={cn(["w-full"])}>
                                        <FormLabel>SITE_KEY</FormLabel>
                                        <FormControl>
                                            <Field>
                                                <FieldIcon>
                                                    <SendIcon />
                                                </FieldIcon>
                                                <TextField
                                                    {...field}
                                                    placeholder="请输入 SITE_KEY"
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
                                name={"captcha.turnstile.secret_key"}
                                render={({ field }) => (
                                    <FormItem className={cn(["w-full"])}>
                                        <FormLabel>SECRET_KEY</FormLabel>
                                        <FormControl>
                                            <Field>
                                                <FieldIcon>
                                                    <SendIcon />
                                                </FieldIcon>
                                                <TextField
                                                    {...field}
                                                    placeholder="请输入 SECRET_KEY"
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
                    </>
                )}
                {form.watch("captcha.provider") === "hcaptcha" && (
                    <>
                        <div className={cn(["flex", "gap-3"])}>
                            <FormField
                                control={form.control}
                                name={"captcha.hcaptcha.url"}
                                render={({ field }) => (
                                    <FormItem className={cn(["w-full"])}>
                                        <FormLabel>API URL</FormLabel>
                                        <FormControl>
                                            <Field>
                                                <FieldIcon>
                                                    <SendIcon />
                                                </FieldIcon>
                                                <TextField
                                                    {...field}
                                                    placeholder="请输入 API URL"
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
                                name={"captcha.hcaptcha.score"}
                                render={({ field }) => (
                                    <FormItem className={cn(["w-full"])}>
                                        <FormLabel>分数要求</FormLabel>
                                        <FormControl>
                                            <Field>
                                                <FieldIcon>
                                                    <ClockIcon />
                                                </FieldIcon>
                                                <TextField
                                                    {...field}
                                                    type={"number"}
                                                    placeholder="请输入分数要求"
                                                    value={field.value || ""}
                                                    onChange={(e) =>
                                                        field.onChange(
                                                            e.target
                                                                .valueAsNumber
                                                        )
                                                    }
                                                />
                                            </Field>
                                        </FormControl>
                                        <FormMessage />
                                    </FormItem>
                                )}
                            />
                        </div>
                        <div className={cn(["flex", "gap-3"])}>
                            <FormField
                                control={form.control}
                                name={"captcha.hcaptcha.site_key"}
                                render={({ field }) => (
                                    <FormItem className={cn(["w-full"])}>
                                        <FormLabel>SITE_KEY</FormLabel>
                                        <FormControl>
                                            <Field>
                                                <FieldIcon>
                                                    <SendIcon />
                                                </FieldIcon>
                                                <TextField
                                                    {...field}
                                                    placeholder="请输入 SITE_KEY"
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
                                name={"captcha.hcaptcha.secret_key"}
                                render={({ field }) => (
                                    <FormItem className={cn(["w-full"])}>
                                        <FormLabel>SECRET_KEY</FormLabel>
                                        <FormControl>
                                            <Field>
                                                <FieldIcon>
                                                    <SendIcon />
                                                </FieldIcon>
                                                <TextField
                                                    {...field}
                                                    placeholder={
                                                        "请输入 SECRET_KEY"
                                                    }
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
                    </>
                )}
                <div className={cn(["flex-1"])} />
                <Button
                    type={"submit"}
                    variant={"solid"}
                    size={"lg"}
                    icon={SaveIcon}
                    className={cn(["mt-2"])}
                >
                    保存
                </Button>
            </form>
        </Form>
    );
}
