import { zodResolver } from "@hookform/resolvers/zod";
import { AnsiUp } from "ansi_up";
import { StatusCodes } from "http-status-codes";
import { LayoutTemplateIcon, SaveIcon } from "lucide-react";
import { useContext, useEffect, useState } from "react";
import { useForm } from "react-hook-form";
import { toast } from "sonner";
import { z } from "zod";

import { Context } from "../context";

import leetChecker from "./examples/leet.cdsx?raw";
import simpleChecker from "./examples/simple.cdsx?raw";
import suidChecker from "./examples/suid.cdsx?raw";

import { updateChallengeChecker } from "@/api/admin/challenges/challenge_id/checker";
import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";
import { Editor } from "@/components/ui/editor";
import { Field, FieldIcon } from "@/components/ui/field";
import { Form, FormControl, FormField, FormItem } from "@/components/ui/form";
import { Label } from "@/components/ui/label";
import { Select } from "@/components/ui/select";
import { useSharedStore } from "@/storages/shared";
import { cn } from "@/utils";

const checkerMap = {
  simple: simpleChecker,
  suid: suidChecker,
  leet: leetChecker,
};

export default function Index() {
  const { challenge } = useContext(Context);
  const sharedStore = useSharedStore();
  const [_loading, setLoading] = useState<boolean>(false);
  const [lint, setLint] = useState<string>();

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
  }, [challenge, form.reset]);

  function onSubmit(values: z.infer<typeof formSchema>) {
    setLoading(true);
    updateChallengeChecker({
      id: challenge?.id,
      ...values,
    })
      .then((res) => {
        if (res.code === StatusCodes.OK) {
          toast.success(`题目 ${challenge?.title} 检查器更新成功`);
          setLint(res?.msg || "Success");
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
        className={cn(["flex", "flex-col", "flex-1", "gap-5"])}
      >
        <div className={cn(["flex", "justify-end", "gap-3", "items-center"])}>
          <Field size={"sm"} className={cn(["flex-1"])}>
            <FieldIcon>
              <LayoutTemplateIcon />
            </FieldIcon>
            <Select
              placeholder={"使用预设模板"}
              options={[
                {
                  value: "simple",
                  content: "简易固定字符串评判",
                },
                {
                  value: "suid",
                  content: "动态伪 UUID 评判",
                },
                {
                  value: "leet",
                  content: "动态 LEET 字符串评判",
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
            保存
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
                  lang={"rust"}
                  tabSize={4}
                  showLineNumbers
                  className={cn(["h-full", "min-h-64"])}
                />
              </FormControl>
            </FormItem>
          )}
        />
        <Label>Lint 输出</Label>
        <Card className={cn(["min-h-40", "p-3", "overflow-auto", "bg-input"])}>
          <pre
            className={cn(["font-mono"])}
            dangerouslySetInnerHTML={{
              __html: new AnsiUp().ansi_to_html(lint || ""),
            }}
          />
        </Card>
      </form>
    </Form>
  );
}
