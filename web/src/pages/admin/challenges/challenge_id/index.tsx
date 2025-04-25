import { useContext, useEffect, useState } from "react";
import { Context } from "./context";
import { z } from "zod";
import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { Editor } from "@/components/ui/editor";
import { cn } from "@/utils";
import { Field, FieldIcon } from "@/components/ui/field";
import { TextField } from "@/components/ui/text-field";
import { updateChallenge } from "@/api/admin/challenges/challenge_id";
import { toast } from "sonner";
import { useSharedStore } from "@/storages/shared";
import {
    Box,
    Container,
    ContainerIcon,
    Folder,
    FolderIcon,
    Library,
    LibraryIcon,
    SaveIcon,
    ShipWheel,
    TagIcon,
    Type,
    TypeIcon,
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
import { Select } from "@/components/ui/select";
import { useCategoryStore } from "@/storages/category";
import { TagsField } from "@/components/ui/tags-field";

export default function Index() {
    const { challenge } = useContext(Context);
    const categoryStore = useCategoryStore();
    const sharedStore = useSharedStore();

    const [loading, setLoading] = useState<boolean>(false);

    const formSchema = z.object({
        title: z.string({
            message: "请输入标题",
        }),
        category: z.number({
            message: "请选择分类",
        }),
        tags: z.array(z.string()).nullish(),
        description: z.string({
            message: "请输入描述",
        }),
        has_attachment: z.boolean({}),
        is_dynamic: z.boolean({}),
    });

    const form = useForm<z.infer<typeof formSchema>>({
        resolver: zodResolver(formSchema),
        defaultValues: challenge,
    });

    useEffect(() => {
        form.reset(challenge, {
            keepDefaultValues: false,
        });
    }, [challenge, form.reset]);

    function onSubmit(values: z.infer<typeof formSchema>) {
        setLoading(true);
        updateChallenge({
            id: challenge?.id,
            ...values,
        })
            .then((res) => {
                if (res.code === 200) {
                    toast.success(`题目 ${res?.data?.title} 更新成功`);
                }
            })
            .finally(() => {
                sharedStore.setRefresh();
                setLoading(false);
            });
    }

    return (
        <Form {...form}>
            <form
                onSubmit={form.handleSubmit(onSubmit)}
                autoComplete={"off"}
                className={cn(["flex", "flex-col", "flex-1", "gap-8"])}
            >
                <div className={cn(["flex", "gap-5"])}>
                    <FormField
                        control={form.control}
                        name={"title"}
                        render={({ field }) => (
                            <FormItem className={cn(["w-3/4"])}>
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
                        name={"category"}
                        render={({ field }) => (
                            <FormItem className={cn(["w-1/4"])}>
                                <FormLabel>分类</FormLabel>
                                <FormControl>
                                    <Field>
                                        <FieldIcon>
                                            <LibraryIcon />
                                        </FieldIcon>
                                        <Select
                                            {...field}
                                            options={categoryStore.categories?.map(
                                                (category) => {
                                                    const Icon =
                                                        category?.icon!;

                                                    return {
                                                        value: String(
                                                            category?.id
                                                        ),
                                                        content: (
                                                            <div
                                                                className={cn([
                                                                    "flex",
                                                                    "gap-2",
                                                                    "items-center",
                                                                ])}
                                                            >
                                                                <Icon />
                                                                {category?.name?.toUpperCase()}
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
                </div>
                <FormField
                    control={form.control}
                    name={"tags"}
                    render={({ field }) => (
                        <FormItem className={cn(["w-full"])}>
                            <FormLabel>标签</FormLabel>
                            <FormControl>
                                <Field>
                                    <FieldIcon>
                                        <TagIcon />
                                    </FieldIcon>
                                    <TagsField
                                        value={field.value || []}
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
                <div className={cn(["flex", "gap-5"])}>
                    <FormField
                        control={form.control}
                        name={"has_attachment"}
                        render={({ field }) => (
                            <FormItem className={cn(["w-1/2"])}>
                                <FormLabel>是否启用附件</FormLabel>
                                <FormControl>
                                    <Field>
                                        <FieldIcon>
                                            <FolderIcon />
                                        </FieldIcon>
                                        <Select
                                            {...field}
                                            options={[
                                                {
                                                    value: String(true),
                                                    content: "启用",
                                                },
                                                {
                                                    value: String(false),
                                                    content: "禁用",
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
                        name={"is_dynamic"}
                        render={({ field }) => (
                            <FormItem className={cn(["w-1/2"])}>
                                <FormLabel>动态性</FormLabel>
                                <FormControl>
                                    <Field>
                                        <FieldIcon>
                                            <ContainerIcon />
                                        </FieldIcon>
                                        <Select
                                            {...field}
                                            options={[
                                                {
                                                    value: String(true),
                                                    content: (
                                                        <div
                                                            className={cn([
                                                                "flex",
                                                                "gap-2",
                                                                "items-center",
                                                            ])}
                                                        >
                                                            <ShipWheel />
                                                            动态环境
                                                        </div>
                                                    ),
                                                },
                                                {
                                                    value: String(false),
                                                    content: (
                                                        <div
                                                            className={cn([
                                                                "flex",
                                                                "gap-2",
                                                                "items-center",
                                                            ])}
                                                        >
                                                            <Box />
                                                            静态环境
                                                        </div>
                                                    ),
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
                                    lang={"markdown"}
                                    showLineNumbers
                                    className={cn(["h-full", "min-h-64"])}
                                />
                            </FormControl>
                            <FormMessage />
                        </FormItem>
                    )}
                />
                <Button
                    variant={"solid"}
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
