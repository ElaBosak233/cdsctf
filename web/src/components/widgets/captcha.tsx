import HCaptcha from "@hcaptcha/react-hcaptcha";
import { Turnstile } from "@marsidev/react-turnstile";
import CryptoJS from "crypto-js";
import { BotIcon, ImageIcon, RefreshCcwIcon } from "lucide-react";
import {
  createContext,
  type Ref,
  useContext,
  useEffect,
  useImperativeHandle,
  useState,
} from "react";

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

type CaptchaProps = {
  onChange: (captcha?: { id?: string; content?: string }) => void;
  ref?: Ref<CaptchaRef>;
};

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
      case "hcaptcha":
        return (
          <HCaptcha
            sitekey={String(configStore?.config?.captcha?.hcaptcha?.site_key)}
            onVerify={(token) => onChange({ content: token })}
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
      const d = Number(res.data?.challenge?.split("#")[0]);
      const c = res.data?.challenge?.split("#")[1];
      setId(res.data?.id);

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
  }, [id, result]);

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
  const sharedStore = useSharedStore();

  const { refresh, setRefresh } = useContext(Context);
  const [_loading, setLoading] = useState<boolean>(false);

  const [result, setResult] = useState<string>();
  const [id, setId] = useState<string>();
  const [challenge, setChallenge] = useState<string>();

  async function fetchCaptchaData() {
    setLoading(true);
    const res = await generateCaptcha();
    setId(res.data?.id);
    setChallenge(res.data?.challenge);
  }

  useEffect(() => {
    fetchCaptchaData();
  }, [refresh, sharedStore.refresh]);

  useEffect(() => {
    onChange({
      id,
      content: result,
    });
  }, [id, result]);

  return (
    <div className={cn(["flex", "items-center", "gap-2"])}>
      <Field className={cn(["flex-1"])}>
        <FieldIcon>
          <ImageIcon />
        </FieldIcon>
        <TextField
          value={result}
          onChange={(e) => setResult(e.target.value)}
          placeholder={"验证码"}
        />
      </Field>
      <img
        src={`data:image/svg+xml;base64,${CryptoJS.enc.Base64.stringify(CryptoJS.enc.Utf8.parse(String(challenge)))}`}
        alt={"captcha"}
        onClick={() => setRefresh?.()}
        draggable={false}
        style={{
          height: 40,
          width: 60,
        }}
      />
    </div>
  );
}
