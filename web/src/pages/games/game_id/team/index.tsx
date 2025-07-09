import { zodResolver } from "@hookform/resolvers/zod";
import { StatusCodes } from "http-status-codes";
import {
  CheckIcon,
  MailIcon,
  MessageCircleIcon,
  TrashIcon,
  TypeIcon,
  UploadCloudIcon,
} from "lucide-react";
import { useEffect, useRef, useState } from "react";
import { useForm } from "react-hook-form";
import { toast } from "sonner";
import { z } from "zod";

import { updateTeam } from "@/api/games/game_id/teams/profile";
import { deleteTeamAvatar } from "@/api/games/game_id/teams/profile/avatar";
import { Avatar } from "@/components/ui/avatar";
import { Button } from "@/components/ui/button";
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
import { useGameStore } from "@/storages/game";
import { useSharedStore } from "@/storages/shared";
import { cn } from "@/utils";

export default function Index() {
  const sharedStore = useSharedStore();
  const { currentGame, selfTeam } = useGameStore();

  const [loading, setLoading] = useState<boolean>(false);
  const disabled = Date.now() / 1000 > Number(currentGame?.ended_at);

  const avatarInput = useRef<HTMLInputElement>(null);
  const [hasAvatar, setHasAvatar] = useState<boolean>(false);

  const formSchema = z.object({
    name: z.string({
      message: "请输入队名",
    }),
    email: z.string().nullish(),
    description: z.string().nullish(),
    slogan: z.string().nullish(),
  });

  const form = useForm<z.infer<typeof formSchema>>({
    resolver: zodResolver(formSchema),
    defaultValues: selfTeam,
  });

  useEffect(() => {
    form.reset(selfTeam, {
      keepDefaultValues: false,
    });
  }, [selfTeam, form.reset]);

  function onSubmit(values: z.infer<typeof formSchema>) {
    if (!selfTeam || !currentGame) return;
    setLoading(true);
    updateTeam({
      id: selfTeam.id!,
      game_id: currentGame.id!,
      ...values,
    })
      .then((res) => {
        if (res.code === StatusCodes.OK) {
          toast.success(`团队 ${res?.data?.name} 更新成功`);
        }
      })
      .finally(() => {
        sharedStore.setRefresh();
        setLoading(false);
      });
  }

  function handleAvatarUpload(event: React.ChangeEvent<HTMLInputElement>) {
    const file = event.target.files?.[0];

    if (!file) return;

    const formData = new FormData();
    formData.append("file", file);
    const xhr = new XMLHttpRequest();
    xhr.open(
      "POST",
      `/api/games/${currentGame?.id}/teams/profile/avatar`,
      true
    );
    xhr.upload.onprogress = (event) => {
      if (event.lengthComputable) {
        const percentComplete = (event.loaded / event.total) * 100;
        toast.loading(`上传进度 ${percentComplete}%`, {
          id: "team-avatar-upload",
        });
      }
    };
    xhr.onload = () => {
      if (xhr.status === StatusCodes.OK) {
        toast.success("头像上传成功", {
          id: "team-avatar-upload",
        });
        sharedStore?.setRefresh();
      } else {
        toast.error("头像上传失败", {
          id: "team-avatar-upload",
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
    if (!selfTeam || !currentGame) return;
    deleteTeamAvatar({
      game_id: currentGame.id!,
      team_id: selfTeam.id!,
    })
      .then((res) => {
        if (res.code === StatusCodes.OK) {
          toast.success(`团队 ${selfTeam?.name} 头像删除成功`);
        }
      })
      .finally(() => {
        sharedStore.setRefresh();
      });
  }

  return (
    <>
      <title>{`团队 - ${currentGame?.title}`}</title>
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
              <div className={cn(["flex", "flex-col", "gap-8", "flex-1"])}>
                <FormField
                  control={form.control}
                  name={"name"}
                  render={({ field }) => (
                    <FormItem>
                      <FormLabel>团队名</FormLabel>
                      <FormControl>
                        <Field>
                          <FieldIcon>
                            <TypeIcon />
                          </FieldIcon>
                          <TextField
                            disabled={disabled}
                            placeholder={"团队名"}
                            {...field}
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
                      <FormLabel>电子邮箱</FormLabel>
                      <FormControl>
                        <Field>
                          <FieldIcon>
                            <MailIcon />
                          </FieldIcon>
                          <TextField
                            disabled={disabled}
                            placeholder={"电子邮箱"}
                            {...field}
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
                  src={`/api/games/${currentGame?.id}/teams/${selfTeam?.id}/avatar?refresh=${sharedStore?.refresh}`}
                  fallback={selfTeam?.name?.charAt(0)}
                  onLoadingStatusChange={(status) =>
                    setHasAvatar(status === "loaded")
                  }
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
              name={"slogan"}
              render={({ field }) => (
                <FormItem>
                  <FormLabel>口号</FormLabel>
                  <FormControl>
                    <Field>
                      <FieldIcon>
                        <MessageCircleIcon />
                      </FieldIcon>
                      <TextField
                        disabled={disabled}
                        placeholder={"口号"}
                        {...field}
                        value={field.value || ""}
                        onChange={field.onChange}
                      />
                    </Field>
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />
            <div className={cn(["flex-1"])} />
            <Button
              size={"lg"}
              type={"submit"}
              level={"info"}
              variant={"solid"}
              icon={<CheckIcon />}
              loading={loading}
              disabled={disabled}
            >
              保存
            </Button>
          </form>
        </Form>
      </div>
    </>
  );
}
