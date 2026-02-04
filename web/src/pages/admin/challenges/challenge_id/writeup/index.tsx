import { zodResolver } from "@hookform/resolvers/zod";
import { StatusCodes } from "http-status-codes";
import { SaveIcon } from "lucide-react";
import { useContext, useEffect, useState } from "react";
import { useForm } from "react-hook-form";
import { useTranslation } from "react-i18next";
import { toast } from "sonner";
import { z } from "zod";
import { updateWriteup } from "@/api/admin/challenges/challenge_id/writeup";
import { Button } from "@/components/ui/button";
import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormMessage,
} from "@/components/ui/form";
import { MarkdownEditor } from "@/components/ui/markdown-editor";
import { useSharedStore } from "@/storages/shared";
import { cn } from "@/utils";
import { Context } from "../context";

export default function Index() {
  const { t } = useTranslation();

  const { challenge } = useContext(Context);
  const sharedStore = useSharedStore();

  const [loading, setLoading] = useState<boolean>(false);

  const formSchema = z.object({
    writeup: z.string(),
  });

  const form = useForm<z.infer<typeof formSchema>>({
    resolver: zodResolver(formSchema),
    defaultValues: {
      writeup: challenge?.writeup ?? "",
    },
  });

  useEffect(() => {
    form.reset(
      {
        writeup: challenge?.writeup ?? "",
      },
      {
        keepDefaultValues: false,
      }
    );
  }, [challenge, form]);

  function onSubmit(values: z.infer<typeof formSchema>) {
    setLoading(true);
    updateWriteup({
      id: challenge?.id,
      writeup: values.writeup,
    })
      .then((res) => {
        if (res.code === StatusCodes.OK) {
          toast.success(`题目 ${res?.data?.title} 题解更新成功`);
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
        <FormField
          control={form.control}
          name={"writeup"}
          render={({ field }) => (
            <FormItem className={cn(["flex-1", "flex", "flex-col"])}>
              <FormControl>
                <MarkdownEditor
                  {...field}
                  placeholder={"Once upon a time..."}
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
          icon={<SaveIcon />}
          loading={loading}
        >
          {t("common.actions.save")}
        </Button>
      </form>
    </Form>
  );
}
