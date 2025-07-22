import { zodResolver } from "@hookform/resolvers/zod";
import { StatusCodes } from "http-status-codes";
import { MessageCircleIcon, SaveIcon, TypeIcon } from "lucide-react";
import { useContext } from "react";
import { useForm } from "react-hook-form";
import { toast } from "sonner";
import { z } from "zod";
import { createGameNotice } from "@/api/admin/games/game_id/notices";
import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";
import { Editor } from "@/components/ui/editor";
import { Field, FieldIcon } from "@/components/ui/field";
import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "@/components/ui/form";
import { TextField } from "@/components/ui/text-field";
import { useSharedStore } from "@/storages/shared";
import { cn } from "@/utils";
import { Context } from "../context";

interface CreateDialogProps {
  onClose: () => void;
}

function CreateDialog(props: CreateDialogProps) {
  const { onClose } = props;

  const { game } = useContext(Context);
  const sharedStore = useSharedStore();

  const formSchema = z.object({
    title: z.string({
      message: "请填写标题",
    }),
    content: z.string({
      message: "请填写内容",
    }),
  });

  const form = useForm<z.infer<typeof formSchema>>({
    resolver: zodResolver(formSchema),
  });

  function onSubmit(values: z.infer<typeof formSchema>) {
    createGameNotice({
      game_id: game?.id,
      ...values,
    }).then((res) => {
      if (res.code === StatusCodes.OK) {
        toast.success(`通知 ${res?.data?.title} 发布成功`);
        sharedStore?.setRefresh();
        onClose();
      }
    });
  }

  return (
    <Card
      className={cn(["p-5", "w-156", "min-h-64", "flex", "flex-col", "gap-5"])}
    >
      <h3 className={cn(["flex", "gap-3", "items-center", "text-md"])}>
        <MessageCircleIcon className={cn(["size-4"])} />
        添加通知
      </h3>
      <Form {...form}>
        <form
          onSubmit={form.handleSubmit(onSubmit)}
          autoComplete={"off"}
          className={cn(["flex", "flex-col", "flex-1", "gap-5"])}
        >
          <FormField
            control={form.control}
            name={"title"}
            render={({ field }) => (
              <FormItem className={cn(["w-full"])}>
                <FormLabel>标题</FormLabel>
                <FormControl>
                  <Field size={"sm"}>
                    <FieldIcon>
                      <TypeIcon />
                    </FieldIcon>
                    <TextField
                      {...field}
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
            name={"content"}
            render={({ field }) => (
              <FormItem className={cn(["w-full"])}>
                <FormLabel>内容</FormLabel>
                <FormControl>
                  <Editor
                    {...field}
                    lang={"markdown"}
                    showLineNumbers
                    className={cn(["h-full", "min-h-64"])}
                  />
                </FormControl>
                <FormMessage />
              </FormItem>
            )}
          />
          <Button icon={<SaveIcon />} variant={"solid"} type={"submit"}>
            保存
          </Button>
        </form>
      </Form>
    </Card>
  );
}

export { CreateDialog };
