import { zodResolver } from "@hookform/resolvers/zod";
import { StatusCodes } from "http-status-codes";
import { HTTPError } from "ky";
import { KeyIcon, LogInIcon, TypeIcon, UserPlusIcon } from "lucide-react";
import { useState } from "react";
import { useForm } from "react-hook-form";
import { useTranslation } from "react-i18next";
import { toast } from "sonner";
import { z } from "zod";
import { createTeam } from "@/api/games/game_id/teams";
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
import { TextField } from "@/components/ui/text-field";
import { useGameStore } from "@/storages/game";
import { useSharedStore } from "@/storages/shared";
import { cn } from "@/utils";
import { formatApiMsg, parseErrorResponse } from "@/utils/query";

type Tab = "create" | "join";

interface TeamGatheringDialogProps {
  onClose: () => void;
}

function TeamGatheringDialog(props: TeamGatheringDialogProps) {
  const { onClose } = props;

  const { t } = useTranslation();

  const sharedStore = useSharedStore();
  const { currentGame } = useGameStore();
  const [loading, setLoading] = useState<boolean>(false);
  const [tab, setTab] = useState<Tab>("create");

  const createFormSchema = z.object({
    name: z.string({
      message: t("team:form.name.required"),
    }),
  });

  const createForm = useForm<z.infer<typeof createFormSchema>>({
    resolver: zodResolver(createFormSchema),
  });

  function onCreateFormSubmit(values: z.infer<typeof createFormSchema>) {
    if (!currentGame) return;

    setLoading(true);
    createTeam({
      game_id: currentGame.id!,
      ...values,
    })
      .then((res) => {
        toast.success(
          t("team:actions.create.success", { name: res?.team?.name })
        );
        onClose();
      })
      .finally(() => {
        sharedStore.setRefresh();
        setLoading(false);
      });
  }

  const joinFormSchema = z.object({
    token: z
      .string({
        message: t("team:form.invite_code.required"),
      })
      .regex(/^\d+:.*$/, {
        message: t("team:form.invite_code.invalid"),
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
      .then(() => {
        toast.success(t("team:actions.join.success"));
        onClose();
      })
      .catch(async (error) => {
        if (!(error instanceof HTTPError)) return;
        const body = await parseErrorResponse(error);

        if (error.response.status === StatusCodes.BAD_REQUEST) {
          toast.error(t("team:actions.join.error"), {
            description: formatApiMsg(body.msg),
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
      className={cn([
        "w-lg",
        "rounded-2xl",
        "shadow-lg",
        "overflow-hidden",
        "flex",
        "flex-col",
      ])}
    >
      {/* Tab bar */}
      <div
        className={cn([
          "flex",
          "p-2.5",
          "gap-1",
          "bg-muted/15",
          "border-b",
          "border-border",
        ])}
      >
        <Button
          variant={tab === "create" ? "tonal" : "ghost"}
          level="info"
          size="sm"
          className={cn(["flex-1"])}
          onClick={() => {
            setTab("create");
            createForm.clearErrors();
          }}
        >
          <UserPlusIcon className={cn(["size-4"])} />
          {t("team:actions.gather.create.title")}
        </Button>
        <Button
          variant={tab === "join" ? "tonal" : "ghost"}
          level="primary"
          size="sm"
          className={cn(["flex-1"])}
          onClick={() => {
            setTab("join");
            joinForm.clearErrors();
          }}
        >
          <LogInIcon className={cn(["size-4"])} />
          {t("team:actions.gather.join.title")}
        </Button>
      </div>

      {/* Content area */}
      <div className={cn(["p-6", "flex", "flex-col", "gap-6"])}>
        {tab === "create" ? (
          <>
            {/* Section header */}
            <div className={cn(["flex", "items-start", "gap-3.5"])}>
              <div
                className={cn([
                  "flex items-center justify-center",
                  "size-10 rounded-xl",
                  "bg-info/10 text-info",
                  "shadow-xs",
                  "shrink-0",
                ])}
              >
                <UserPlusIcon className={cn(["size-5"])} />
              </div>
              <div className={cn(["flex flex-col gap-1", "pt-0.5"])}>
                <h3 className={cn(["text-sm", "font-semibold", "text-foreground"])}>
                  {t("team:actions.gather.create.title")}
                </h3>
                <p className={cn(["text-xs", "text-muted-foreground/80", "leading-relaxed"])}>
                  {t("team:form.name.placeholder")}
                </p>
              </div>
            </div>

            {/* Form */}
            <Form {...createForm}>
              <form
                onSubmit={createForm.handleSubmit(onCreateFormSubmit)}
                autoComplete="off"
                className={cn([
                  "flex",
                  "flex-wrap",
                  "items-end",
                  "gap-3",
                  "sm:flex-nowrap",
                ])}
              >
                <FormField
                  control={createForm.control}
                  name="name"
                  render={({ field }) => (
                    <FormItem className="min-w-0 flex-1">
                      <FormLabel className="sr-only">
                        {t("team:form.name._")}
                      </FormLabel>
                      <FormControl>
                        <Field size="sm">
                          <FieldIcon>
                            <TypeIcon />
                          </FieldIcon>
                          <TextField
                            {...field}
                            value={field.value ?? ""}
                            onChange={field.onChange}
                            placeholder={t("team:form.name._")}
                          />
                        </Field>
                      </FormControl>
                      <FormMessage />
                    </FormItem>
                  )}
                />
                <Button
                  variant="solid"
                  level="info"
                  type="submit"
                  loading={loading}
                  className={cn(["shrink-0"])}
                >
                  {t("team:actions.gather.create._")}
                </Button>
              </form>
            </Form>
          </>
        ) : (
          <>
            {/* Section header */}
            <div className={cn(["flex", "items-start", "gap-3.5"])}>
              <div
                className={cn([
                  "flex items-center justify-center",
                  "size-10 rounded-xl",
                  "bg-primary/10 text-primary",
                  "shadow-xs",
                  "shrink-0",
                ])}
              >
                <LogInIcon className={cn(["size-5"])} />
              </div>
              <div className={cn(["flex flex-col gap-1", "pt-0.5"])}>
                <h3 className={cn(["text-sm", "font-semibold", "text-foreground"])}>
                  {t("team:actions.gather.join.title")}
                </h3>
                <p className={cn(["text-xs", "text-muted-foreground/80", "leading-relaxed"])}>
                  {t("team:form.invite_code.placeholder")}
                </p>
              </div>
            </div>

            {/* Form */}
            <Form {...joinForm}>
              <form
                onSubmit={joinForm.handleSubmit(onJoinFormSubmit)}
                autoComplete="off"
                className={cn([
                  "flex",
                  "flex-wrap",
                  "items-end",
                  "gap-3",
                  "sm:flex-nowrap",
                ])}
              >
                <FormField
                  control={joinForm.control}
                  name="token"
                  render={({ field }) => (
                    <FormItem className="min-w-0 flex-1">
                      <FormLabel className="sr-only">
                        {t("team:form.invite_code._")}
                      </FormLabel>
                      <FormControl>
                        <Field size="sm">
                          <FieldIcon>
                            <KeyIcon />
                          </FieldIcon>
                          <TextField
                            {...field}
                            value={field.value ?? ""}
                            onChange={field.onChange}
                            placeholder={t("team:form.invite_code._")}
                          />
                        </Field>
                      </FormControl>
                      <FormMessage />
                    </FormItem>
                  )}
                />
                <Button
                  variant="solid"
                  type="submit"
                  loading={loading}
                  className={cn(["shrink-0"])}
                >
                  {t("team:actions.gather.join._")}
                </Button>
              </form>
            </Form>
          </>
        )}
      </div>
    </Card>
  );
}

export { TeamGatheringDialog };
