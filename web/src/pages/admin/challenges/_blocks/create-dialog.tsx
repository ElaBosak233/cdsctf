import { zodResolver } from "@hookform/resolvers/zod";
import { CheckIcon, LibraryIcon, TypeIcon } from "lucide-react";
import { useState } from "react";
import { useForm } from "react-hook-form";
import { useTranslation } from "react-i18next";
import { toast } from "sonner";
import { z } from "zod";
import { createChallenge } from "@/api/admin/challenges";
import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";
import { DialogHeader } from "@/components/ui/dialog";
import { Field, FieldIcon } from "@/components/ui/field";
import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "@/components/ui/form";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { TextField } from "@/components/ui/text-field";
import { useSharedStore } from "@/storages/shared";
import { cn } from "@/utils";
import { categories } from "@/utils/category";
import { notifyApiError } from "@/utils/query";

interface CreateDialogProps {
  onClose: () => void;
}

function CreateDialog(props: CreateDialogProps) {
  const { onClose } = props;
  const { t } = useTranslation();

  const sharedStore = useSharedStore();

  const [loading, setLoading] = useState<boolean>(false);

  const formSchema = z.object({
    title: z.string({
      message: t("challenge:form.title.message"),
    }),
    category: z.number({
      message: t("challenge:form.category.message"),
    }),
  });

  const form = useForm<z.infer<typeof formSchema>>({
    resolver: zodResolver(formSchema),
    defaultValues: {
      category: 1,
    },
  });

  async function onSubmit(values: z.infer<typeof formSchema>) {
    setLoading(true);
    try {
      const res = await createChallenge({
        ...values,
        description: "",
        has_instance: false,
        public: false,
        has_attachment: false,
      });
      toast.success(
        t("challenge:actions.create.success", { title: res.challenge?.title })
      );
      onClose();
    } catch (error) {
      await notifyApiError(error);
    } finally {
      sharedStore.setRefresh();
      setLoading(false);
    }
  }

  return (
    <Card
      className={cn([
        "w-full",
        "max-w-xl",
        "min-h-64",
        "rounded-elevated",
        "shadow-lg",
        "overflow-hidden",
        "flex",
        "flex-col",
      ])}
    >
      <div className={cn(["p-5", "flex", "flex-col", "gap-5"])}>
        <DialogHeader
          icon={<LibraryIcon />}
          title={t("challenge:actions.create._")}
        />
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
                <FormItem>
                  <FormLabel>{t("challenge:form.title._")}</FormLabel>
                  <FormControl>
                    <Field size={"sm"}>
                      <FieldIcon>
                        <TypeIcon />
                      </FieldIcon>
                      <TextField
                        {...field}
                        placeholder={t("challenge:form.title.placeholder")}
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
              name={"category"}
              render={({ field }) => (
                <FormItem>
                  <FormLabel>{t("challenge:form.category._")}</FormLabel>
                  <FormControl>
                    <Field size={"sm"}>
                      <FieldIcon>
                        <LibraryIcon />
                      </FieldIcon>
                      <Select
                        {...field}
                        onValueChange={(value) => {
                          field.onChange(Number(value));
                        }}
                        value={String(field.value)}
                      >
                        <SelectTrigger>
                          <SelectValue />
                        </SelectTrigger>
                        <SelectContent>
                          {categories?.map((category) => {
                            const Icon = category.icon!;
                            return (
                              <SelectItem
                                key={category.id}
                                value={String(category.id)}
                              >
                                <span
                                  className={cn([
                                    "flex",
                                    "gap-2",
                                    "items-center",
                                  ])}
                                >
                                  <Icon />
                                  {category.name?.toUpperCase()}
                                </span>
                              </SelectItem>
                            );
                          })}
                        </SelectContent>
                      </Select>
                    </Field>
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />
            <Button
              variant={"solid"}
              icon={<CheckIcon />}
              level={"success"}
              loading={loading}
              type={"submit"}
            >
              {t("common:actions.confirm")}
            </Button>
          </form>
        </Form>
      </div>
    </Card>
  );
}

export { CreateDialog };
