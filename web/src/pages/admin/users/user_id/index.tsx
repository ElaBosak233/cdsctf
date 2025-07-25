import { zodResolver } from "@hookform/resolvers/zod";
import { StatusCodes } from "http-status-codes";
import {
  MailCheckIcon,
  MailIcon,
  SaveIcon,
  ShieldIcon,
  UserRoundCheckIcon,
  UserRoundIcon,
  UserRoundXIcon,
} from "lucide-react";
import { useContext, useEffect, useState } from "react";
import { useForm } from "react-hook-form";
import { useParams } from "react-router";
import { toast } from "sonner";
import { z } from "zod";

import { updateUser } from "@/api/admin/users/user_id";
import { Avatar } from "@/components/ui/avatar";
import { Button } from "@/components/ui/button";
import {
  DropZoneArea,
  Dropzone,
  DropzoneTrigger,
  useDropzone,
} from "@/components/ui/dropzone";
import { Editor } from "@/components/ui/editor";
import { Field, FieldIcon } from "@/components/ui/field";
import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "@/components/ui/form";
import { Select } from "@/components/ui/select";
import { TextField } from "@/components/ui/text-field";
import { useRefresh } from "@/hooks/use-refresh";
import { Group } from "@/models/user";
import { useSharedStore } from "@/storages/shared";
import { cn } from "@/utils";
import { Context } from "./context";

export default function Index() {
  const { user } = useContext(Context);
  const { user_id } = useParams<{ user_id: string }>();

  const sharedStore = useSharedStore();
  const [loading, setLoading] = useState<boolean>(false);
  const { tick, bump } = useRefresh();

  const groupOptions = [
    { id: Group.Guest.toString(), name: "访客", icon: UserRoundIcon },
    { id: Group.Banned.toString(), name: "封禁", icon: UserRoundXIcon },
    { id: Group.User.toString(), name: "用户", icon: UserRoundCheckIcon },
    { id: Group.Admin.toString(), name: "管理员", icon: ShieldIcon },
  ];

  const formSchema = z.object({
    username: z.string({}),
    name: z.string({
      message: "请输入昵称",
    }),
    group: z.number({
      message: "请选择用户组",
    }),
    description: z.string().nullish(),
    email: z
      .string({
        message: "请输入邮箱",
      })
      .email({
        message: "邮箱不合法",
      }),
    is_verified: z.boolean(),
  });

  const form = useForm<z.infer<typeof formSchema>>({
    resolver: zodResolver(formSchema),
    defaultValues: user,
  });

  useEffect(() => {
    form.reset(user, {
      keepDefaultValues: false,
    });
  }, [user, form.reset]);

  function onSubmit(values: z.infer<typeof formSchema>) {
    if (!user) return;

    setLoading(true);
    updateUser({
      id: user.id!,
      ...values,
    })
      .then((res) => {
        if (res.code === StatusCodes.OK) {
          toast.success(`用户 ${res?.data?.username} 更新成功`);
        }
      })
      .finally(() => {
        sharedStore.setRefresh();
        setLoading(false);
      });
  }

  const dropzone = useDropzone({
    onDropFile: async (file) => {
      if (!user?.id) return { status: "error", error: "用户 ID 不存在" };

      const formData = new FormData();
      formData.append("file", file);

      const xhr = new XMLHttpRequest();
      xhr.open("POST", `/api/users/${user.id}/avatar`, true);

      xhr.upload.onprogress = (event) => {
        if (event.lengthComputable) {
          const percentComplete = (event.loaded / event.total) * 100;
          toast.loading(`上传进度 ${percentComplete.toFixed(0)}%`, {
            id: "avatar-upload",
          });
        }
      };

      xhr.onload = () => {
        if (xhr.status === StatusCodes.OK) {
          toast.success("头像上传成功", {
            id: "avatar-upload",
          });
          bump();
        } else {
          toast.error("头像上传失败", {
            id: "avatar-upload",
            description: xhr.responseText,
          });
        }
      };

      xhr.onerror = () => {
        toast.error("头像上传失败", {
          id: "avatar-upload",
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

  return (
    <div className={cn(["flex", "flex-col", "gap-6", "flex-1"])}>
      <div className={cn(["flex", "flex-col", "items-center", "gap-4"])}>
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
                src={`/api/users/${user_id}/avatar?refresh=${tick}`}
                fallback={user?.username?.charAt(0)}
              />
            </DropzoneTrigger>
          </DropZoneArea>
        </Dropzone>
      </div>
      <Form {...form}>
        <form
          onSubmit={form.handleSubmit(onSubmit)}
          autoComplete={"off"}
          className={cn(["flex", "flex-col", "flex-1", "gap-8"])}
        >
          <div
            className={cn(["grid", "grid-cols-1", "md:grid-cols-3", "gap-4"])}
          >
            <FormField
              control={form.control}
              name={"username"}
              render={({ field }) => (
                <FormItem className={cn(["w-full"])}>
                  <FormLabel>用户名</FormLabel>
                  <FormControl>
                    <Field>
                      <FieldIcon>
                        <UserRoundIcon />
                      </FieldIcon>
                      <TextField
                        {...field}
                        placeholder="请输入用户名"
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
              name={"name"}
              render={({ field }) => (
                <FormItem className={cn(["w-full"])}>
                  <FormLabel>昵称</FormLabel>
                  <FormControl>
                    <Field>
                      <FieldIcon>
                        <UserRoundIcon />
                      </FieldIcon>
                      <TextField
                        {...field}
                        placeholder="请输入昵称"
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
                <FormItem className={cn(["w-full"])}>
                  <FormLabel>用户组</FormLabel>
                  <FormControl>
                    <Field>
                      <FieldIcon>
                        <ShieldIcon />
                      </FieldIcon>
                      <Select
                        {...field}
                        options={groupOptions.map((group) => ({
                          value: group.id,
                          content: (
                            <div
                              className={cn(["flex", "gap-2", "items-center"])}
                            >
                              <group.icon className="size-4" />
                              {group.name}
                            </div>
                          ),
                        }))}
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

          <div className={cn(["flex", "gap-3"])}>
            <FormField
              control={form.control}
              name={"email"}
              render={({ field }) => (
                <FormItem className={cn(["basis-3/4"])}>
                  <FormLabel>邮箱</FormLabel>
                  <FormControl>
                    <Field>
                      <FieldIcon>
                        <MailIcon />
                      </FieldIcon>
                      <TextField
                        {...field}
                        type={"email"}
                        placeholder="请输入邮箱"
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
              name={"is_verified"}
              render={({ field }) => (
                <FormItem className={cn(["basis-1/4"])}>
                  <FormLabel>是否已验证</FormLabel>
                  <FormControl>
                    <Field>
                      <FieldIcon>
                        <MailCheckIcon />
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
                          field.onChange(value === "true");
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
              <FormItem className={cn(["flex-1", "flex", "flex-col"])}>
                <FormLabel>描述</FormLabel>
                <FormControl>
                  <Editor
                    {...field}
                    value={field.value ?? ""}
                    className={cn(["h-full", "min-h-64"])}
                    lang="markdown"
                    tabSize={2}
                    showLineNumbers
                  />
                </FormControl>
                <FormMessage />
              </FormItem>
            )}
          />

          <Button
            size={"lg"}
            variant={"solid"}
            level={"primary"}
            type={"submit"}
            icon={<SaveIcon />}
            loading={loading}
            className={cn(["w-full"])}
          >
            保存
          </Button>
        </form>
      </Form>
    </div>
  );
}
