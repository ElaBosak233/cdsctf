import { Turnstile } from "@marsidev/react-turnstile";
import { BotIcon, ImageIcon, RefreshCcwIcon } from "lucide-react";
import {
  createContext,
  type Ref,
  useCallback,
  useContext,
  useEffect,
  useImperativeHandle,
  useState,
} from "react";
import { useTranslation } from "react-i18next";

import { generateCaptcha } from "@/api/configs/captcha";
import { Field, FieldButton, FieldIcon } from "@/components/ui/field";
import { TextField } from "@/components/ui/text-field";
import { useRefresh } from "@/hooks/use-refresh";
import { useApperanceStore } from "@/storages/appearance";
import { useConfigStore } from "@/storages/config";
import { useSharedStore } from "@/storages/shared";
import { cn } from "@/utils";

export const Context = createContext<{
  refresh: number;
  setRefresh?: () => void;
}>({
  refresh: 0,
});

export type CaptchaRef = {
  refresh: () => void;
};

interface CaptchaProps {
  onChange: (captcha?: { id?: string; content?: string }) => void;
  ref?: Ref<CaptchaRef>;
}

export function Captcha(props: CaptchaProps) {
  const { onChange, ref } = props;
  const configStore = useConfigStore();
  const themeStore = useApperanceStore();
  const { tick, bump } = useRefresh();

  useImperativeHandle(ref, () => ({
    refresh: () => bump(),
  }));

  function renderCaptcha() {
    switch (configStore?.config?.captcha?.provider) {
      case "none":
        return null;
      case "turnstile":
        return (
          <Turnstile
            siteKey={String(configStore?.config?.captcha?.turnstile?.site_key)}
            onSuccess={(token) => onChange({ content: token })}
            options={{
              size: "flexible",
              theme: themeStore?.theme === "dark" ? "dark" : "light",
            }}
          />
        );
      case "pow":
        return <PowCaptcha onChange={onChange} />;
      case "image":
        return <ImageCaptcha onChange={onChange} />;
      default:
        return null;
    }
  }

  return (
    <Context.Provider
      value={{
        refresh: tick,
        setRefresh: bump,
      }}
    >
      {renderCaptcha()}
    </Context.Provider>
  );
}

function PowCaptcha(props: CaptchaProps) {
  const { onChange } = props;

  const { refresh, setRefresh } = useContext(Context);
  const [loading, setLoading] = useState<boolean>(false);
  const sharedStore = useSharedStore();

  const [result, setResult] = useState<string>("");
  const [id, setId] = useState<string>();

  useEffect(() => {
    void refresh;
    void sharedStore.refresh;

    const calculateWorker = new Worker(
      new URL("@/workers/pow.ts", import.meta.url),
      { type: "module" }
    );

    calculateWorker.onmessage = (e) => {
      const result = e.data;
      setResult(result);
      setLoading(false);
    };

    async function fetchCaptchaData() {
      setLoading(true);
      const res = await generateCaptcha();
      const d = Number(res.challenge?.split("#")[0]);
      const c = res.challenge?.split("#")[1];
      setId(res.id);

      calculateWorker.postMessage({ c, d });
    }

    fetchCaptchaData();

    return () => {
      calculateWorker.terminate();
    };
  }, [refresh, sharedStore.refresh]);

  useEffect(() => {
    onChange({
      id,
      content: result,
    });
  }, [id, result, onChange]);

  return (
    <Field>
      <FieldIcon>
        <BotIcon />
      </FieldIcon>
      <TextField readOnly disabled value={result} onChange={() => {}} />
      <FieldButton
        disabled={loading}
        onClick={() => setRefresh?.()}
        loading={loading}
        icon={<RefreshCcwIcon />}
      />
    </Field>
  );
}

function ImageCaptcha(props: CaptchaProps) {
  const { onChange } = props;
  const { t } = useTranslation();

  const sharedStore = useSharedStore();

  const { refresh, setRefresh } = useContext(Context);
  const [_loading, setLoading] = useState<boolean>(false);

  const [result, setResult] = useState<string>();
  const [id, setId] = useState<string>();
  const [challenge, setChallenge] = useState<string>();

  const fetchCaptchaData = useCallback(async () => {
    setLoading(true);
    const res = await generateCaptcha();
    setId(res.id);
    setChallenge(res.challenge);
  }, []);

  useEffect(() => {
    void refresh;
    void sharedStore.refresh;
    fetchCaptchaData();
  }, [fetchCaptchaData, refresh, sharedStore.refresh]);

  useEffect(() => {
    onChange({
      id,
      content: result,
    });
  }, [id, result, onChange]);

  return (
    <Field>
      <FieldIcon>
        <ImageIcon />
      </FieldIcon>
      <TextField
        value={result}
        onChange={(e) => setResult(e.target.value)}
        placeholder={t("common:captcha.placeholder")}
      />
      <FieldButton
        className={cn(["!aspect-auto", "w-20", "px-1"])}
        aria-label={t("common:refresh")}
        onClick={() => setRefresh?.()}
      >
        <img
          src={
            challenge
              ? `data:image/svg+xml;charset=utf-8,${encodeURIComponent(challenge)}`
              : undefined
          }
          alt=""
          className={cn(["h-10", "w-18", "object-contain"])}
          draggable={false}
        />
      </FieldButton>
    </Field>
  );
}
