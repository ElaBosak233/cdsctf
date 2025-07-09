import { zodResolver } from "@hookform/resolvers/zod";
import { StatusCodes } from "http-status-codes";
import {
  MailIcon,
  SaveIcon,
  TrashIcon,
  TypeIcon,
  UploadCloudIcon,
  UserRoundIcon,
} from "lucide-react";
import { useRef, useState } from "react";
import { useForm } from "react-hook-form";
import { toast } from "sonner";
import { z } from "zod";

import { EmailVerifyDialog } from "./email-verify-dialog";

import { updateUserProfile } from "@/api/users/profile";
import { deleteUserAvatar } from "@/api/users/profile/avatar";
import { Alert } from "@/components/ui/alert";
import { Avatar } from "@/components/ui/avatar";
import { Button } from "@/components/ui/button";
import { Dialog, DialogContent } from "@/components/ui/dialog";
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
import { Label } from "@/components/ui/label";
import { TextField } from "@/components/ui/text-field";
import { useAuthStore } from "@/storages/auth";
import { useConfigStore } from "@/storages/config";
import { useSharedStore } from "@/storages/shared";
import { cn } from "@/utils";

export default function Index() {
  const authStore = useAuthStore();
  const sharedStore = useSharedStore();
  const configStore = useConfigStore();
  const [loading, setLoading] = useState<boolean>(false);

  const avatarInput = useRef<HTMLInputElement>(null);
  const [hasAvatar, setHasAvatar] = useState<boolean>(false);

  const [emailVerifyDialogOpen, setEmailVerifyDialogOpen] =
    useState<boolean>(false);

  const formSchema = z.object({
    username: z.string().nullish(),
    name: z.string({
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
        if (res.code === StatusCodes.OK) {
          authStore?.setUser(res.data);
          toast.success("个人资料更新成功");
        }
      })
      .finally(() => {
        setLoading(false);
      });
  }

  function handleAvatarUpload(event: React.ChangeEvent<HTMLInputElement>) {
    const file = event.target.files?.[0];

    if (!file) return;

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
      if (xhr.status === StatusCodes.OK) {
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

    event.target.value = "";
  }

  function handleAvatarDelete() {
    if (!authStore?.user) return;

    deleteUserAvatar()
      .then((res) => {
        if (res.code === StatusCodes.OK) {
          toast.success(`头像删除成功`);
        }
      })
      .finally(() => {
        sharedStore.setRefresh();
      });
  }

  return (
    <>
      <title>{`基本信息 - ${configStore?.config?.meta?.title}`}</title>
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
            <div
              className={cn([
                "flex",
                "flex-wrap-reverse",
                "gap-5",
                "items-center",
                "justify-center",
              ])}
            >
              <div
                className={cn([
                  "flex",
                  "flex-col",
                  "gap-8",
                  "flex-1",
                  "min-w-2xs",
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
                  name={"name"}
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
              <div className={cn(["flex", "flex-col", "gap-3"])}>
                <div
                  className={cn([
                    "flex",
                    "gap-3",
                    "items-center",
                    "justify-between",
                  ])}
                >
                  <Label>头像</Label>
                </div>
                <Avatar
                  className={cn([
                    "h-30",
                    "w-30",
                    "transition-all",
                    "duration-300",
                    "border",
                  ])}
                  src={`/api/users/${authStore?.user?.id}/avatar?refresh=${sharedStore?.refresh}`}
                  onLoadingStatusChange={(status) =>
                    setHasAvatar(status === "loaded")
                  }
                  fallback={authStore?.user?.name?.charAt(0)}
                >
                  <Button
                    className={cn([
                      "absolute",
                      "top-0",
                      "left-0",
                      "w-full",
                      "h-full",
                      "opacity-0",
                      "backdrop-blur-xs",
                      "transition-all",
                      "hover:opacity-100",
                    ])}
                    onClick={() => {
                      if (hasAvatar) {
                        handleAvatarDelete();
                      } else {
                        avatarInput?.current?.click();
                      }
                    }}
                  >
                    <input
                      type={"file"}
                      className={"hidden"}
                      ref={avatarInput}
                      accept={".png,.jpg,.jpeg,.webp"}
                      onChange={handleAvatarUpload}
                    />

                    {hasAvatar ? (
                      <TrashIcon className={cn(["shrink-0", "text-error"])} />
                    ) : (
                      <UploadCloudIcon className="shrink-0" />
                    )}
                  </Button>
                </Avatar>
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
                className={cn(["flex", "items-center", "justify-between"])}
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
                      onClose={() => setEmailVerifyDialogOpen(false)}
                    />
                  </DialogContent>
                </Dialog>
              </Alert>
            )}
            <FormField
              control={form.control}
              name={"description"}
              render={({ field }) => (
                <FormItem className={cn(["flex-1", "flex", "flex-col"])}>
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
              icon={<SaveIcon />}
              loading={loading}
            >
              保存
            </Button>
          </form>
        </Form>
      </div>
    </>
  );
}
