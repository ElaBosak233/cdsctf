import { zodResolver } from "@hookform/resolvers/zod";
import { StatusCodes } from "http-status-codes";
import {
  ClockIcon,
  ContainerIcon,
  CpuIcon,
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
import { toast } from "sonner";
import { z } from "zod";

import { Context } from "../context";

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

export default function Index() {
  const { challenge } = useContext(Context);
  const sharedStore = useSharedStore();

  const [loading, setLoading] = useState<boolean>(false);

  const formSchema = z.object({
    duration: z.number({
      message: "请输入持续时长",
    }),
    internet: z.boolean({
      message: "请选择是否允许出网",
    }),
    containers: z.array(
      z.object({
        image: z.string({
          message: "请输入镜像名",
        }),
        cpu_limit: z.number({
          message: "请输入 CPU 限制参数",
        }),
        memory_limit: z.number({
          message: "请输入内存限制参数",
        }),
        envs: z.array(
          z.object({
            key: z.string(),
            value: z.string(),
          })
        ),
        ports: z.array(
          z.object({
            port: z.number(),
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
      internet: challenge?.env?.internet || true,
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
      { port: 9999, protocol: "TCP" },
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
                <FormLabel>持续时间（秒）</FormLabel>
                <FormControl>
                  <Field>
                    <FieldIcon>
                      <ClockIcon />
                    </FieldIcon>
                    <TextField
                      {...field}
                      type={"number"}
                      placeholder={"1800"}
                      value={field.value || ""}
                      onChange={(e) => field.onChange(Number(e.target.value))}
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
                <FormLabel>是否允许出网</FormLabel>
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
                          content: "是",
                        },
                        {
                          value: String(false),
                          content: "否",
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
                  <FormLabel>镜像名</FormLabel>
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
                  <FormItem className={cn(["w-1/2"])}>
                    <FormLabel>CPU 限制</FormLabel>
                    <FormControl>
                      <Field>
                        <FieldIcon>
                          <CpuIcon />
                        </FieldIcon>
                        <TextField
                          {...field}
                          type={"number"}
                          placeholder={"2"}
                          value={field.value || ""}
                          onChange={(e) =>
                            field.onChange(Number(e.target.value))
                          }
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
                  <FormItem className={cn(["w-1/2"])}>
                    <FormLabel>内存限制（MB）</FormLabel>
                    <FormControl>
                      <Field>
                        <FieldIcon>
                          <MemoryStickIcon />
                        </FieldIcon>
                        <TextField
                          {...field}
                          type={"number"}
                          placeholder={"2"}
                          value={field.value || ""}
                          onChange={(e) =>
                            field.onChange(Number(e.target.value))
                          }
                        />
                      </Field>
                    </FormControl>
                    <FormMessage />
                  </FormItem>
                )}
              />
            </div>
            <Label>暴露端口</Label>
            <div className={cn(["grid", "grid-cols-3", "gap-5"])}>
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
                        <FormMessage />
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
            <Label>环境变量</Label>
            <div className={cn(["grid", "grid-cols-2", "gap-5"])}>
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
                              placeholder={"键"}
                              value={field.value || ""}
                              onChange={field.onChange}
                            />
                          </Field>
                        </FormControl>
                        <FormMessage />
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
                              placeholder={"值"}
                              value={field.value || ""}
                              onChange={field.onChange}
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
              删除容器
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
          添加容器
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
          保存
        </Button>
      </form>
    </Form>
  );
}
