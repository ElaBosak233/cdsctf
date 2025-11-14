import { zodResolver } from "@hookform/resolvers/zod";
import { StatusCodes } from "http-status-codes";
import { CalendarIcon, CheckIcon, FlagIcon, TypeIcon } from "lucide-react";
import { useState } from "react";
import { useForm } from "react-hook-form";
import { useTranslation } from "react-i18next";
import { toast } from "sonner";
import { z } from "zod";
import { createGame } from "@/api/admin/games";
import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";
import { DateTimePicker } from "@/components/ui/datetime-picker";
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
import { useSharedStore } from "@/storages/shared";
import { cn } from "@/utils";

interface CreateDialogProps {
  onClose: () => void;
}

function CreateDialog(props: CreateDialogProps) {
  const { onClose } = props;
  const { t } = useTranslation();

  const sharedStore = useSharedStore();
  const [loading, setLoading] = useState<boolean>(false);
  const formSchema = z.object({
    title: z.string({
      message: t("game.form.title.message"),
    }),
    started_at: z.date({
      message: t("game.form.started_at.message"),
    }),
    ended_at: z.date({
      message: t("game.form.ended_at.message"),
    }),
  });

  const form = useForm<z.infer<typeof formSchema>>({
    resolver: zodResolver(formSchema),
    defaultValues: {},
  });

  function onSubmit(values: z.infer<typeof formSchema>) {
    setLoading(true);
    createGame({
      ...values,
      description: "",
      is_need_write_up: false,
      is_public: false,
      started_at: Math.floor(values.started_at.getTime() / 1000),
      ended_at: Math.floor(values.ended_at.getTime() / 1000),
    })
      .then((res) => {
        if (res.code === StatusCodes.OK) {
          toast.success(
            t("game.actions.create.success", { title: res?.data?.title })
          );
          onClose();
        }
      })
      .finally(() => {
        sharedStore.setRefresh();
        setLoading(false);
      });
  }
  return (
    <Card
      className={cn(["w-lg", "min-h-64", "p-5", "flex", "flex-col", "gap-5"])}
    >
      <h3 className={cn(["flex", "gap-3", "items-center", "text-md"])}>
        <FlagIcon className={cn(["size-4"])} />
        {t("game.actions.create._")}
      </h3>
      <Form {...form}>
        <form
          onSubmit={form.handleSubmit(onSubmit)}
          autoComplete={"off"}
          className={cn(["flex", "flex-col", "flex-1", "gap-5"])}
        >
          <FormField
            control={form.control}
            name={"title"}
            render={({ field }) => (
              <FormItem>
                <FormLabel>{t("game.form.title._")}</FormLabel>
                <FormControl>
                  <Field size={"sm"}>
                    <FieldIcon>
                      <TypeIcon />
                    </FieldIcon>
                    <TextField
                      {...field}
                      placeholder={"My CTF Game"}
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
            name={"started_at"}
            render={({ field }) => (
              <FormItem>
                <FormLabel>{t("game.form.started_at._")}</FormLabel>
                <FormControl>
                  <Field size={"sm"}>
                    <FieldIcon>
                      <CalendarIcon />
                    </FieldIcon>
                    <DateTimePicker {...field} clearable />
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
                <FormLabel>{t("game.form.ended_at._")}</FormLabel>
                <FormControl>
                  <Field size={"sm"}>
                    <FieldIcon>
                      <CalendarIcon />
                    </FieldIcon>
                    <DateTimePicker {...field} clearable />
                  </Field>
                </FormControl>
                <FormMessage />
              </FormItem>
            )}
          />
          <Button
            type={"submit"}
            variant={"solid"}
            icon={<CheckIcon />}
            level={"success"}
            loading={loading}
          >
            {t("common.actions.confirm")}
          </Button>
        </form>
      </Form>
    </Card>
  );
}
export { CreateDialog };
