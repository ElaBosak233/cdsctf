import { zodResolver } from "@hookform/resolvers/zod";
import { StatusCodes } from "http-status-codes";
import {
  BadgeCheckIcon,
  InfoIcon,
  ListEndIcon,
  SaveIcon,
  TextIcon,
  TypeIcon,
  UndoIcon,
  UserRoundCheckIcon,
} from "lucide-react";
import { useEffect, useState } from "react";
import { useForm } from "react-hook-form";
import { toast } from "sonner";
import { z } from "zod";

import { getConfigs, updateConfig } from "@/api/admin/configs";
import { deleteLogo } from "@/api/admin/configs/logo";
import { Button } from "@/components/ui/button";
import {
  DropZoneArea,
  Dropzone,
  DropzoneTrigger,
  useDropzone,
} from "@/components/ui/dropzone";
import { Field, FieldIcon } from "@/components/ui/field";
import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "@/components/ui/form";
import { Image } from "@/components/ui/image";
import { Label } from "@/components/ui/label";
import { LoadingOverlay } from "@/components/ui/loading-overlay";
import { Select } from "@/components/ui/select";
import { Separator } from "@/components/ui/separator";
import { TextField } from "@/components/ui/text-field";
import { useRefresh } from "@/hooks/use-refresh";
import type { Config } from "@/models/config";
import { cn } from "@/utils";
import { uploadFile } from "@/utils/file";

export default function Index() {
  const { tick, bump } = useRefresh();
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
        keywords: z.array(z.string()).optional(),
        footer: z.string().optional(),
      })
      .optional(),
    auth: z
      .object({
        is_registration_enabled: z.boolean().optional(),
      })
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
  }, [config, form]);

  function onSubmit(values: z.infer<typeof formSchema>) {
    updateConfig({
      ...config,
      ...values,
    }).then((res) => {
      if (res.code === StatusCodes.OK) {
        toast.success("平台配置更新成功");
      }
    });
  }

  const iconDropzone = useDropzone({
    onDropFile: async (file) => {
      try {
        await uploadFile("/api/admin/configs/logo", [file], ({ percent }) => {
          toast.loading(`上传进度 ${percent.toFixed(0)}%`, {
            id: "logo-upload",
          });
        });
        toast.success("标志更新成功", {
          id: "logo-upload",
        });
      } catch {
        toast.error("标志上传失败", {
          id: "logo-upload",
        });
      } finally {
        bump();
      }

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

  async function handleLogoDelete() {
    const res = await deleteLogo();
    if (res.code === StatusCodes.OK) {
      toast.success("标志重置成功");
    }
    bump();
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
          "relative",
        ])}
      >
        <LoadingOverlay loading={!config} />
        <h2
          className={cn(["flex", "gap-2", "items-center", "text-xl", "mt-2"])}
        >
          <InfoIcon />
          元信息
        </h2>
        <Separator />
        <div className={cn(["flex", "gap-3"])}>
          <div className={cn(["flex", "flex-col", "gap-3", "flex-1"])}>
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
          <div className={cn(["space-y-1"])}>
            <div
              className={cn([
                "flex",
                "gap-3",
                "items-center",
                "justify-between",
              ])}
            >
              <Label>标志</Label>
              <Button
                icon={<UndoIcon />}
                size={"sm"}
                level={"warning"}
                square
                onClick={handleLogoDelete}
              />
            </div>
            <Dropzone {...iconDropzone}>
              <DropZoneArea
                className={cn([
                  "relative",
                  "aspect-square",
                  "h-27",
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
                    src={`/api/configs/logo?r=${tick}`}
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
        <FormField
          control={form.control}
          name={"meta.footer"}
          render={({ field }) => (
            <FormItem className={cn(["w-full"])}>
              <FormLabel>页脚（支持 Markdown）</FormLabel>
              <FormControl>
                <Field>
                  <FieldIcon>
                    <ListEndIcon />
                  </FieldIcon>
                  <TextField
                    {...field}
                    placeholder="请输入页脚"
                    value={field.value || ""}
                    onChange={field.onChange}
                  />
                </Field>
              </FormControl>
              <FormMessage />
            </FormItem>
          )}
        />
        <h2
          className={cn(["flex", "gap-2", "items-center", "text-xl", "mt-2"])}
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
                    onValueChange={(value) => field.onChange(value === "true")}
                    value={String(field.value)}
                  />
                </Field>
              </FormControl>
              <FormMessage />
            </FormItem>
          )}
        />
        <div className={cn(["flex-1"])} />
        <Button
          type={"submit"}
          variant={"solid"}
          size={"lg"}
          icon={<SaveIcon />}
          className={cn(["mt-2"])}
        >
          保存
        </Button>
      </form>
    </Form>
  );
}
