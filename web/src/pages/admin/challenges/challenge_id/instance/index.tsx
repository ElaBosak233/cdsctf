import { zodResolver } from "@hookform/resolvers/zod";
import {
  ClockIcon,
  ContainerIcon,
  CpuIcon,
  DownloadIcon,
  HandshakeIcon,
  KeyIcon,
  MemoryStickIcon,
  MinusIcon,
  NetworkIcon,
  PlusIcon,
  SaveIcon,
  TextIcon,
  TrashIcon,
} from "lucide-react";
import { useContext, useEffect, useState } from "react";
import { useForm } from "react-hook-form";
import { useTranslation } from "react-i18next";
import { toast } from "sonner";
import { z } from "zod";
import { updateChallengeInstance } from "@/api/management/challenges/challenge_id/instance";
import { Button } from "@/components/ui/button";
import { Field, FieldIcon } from "@/components/ui/field";
import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "@/components/ui/form";
import { Label } from "@/components/ui/label";
import { NumberField } from "@/components/ui/number-field";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { TextField } from "@/components/ui/text-field";
import type { Instance as ChallengeInstance } from "@/models/challenge";
import { useSharedStore } from "@/storages/shared";
import { cn } from "@/utils";
import { notifyApiError } from "@/utils/query";
import { Context } from "../context";

let nextFieldId = 0;

function createFieldKey() {
  return `field-${nextFieldId++}`;
}

function toFormValues(instance: ChallengeInstance | null | undefined) {
  return {
    duration: instance?.duration ?? 1800,
    internet: instance?.internet ?? false,
    containers:
      instance?.containers?.map((container) => ({
        ...container,
        _key: createFieldKey(),
        envs: (container.envs ?? []).map((env) => ({
          ...env,
          _key: createFieldKey(),
        })),
        ports: (container.ports ?? []).map((port) => ({
          ...port,
          _key: createFieldKey(),
        })),
      })) ?? [],
  };
}

export default function Index() {
  const { t } = useTranslation();

  const { challenge } = useContext(Context);
  const sharedStore = useSharedStore();

  const [loading, setLoading] = useState<boolean>(false);

  const formSchema = z.object({
    duration: z.number({
      message: t("challenge:form.instance.duration.message"),
    }),
    internet: z.boolean({
      message: t("challenge:form.instance.internet.message"),
    }),
    containers: z.array(
      z.object({
        _key: z.string(),
        image: z.string({
          message: t("challenge:form.instance.containers.image.message"),
        }),
        cpu_limit: z.number({
          message: t("challenge:form.instance.containers.cpu_limit.message"),
        }),
        memory_limit: z.number({
          message: t("challenge:form.instance.containers.memory_limit.message"),
        }),
        image_pull_policy: z.string({
          message: t(
            "challenge:form.instance.containers.image_pull_policy.message"
          ),
        }),
        envs: z.array(
          z.object({
            _key: z.string(),
            key: z.string().min(1, {
              message: t("challenge:form.instance.containers.envs.key.message"),
            }),
            value: z.string().min(1, {
              message: t(
                "challenge:form.instance.containers.envs.value.message"
              ),
            }),
          })
        ),
        ports: z.array(
          z.object({
            _key: z.string(),
            port: z
              .number({
                message: t(
                  "challenge:form.instance.containers.ports.port.message"
                ),
              })
              .min(0)
              .max(65535),
            protocol: z.enum(["TCP", "UDP"]),
          })
        ),
      })
    ),
  });

  const form = useForm<z.infer<typeof formSchema>>({
    resolver: zodResolver(formSchema),
    defaultValues: toFormValues(challenge?.instance),
  });

  useEffect(() => {
    form.reset(toFormValues(challenge?.instance));
  }, [challenge?.instance, form]);

  const handleAddContainer = () => {
    const containers = form.getValues("containers") || [];
    form.setValue("containers", [
      ...containers,
      {
        _key: createFieldKey(),
        image: "",
        cpu_limit: 1,
        memory_limit: 1024,
        envs: [],
        ports: [],
        image_pull_policy: "Always",
      },
    ]);
  };

  const handleRemoveContainer = (index: number) => {
    const containers = form.getValues("containers") || [];
    form.setValue(
      "containers",
      containers.filter((_, i) => i !== index)
    );
  };

  const handleAddPort = (containerIndex: number) => {
    const containers = form.getValues("containers") || [];
    const ports = containers[containerIndex]?.ports || [];
    const newContainers = [...containers];
    newContainers[containerIndex] = {
      ...newContainers[containerIndex],
      ports: [...ports, { _key: createFieldKey(), port: NaN, protocol: "TCP" }],
    };
    form.setValue("containers", newContainers);
  };

  const handleRemovePort = (containerIndex: number, portIndex: number) => {
    const containers = form.getValues("containers") || [];
    const ports = containers[containerIndex]?.ports || [];
    const newContainers = [...containers];
    newContainers[containerIndex] = {
      ...newContainers[containerIndex],
      ports: ports.filter((_, i) => i !== portIndex),
    };
    form.setValue("containers", newContainers);
  };

  const handleAddEnv = (containerIndex: number) => {
    const containers = form.getValues("containers") || [];
    const envs = containers[containerIndex]?.envs || [];
    const newContainers = [...containers];
    newContainers[containerIndex] = {
      ...newContainers[containerIndex],
      envs: [...envs, { _key: createFieldKey(), key: "", value: "" }],
    };
    form.setValue("containers", newContainers);
  };

  const handleRemoveEnv = (containerIndex: number, envIndex: number) => {
    const containers = form.getValues("containers") || [];
    const envs = containers[containerIndex]?.envs || [];
    const newContainers = [...containers];
    newContainers[containerIndex] = {
      ...newContainers[containerIndex],
      envs: envs.filter((_, i) => i !== envIndex),
    };
    form.setValue("containers", newContainers);
  };

  async function onSubmit(values: z.infer<typeof formSchema>) {
    setLoading(true);
    try {
      const { containers, ...instance } = values;
      await updateChallengeInstance({
        id: challenge?.id,
        instance: {
          ...instance,
          containers: containers.map(({ _key, envs, ports, ...container }) => ({
            ...container,
            envs: envs.map(({ _key: _, ...env }) => env),
            ports: ports.map(({ _key: _, ...port }) => port),
          })),
        },
      });
      toast.success(
        t("challenge:actions.instance_update.success", {
          title: challenge?.title,
        })
      );
    } catch (error) {
      await notifyApiError(error);
    } finally {
      sharedStore.setRefresh();
      setLoading(false);
    }
  }

  return (
    <Form {...form}>
      <form
        onSubmit={form.handleSubmit(onSubmit)}
        autoComplete={"off"}
        className={cn(["flex", "flex-col", "flex-1", "gap-8"])}
      >
        <div className={cn(["grid", "grid-cols-1", "sm:grid-cols-2", "gap-5"])}>
          <FormField
            control={form.control}
            name={"duration"}
            render={({ field }) => (
              <FormItem className={cn(["w-full"])}>
                <FormLabel>{t("challenge:form.instance.duration._")}</FormLabel>
                <Field>
                  <FieldIcon>
                    <ClockIcon />
                  </FieldIcon>
                  <FormControl>
                    <NumberField
                      placeholder={t(
                        "challenge:form.instance.duration.placeholder"
                      )}
                      value={field.value}
                      onValueChange={(value) => field.onChange(value)}
                    />
                  </FormControl>
                </Field>
                <FormMessage />
              </FormItem>
            )}
          />
          <FormField
            control={form.control}
            name={"internet"}
            render={({ field }) => (
              <FormItem className={cn(["w-full"])}>
                <FormLabel>{t("challenge:form.instance.internet._")}</FormLabel>
                <Field>
                  <FieldIcon>
                    <NetworkIcon />
                  </FieldIcon>
                  <Select
                    {...field}
                    onValueChange={(value) => {
                      field.onChange(value === "true");
                    }}
                    value={String(field.value)}
                  >
                    <FormControl>
                      <SelectTrigger>
                        <SelectValue />
                      </SelectTrigger>
                    </FormControl>
                    <SelectContent>
                      <SelectItem value="true">
                        {t("challenge:form.instance.internet.true")}
                      </SelectItem>
                      <SelectItem value="false">
                        {t("challenge:form.instance.internet.false")}
                      </SelectItem>
                    </SelectContent>
                  </Select>
                </Field>
                <FormMessage />
              </FormItem>
            )}
          />
        </div>
        {form.watch("containers")?.map((container, containerIndex) => (
          <div
            key={container._key}
            className={cn([
              "flex",
              "flex-col",
              "gap-5",
              "border",
              "p-5",
              "rounded",
            ])}
          >
            <FormField
              control={form.control}
              name={`containers.${containerIndex}.image`}
              render={({ field }) => (
                <FormItem className={cn(["w-full"])}>
                  <FormLabel>
                    {t("challenge:form.instance.containers.image._")}
                  </FormLabel>
                  <Field>
                    <FieldIcon>
                      <ContainerIcon />
                    </FieldIcon>
                    <FormControl>
                      <TextField
                        {...field}
                        placeholder={t(
                          "challenge:form.instance.containers.image.placeholder"
                        )}
                        value={field.value || ""}
                        onChange={field.onChange}
                      />
                    </FormControl>
                  </Field>
                  <FormMessage />
                </FormItem>
              )}
            />
            <div className={cn(["flex", "flex-col", "gap-5", "sm:flex-row"])}>
              <FormField
                control={form.control}
                name={`containers.${containerIndex}.cpu_limit`}
                render={({ field }) => (
                  <FormItem className={cn(["w-full", "sm:w-1/3"])}>
                    <FormLabel>
                      {t("challenge:form.instance.containers.cpu_limit._")}
                    </FormLabel>
                    <Field>
                      <FieldIcon>
                        <CpuIcon />
                      </FieldIcon>
                      <FormControl>
                        <NumberField
                          value={field.value}
                          onValueChange={(value) => field.onChange(value)}
                        />
                      </FormControl>
                    </Field>
                    <FormMessage />
                  </FormItem>
                )}
              />
              <FormField
                control={form.control}
                name={`containers.${containerIndex}.memory_limit`}
                render={({ field }) => (
                  <FormItem className={cn(["w-full", "sm:w-1/3"])}>
                    <FormLabel>
                      {t("challenge:form.instance.containers.memory_limit._")}
                    </FormLabel>
                    <Field>
                      <FieldIcon>
                        <MemoryStickIcon />
                      </FieldIcon>
                      <FormControl>
                        <NumberField
                          value={field.value}
                          onValueChange={(value) => field.onChange(value)}
                        />
                      </FormControl>
                    </Field>
                    <FormMessage />
                  </FormItem>
                )}
              />
              <FormField
                control={form.control}
                name={`containers.${containerIndex}.image_pull_policy`}
                render={({ field }) => (
                  <FormItem className={cn(["w-full", "sm:w-1/3"])}>
                    <FormLabel>
                      {t(
                        "challenge:form.instance.containers.image_pull_policy._"
                      )}
                    </FormLabel>
                    <Field>
                      <FieldIcon>
                        <DownloadIcon />
                      </FieldIcon>
                      <Select
                        {...field}
                        onValueChange={field.onChange}
                        value={field.value}
                      >
                        <FormControl>
                          <SelectTrigger>
                            <SelectValue />
                          </SelectTrigger>
                        </FormControl>
                        <SelectContent>
                          <SelectItem value="Always">
                            {t(
                              "challenge:form.instance.containers.image_pull_policy.always"
                            )}
                          </SelectItem>
                          <SelectItem value="IfNotPresent">
                            {t(
                              "challenge:form.instance.containers.image_pull_policy.if_not_present"
                            )}
                          </SelectItem>
                          <SelectItem value="Never">
                            {t(
                              "challenge:form.instance.containers.image_pull_policy.never"
                            )}
                          </SelectItem>
                        </SelectContent>
                      </Select>
                    </Field>
                    <FormMessage />
                  </FormItem>
                )}
              />
            </div>
            <Label>{t("challenge:form.instance.containers.ports._")}</Label>
            <div
              className={cn([
                "grid",
                "grid-cols-1",
                "sm:grid-cols-2",
                "xl:grid-cols-3",
                "gap-7",
              ])}
            >
              {container.ports?.map((port, portIndex) => (
                <div
                  key={port._key}
                  className={cn(["flex", "items-center", "gap-3"])}
                >
                  <FormField
                    control={form.control}
                    name={`containers.${containerIndex}.ports.${portIndex}.port`}
                    render={({ field }) => (
                      <FormItem className={cn(["flex-1"])}>
                        <Field size={"sm"}>
                          <FieldIcon>
                            <MemoryStickIcon />
                          </FieldIcon>
                          <FormControl>
                            <NumberField
                              value={field.value}
                              onValueChange={(value) => field.onChange(value)}
                            />
                          </FormControl>
                        </Field>
                        <FormMessage className={cn(["-top-5"])} />
                      </FormItem>
                    )}
                  />
                  <FormField
                    control={form.control}
                    name={`containers.${containerIndex}.ports.${portIndex}.protocol`}
                    render={({ field }) => (
                      <FormItem className={cn(["flex-1"])}>
                        <Field size={"sm"}>
                          <FieldIcon>
                            <HandshakeIcon />
                          </FieldIcon>
                          <Select
                            {...field}
                            onValueChange={field.onChange}
                            value={field.value}
                          >
                            <FormControl>
                              <SelectTrigger>
                                <SelectValue />
                              </SelectTrigger>
                            </FormControl>
                            <SelectContent>
                              <SelectItem value="TCP">
                                {t(
                                  "challenge:form.instance.containers.ports.protocol.tcp"
                                )}
                              </SelectItem>
                              <SelectItem value="UDP">
                                {t(
                                  "challenge:form.instance.containers.ports.protocol.udp"
                                )}
                              </SelectItem>
                            </SelectContent>
                          </Select>
                        </Field>
                        <FormMessage />
                      </FormItem>
                    )}
                  />
                  <Button
                    type={"button"}
                    icon={<MinusIcon />}
                    size={"sm"}
                    square
                    onClick={() => handleRemovePort(containerIndex, portIndex)}
                  />
                </div>
              ))}
              <Button
                type={"button"}
                size={"sm"}
                icon={<PlusIcon />}
                className={cn(["self-center"])}
                square
                onClick={() => handleAddPort(containerIndex)}
              />
            </div>
            <Label>{t("challenge:form.instance.containers.envs._")}</Label>
            <div
              className={cn(["grid", "grid-cols-1", "sm:grid-cols-2", "gap-7"])}
            >
              {container.envs?.map((env, envIndex) => (
                <div
                  key={env._key}
                  className={cn(["flex", "items-center", "gap-3"])}
                >
                  <FormField
                    control={form.control}
                    name={`containers.${containerIndex}.envs.${envIndex}.key`}
                    render={({ field }) => (
                      <FormItem className={cn(["flex-1"])}>
                        <Field size={"sm"}>
                          <FieldIcon>
                            <KeyIcon />
                          </FieldIcon>
                          <FormControl>
                            <TextField
                              {...field}
                              placeholder={t(
                                "challenge:form.instance.containers.envs.key._"
                              )}
                              value={field.value || ""}
                              onChange={field.onChange}
                            />
                          </FormControl>
                        </Field>
                        <FormMessage className={cn(["-top-5"])} />
                      </FormItem>
                    )}
                  />
                  <FormField
                    control={form.control}
                    name={`containers.${containerIndex}.envs.${envIndex}.value`}
                    render={({ field }) => (
                      <FormItem className={cn(["flex-1"])}>
                        <Field size={"sm"}>
                          <FieldIcon>
                            <TextIcon />
                          </FieldIcon>
                          <FormControl>
                            <TextField
                              {...field}
                              placeholder={t(
                                "challenge:form.instance.containers.envs.value._"
                              )}
                              value={field.value || ""}
                              onChange={field.onChange}
                            />
                          </FormControl>
                        </Field>
                        <FormMessage className={cn(["-top-5"])} />
                      </FormItem>
                    )}
                  />
                  <Button
                    type={"button"}
                    icon={<MinusIcon />}
                    size={"sm"}
                    square
                    onClick={() => handleRemoveEnv(containerIndex, envIndex)}
                  />
                </div>
              ))}
              <Button
                type={"button"}
                size={"sm"}
                icon={<PlusIcon />}
                className={cn(["self-center"])}
                square
                onClick={() => handleAddEnv(containerIndex)}
              />
            </div>
            <Button
              type={"button"}
              variant={"tonal"}
              level={"error"}
              size={"sm"}
              icon={<TrashIcon />}
              onClick={() => handleRemoveContainer(containerIndex)}
            >
              {t("challenge:form.instance.containers.actions.delete")}
            </Button>
          </div>
        ))}
        <Button
          type={"button"}
          variant={"tonal"}
          size={"sm"}
          icon={<PlusIcon />}
          onClick={handleAddContainer}
        >
          {t("challenge:form.instance.containers.actions.add")}
        </Button>
        <div className={cn(["flex-1"])} />
        <Button
          variant={"solid"}
          type={"submit"}
          size={"lg"}
          className={cn(["w-full"])}
          icon={<SaveIcon />}
          loading={loading}
        >
          {t("common:actions.save")}
        </Button>
      </form>
    </Form>
  );
}
