import { useQuery, useQueryClient } from "@tanstack/react-query";
import { HTTPError } from "ky";
import { useTranslation } from "react-i18next";
import { z } from "zod";
import { getConfigs } from "@/api/configs";
import { isEmailDomainAllowed } from "@/utils/email-domain";
import { getApiErrorCode } from "@/utils/query";

export function useEmailValidation(invalidMessage: string) {
  const { t } = useTranslation();
  const queryClient = useQueryClient();
  const { data: emailConfig } = useQuery({
    queryKey: ["configs"],
    queryFn: getConfigs,
    select: ({ config }) => config.email,
  });

  const whitelist = emailConfig?.whitelist;
  const message = t("common:errors.email_domain_not_allowed");
  const schema = z.email(invalidMessage).pipe(
    z.string().refine((address) => isEmailDomainAllowed(address, whitelist), {
      message,
    })
  );

  function handleServerError(
    error: unknown,
    setError: (name: "email", error: { type: string; message: string }) => void
  ) {
    const code =
      error instanceof HTTPError ? getApiErrorCode(error.data) : undefined;
    if (code !== "email_domain_not_allowed") return false;

    setError("email", { type: "server", message });
    void queryClient.invalidateQueries({ queryKey: ["configs"] });
    return true;
  }

  return {
    schema,
    whitelist,
    verificationEnabled: emailConfig?.enabled,
    handleServerError,
  };
}
