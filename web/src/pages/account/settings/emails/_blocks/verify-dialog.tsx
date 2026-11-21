import { zodResolver } from "@hookform/resolvers/zod";
import { CheckIcon, KeyRoundIcon, MailCheckIcon, SendIcon } from "lucide-react";
import { useForm } from "react-hook-form";
import { useTranslation } from "react-i18next";
import { toast } from "sonner";
import { z } from "zod";
import { sendVerifyEmail, verifyEmail } from "@/api/users/me/emails";
import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";
import { DialogBody, DialogFooter, DialogHeader } from "@/components/ui/dialog";
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
import { useEmailVerification } from "@/hooks/use-email-verification";
import { patchAuthenticatedUser } from "@/storages/auth";
import { useConfigStore } from "@/storages/config";
import { cn } from "@/utils";
import { notifyApiError } from "@/utils/query";

interface VerifyDialogProps {
  email: string;
  onClose: () => void;
  bump: () => void;
}

export function VerifyDialog({ email, onClose, bump }: VerifyDialogProps) {
  const { t } = useTranslation();
  const config = useConfigStore((state) => state.config);
  const requiresCode = config?.email?.enabled;
  const verification = useEmailVerification(email, sendVerifyEmail);

  const schema = z.object({
    code: z
      .string()
      .trim()
      .refine((value) => !requiresCode || value.length > 0, {
        message: t("user:emails.verify_code_required"),
      }),
  });
  const form = useForm<z.infer<typeof schema>>({
    resolver: zodResolver(schema),
    defaultValues: { code: "" },
  });

  async function handleVerify({ code }: z.infer<typeof schema>) {
    try {
      await verifyEmail({ email, code });
      patchAuthenticatedUser({ verified: true });
      toast.success(t("user:emails.actions.verify.success", { email }));
      bump();
      onClose();
    } catch (error) {
      const message = verification.codeErrorMessage(error);
      if (message) {
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
        icon={<MailCheckIcon />}
        title={t("user:emails.actions.verify._")}
        description={email}
        descriptionClassName="break-all"
      />
      <DialogBody className="p-5 pt-4">
        <Form {...form}>
          <form
            onSubmit={form.handleSubmit(handleVerify)}
            className="flex flex-col gap-3"
          >
            {requiresCode ? (
              <>
                {verification.feedback && (
                  <p
                    role="status"
                    className={cn(
                      "text-sm",
                      verification.failed
                        ? "text-error"
                        : "text-muted-foreground"
                    )}
                  >
                    {verification.feedback}
                  </p>
                )}
                <FormField
                  control={form.control}
                  name="code"
                  render={({ field }) => (
                    <FormItem>
                      <FormLabel>
                        {t("user:emails.verify_code_label")}
                      </FormLabel>
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
                            placeholder={t(
                              "user:emails.verify_code_placeholder"
                            )}
                          />
                        </FormControl>
                      </Field>
                      <FormMessage className="static block" />
                    </FormItem>
                  )}
                />
              </>
            ) : (
              <p className="text-muted-foreground text-sm">
                {t("user:emails.actions.verify.disabled", {
                  title: config?.meta?.title,
                })}
              </p>
            )}
            <DialogFooter className="flex-wrap">
              {requiresCode && (
                <Button
                  type="button"
                  variant="outline"
                  size="sm"
                  icon={<SendIcon />}
                  loading={verification.sending}
                  disabled={
                    verification.coolingDown || form.formState.isSubmitting
                  }
                  onClick={() => verification.request()}
                >
                  {verification.sendLabel}
                </Button>
              )}
              <Button
                type="submit"
                variant="solid"
                size="sm"
                level="success"
                icon={<CheckIcon />}
                loading={form.formState.isSubmitting}
                disabled={verification.sending}
              >
                {t("user:emails.actions.verify._")}
              </Button>
            </DialogFooter>
          </form>
        </Form>
      </DialogBody>
    </Card>
  );
}
