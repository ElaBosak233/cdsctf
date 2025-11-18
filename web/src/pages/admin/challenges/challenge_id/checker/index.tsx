import { zodResolver } from "@hookform/resolvers/zod";
import { StatusCodes } from "http-status-codes";
import { LayoutTemplateIcon, SaveIcon } from "lucide-react";
import { useContext, useEffect, useState } from "react";
import { useForm } from "react-hook-form";
import { useTranslation } from "react-i18next";
import { toast } from "sonner";
import { z } from "zod";
import {
  type DiagnosticMarker,
  lintChallengeChecker,
  updateChallengeChecker,
} from "@/api/admin/challenges/challenge_id/checker";
import { Button } from "@/components/ui/button";
import { Editor } from "@/components/ui/editor";
import { Field, FieldIcon } from "@/components/ui/field";
import { Form, FormControl, FormField, FormItem } from "@/components/ui/form";
import { Select } from "@/components/ui/select";
import { useDebounce } from "@/hooks/use-debounce";
import { useSharedStore } from "@/storages/shared";
import { cn } from "@/utils";
import { Context } from "../context";
import leetChecker from "./examples/leet.cdsx?raw";
import simpleChecker from "./examples/simple.cdsx?raw";
import suidChecker from "./examples/suid.cdsx?raw";

const checkerMap = {
  simple: simpleChecker,
  suid: suidChecker,
  leet: leetChecker,
};

export default function Index() {
  const { t } = useTranslation();

  const { challenge } = useContext(Context);
  const sharedStore = useSharedStore();
  const [_loading, setLoading] = useState<boolean>(false);
  const [lint, setLint] = useState<Array<DiagnosticMarker>>();

  const formSchema = z.object({
    checker: z.string({
      message: "请为检查器编写脚本",
    }),
  });

  const form = useForm<z.infer<typeof formSchema>>({
    resolver: zodResolver(formSchema),
    defaultValues: {
      checker: challenge?.checker || "",
    },
  });

  useEffect(() => {
    form.reset({
      checker: challenge?.checker,
    });
  }, [challenge, form]);

  function onSubmit(values: z.infer<typeof formSchema>) {
    setLoading(true);
    updateChallengeChecker({
      id: challenge?.id,
      ...values,
    })
      .then((res) => {
        if (res.code === StatusCodes.OK) {
          toast.success(`题目 ${challenge?.title} 检查器更新成功`);
        }
      })
      .finally(() => {
        sharedStore.setRefresh();
        setLoading(false);
      });
  }

  const debouncedChecker = useDebounce(form.watch("checker"), 500);

  useEffect(() => {
    if (debouncedChecker) {
      lintChallengeChecker({
        id: challenge?.id,
        checker: debouncedChecker,
      }).then((res) => {
        setLint(res.data);
      });
    }
  }, [challenge?.id, debouncedChecker]);

  return (
    <Form {...form}>
      <form
        onSubmit={form.handleSubmit(onSubmit)}
        autoComplete={"off"}
        className={cn(["flex", "flex-col", "flex-1", "gap-5"])}
      >
        <div className={cn(["flex", "justify-end", "gap-3", "items-center"])}>
          <Field size={"sm"} className={cn(["flex-1"])}>
            <FieldIcon>
              <LayoutTemplateIcon />
            </FieldIcon>
            <Select
              placeholder={t("challenge.checker.templates._")}
              options={[
                {
                  value: "simple",
                  content: t("challenge.checker.templates.simple"),
                },
                {
                  value: "suid",
                  content: t("challenge.checker.templates.suid"),
                },
                {
                  value: "leet",
                  content: t("challenge.checker.templates.leet"),
                },
              ]}
              onValueChange={(value: "simple" | "suid" | "leet") => {
                form.setValue("checker", checkerMap[value]);
              }}
            />
          </Field>

          <Button
            type={"submit"}
            variant={"solid"}
            size={"sm"}
            icon={<SaveIcon />}
          >
            {t("common.actions.save")}
          </Button>
        </div>
        <FormField
          control={form.control}
          name={"checker"}
          render={({ field }) => (
            <FormItem className={cn(["flex-1", "flex", "flex-col"])}>
              <FormControl>
                <Editor
                  {...field}
                  value={field.value ?? ""}
                  lang={"rune"}
                  tabSize={4}
                  showLineNumbers
                  className={cn(["h-full", "min-h-120"])}
                  diagnostics={lint}
                />
              </FormControl>
            </FormItem>
          )}
        />
      </form>
    </Form>
  );
}
