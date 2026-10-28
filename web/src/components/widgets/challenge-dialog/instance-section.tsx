import { useQuery, useQueryClient } from "@tanstack/react-query";
import { HTTPError } from "ky";
import {
  AlertCircleIcon,
  ClipboardCheckIcon,
  ClipboardIcon,
  ClockIcon,
  EthernetPortIcon,
  LoaderCircleIcon,
  PlayIcon,
  TrashIcon,
} from "lucide-react";
import { useContext, useEffect, useState } from "react";
import { useTranslation } from "react-i18next";
import { toast } from "sonner";
import { createInstance, getInstances } from "@/api/instances";
import { renewInstance, stopInstance } from "@/api/instances/instance_id";
import { createDebugInstance } from "@/api/management/instances";
import { Button } from "@/components/ui/button";
import { Field, FieldButton, FieldIcon } from "@/components/ui/field";
import { TextField } from "@/components/ui/text-field";
import { useClipboard } from "@/hooks/use-clipboard";
import { useTickerTime } from "@/hooks/use-ticker-time";
import type { Port } from "@/models/challenge";
import type { Instance, Nat } from "@/models/instance";
import { useAuthStore } from "@/storages/auth";
import { cn } from "@/utils";
import { notifyApiError } from "@/utils/query";
import { secondsUntil } from "@/utils/time";
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
  const { challenge, team, debug } = useContext(Context);
  const userId = useAuthStore((state) => state.user?.id);
  const gameId = debug ? undefined : team?.game_id;
  const teamId = debug ? undefined : team?.id;

  return (
    <InstanceControls
      key={`${challenge?.id}:${userId}:${gameId}:${teamId}:${debug}`}
      challengeId={challenge?.id}
      userId={userId}
      gameId={gameId}
      teamId={teamId}
      debug={!!debug}
    />
  );
}

function InstanceControls({
  challengeId,
  userId,
  gameId,
  teamId,
  debug,
}: {
  challengeId?: number;
  userId?: number;
  gameId?: number;
  teamId?: number;
  debug: boolean;
}) {
  const { t } = useTranslation();
  const queryClient = useQueryClient();
  const now = useTickerTime();
  const [trackedId, setTrackedId] = useState<string>();
  const [stoppingId, setStoppingId] = useState<string>();
  const [creating, setCreating] = useState(false);
  const [renewing, setRenewing] = useState(false);
  const queryKey = [
    "challenge-instances",
    challengeId,
    userId,
    gameId,
    teamId,
    debug,
  ];
  const toastId = `instance:${challengeId}:${userId}:${teamId ?? "personal"}`;
  const enabled =
    challengeId != null && userId != null && (teamId == null || gameId != null);
  const query = useQuery({
    queryKey,
    queryFn: () =>
      getInstances({
        challenge_id: challengeId,
        user_id: teamId == null ? userId : undefined,
        game_id: gameId,
        team_id: teamId,
      }),
    enabled,
    refetchInterval: 2000,
    refetchIntervalInBackground: false,
    retry: false,
    meta: { suppressErrorToast: true },
  });
  const instances = query.data?.instances ?? [];
  const instance = trackedId
    ? instances.find((candidate) => candidate.id === trackedId)
    : instances[0];
  const stopping = stoppingId != null || instance?.state === "stopping";
  const terminal =
    instance?.state === "succeeded" || instance?.state === "failed";
  const timeLeft = Math.max(0, secondsUntil(instance?.expires_at, now) ?? 0);
  const displayedState = stopping ? "stopping" : instance?.state;
  const transitioning =
    displayedState === "starting" || displayedState === "stopping";

  useEffect(() => {
    if (
      trackedId &&
      !creating &&
      query.isSuccess &&
      !query.isFetching &&
      !query.data.instances.some((candidate) => candidate.id === trackedId)
    ) {
      setTrackedId(undefined);
      if (stoppingId === trackedId) setStoppingId(undefined);
    }
  }, [
    trackedId,
    stoppingId,
    creating,
    query.isSuccess,
    query.isFetching,
    query.data,
  ]);

  async function handleCreate() {
    if (!enabled || creating || stopping) return;
    setCreating(true);
    toast.loading(t("instance:actions.start.creating"), { id: toastId });
    try {
      const response = debug
        ? await createDebugInstance({ challenge_id: challengeId })
        : await createInstance({
            challenge_id: challengeId,
            game_id: gameId,
            team_id: teamId,
          });
      setTrackedId(response.instance_id);
      toast.success(t("instance:actions.start.sent"), {
        id: toastId,
        description: t("instance:actions.start.description"),
      });
      await query.refetch();
    } catch (error) {
      await notifyApiError(error, { id: toastId });
    } finally {
      setCreating(false);
    }
  }

  async function handleRenew() {
    if (!instance?.can_renew || renewing || stopping) return;
    setRenewing(true);
    try {
      const renewed = await renewInstance({ id: instance.id });
      queryClient.setQueryData<{ instances: Instance[] }>(
        queryKey,
        (previous) => ({
          instances: (previous?.instances ?? []).map((candidate) =>
            candidate.id === renewed.id ? renewed : candidate
          ),
        })
      );
      toast.success(t("instance:actions.renew.success"), {
        id: `${toastId}:renew`,
      });
      await query.refetch();
    } catch (error) {
      await notifyApiError(error, { id: `${toastId}:renew` });
    } finally {
      setRenewing(false);
    }
  }

  async function handleStop() {
    if (!instance?.can_stop || stopping || creating) return;
    setTrackedId(instance.id);
    setStoppingId(instance.id);
    try {
      await stopInstance({ id: instance.id });
      toast.info(t("instance:actions.stop.sent"), { id: `${toastId}:stop` });
      await query.refetch();
    } catch (error) {
      // The known instance may have expired between the last poll and this request.
      if (!(error instanceof HTTPError && error.response.status === 404)) {
        setStoppingId(undefined);
        await notifyApiError(error, { id: `${toastId}:stop` });
      }
      await query.refetch();
    }
  }

  return (
    <div className="flex min-w-0 flex-col gap-3">
      {query.isError && (
        <div
          role="alert"
          className="flex items-center gap-2 text-sm text-warning"
        >
          <AlertCircleIcon className="size-4 shrink-0" />
          <span>{t("instance:query_error")}</span>
          <Button
            size="sm"
            variant="ghost"
            onClick={() => void query.refetch()}
          >
            {t("instance:retry")}
          </Button>
        </div>
      )}
      <div className={cn(["flex", "gap-5", "justify-between", "items-end"])}>
        {instance ? (
          <>
            <div
              className={cn(["min-w-0", "flex-1", "flex", "flex-col", "gap-3"])}
            >
              {instance.ready && !stopping ? (
                instance.nats?.length ? (
                  instance.nats.map((nat) => (
                    <NatInfo
                      nat={nat}
                      instance={instance}
                      key={`${nat.protocol}:${nat.node_port}`}
                    />
                  ))
                ) : (
                  instance.ports?.map((port) => (
                    <PortInfo
                      instance={instance}
                      port={port}
                      key={`${port.protocol}:${port.port}`}
                    />
                  ))
                )
              ) : (
                <div
                  role="status"
                  className="flex flex-col gap-1 text-sm text-secondary-foreground"
                >
                  <span
                    className={cn(
                      "inline-flex items-center gap-2",
                      displayedState === "starting" && "text-info",
                      displayedState === "stopping" && "text-error"
                    )}
                  >
                    {transitioning && (
                      <LoaderCircleIcon
                        aria-hidden="true"
                        className="size-4 shrink-0 motion-safe:animate-spin"
                      />
                    )}
                    {t(`instance:states.${displayedState}`)}
                  </span>
                  <span>
                    {t(
                      `instance:reasons.${stopping ? "stopping" : instance.reason_code}`
                    )}
                  </span>
                </div>
              )}
            </div>
            <div className={cn(["flex", "flex-col", "gap-2", "items-center"])}>
              {instance.expires_at && !terminal && (
                <span
                  className={cn([
                    "text-secondary-foreground",
                    "text-sm",
                    "select-none",
                  ])}
                >
                  {t("instance:remaining", {
                    hours: String(Math.floor(timeLeft / 3600)).padStart(2, "0"),
                    minutes: String(
                      Math.floor((timeLeft % 3600) / 60)
                    ).padStart(2, "0"),
                    seconds: String(timeLeft % 60).padStart(2, "0"),
                  })}
                </span>
              )}
              <div className={cn(["flex", "gap-3"])}>
                {terminal ? (
                  <Button
                    icon={<PlayIcon />}
                    level="success"
                    variant="solid"
                    onClick={() => void handleCreate()}
                    loading={creating}
                    disabled={
                      !enabled || query.isPending || query.isError || stopping
                    }
                  >
                    {t("instance:actions.start._")}
                  </Button>
                ) : (
                  <Button
                    icon={<ClockIcon />}
                    level="info"
                    variant="solid"
                    onClick={() => void handleRenew()}
                    loading={renewing}
                    disabled={!instance.can_renew || stopping || creating}
                    className={cn(["items-center"])}
                  >
                    {t("instance:actions.renew._")}
                  </Button>
                )}
                <Button
                  icon={<TrashIcon />}
                  variant="solid"
                  level="error"
                  onClick={() => void handleStop()}
                  loading={stopping}
                  disabled={
                    !instance.can_stop || stopping || creating || renewing
                  }
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
              variant="solid"
              level="success"
              onClick={() => void handleCreate()}
              loading={creating}
              disabled={
                !enabled || query.isPending || query.isError || stopping
              }
            >
              {t("instance:actions.start._")}
            </Button>
          </>
        )}
      </div>
    </div>
  );
}

export { InstanceSection };
