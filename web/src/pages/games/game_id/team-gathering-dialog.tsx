import { zodResolver } from "@hookform/resolvers/zod";
import { StatusCodes } from "http-status-codes";
import { HTTPError } from "ky";
import { KeyIcon, SwordsIcon, TypeIcon } from "lucide-react";
import { useState } from "react";
import { useForm } from "react-hook-form";
import { useTranslation } from "react-i18next";
import { toast } from "sonner";
import { z } from "zod";
import { teamRegister } from "@/api/games/game_id/teams";
import { joinTeam } from "@/api/games/game_id/teams/team_id";
import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";
import { Field, FieldIcon } from "@/components/ui/field";
import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "@/components/ui/form";
import { Separator } from "@/components/ui/separator";
import { TextField } from "@/components/ui/text-field";
import { useGameStore } from "@/storages/game";
import { useSharedStore } from "@/storages/shared";
import { cn } from "@/utils";
import { parseErrorResponse } from "@/utils/query";

interface TeamGatheringDialogProps {
  onClose: () => void;
}

function TeamGatheringDialog(props: TeamGatheringDialogProps) {
  const { onClose } = props;

  const { t } = useTranslation();

  const sharedStore = useSharedStore();
  const { currentGame } = useGameStore();
  const [_loading, setLoading] = useState<boolean>(false);

  const createFormSchema = z.object({
    name: z.string({
      message: t("team.form.name.required"),
    }),
  });

  const createForm = useForm<z.infer<typeof createFormSchema>>({
    resolver: zodResolver(createFormSchema),
  });

  function onCreateFormSubmit(values: z.infer<typeof createFormSchema>) {
    if (!currentGame) return;

    setLoading(true);
    teamRegister({
      game_id: currentGame.id!,
      ...values,
    })
      .then((res) => {
        if (res.code === StatusCodes.OK) {
          toast.success(`团队 ${res?.data?.name} 创建成功`);
          onClose();
        }
      })
      .finally(() => {
        sharedStore.setRefresh();
        setLoading(false);
      });
  }

  const joinFormSchema = z.object({
    token: z
      .string({
        message: t("team.form.invite_code.required"),
      })
      .regex(/^\d+:.*$/, {
        message: t("team.form.invite_code.invalid"),
      }),
  });

  const joinForm = useForm<z.infer<typeof joinFormSchema>>({
    resolver: zodResolver(joinFormSchema),
  });

  function onJoinFormSubmit(values: z.infer<typeof joinFormSchema>) {
    const tokens = values.token.split(":");
    const team_id = Number(tokens[0]);
    const token = tokens[1];

    if (!currentGame) return;

    setLoading(true);
    joinTeam({
      game_id: currentGame.id!,
      team_id: team_id,
      token: token,
    })
      .then((res) => {
        if (res.code === StatusCodes.OK) {
          toast.success(`成功加入团队`);
          onClose();
        }
      })
      .catch(async (error) => {
        if (!(error instanceof HTTPError)) return;
        const res = await parseErrorResponse(error);

        if (res.code === StatusCodes.BAD_REQUEST) {
          toast.error("加入失败", {
            description: res.msg,
          });
        }
      })
      .finally(() => {
        sharedStore.setRefresh();
        setLoading(false);
      });
  }

  return (
    <Card
      className={cn(["p-5", "min-h-64", "w-lg", "flex", "flex-col", "gap-5"])}
    >
      <h3 className={cn(["flex", "gap-3", "items-center", "text-md"])}>
        <SwordsIcon className={cn(["size-4"])} />
        {t("team.actions.gather.create.title")}
      </h3>
      <Form {...createForm}>
        <form
          onSubmit={createForm.handleSubmit(onCreateFormSubmit)}
          autoComplete={"off"}
          className={cn(["flex", "gap-5", "items-end"])}
        >
          <FormField
            control={createForm.control}
            name={"name"}
            render={({ field }) => (
              <FormItem className={cn(["flex-1"])}>
                <FormLabel>{t("team.form.name._")}</FormLabel>
                <FormControl>
                  <Field size={"sm"}>
                    <FieldIcon>
                      <TypeIcon />
                    </FieldIcon>
                    <TextField
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
          <Button variant={"solid"} type={"submit"}>
            {t("team.actions.gather.create._")}
          </Button>
        </form>
      </Form>
      <Separator />
      <h3 className={cn(["flex", "gap-3", "items-center", "text-md"])}>
        <SwordsIcon className={cn(["size-4"])} />
        {t("team.actions.gather.join.title")}
      </h3>
      <Form {...joinForm}>
        <form
          onSubmit={joinForm.handleSubmit(onJoinFormSubmit)}
          autoComplete={"off"}
          className={cn(["flex", "gap-5", "items-end"])}
        >
          <FormField
            control={joinForm.control}
            name={"token"}
            render={({ field }) => (
              <FormItem className={cn(["flex-1"])}>
                <FormLabel>{t("team.form.invite_code._")}</FormLabel>
                <FormControl>
                  <Field size={"sm"}>
                    <FieldIcon>
                      <KeyIcon />
                    </FieldIcon>
                    <TextField
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
          <Button variant={"solid"} type={"submit"}>
            {t("team.actions.gather.join._")}
          </Button>
        </form>
      </Form>
    </Card>
  );
}

export { TeamGatheringDialog };
