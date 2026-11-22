import { zodResolver } from "@hookform/resolvers/zod";
import { useQueryClient } from "@tanstack/react-query";
import { HTTPError } from "ky";
import {
  CheckIcon,
  KeyRoundIcon,
  MailIcon,
  MailPlusIcon,
  SendIcon,
} from "lucide-react";
import { useForm, useWatch } from "react-hook-form";
import { useTranslation } from "react-i18next";
import { toast } from "sonner";
import z from "zod";
import { addEmail, sendBindEmail } from "@/api/users/me/emails";
import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";
import { DialogBody, DialogFooter, DialogHeader } from "@/components/ui/dialog";
import { Field, FieldButton, FieldIcon } from "@/components/ui/field";
import {
  Form,
  FormControl,
  FormDescription,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "@/components/ui/form";
import { TextField } from "@/components/ui/text-field";
import { EmailDomainHint } from "@/components/widgets/email-domain-hint";
import { useEmailValidation } from "@/hooks/use-email-validation";
import { useEmailVerification } from "@/hooks/use-email-verification";
import { sessionQueryKey } from "@/storages/auth";
import { useConfigStore } from "@/storages/config";
import { cn } from "@/utils";
import { getApiErrorCode, notifyApiError } from "@/utils/query";

interface CreateDialogProps {
  onClose: () => void;
  bump: () => void;
}

export function CreateDialog({ onClose, bump }: CreateDialogProps) {
  const { t } = useTranslation();
  const queryClient = useQueryClient();
  const cachedVerification = useConfigStore(
    (state) => state.config?.email?.enabled
  );
  const emailValidation = useEmailValidation(
    t("user:emails.form.email.message")
  );
  const requiresVerification =
    emailValidation.verificationEnabled ?? cachedVerification;
  const schema = z.object({
    email: emailValidation.schema,
    code: z
      .string()
      .trim()
      .refine((value) => !requiresVerification || value.length > 0, {
        message: t("user:emails.verify_code_required"),
      }),
  });
  const form = useForm<z.infer<typeof schema>>({
    resolver: zodResolver(schema),
    defaultValues: { email: "", code: "" },
    mode: "onChange",
  });
  const email = useWatch({ control: form.control, name: "email" });
  const verification = useEmailVerification(email, sendBindEmail);
  const submitting = form.formState.isSubmitting;

  function handleEmailError(error: unknown) {
    if (emailValidation.handleServerError(error, form.setError)) return true;
    if (
      error instanceof HTTPError &&
      getApiErrorCode(error.data) === "email_already_exists"
    ) {
      form.setError("email", {
        type: "server",
        message: t("account:errors.email_already_exists"),
      });
      return true;
    }
    return false;
  }

  async function requestCode() {
    const address = form.getValues("email");
    if (
      !(await form.trigger("email", { shouldFocus: true })) ||
      form.getValues("email") !== address
    ) {
      return;
    }
    if (await verification.request(handleEmailError)) {
      form.resetField("code");
      form.setFocus("code");
    }
  }

  async function onSubmit(values: z.infer<typeof schema>) {
    try {
      const { email } = await addEmail({
        email: values.email,
        code: requiresVerification ? values.code : undefined,
      });
      if (email.verified)
        await queryClient.invalidateQueries({ queryKey: sessionQueryKey });
      toast.success(
        t("user:emails.actions.create.success", { email: email.email })
      );
      bump();
      onClose();
    } catch (error) {
      if (handleEmailError(error)) return;
      const message = verification.codeErrorMessage(error);
      if (message && requiresVerification) {
        form.setError("code", { type: "server", message });
        return;
      }
      await notifyApiError(error);
    }
  }

  return (
    <Card className="flex w-full max-w-xl flex-col overflow-hidden rounded-elevated shadow-lg">
      <DialogHeader
        className="p-5 pb-0"
        icon={<MailPlusIcon />}
        title={t("user:emails.actions.create._")}
      />
      <DialogBody className="p-5 pt-4">
        <Form {...form}>
          <form
            onSubmit={form.handleSubmit(onSubmit)}
            autoComplete="off"
            className="flex flex-col gap-4"
          >
            <FormField
              control={form.control}
              name="email"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>{t("user:emails.form.email._")}</FormLabel>
                  <Field
                    size="sm"
                    disabled={verification.sending || submitting}
                  >
                    <FieldIcon>
                      <MailIcon />
                    </FieldIcon>
                    <FormControl>
                      <TextField
                        {...field}
                        inputMode="email"
                        autoCapitalize="none"
                        spellCheck={false}
                        placeholder={t("user:emails.form.email.placeholder")}
                        onChange={(event) => {
                          if (
                            event.target.value.toLowerCase() !==
                            field.value.toLowerCase()
                          ) {
                            form.resetField("code");
                          }
                          field.onChange(event);
                        }}
                      />
                    </FormControl>
                  </Field>
                  <EmailDomainHint whitelist={emailValidation.whitelist} />
                  <FormMessage className="static block" />
                </FormItem>
              )}
            />
            {requiresVerification && (
              <FormField
                control={form.control}
                name="code"
                render={({ field }) => (
                  <FormItem>
                    <FormLabel>{t("user:emails.verify_code_label")}</FormLabel>
                    <Field size="sm">
                      <FieldIcon>
                        <KeyRoundIcon />
                      </FieldIcon>
                      <FormControl>
                        <TextField
                          {...field}
                          autoComplete="one-time-code"
                          autoCapitalize="none"
                          spellCheck={false}
                          placeholder={t("user:emails.verify_code_placeholder")}
                        />
                      </FormControl>
                      <FieldButton
                        icon={<SendIcon />}
                        className="aspect-auto shrink-0"
                        onClick={requestCode}
                        loading={verification.sending}
                        disabled={
                          submitting ||
                          verification.coolingDown ||
                          !emailValidation.schema.safeParse(email).success
                        }
                      >
                        {verification.sendLabel}
                      </FieldButton>
                    </Field>
                    {verification.feedback && (
                      <FormDescription
                        role="status"
                        className={cn(verification.failed && "text-error")}
                      >
                        {verification.feedback}
                      </FormDescription>
                    )}
                    <FormMessage className="static block" />
                  </FormItem>
                )}
              />
            )}
            <DialogFooter>
              <Button
                icon={<CheckIcon />}
                level="success"
                variant="solid"
                size="sm"
                type="submit"
                loading={submitting}
                disabled={verification.sending}
              >
                {t("user:emails.actions.create._")}
              </Button>
            </DialogFooter>
          </form>
        </Form>
      </DialogBody>
    </Card>
  );
}
