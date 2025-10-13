import { enUS, ja, zhCN, zhHK } from "date-fns/locale";
import { useMemo } from "react";
import { useTranslation } from "react-i18next";

function useLocale() {
  const { i18n } = useTranslation();
  const locale = useMemo(() => {
    switch (i18n.language) {
      case "zh-CN":
        return zhCN;
      case "zh-TW":
        return zhHK;
      case "ja":
        return ja;
      default:
        return enUS;
    }
  }, [i18n.language]);

  return locale;
}

export { useLocale };
