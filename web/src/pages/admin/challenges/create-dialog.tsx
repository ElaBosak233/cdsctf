import { zodResolver } from "@hookform/resolvers/zod";
import { StatusCodes } from "http-status-codes";
import { CheckIcon, LibraryIcon, TypeIcon } from "lucide-react";
import { useState } from "react";
import { useForm } from "react-hook-form";
import { useTranslation } from "react-i18next";
import { toast } from "sonner";
import { z } from "zod";
import { createChallenge } from "@/api/admin/challenges";
import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";
import { Field, FieldIcon } from "@/components/ui/field";
import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "@/components/ui/form";
import { Select } from "@/components/ui/select";
import { TextField } from "@/components/ui/text-field";
import { useSharedStore } from "@/storages/shared";
import { cn } from "@/utils";
import { categories } from "@/utils/category";

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
      message: t("challenge.form.title.message"),
    }),
    category: z.number({
      message: t("challenge.form.category.message"),
    }),
  });

  const form = useForm<z.infer<typeof formSchema>>({
    resolver: zodResolver(formSchema),
    defaultValues: {
      category: 1,
    },
  });

  function onSubmit(values: z.infer<typeof formSchema>) {
    setLoading(true);
    createChallenge({
      ...values,
      description: "",
      is_dynamic: false,
      is_public: false,
      has_attachment: false,
    })
      .then((res) => {
        if (res.code === StatusCodes.OK) {
          toast.success(
            t("challenge.actions.create.success", { title: res.data?.title })
          );
          onClose();
        }
      })
      .finally(() => {
        sharedStore.setRefresh();
        setLoading(false);
      });
  }

  return (
    <Card
      className={cn(["w-lg", "min-h-64", "p-5", "flex", "flex-col", "gap-5"])}
    >
      <h3 className={cn(["flex", "gap-3", "items-center", "text-md"])}>
        <LibraryIcon className={cn(["size-4"])} />
        {t("challenge.actions.create._")}
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
              <FormItem>
                <FormLabel>{t("challenge.form.title._")}</FormLabel>
                <FormControl>
                  <Field size={"sm"}>
                    <FieldIcon>
                      <TypeIcon />
                    </FieldIcon>
                    <TextField
                      {...field}
                      placeholder={"Try hack me..."}
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
                <FormLabel>{t("challenge.form.category._")}</FormLabel>
                <FormControl>
                  <Field size={"sm"}>
                    <FieldIcon>
                      <LibraryIcon />
                    </FieldIcon>
                    <Select
                      {...field}
                      options={categories?.map((category) => {
                        const Icon = category.icon!;

                        return {
                          value: String(category?.id),
                          content: (
                            <div
                              className={cn(["flex", "gap-2", "items-center"])}
                            >
                              <Icon />
                              {category?.name?.toUpperCase()}
                            </div>
                          ),
                        };
                      })}
                      onValueChange={(value) => {
                        field.onChange(Number(value));
                      }}
                      value={String(field.value)}
                    />
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
            {t("common.actions.confirm")}
          </Button>
        </form>
      </Form>
    </Card>
  );
}

export { CreateDialog };
