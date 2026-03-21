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
import { useCallback, useContext, useEffect, useMemo, useState } from "react";
import { useTranslation } from "react-i18next";
import { toast } from "sonner";
import { createDebugInstance } from "@/api/admin/instances";
import { createInstance, getInstances } from "@/api/instances";
import { renewInstance, stopInstance } from "@/api/instances/instance_id";
import { Button } from "@/components/ui/button";
import { Field, FieldButton, FieldIcon } from "@/components/ui/field";
import { TextField } from "@/components/ui/text-field";
import { useClipboard } from "@/hooks/use-clipboard";
import { useInterval } from "@/hooks/use-interval";
import type { Port } from "@/models/challenge";
import type { Instance, Nat } from "@/models/instance";
import { useAuthStore } from "@/storages/auth";
import { cn } from "@/utils";
import { formatApiMsg, parseErrorResponse } from "@/utils/query";
import { Context } from "./context";

function PortInfo({ instance, port }: { instance: Instance; port: Port }) {
  const { isCopied, copyToClipboard } = useClipboard();
  const url = `${window.location.protocol.replace("http", "ws")}//${window.location.host}/api/instances/${instance?.id}/wsrx?port=${port.port}`;

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

function NatInfo({ instance, nat }: { instance: Instance; nat: Nat }) {
  const { isCopied, copyToClipboard } = useClipboard();
  const address = `${instance.public_entry}:${nat.node_port}`;

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

function InstanceSection() {
  const { t } = useTranslation();

  const { challenge, team, debug } = useContext(Context);
  const authStore = useAuthStore();

  const mode = useMemo(() => {
    if (team) {
      return "game";
    }

    return "default";
  }, [team]);

  const [instance, setInstance] = useState<Instance>();
  const [instanceStopLoading, setInstanceStopLoading] =
    useState<boolean>(false);
  const [instanceCreateLoading, setInstanceCreateLoading] =
    useState<boolean>(false);
  const [timeLeft, setTimeLeft] = useState(0);

  useEffect(() => {
    if (timeLeft <= 0) return;

    const timer = setInterval(() => {
      setTimeLeft((prev) => prev - 1);
    }, 1000);

    return () => clearInterval(timer);
  }, [timeLeft]);

  function fetchInstances() {
    getInstances({
      challenge_id: challenge?.id,
      user_id: mode !== "game" ? authStore?.user?.id : undefined,
      game_id: mode === "game" ? Number(team?.game_id) : undefined,
      team_id: mode === "game" ? Number(team?.id) : undefined,
    }).then((res) => {
      {
        const p = res.instances?.[0];
        setInstance(p);
        setTimeLeft(
          Math.ceil(
            Number(p?.started_at) +
              (Number(p?.renew) + 1) * Number(p?.duration) -
              Date.now() / 1000
          )
        );

        if (p?.status !== "waiting") {
          setInstanceCreateLoading(false);
        }

        if (p?.status === "running") {
          toast.dismiss("instance");
        }

        if (p?.status === "waiting" && p?.reason !== "ContainerCreating") {
          toast.warning(t("instance:actions.start.error"), {
            id: "instance",
            description: p?.reason,
          });
          setInstanceStopLoading(true);
        }
      }
    });
  }

  async function handleInstanceRenew() {
    if (!instance) return;

    try {
      await renewInstance({
        id: instance.id!,
      });

      toast.success(t("challenge:instance.renew_success"), {
        id: "renew",
      });
    } catch (error) {
      if (!(error instanceof HTTPError)) return;
      const body = await parseErrorResponse(error);

      if (error.response.status === StatusCodes.BAD_REQUEST) {
        toast.error(t("challenge:instance.renew_error"), {
          id: "renew",
          description: formatApiMsg(body.msg),
        });
      }
    }
  }

  const handleInstanceStop = useCallback(async () => {
    if (!instance) return;

    await stopInstance({
      id: instance.id!,
    });

    toast.info(t("instance:actions.stop.sent"), {
      id: "instance-stop",
    });
    setInstance(undefined);
    setInstanceStopLoading(false);
  }, [instance, t]);

  useEffect(() => {
    if (instanceStopLoading) {
      handleInstanceStop();
    }
  }, [handleInstanceStop, instanceStopLoading]);

  async function handleInstanceCreate() {
    setInstanceCreateLoading(true);
    toast.loading(t("instance:actions.start.creating"), {
      id: "instance",
    });
    try {
      if (debug) {
        await createDebugInstance({
          challenge_id: challenge?.id,
        });
      } else {
        await createInstance({
          challenge_id: challenge?.id,
          game_id: mode === "game" ? Number(team?.game_id) : undefined,
          team_id: mode === "game" ? Number(team?.id) : undefined,
        });
      }

      toast.loading(t("instance:actions.start.sent"), {
        id: "instance",
        description: t("instance:actions.start.description"),
      });
      fetchInstances();
    } catch (error) {
      if (!(error instanceof HTTPError)) return;
      const body = await parseErrorResponse(error);

      toast.error(t("instance:error"), {
        id: "instance",
        description: formatApiMsg(body.msg),
      });
    }
  }

  useInterval(fetchInstances, 2000, [], { immediate: true });

  return (
    <div className={cn(["flex", "gap-5", "justify-between", "items-end"])}>
      {instance?.id ? (
        <>
          <div className={cn(["flex-1", "flex", "flex-col", "gap-3"])}>
            {instance?.nats?.length
              ? instance?.nats.map((nat) => (
                  <NatInfo nat={nat} instance={instance} key={nat.node_port} />
                ))
              : instance?.ports?.map((port) => (
                  <PortInfo instance={instance} port={port} key={port.port} />
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
              {t("instance:remaining", {
                hours: String(Math.floor(timeLeft / 3600)).padStart(2, "0"),
                minutes: String(Math.floor((timeLeft % 3600) / 60)).padStart(
                  2,
                  "0"
                ),
                seconds: String(timeLeft % 60).padStart(2, "0"),
              })}
            </span>
            <div className={cn(["flex", "gap-3"])}>
              <Button
                icon={<ClockIcon />}
                level={"info"}
                variant={"solid"}
                onClick={() => handleInstanceRenew()}
                disabled={Number(instance.renew) === 3}
                className={cn(["items-center"])}
              >
                {t("instance:actions.renew._")}
              </Button>
              <Button
                icon={<TrashIcon />}
                variant={"solid"}
                level={"error"}
                onClick={() => handleInstanceStop()}
                loading={instanceStopLoading}
              >
                {t("instance:actions.stop._")}
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
            <span>{t("instance:hint1")}</span>
            <span>{t("instance:hint2")}</span>
          </div>
          <Button
            icon={<PlayIcon />}
            variant={"solid"}
            level={"success"}
            onClick={handleInstanceCreate}
            loading={instanceCreateLoading}
          >
            {t("instance:actions.start._")}
          </Button>
        </>
      )}
    </div>
  );
}

export { InstanceSection };
