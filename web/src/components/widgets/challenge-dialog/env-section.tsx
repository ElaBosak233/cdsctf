import { StatusCodes } from "http-status-codes";
import { HTTPError } from "ky";
import {
  ClipboardCheckIcon,
  ClipboardIcon,
  ClockIcon,
  EthernetPortIcon,
  PlayIcon,
  TrashIcon,
} from "lucide-react";
import { useContext, useEffect, useMemo, useState } from "react";
import { toast } from "sonner";

import { createEnv, getEnvs } from "@/api/envs";
import { renewEnv, stopEnv } from "@/api/envs/env_id";
import { Button } from "@/components/ui/button";
import { Field, FieldButton, FieldIcon } from "@/components/ui/field";
import { TextField } from "@/components/ui/text-field";
import { useClipboard } from "@/hooks/use-clipboard";
import { useInterval } from "@/hooks/use-interval";
import type { Port } from "@/models/challenge";
import type { Env, Nat } from "@/models/env";
import { useAuthStore } from "@/storages/auth";
import { cn } from "@/utils";
import { parseErrorResponse } from "@/utils/query";
import { Context } from "./context";

function PortInfo({ env, port }: { env: Env; port: Port }) {
  const { isCopied, copyToClipboard } = useClipboard();
  const url = `${window.location.protocol.replace("http", "ws")}//${window.location.host}/api/envs/${env?.id}/wsrx?port=${port.port}`;

  return (
    <div className={cn(["flex"])}>
      <Field size={"sm"} className={cn(["flex-1"])}>
        <FieldIcon className={cn(["w-fit", "px-4"])}>
          <EthernetPortIcon />
          <span
            className={cn(["text-xs"])}
          >{`${port.protocol} | ${port.port}`}</span>
        </FieldIcon>
        <TextField readOnly value={url} />
        <FieldButton
          icon={isCopied ? <ClipboardCheckIcon /> : <ClipboardIcon />}
          onClick={() => copyToClipboard(url)}
        />
      </Field>
    </div>
  );
}

function NatInfo({ env, nat }: { env: Env; nat: Nat }) {
  const { isCopied, copyToClipboard } = useClipboard();
  const address = `${env.public_entry}:${nat.node_port}`;

  return (
    <div className={cn(["flex"])}>
      <Field size={"sm"} className={cn(["flex-1"])}>
        <FieldIcon className={cn(["w-fit", "px-4"])}>
          <EthernetPortIcon />
          <span
            className={cn(["text-xs"])}
          >{`${nat.protocol} | ${nat.port}`}</span>
        </FieldIcon>
        <TextField readOnly value={address} />
        <FieldButton
          icon={isCopied ? <ClipboardCheckIcon /> : <ClipboardIcon />}
          onClick={() => copyToClipboard(address)}
        />
      </Field>
    </div>
  );
}

function EnvSection() {
  const { challenge, team } = useContext(Context);
  const authStore = useAuthStore();

  const mode = useMemo(() => {
    if (team) {
      return "game";
    }

    return "default";
  }, [team]);

  const [env, setEnv] = useState<Env>();
  const [envStopLoading, envPodStopLoading] = useState<boolean>(false);
  const [envCreateLoading, envPodCreateLoading] = useState<boolean>(false);
  const [timeLeft, setTimeLeft] = useState(0);

  useEffect(() => {
    if (timeLeft <= 0) return;

    const timer = setInterval(() => {
      setTimeLeft((prev) => prev - 1);
    }, 1000);

    return () => clearInterval(timer);
  }, [timeLeft]);

  function fetchPods() {
    getEnvs({
      challenge_id: challenge?.id,
      user_id: mode !== "game" ? authStore?.user?.id : undefined,
      game_id: mode === "game" ? Number(team?.game_id) : undefined,
      team_id: mode === "game" ? Number(team?.id) : undefined,
    }).then((res) => {
      if (res.code === StatusCodes.OK) {
        const p = res.data?.[0];
        setEnv(p);
        setTimeLeft(
          Math.ceil(
            Number(p?.started_at) +
              (Number(p?.renew) + 1) * Number(p?.duration) -
              Date.now() / 1000
          )
        );

        if (p?.status !== "waiting") {
          envPodCreateLoading(false);
        }

        if (p?.status === "running") {
          toast.dismiss("pod");
        }

        if (p?.status === "waiting" && p?.reason !== "ContainerCreating") {
          toast.warning("容器创建时发生错误，即将触发销毁", {
            id: "pod",
            description: p?.reason,
          });
          envPodStopLoading(true);
        }
      }
    });
  }

  async function handlePodRenew() {
    if (!env) return;

    try {
      const res = await renewEnv({
        id: env.id!,
      });

      if (res.code === StatusCodes.OK) {
        toast.success("续期成功", {
          id: "renew",
          description: null,
        });
      }
    } catch (error) {
      if (!(error instanceof HTTPError)) return;
      const res = await parseErrorResponse(error);

      if (res.code === StatusCodes.BAD_REQUEST) {
        toast.error("续期失败", {
          id: "renew",
          description: res.msg,
        });
      }
    }
  }

  async function handlePodStop() {
    if (!env) return;

    await stopEnv({
      id: env.id!,
    });

    toast.info("已下发容器停止命令", {
      id: "pod-stop",
    });
    setEnv(undefined);
    envPodStopLoading(false);
  }

  useEffect(() => {
    if (envStopLoading) {
      handlePodStop();
    }
  }, [envStopLoading]);

  function handlePodCreate() {
    envPodCreateLoading(true);
    toast.loading("正在发送容器创建请求", {
      id: "pod",
    });
    createEnv({
      challenge_id: challenge?.id,
      game_id: mode === "game" ? Number(team?.game_id) : undefined,
      team_id: mode === "game" ? Number(team?.id) : undefined,
    }).then((res) => {
      switch (res.code) {
        case StatusCodes.OK: {
          setEnv(res.data);
          toast.loading("已下发容器启动命令", {
            id: "pod",
            description: "这可能需要一些时间",
          });
          fetchPods();
          break;
        }
        default: {
          toast.error("发生错误", {
            id: "pod",
            description: res.msg,
          });
        }
      }
    });
  }

  useInterval(fetchPods, 2000, [], { immediate: true });

  return (
    <div className={cn(["flex", "gap-5", "justify-between", "items-end"])}>
      {env?.id ? (
        <>
          <div className={cn(["flex-1", "flex", "flex-col", "gap-3"])}>
            {env?.nats?.length
              ? env?.nats.map((nat) => (
                  <NatInfo nat={nat} env={env} key={nat.node_port} />
                ))
              : env?.ports?.map((port) => (
                  <PortInfo env={env} port={port} key={port.port} />
                ))}
          </div>
          <div className={cn(["flex", "flex-col", "gap-2", "items-center"])}>
            <span
              className={cn([
                "text-secondary-foreground",
                "text-sm",
                "select-none",
              ])}
            >
              {`剩余时间 ${String(Math.floor(timeLeft / 3600)).padStart(2, "0")}:${String(Math.floor((timeLeft % 3600) / 60)).padStart(2, "0")}:${String(timeLeft % 60).padStart(2, "0")}`}
            </span>
            <div className={cn(["flex", "gap-3"])}>
              <Button
                icon={<ClockIcon />}
                level={"info"}
                variant={"solid"}
                onClick={() => handlePodRenew()}
                disabled={Number(env.renew) === 3}
                className={cn(["items-center"])}
              >
                续期
              </Button>
              <Button
                icon={<TrashIcon />}
                variant={"solid"}
                level={"error"}
                onClick={() => handlePodStop()}
                loading={envStopLoading}
              >
                停止
              </Button>
            </div>
          </div>
        </>
      ) : (
        <>
          <div
            className={cn([
              "flex",
              "flex-col",
              "text-secondary-foreground",
              "text-sm",
              "select-none",
            ])}
          >
            <span>本题需要使用动态容器，</span>
            <span>点击“启动”进行容器下发。</span>
          </div>
          <Button
            icon={<PlayIcon />}
            variant={"solid"}
            level={"success"}
            onClick={() => handlePodCreate()}
            loading={envCreateLoading}
          >
            启动
          </Button>
        </>
      )}
    </div>
  );
}

export { EnvSection };
