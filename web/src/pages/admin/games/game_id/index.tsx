import { useContext, useEffect, useState } from "react";
import { Context } from "./context";
import { z } from "zod";
import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { Editor } from "@/components/ui/editor";
import { cn } from "@/utils";
import { Field, FieldIcon } from "@/components/ui/field";
import { TextField } from "@/components/ui/text-field";
import { toast } from "sonner";
import { useSharedStore } from "@/storages/shared";
import {
    CalendarIcon,
    ClockAlertIcon,
    ClockFadingIcon,
    ClockIcon,
    FileCheck2Icon,
    LockOpenIcon,
    SaveIcon,
    TrashIcon,
    Type,
    TypeIcon,
    UsersRoundIcon,
} from "lucide-react";
import {
    Form,
    FormControl,
    FormField,
    FormItem,
    FormLabel,
    FormMessage,
} from "@/components/ui/form";
import { Button } from "@/components/ui/button";
import { Image } from "@/components/ui/image";
import { Select } from "@/components/ui/select";
import { updateGame } from "@/api/admin/games/game_id";
import { DateTimePicker } from "@/components/ui/datetime-picker";
import {
    Dropzone,
    DropZoneArea,
    DropzoneTrigger,
    useDropzone,
} from "@/components/ui/dropzone";
import { Label } from "@/components/ui/label";
import { deleteGamePoster } from "@/api/admin/games/game_id/poster";
import { deleteGameIcon } from "@/api/admin/games/game_id/icon";

export default function Index() {
    const { game } = useContext(Context);
    const sharedStore = useSharedStore();

    const [loading, setLoading] = useState<boolean>(false);
    const [refresh, setRefresh] = useState<number>(0);

    const formSchema = z.object({
        title: z.string({
            message: "请输入标题",
        }),
        sketch: z
            .string({
                message: "请选择简述",
            })
            .nullish(),
        description: z
            .string({
                message: "请输入描述",
            })
            .nullish(),
        is_public: z.boolean({
            message: "请明确是否为公开赛",
        }),
        is_need_write_up: z.boolean({
            message: "请明确是否需要 Write-up",
        }),
        member_limit_min: z.number({
            message: "请提供团队人数最小值",
        }),
        member_limit_max: z.number({
            message: "请提供团队人数最大值",
        }),
        started_at: z.date({
            message: "请提供开始时间",
        }),
        frozen_at: z.date({
            message: "请提供冻结时间",
        }),
        ended_at: z.date({
            message: "请提供结束时间",
        }),
    });

    const form = useForm<z.infer<typeof formSchema>>({
        resolver: zodResolver(formSchema),
        defaultValues: {
            ...game,
            started_at: undefined,
            frozen_at: undefined,
            ended_at: undefined,
        },
    });

    useEffect(() => {
        form.reset(
            {
                ...game,
                started_at: new Date(Number(game?.started_at || 0) * 1000),
                frozen_at: new Date(Number(game?.frozen_at || 0) * 1000),
                ended_at: new Date(Number(game?.ended_at || 0) * 1000),
            },
            {
                keepDefaultValues: false,
            }
        );
    }, [game, form.reset]);

    function onSubmit(values: z.infer<typeof formSchema>) {
        setLoading(true);
        updateGame({
            ...values,
            id: game?.id,
            started_at: Math.floor(values.started_at?.getTime() / 1000),
            frozen_at: Math.floor(values.frozen_at?.getTime() / 1000),
            ended_at: Math.floor(values.ended_at?.getTime() / 1000),
        })
            .then((res) => {
                if (res.code === 200) {
                    toast.success(`比赛 ${res?.data?.title} 更新成功`);
                }
            })
            .finally(() => {
                sharedStore.setRefresh();
                setLoading(false);
            });
    }

    const posterDropzone = useDropzone({
        onDropFile: async (file) => {
            const formData = new FormData();
            formData.append("file", file);

            const xhr = new XMLHttpRequest();
            xhr.open("POST", `/api/admin/games/${game?.id}/poster`, true);

            xhr.upload.onprogress = (event) => {
                if (event.lengthComputable) {
                    const percentComplete = (event.loaded / event.total) * 100;
                    toast.loading(`上传进度 ${percentComplete.toFixed(0)}%`, {
                        id: "poster-upload",
                    });
                }
            };

            xhr.onload = () => {
                if (xhr.status === 200) {
                    toast.success("海报上传成功", {
                        id: "poster-upload",
                    });
                    setRefresh((prev) => prev + 1);
                } else {
                    toast.error("海报上传失败", {
                        id: "poster-upload",
                        description: xhr.responseText,
                    });
                }
            };

            xhr.onerror = () => {
                toast.error("海报上传失败", {
                    id: "poster-upload",
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

    function handlePosterDelete() {
        deleteGamePoster({
            game_id: game?.id!,
        })
            .then((res) => {
                if (res.code === 200) {
                    toast.success(`海报删除成功`);
                }
            })
            .finally(() => {
                setRefresh((prev) => prev + 1);
            });
    }

    const iconDropzone = useDropzone({
        onDropFile: async (file) => {
            const formData = new FormData();
            formData.append("file", file);

            const xhr = new XMLHttpRequest();
            xhr.open("POST", `/api/admin/games/${game?.id}/icon`, true);

            xhr.upload.onprogress = (event) => {
                if (event.lengthComputable) {
                    const percentComplete = (event.loaded / event.total) * 100;
                    toast.loading(`上传进度 ${percentComplete.toFixed(0)}%`, {
                        id: "icon-upload",
                    });
                }
            };

            xhr.onload = () => {
                if (xhr.status === 200) {
                    toast.success("图标上传成功", {
                        id: "icon-upload",
                    });
                    setRefresh((prev) => prev + 1);
                } else {
                    toast.error("图标上传失败", {
                        id: "icon-upload",
                        description: xhr.responseText,
                    });
                }
            };

            xhr.onerror = () => {
                toast.error("图标上传失败", {
                    id: "icon-upload",
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

    function handleIconDelete() {
        deleteGameIcon({
            game_id: game?.id!,
        })
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
                className={cn(["flex", "flex-col", "flex-1", "gap-8"])}
            >
                <div
                    className={cn([
                        "flex",
                        "gap-8",
                        "flex-col",
                        "xl:flex-row",
                        "items-center",
                    ])}
                >
                    <div
                        className={cn([
                            "flex",
                            "flex-col",
                            "flex-1",
                            "gap-8",
                            "w-full",
                        ])}
                    >
                        <FormField
                            control={form.control}
                            name={"title"}
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
                                                placeholder={"标题"}
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
                            name={"sketch"}
                            render={({ field }) => (
                                <FormItem className={cn(["w-full"])}>
                                    <FormLabel>简述</FormLabel>
                                    <FormControl>
                                        <Editor
                                            {...field}
                                            value={field.value || ""}
                                            lang={"markdown"}
                                            className={cn(["h-32"])}
                                        />
                                    </FormControl>
                                    <FormMessage />
                                </FormItem>
                            )}
                        />
                    </div>
                    <div className={cn(["flex", "gap-8"])}>
                        <div className={cn(["flex", "flex-col", "gap-1"])}>
                            <div
                                className={cn([
                                    "flex",
                                    "gap-3",
                                    "items-center",
                                    "justify-between",
                                ])}
                            >
                                <Label>海报</Label>
                                <Button
                                    type={"button"}
                                    icon={TrashIcon}
                                    size={"sm"}
                                    level={"error"}
                                    square
                                    onClick={handlePosterDelete}
                                />
                            </div>
                            <Dropzone {...posterDropzone}>
                                <DropZoneArea
                                    className={cn([
                                        "relative",
                                        "aspect-16/9",
                                        "h-52",
                                        "p-0",
                                        "overflow-hidden",
                                    ])}
                                >
                                    <DropzoneTrigger
                                        className={cn([
                                            "bg-transparent",
                                            "text-center",
                                            "h-full",
                                            "aspect-16/9",
                                        ])}
                                    >
                                        <Image
                                            src={`/api/games/${game?.id}/poster?r=${refresh}`}
                                            className={cn([
                                                "object-cover",
                                                "rounded-md",
                                                "overflow-hidden",
                                                "aspect-16/9",
                                                "w-full",
                                                "select-none",
                                            ])}
                                        />
                                    </DropzoneTrigger>
                                </DropZoneArea>
                            </Dropzone>
                        </div>
                        <div className={cn(["flex", "flex-col", "gap-1"])}>
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
                                    type={"button"}
                                    icon={TrashIcon}
                                    size={"sm"}
                                    level={"error"}
                                    square
                                    onClick={handleIconDelete}
                                />
                            </div>
                            <Dropzone {...iconDropzone}>
                                <DropZoneArea
                                    className={cn([
                                        "relative",
                                        "aspect-square",
                                        "h-52",
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
                                            src={`/api/games/${game?.id}/icon?r=${refresh}`}
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
                </div>
                <div className={cn(["grid", "grid-cols-4", "gap-5"])}>
                    <FormField
                        control={form.control}
                        name={"is_public"}
                        render={({ field }) => (
                            <FormItem className={cn(["w-full"])}>
                                <FormLabel>是否为公开赛（免审核）</FormLabel>
                                <FormControl>
                                    <Field size={"sm"}>
                                        <FieldIcon>
                                            <LockOpenIcon />
                                        </FieldIcon>
                                        <Select
                                            {...field}
                                            options={[
                                                {
                                                    value: String(true),
                                                    content: "是",
                                                },
                                                {
                                                    value: String(false),
                                                    content: "否",
                                                },
                                            ]}
                                            onValueChange={(value) => {
                                                field.onChange(
                                                    value === "true"
                                                );
                                            }}
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
                        name={"is_need_write_up"}
                        render={({ field }) => (
                            <FormItem className={cn(["w-full"])}>
                                <FormLabel>是否需要提交 Write-up</FormLabel>
                                <FormControl>
                                    <Field size={"sm"}>
                                        <FieldIcon>
                                            <FileCheck2Icon />
                                        </FieldIcon>
                                        <Select
                                            {...field}
                                            options={[
                                                {
                                                    value: String(true),
                                                    content: "是",
                                                },
                                                {
                                                    value: String(false),
                                                    content: "否",
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
                    <FormField
                        control={form.control}
                        name={"member_limit_min"}
                        render={({ field }) => (
                            <FormItem className={cn(["w-full"])}>
                                <FormLabel>团队所需最小人数</FormLabel>
                                <FormControl>
                                    <Field size={"sm"}>
                                        <FieldIcon>
                                            <UsersRoundIcon />
                                        </FieldIcon>
                                        <TextField
                                            {...field}
                                            type={"number"}
                                            placeholder={"3"}
                                            value={field.value || ""}
                                            onChange={(e) =>
                                                field.onChange(
                                                    Number(e.target.value)
                                                )
                                            }
                                        />
                                    </Field>
                                </FormControl>
                                <FormMessage />
                            </FormItem>
                        )}
                    />
                    <FormField
                        control={form.control}
                        name={"member_limit_max"}
                        render={({ field }) => (
                            <FormItem className={cn(["w-full"])}>
                                <FormLabel>团队所需最大人数</FormLabel>
                                <FormControl>
                                    <Field size={"sm"}>
                                        <FieldIcon>
                                            <UsersRoundIcon />
                                        </FieldIcon>
                                        <TextField
                                            {...field}
                                            type={"number"}
                                            placeholder={"3"}
                                            value={field.value || ""}
                                            onChange={(e) =>
                                                field.onChange(
                                                    Number(e.target.value)
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
                <div className={cn(["grid", "grid-cols-3", "gap-5"])}>
                    <FormField
                        control={form.control}
                        name={"started_at"}
                        render={({ field }) => (
                            <FormItem>
                                <FormLabel>开始时间</FormLabel>
                                <FormControl>
                                    <Field>
                                        <FieldIcon>
                                            <ClockIcon />
                                        </FieldIcon>
                                        <DateTimePicker {...field} />
                                    </Field>
                                </FormControl>
                                <FormMessage />
                            </FormItem>
                        )}
                    />
                    <FormField
                        control={form.control}
                        name={"frozen_at"}
                        render={({ field }) => (
                            <FormItem>
                                <FormLabel>冻结时间</FormLabel>
                                <FormControl>
                                    <Field>
                                        <FieldIcon>
                                            <ClockFadingIcon />
                                        </FieldIcon>
                                        <DateTimePicker {...field} />
                                    </Field>
                                </FormControl>
                                <FormMessage />
                            </FormItem>
                        )}
                    />
                    <FormField
                        control={form.control}
                        name={"ended_at"}
                        render={({ field }) => (
                            <FormItem>
                                <FormLabel>结束时间</FormLabel>
                                <FormControl>
                                    <Field>
                                        <FieldIcon>
                                            <ClockAlertIcon />
                                        </FieldIcon>
                                        <DateTimePicker {...field} />
                                    </Field>
                                </FormControl>
                                <FormMessage />
                            </FormItem>
                        )}
                    />
                </div>
                <FormField
                    control={form.control}
                    name={"description"}
                    render={({ field }) => (
                        <FormItem
                            className={cn(["flex-1", "flex", "flex-col"])}
                        >
                            <FormLabel>描述</FormLabel>
                            <FormControl>
                                <Editor
                                    {...field}
                                    value={field.value || ""}
                                    lang={"markdown"}
                                    showLineNumbers
                                    className={cn(["h-full", "min-h-128"])}
                                />
                            </FormControl>
                            <FormMessage />
                        </FormItem>
                    )}
                />
                <Button
                    variant={"solid"}
                    level={"primary"}
                    type={"submit"}
                    size={"lg"}
                    className={cn(["w-full"])}
                    icon={SaveIcon}
                    loading={loading}
                >
                    保存
                </Button>
            </form>
        </Form>
    );
}
