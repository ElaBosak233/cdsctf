import { zodResolver } from "@hookform/resolvers/zod";
import { StatusCodes } from "http-status-codes";
import {
  BoxIcon,
  ContainerIcon,
  FolderIcon,
  LibraryIcon,
  SaveIcon,
  ShipWheelIcon,
  TagIcon,
  TypeIcon,
} from "lucide-react";
import { useContext, useEffect, useState } from "react";
import { useForm } from "react-hook-form";
import { useTranslation } from "react-i18next";
import { toast } from "sonner";
import { z } from "zod";
import { updateChallenge } from "@/api/admin/challenges/challenge_id";
import { Button } from "@/components/ui/button";
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
import { Select } from "@/components/ui/select";
import { TagsField } from "@/components/ui/tags-field";
import { TextField } from "@/components/ui/text-field";
import { useSharedStore } from "@/storages/shared";
import { cn } from "@/utils";
import { categories } from "@/utils/category";
import { Context } from "./context";

export default function Index() {
  const { t } = useTranslation();

  const { challenge } = useContext(Context);
  const sharedStore = useSharedStore();

  const [loading, setLoading] = useState<boolean>(false);

  const formSchema = z.object({
    title: z.string({
      message: t("challenge.form.title.message"),
    }),
    category: z.number({
      message: t("challenge.form.category.message"),
    }),
    tags: z.array(z.string()).nullish(),
    description: z.string({
      message: t("challenge.form.description.message"),
    }),
    has_attachment: z.boolean({}),
    is_dynamic: z.boolean({}),
  });

  const form = useForm<z.infer<typeof formSchema>>({
    resolver: zodResolver(formSchema),
    defaultValues: challenge,
  });

  useEffect(() => {
    form.reset(challenge, {
      keepDefaultValues: false,
    });
  }, [challenge, form]);

  function onSubmit(values: z.infer<typeof formSchema>) {
    setLoading(true);
    updateChallenge({
      id: challenge?.id,
      ...values,
    })
      .then((res) => {
        if (res.code === StatusCodes.OK) {
          toast.success(`题目 ${res?.data?.title} 更新成功`);
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
        <div className={cn(["flex", "gap-5"])}>
          <FormField
            control={form.control}
            name={"title"}
            render={({ field }) => (
              <FormItem className={cn(["w-3/4"])}>
                <FormLabel>{t("challenge.form.title._")}</FormLabel>
                <FormControl>
                  <Field>
                    <FieldIcon>
                      <TypeIcon />
                    </FieldIcon>
                    <TextField
                      {...field}
                      placeholder={"Try Hack Me!"}
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
              <FormItem className={cn(["w-1/4"])}>
                <FormLabel>{t("challenge.form.category._")}</FormLabel>
                <FormControl>
                  <Field>
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
        </div>
        <FormField
          control={form.control}
          name={"tags"}
          render={({ field }) => (
            <FormItem className={cn(["w-full"])}>
              <FormLabel>{t("challenge.form.tags._")}</FormLabel>
              <FormControl>
                <Field>
                  <FieldIcon>
                    <TagIcon />
                  </FieldIcon>
                  <TagsField
                    value={field.value || []}
                    onValueChange={(value) => field.onChange(value)}
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
            name={"has_attachment"}
            render={({ field }) => (
              <FormItem className={cn(["w-1/2"])}>
                <FormLabel>{t("challenge.form.has_attachment._")}</FormLabel>
                <FormControl>
                  <Field>
                    <FieldIcon>
                      <FolderIcon />
                    </FieldIcon>
                    <Select
                      {...field}
                      options={[
                        {
                          value: String(true),
                          content: t("challenge.form.has_attachment.true"),
                        },
                        {
                          value: String(false),
                          content: t("challenge.form.has_attachment.false"),
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
          <FormField
            control={form.control}
            name={"is_dynamic"}
            render={({ field }) => (
              <FormItem className={cn(["w-1/2"])}>
                <FormLabel>{t("challenge.form.is_dynamic._")}</FormLabel>
                <FormControl>
                  <Field>
                    <FieldIcon>
                      <ContainerIcon />
                    </FieldIcon>
                    <Select
                      {...field}
                      options={[
                        {
                          value: String(true),
                          content: (
                            <div
                              className={cn(["flex", "gap-2", "items-center"])}
                            >
                              <ShipWheelIcon />
                              {t("challenge.form.is_dynamic.true")}
                            </div>
                          ),
                        },
                        {
                          value: String(false),
                          content: (
                            <div
                              className={cn(["flex", "gap-2", "items-center"])}
                            >
                              <BoxIcon />
                              {t("challenge.form.is_dynamic.false")}
                            </div>
                          ),
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
        <FormField
          control={form.control}
          name={"description"}
          render={({ field }) => (
            <FormItem className={cn(["flex-1", "flex", "flex-col"])}>
              <FormLabel>{t("challenge.form.description._")}</FormLabel>
              <FormControl>
                <Editor
                  {...field}
                  placeholder={"Once upon a time..."}
                  lang={"markdown"}
                  className={cn(["h-full", "min-h-64"])}
                />
              </FormControl>
              <FormMessage />
            </FormItem>
          )}
        />
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
