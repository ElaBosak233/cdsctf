import { HTTPError } from "ky";
import { useState } from "react";
import { useTranslation } from "react-i18next";
import { z } from "zod";
import { useInterval } from "@/hooks/use-interval";
import { getApiErrorCode } from "@/utils/query";

type Delivery =
  | { status: "requested" | "cooldown"; retryAt: number }
  | { status: "failed" };

const cooldownError = z.object({
  code: z.literal("email_send_too_frequently"),
  details: z.object({ retry_after_seconds: z.number().positive() }),
});

export function useEmailVerification(
  address: string,
  sendCode: (request: {
    email: string;
  }) => Promise<{ retry_after_seconds: number }>
) {
  const { t } = useTranslation();
  const email = address.trim().toLowerCase();
  const [attempt, setAttempt] = useState<{
    email: string;
    delivery: Delivery;
  }>();
  const [sending, setSending] = useState(false);
  const [now, setNow] = useState(Date.now);
  const delivery = attempt?.email === email ? attempt.delivery : undefined;
  const seconds =
    delivery && "retryAt" in delivery
      ? Math.max(0, Math.ceil((delivery.retryAt - now) / 1000))
      : 0;

  useInterval(() => setNow(Date.now()), 1000, [], { enabled: seconds > 0 });

  async function request(handleError?: (error: unknown) => boolean) {
    setSending(true);
    try {
      const response = await sendCode({ email });
      const sentAt = Date.now();
      setNow(sentAt);
      setAttempt({
        email,
        delivery: {
          status: "requested",
          retryAt: sentAt + response.retry_after_seconds * 1000,
        },
      });
      return true;
    } catch (error) {
      const cooldown = cooldownError.safeParse(
        error instanceof HTTPError ? error.data : undefined
      );
      if (cooldown.success) {
        const receivedAt = Date.now();
        setNow(receivedAt);
        setAttempt({
          email,
          delivery: {
            status: "cooldown",
            retryAt:
              receivedAt + cooldown.data.details.retry_after_seconds * 1000,
          },
        });
      } else {
        setAttempt(
          handleError?.(error)
            ? undefined
            : { email, delivery: { status: "failed" } }
        );
      }
    } finally {
      setSending(false);
    }
    return false;
  }

  function codeErrorMessage(error: unknown) {
    const code =
      error instanceof HTTPError ? getApiErrorCode(error.data) : undefined;
    switch (code) {
      case "email_code_required":
        return t("user:emails.verify_code_required");
      case "email_code_incorrect":
        return t("account:errors.email_code_incorrect");
      case "email_code_expired":
        return t("account:errors.email_code_expired.title");
      default:
        return undefined;
    }
  }

  const sendLabel =
    seconds > 0
      ? t("user:emails.actions.send_verify.retry_in", { seconds })
      : t(
          delivery && delivery.status !== "failed"
            ? "user:emails.actions.send_verify.resend"
            : "user:emails.actions.send_verify._"
        );
  const feedback =
    delivery &&
    t(
      `user:emails.actions.send_verify.${
        {
          requested: "success",
          cooldown: "cooldown",
          failed: "failed",
        }[delivery.status]
      }`
    );

  return {
    sending,
    coolingDown: seconds > 0,
    sendLabel,
    feedback,
    failed: delivery?.status === "failed",
    request,
    codeErrorMessage,
  };
}
