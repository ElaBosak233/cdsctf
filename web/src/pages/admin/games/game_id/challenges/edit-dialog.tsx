import { zodResolver } from "@hookform/resolvers/zod";
import { StatusCodes } from "http-status-codes";
import {
  ClockFadingIcon,
  HashIcon,
  LibraryIcon,
  MinusIcon,
  PlusIcon,
  SaveIcon,
  StarIcon,
} from "lucide-react";
import { useContext, useEffect, useMemo } from "react";
import { useForm } from "react-hook-form";
import { Area, AreaChart, CartesianGrid, XAxis, YAxis } from "recharts";
import { toast } from "sonner";
import { z } from "zod";
import { updateGameChallenge } from "@/api/admin/games/game_id/challenges/challenge_id";
import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";
import {
  type ChartConfig,
  ChartContainer,
  ChartTooltip,
  ChartTooltipContent,
} from "@/components/ui/chart";
import { DateTimePicker } from "@/components/ui/datetime-picker";
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
import { ScrollArea } from "@/components/ui/scroll-area";
import type { GameChallenge } from "@/models/game_challenge";
import { useSharedStore } from "@/storages/shared";
import { cn } from "@/utils";
import { curve } from "@/utils/math";
import { Context } from "../context";

interface EditDialogProps {
  onClose: () => void;
  gameChallenge: GameChallenge;
}

function EditDialog(props: EditDialogProps) {
  const { onClose, gameChallenge } = props;

  const { game } = useContext(Context);
  const sharedStore = useSharedStore();

  const formSchema = z.object({
    difficulty: z.number({
      message: "请填写难度系数",
    }),
    max_pts: z.number({
      message: "请填写最大分数",
    }),
    min_pts: z.number({
      message: "请填写最小分数",
    }),
    bonus_ratios: z.array(z.number()),
    frozen_at: z.date().optional(),
  });

  const form = useForm<z.infer<typeof formSchema>>({
    resolver: zodResolver(formSchema),
    defaultValues: {
      ...gameChallenge,
      frozen_at: undefined,
    },
  });

  useEffect(() => {
    form.reset(
      {
        ...gameChallenge,
        frozen_at: gameChallenge.frozen_at
          ? new Date(Number(gameChallenge.frozen_at) * 1000)
          : undefined,
      },
      {
        keepDefaultValues: false,
      }
    );
  }, [gameChallenge, form.reset]);

  const maxPts = form.watch("max_pts");
  const minPts = form.watch("min_pts");
  const difficulty = form.watch("difficulty");

  const data = useMemo(() => {
    const rangeFactor = Math.max(1, difficulty);
    const maxTime = 15 * rangeFactor;
    const step = Math.max(5, Math.ceil(maxTime / 20));
    const points: { times: number; pts: number }[] = [];

    for (let x = 0; x <= maxTime; x += step) {
      points.push({
        times: x,
        pts: curve(Number(maxPts), Number(minPts), Number(difficulty), x),
      });
    }

    return points;
  }, [maxPts, minPts, difficulty]);

  function handleAddBonusRatio() {
    const bonus_ratios = form.getValues("bonus_ratios") || [];
    form.setValue("bonus_ratios", [...bonus_ratios, 0]);
  }

  function handleRemoveBonusRatio(index: number) {
    const bonus_ratios = form.getValues("bonus_ratios") || [];
    form.setValue(
      "bonus_ratios",
      bonus_ratios.filter((_, i) => i !== index)
    );
  }

  function onSubmit(values: z.infer<typeof formSchema>) {
    updateGameChallenge({
      game_id: game?.id,
      challenge_id: gameChallenge?.challenge_id,
      ...values,
      frozen_at: values.frozen_at
        ? Math.floor(values.frozen_at.getTime() / 1000)
        : null,
    }).then((res) => {
      if (res.code === StatusCodes.OK) {
        toast.success(`赛题 ${gameChallenge?.challenge_title} 配置更新成功`);
        sharedStore?.setRefresh();
        onClose();
      }
    });
  }

  return (
    <Card
      className={cn(["p-5", "w-156", "min-h-64", "flex", "flex-col", "gap-8"])}
    >
      <h3 className={cn(["flex", "gap-3", "items-center", "text-md"])}>
        <LibraryIcon className={cn(["size-4"])} />
        编辑赛题
      </h3>
      <ScrollArea className={cn(["max-h-144", "overflow-y-auto"])}>
        <Form {...form}>
          <form
            onSubmit={form.handleSubmit(onSubmit)}
            autoComplete={"off"}
            className={cn(["flex", "flex-col", "flex-1", "gap-5"])}
          >
            <div className={cn(["grid", "grid-cols-3", "gap-3"])}>
              <FormField
                control={form.control}
                name={"max_pts"}
                render={({ field }) => (
                  <FormItem className={cn(["w-full"])}>
                    <FormLabel>最大分值</FormLabel>
                    <FormControl>
                      <Field size={"sm"}>
                        <FieldIcon>
                          <HashIcon />
                        </FieldIcon>
                        <NumberField
                          placeholder={"最大分值"}
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
                name={"min_pts"}
                render={({ field }) => (
                  <FormItem className={cn(["w-full"])}>
                    <FormLabel>最小分值</FormLabel>
                    <FormControl>
                      <Field size={"sm"}>
                        <FieldIcon>
                          <HashIcon />
                        </FieldIcon>
                        <NumberField
                          placeholder={"最小分值"}
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
                name={"difficulty"}
                render={({ field }) => (
                  <FormItem className={cn(["w-full"])}>
                    <FormLabel>难度系数</FormLabel>
                    <FormControl>
                      <Field size={"sm"}>
                        <FieldIcon>
                          <HashIcon />
                        </FieldIcon>
                        <NumberField
                          placeholder={"难度系数"}
                          value={field.value}
                          onValueChange={(value) => field.onChange(value)}
                        />
                      </Field>
                    </FormControl>
                    <FormMessage />
                  </FormItem>
                )}
              />
            </div>
            <FormField
              control={form.control}
              name={"frozen_at"}
              render={({ field }) => (
                <FormItem>
                  <FormLabel>冻结时间</FormLabel>
                  <FormControl>
                    <Field>
                      <FieldIcon>
                        <ClockFadingIcon />
                      </FieldIcon>
                      <DateTimePicker {...field} clearable />
                    </Field>
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />
            <div className={cn(["flex", "flex-col", "gap-2"])}>
              <Label>奖励分数（%）</Label>
              <div className={cn(["grid", "grid-cols-3", "gap-3"])}>
                {form.watch("bonus_ratios").map((_ratio, ratioIndex) => (
                  <FormField
                    key={ratioIndex}
                    control={form.control}
                    name={`bonus_ratios.${ratioIndex}`}
                    render={({ field }) => (
                      <FormItem className={cn(["flex-1", "flex", "flex-col"])}>
                        <FormControl>
                          <div
                            className={cn(["flex", "items-center", "gap-3"])}
                          >
                            <Field size={"sm"} className={cn(["flex-1"])}>
                              <FieldIcon>
                                <StarIcon />
                              </FieldIcon>
                              <NumberField
                                placeholder={`No. ${ratioIndex + 1}`}
                                value={field.value}
                                onValueChange={(value) => field.onChange(value)}
                              />
                            </Field>
                            <Button
                              type={"button"}
                              icon={<MinusIcon />}
                              size={"sm"}
                              square
                              onClick={() => handleRemoveBonusRatio(ratioIndex)}
                            />
                          </div>
                        </FormControl>
                        <FormMessage />
                      </FormItem>
                    )}
                  />
                ))}
                <Button
                  type={"button"}
                  size={"sm"}
                  icon={<PlusIcon />}
                  className={cn(["self-center"])}
                  square
                  onClick={() => handleAddBonusRatio()}
                />
              </div>
            </div>
            <ChartContainer
              config={{} satisfies ChartConfig}
              className={cn(["h-[12.5rem]", "w-full"])}
            >
              <AreaChart
                accessibilityLayer
                data={data}
                margin={{
                  left: 12,
                  right: 12,
                }}
              >
                <CartesianGrid vertical={false} />
                <XAxis
                  dataKey={"times"}
                  tickLine={false}
                  axisLine={false}
                  tickMargin={8}
                />
                <YAxis dataKey={"pts"} />
                <Area
                  dataKey="pts"
                  type="monotone"
                  fill="var(--color-foreground)"
                  fillOpacity={0.4}
                  stroke="var(--color-foreground)"
                />
                <ChartTooltip
                  cursor={false}
                  content={<ChartTooltipContent indicator="line" hideLabel />}
                />
              </AreaChart>
            </ChartContainer>
            <Button type={"submit"} variant={"solid"} icon={<SaveIcon />}>
              保存
            </Button>
          </form>
        </Form>
      </ScrollArea>
    </Card>
  );
}

export { EditDialog };
