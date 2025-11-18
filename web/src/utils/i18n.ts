import i18n from "i18next";
import LanguageDetector from "i18next-browser-languagedetector";
import Backend from "i18next-http-backend";
import yaml from "js-yaml";
import { initReactI18next } from "react-i18next";

i18n
  .use(Backend)
  .use(LanguageDetector)
  .use(initReactI18next)
  .init({
    fallbackLng: {
      en: ["en-US"],
      zh: ["zh-CN"],
      default: ["en-US"],
    },
    ns: [
      "common",
      "sigtrap",
      "account",
      "user",
      "about",
      "challenge",
      "game",
      "team",
      "submission",
      "env",
      "admin",
    ],
    defaultNS: false,
    load: "currentOnly",
    nsSeparator: ".",
    backend: {
      loadPath: "/locales/{{lng}}/{{ns}}.yaml",
      parse: (data: string) => yaml.load(data),
    },
    interpolation: {
      escapeValue: false,
    },
    debug: process.env.NODE_ENV === "development",
  });
