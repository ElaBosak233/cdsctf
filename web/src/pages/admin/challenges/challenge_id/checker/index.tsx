import { zodResolver } from "@hookform/resolvers/zod";
import {
  CircleCheckIcon,
  CircleXIcon,
  LayoutTemplateIcon,
  LoaderCircleIcon,
  SaveIcon,
  TriangleAlertIcon,
} from "lucide-react";
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
import { Alert, AlertDescription } from "@/components/ui/alert";
import { Button } from "@/components/ui/button";
import { Editor } from "@/components/ui/editor";
import { Field, FieldIcon } from "@/components/ui/field";
import { Form, FormControl, FormField, FormItem } from "@/components/ui/form";
import { Select } from "@/components/ui/select";
import { useDebounce } from "@/hooks/use-debounce";
import { useSharedStore } from "@/storages/shared";
import { cn } from "@/utils";
import { Context } from "../context";

import leetChecker from "./_blocks/examples/leet.lua?raw";
import leetCustomKeyChecker from "./_blocks/examples/leet-custom-key.lua?raw";
import regexChecker from "./_blocks/examples/regex.lua?raw";
import simpleChecker from "./_blocks/examples/simple.lua?raw";
import suidChecker from "./_blocks/examples/suid.lua?raw";
import suidCustomKeyChecker from "./_blocks/examples/suid-custom-key.lua?raw";

const checkerMap = {
  simple: simpleChecker,
  regex: regexChecker,
  suid: suidChecker,
  suid_custom_key: suidCustomKeyChecker,
  leet: leetChecker,
  leet_custom_key: leetCustomKeyChecker,
};

type CheckerTemplate = keyof typeof checkerMap;

export default function Index() {
  const { t } = useTranslation();

  const { challenge } = useContext(Context);
  const sharedStore = useSharedStore();
  const [_loading, setLoading] = useState<boolean>(false);
  const [lint, setLint] = useState<Array<DiagnosticMarker>>();
  const [lintState, setLintState] = useState<
    "idle" | "checking" | "valid" | "invalid" | "error"
  >("idle");

  const formSchema = z.object({
    checker: z.string({
      message: t("challenge:checker.form.script_required"),
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
      .then(() => {
        toast.success(
          t("challenge:checker.actions.update_success", {
            title: challenge?.title,
          })
        );
      })
      .finally(() => {
        sharedStore.setRefresh();
        setLoading(false);
      });
  }

  const checker = form.watch("checker");
  const debouncedChecker = useDebounce(checker, 500);

  useEffect(() => {
    let active = true;
    if (!debouncedChecker || challenge?.id == null) {
      setLint([]);
      setLintState("idle");
      return () => {
        active = false;
      };
    }

    setLintState("checking");
    lintChallengeChecker({
      id: challenge.id,
      checker: debouncedChecker,
    })
      .then((res) => {
        if (active) {
          setLint(res.markers);
          setLintState(res.markers.length === 0 ? "valid" : "invalid");
        }
      })
      .catch(() => {
        if (active) {
          setLint([]);
          setLintState("error");
        }
      });

    return () => {
      active = false;
    };
  }, [challenge?.id, debouncedChecker]);

  const isLintPending = checker !== debouncedChecker;
  const displayedLintState = isLintPending ? "checking" : lintState;
  const lintStatusMessage = {
    idle: t("challenge:checker.lint.idle"),
    checking: t("challenge:checker.lint.checking"),
    valid: t("challenge:checker.lint.valid"),
    invalid: t("challenge:checker.lint.invalid", {
      count: lint?.length ?? 0,
    }),
    error: t("challenge:checker.lint.error"),
  }[displayedLintState];

  return (
    <Form {...form}>
      <form
        onSubmit={form.handleSubmit(onSubmit)}
        autoComplete={"off"}
        className={cn(["flex", "flex-col", "flex-1", "gap-2"])}
      >
        <div className={cn(["flex", "justify-end", "gap-3", "items-center"])}>
          <Field size={"sm"} className={cn(["flex-1"])}>
            <FieldIcon>
              <LayoutTemplateIcon />
            </FieldIcon>
            <Select
              placeholder={t("challenge:checker.templates._")}
              options={[
                {
                  value: "simple",
                  content: t("challenge:checker.templates.simple"),
                },
                {
                  value: "regex",
                  content: t("challenge:checker.templates.regex"),
                },
                {
                  value: "suid",
                  content: t("challenge:checker.templates.suid"),
                },
                {
                  value: "suid_custom_key",
                  content: t("challenge:checker.templates.suid_custom_key"),
                },
                {
                  value: "leet",
                  content: t("challenge:checker.templates.leet"),
                },
                {
                  value: "leet_custom_key",
                  content: t("challenge:checker.templates.leet_custom_key"),
                },
              ]}
              onValueChange={(value: CheckerTemplate) => {
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
            {t("common:actions.save")}
          </Button>
        </div>
        <Alert
          data-testid="checker-lint-status"
          className={cn([
            "h-7",
            "min-h-7",
            "shrink-0",
            "rounded-md",
            "px-2.5",
            "py-0",
            "grid-cols-[auto_1fr]",
            "items-center",
            "gap-x-1.5",
            "gap-y-0",
            "has-[>svg]:grid-cols-[14px_1fr]",
            "[&>svg]:size-3.5",
            "[&>svg]:translate-y-0",
            displayedLintState === "idle" && [
              "border-border",
              "bg-muted/30",
              "text-muted-foreground",
            ],
            displayedLintState === "checking" && [
              "border-primary/30",
              "bg-primary/10",
              "text-primary",
            ],
            displayedLintState === "valid" && [
              "border-success/40",
              "bg-success/10",
              "text-success",
            ],
            displayedLintState === "invalid" && [
              "border-warning/40",
              "bg-warning/10",
              "text-warning",
            ],
            displayedLintState === "error" && [
              "border-error/40",
              "bg-error/10",
              "text-error",
            ],
          ])}
        >
          {displayedLintState === "checking" ? (
            <LoaderCircleIcon className={cn(["animate-spin"])} />
          ) : displayedLintState === "valid" ? (
            <CircleCheckIcon />
          ) : displayedLintState === "invalid" ? (
            <TriangleAlertIcon />
          ) : displayedLintState === "error" ? (
            <CircleXIcon />
          ) : null}
          <AlertDescription
            className={cn([
              "self-center",
              "min-h-0",
              "text-inherit",
              "text-xs",
              "font-normal",
              "leading-none",
            ])}
          >
            {lintStatusMessage}
          </AlertDescription>
        </Alert>
        <FormField
          control={form.control}
          name={"checker"}
          render={({ field }) => (
            <FormItem className={cn(["flex-1", "flex", "flex-col"])}>
              <FormControl>
                <Editor
                  {...field}
                  value={field.value ?? ""}
                  lang={"lua"}
                  tabSize={4}
                  showLineNumbers
                  className={cn(["h-full", "min-h-120"])}
                  diagnostics={isLintPending ? [] : lint}
                />
              </FormControl>
            </FormItem>
          )}
        />
      </form>
    </Form>
  );
}
