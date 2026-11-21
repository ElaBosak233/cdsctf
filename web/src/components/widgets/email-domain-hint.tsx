import { useTranslation } from "react-i18next";
import { FormDescription } from "@/components/ui/form";

export function EmailDomainHint({ whitelist }: { whitelist?: string[] }) {
  const { t } = useTranslation();
  if (!whitelist?.length) return null;

  return (
    <FormDescription className="wrap-anywhere">
      {t("common:email_allowed_domains", { domains: whitelist.join(", ") })}
    </FormDescription>
  );
}
