import { zodResolver } from "@hookform/resolvers/zod";
import { StatusCodes } from "http-status-codes";
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
import { updateChallengeEnv } from "@/api/admin/challenges/challenge_id/env";
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
import { Select } from "@/components/ui/select";
import { TextField } from "@/components/ui/text-field";
import { useSharedStore } from "@/storages/shared";
import { cn } from "@/utils";
import { Context } from "../context";

export default function Index() {
  const { t } = useTranslation();

  const { challenge } = useContext(Context);
  const sharedStore = useSharedStore();

  const [loading, setLoading] = useState<boolean>(false);

  const formSchema = z.object({
    duration: z.number({
      message: t("challenge.form.env.duration.message"),
    }),
    internet: z.boolean({
      message: t("challenge.form.env.internet.message"),
    }),
    containers: z.array(
      z.object({
        image: z.string({
          message: t("challenge.form.env.containers.image.message"),
        }),
        cpu_limit: z.number({
          message: t("challenge.form.env.containers.cpu_limit.message"),
        }),
        memory_limit: z.number({
          message: t("challenge.form.env.containers.memory_limit.message"),
        }),
        image_pull_policy: z.string({
          message: t("challenge.form.env.containers.image_pull_policy.message"),
        }),
        envs: z.array(
          z.object({
            key: z.string().min(1, {
              message: t("challenge.form.env.containers.envs.key.message"),
            }),
            value: z.string().min(1, {
              message: t("challenge.form.env.containers.envs.value.message"),
            }),
          })
        ),
        ports: z.array(
          z.object({
            port: z
              .number({
                message: t("challenge.form.env.containers.ports.port.message"),
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
    defaultValues: {
      duration: challenge?.env?.duration || 1800,
      internet: challenge?.env?.internet || false,
      containers: challenge?.env?.containers || [],
    },
  });

  useEffect(() => {
    form.reset(challenge?.env);
  }, [challenge?.env, form]);

  const handleAddContainer = () => {
    const containers = form.getValues("containers") || [];
    form.setValue("containers", [
      ...containers,
      {
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
    containers[containerIndex].ports = [
      ...ports,
      { port: NaN, protocol: "TCP" },
    ];
    form.setValue("containers", containers);
  };

  const handleRemovePort = (containerIndex: number, portIndex: number) => {
    const containers = form.getValues("containers") || [];
    const ports = containers[containerIndex]?.ports || [];
    containers[containerIndex].ports = ports.filter((_, i) => i !== portIndex);
    form.setValue("containers", containers);
  };

  const handleAddEnv = (containerIndex: number) => {
    const containers = form.getValues("containers") || [];
    const envs = containers[containerIndex]?.envs || [];
    containers[containerIndex].envs = [...envs, { key: "", value: "" }];
    form.setValue("containers", containers);
  };

  const handleRemoveEnv = (containerIndex: number, envIndex: number) => {
    const containers = form.getValues("containers") || [];
    const envs = containers[containerIndex]?.envs || [];
    containers[containerIndex].envs = envs.filter((_, i) => i !== envIndex);
    form.setValue("containers", containers);
  };

  function onSubmit(values: z.infer<typeof formSchema>) {
    setLoading(true);
    updateChallengeEnv({
      id: challenge?.id,
      env: values,
    })
      .then((res) => {
        if (res.code === StatusCodes.OK) {
          toast.success(`题目 ${challenge?.title} 动态环境更新成功`);
        }
      })
      .finally(() => {
        sharedStore.setRefresh();
        setLoading(false);
      });
  }

  return (
    <Form {...form}>
      <form
        onSubmit={form.handleSubmit(onSubmit)}
        autoComplete={"off"}
        className={cn(["flex", "flex-col", "flex-1", "gap-8"])}
      >
        <div className={cn(["grid", "grid-cols-2", "gap-5"])}>
          <FormField
            control={form.control}
            name={"duration"}
            render={({ field }) => (
              <FormItem className={cn(["w-full"])}>
                <FormLabel>{t("challenge.form.env.duration._")}</FormLabel>
                <FormControl>
                  <Field>
                    <FieldIcon>
                      <ClockIcon />
                    </FieldIcon>
                    <NumberField
                      placeholder={"1800"}
                      value={field.value}
                      onValueChange={(value) => field.onChange(value)}
                    />
                  </Field>
                </FormControl>
                <FormMessage />
              </FormItem>
            )}
          />
          <FormField
            control={form.control}
            name={"internet"}
            render={({ field }) => (
              <FormItem className={cn(["w-full"])}>
                <FormLabel>{t("challenge.form.env.internet._")}</FormLabel>
                <FormControl>
                  <Field>
                    <FieldIcon>
                      <NetworkIcon />
                    </FieldIcon>
                    <Select
                      {...field}
                      options={[
                        {
                          value: String(true),
                          content: t("challenge.form.env.internet.true"),
                        },
                        {
                          value: String(false),
                          content: t("challenge.form.env.internet.false"),
                        },
                      ]}
                      onValueChange={(value) => {
                        field.onChange(value === "true");
                      }}
                      value={String(field.value)}
                    />
                  </Field>
                </FormControl>
                <FormMessage />
              </FormItem>
            )}
          />
        </div>
        {form.watch("containers")?.map((container, containerIndex) => (
          <div
            key={containerIndex}
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
                    {t("challenge.form.env.containers.image._")}
                  </FormLabel>
                  <FormControl>
                    <Field>
                      <FieldIcon>
                        <ContainerIcon />
                      </FieldIcon>
                      <TextField
                        {...field}
                        placeholder={"repository:tag"}
                        value={field.value || ""}
                        onChange={field.onChange}
                      />
                    </Field>
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />
            <div className={cn(["flex", "gap-5"])}>
              <FormField
                control={form.control}
                name={`containers.${containerIndex}.cpu_limit`}
                render={({ field }) => (
                  <FormItem className={cn(["w-1/3"])}>
                    <FormLabel>
                      {t("challenge.form.env.containers.cpu_limit._")}
                    </FormLabel>
                    <FormControl>
                      <Field>
                        <FieldIcon>
                          <CpuIcon />
                        </FieldIcon>
                        <NumberField
                          value={field.value}
                          onValueChange={(value) => field.onChange(value)}
                        />
                      </Field>
                    </FormControl>
                    <FormMessage />
                  </FormItem>
                )}
              />
              <FormField
                control={form.control}
                name={`containers.${containerIndex}.memory_limit`}
                render={({ field }) => (
                  <FormItem className={cn(["w-1/3"])}>
                    <FormLabel>
                      {t("challenge.form.env.containers.memory_limit._")}
                    </FormLabel>
                    <FormControl>
                      <Field>
                        <FieldIcon>
                          <MemoryStickIcon />
                        </FieldIcon>
                        <NumberField
                          value={field.value}
                          onValueChange={(value) => field.onChange(value)}
                        />
                      </Field>
                    </FormControl>
                    <FormMessage />
                  </FormItem>
                )}
              />
              <FormField
                control={form.control}
                name={`containers.${containerIndex}.image_pull_policy`}
                render={({ field }) => (
                  <FormItem className={cn(["w-1/3"])}>
                    <FormLabel>
                      {t("challenge.form.env.containers.image_pull_policy._")}
                    </FormLabel>
                    <FormControl>
                      <Field>
                        <FieldIcon>
                          <DownloadIcon />
                        </FieldIcon>
                        <Select
                          {...field}
                          options={[
                            {
                              value: "Always",
                              content: t(
                                "challenge.form.env.containers.image_pull_policy.always"
                              ),
                            },
                            {
                              value: "IfNotPresent",
                              content: t(
                                "challenge.form.env.containers.image_pull_policy.if_not_present"
                              ),
                            },
                            {
                              value: "Never",
                              content: t(
                                "challenge.form.env.containers.image_pull_policy.never"
                              ),
                            },
                          ]}
                          onValueChange={field.onChange}
                          value={field.value}
                        />
                      </Field>
                    </FormControl>
                    <FormMessage />
                  </FormItem>
                )}
              />
            </div>
            <Label>{t("challenge.form.env.containers.ports._")}</Label>
            <div className={cn(["grid", "grid-cols-3", "gap-7"])}>
              {container.ports?.map((_port, portIndex) => (
                <div
                  key={portIndex}
                  className={cn(["flex", "items-center", "gap-3"])}
                >
                  <FormField
                    control={form.control}
                    name={`containers.${containerIndex}.ports.${portIndex}.port`}
                    render={({ field }) => (
                      <FormItem className={cn(["flex-1"])}>
                        <FormControl>
                          <Field size={"sm"}>
                            <FieldIcon>
                              <MemoryStickIcon />
                            </FieldIcon>
                            <NumberField
                              value={field.value}
                              onValueChange={(value) => field.onChange(value)}
                            />
                          </Field>
                        </FormControl>
                        <FormMessage className={cn(["-top-5"])} />
                      </FormItem>
                    )}
                  />
                  <FormField
                    control={form.control}
                    name={`containers.${containerIndex}.ports.${portIndex}.protocol`}
                    render={({ field }) => (
                      <FormItem className={cn(["flex-1"])}>
                        <FormControl>
                          <Field size={"sm"}>
                            <FieldIcon>
                              <HandshakeIcon />
                            </FieldIcon>
                            <Select
                              {...field}
                              options={[
                                { value: "TCP", content: "TCP" },
                                { value: "UDP", content: "UDP" },
                              ]}
                              onValueChange={field.onChange}
                              value={field.value}
                            />
                          </Field>
                        </FormControl>
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
            <Label>{t("challenge.form.env.containers.envs._")}</Label>
            <div className={cn(["grid", "grid-cols-2", "gap-7"])}>
              {container.envs?.map((_env, envIndex) => (
                <div
                  key={envIndex}
                  className={cn(["flex", "items-center", "gap-3"])}
                >
                  <FormField
                    control={form.control}
                    name={`containers.${containerIndex}.envs.${envIndex}.key`}
                    render={({ field }) => (
                      <FormItem className={cn(["flex-1"])}>
                        <FormControl>
                          <Field size={"sm"}>
                            <FieldIcon>
                              <KeyIcon />
                            </FieldIcon>
                            <TextField
                              {...field}
                              placeholder={t(
                                "challenge.form.env.containers.envs.key._"
                              )}
                              value={field.value || ""}
                              onChange={field.onChange}
                            />
                          </Field>
                        </FormControl>
                        <FormMessage className={cn(["-top-5"])} />
                      </FormItem>
                    )}
                  />
                  <FormField
                    control={form.control}
                    name={`containers.${containerIndex}.envs.${envIndex}.value`}
                    render={({ field }) => (
                      <FormItem className={cn(["flex-1"])}>
                        <FormControl>
                          <Field size={"sm"}>
                            <FieldIcon>
                              <TextIcon />
                            </FieldIcon>
                            <TextField
                              {...field}
                              placeholder={t(
                                "challenge.form.env.containers.envs.value._"
                              )}
                              value={field.value || ""}
                              onChange={field.onChange}
                            />
                          </Field>
                        </FormControl>
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
              {t("challenge.form.env.containers.actions.delete")}
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
          {t("challenge.form.env.containers.actions.add")}
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
          {t("common.actions.save")}
        </Button>
      </form>
    </Form>
  );
}
