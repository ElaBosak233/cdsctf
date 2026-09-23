import { zodResolver } from "@hookform/resolvers/zod";
import {
  BoxIcon,
  ContainerIcon,
  FolderIcon,
  LibraryIcon,
  PencilLineIcon,
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
import { Field, FieldIcon } from "@/components/ui/field";
import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "@/components/ui/form";
import { MarkdownEditor } from "@/components/ui/markdown-editor";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
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
      message: t("challenge:form.title.message"),
    }),
    category: z.number({
      message: t("challenge:form.category.message"),
    }),
    tags: z.array(z.string()).nullish(),
    description: z.string({
      message: t("challenge:form.description.message"),
    }),
    has_attachment: z.boolean({}),
    has_writeup: z.boolean({}),
    has_instance: z.boolean({}),
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
        toast.success(
          t("challenge:actions.update.success", {
            title: res?.challenge?.title,
          })
        );
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
        <div className={cn(["flex", "flex-col", "gap-5", "sm:flex-row"])}>
          <FormField
            control={form.control}
            name={"title"}
            render={({ field }) => (
              <FormItem className={cn(["w-full", "sm:w-3/4"])}>
                <FormLabel>{t("challenge:form.title._")}</FormLabel>
                <FormControl>
                  <Field>
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
              <FormItem className={cn(["w-full", "sm:w-1/4"])}>
                <FormLabel>{t("challenge:form.category._")}</FormLabel>
                <FormControl>
                  <Field>
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
        </div>
        <FormField
          control={form.control}
          name={"tags"}
          render={({ field }) => (
            <FormItem className={cn(["w-full"])}>
              <FormLabel>{t("challenge:form.tags._")}</FormLabel>
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
        <div className={cn(["flex", "flex-col", "gap-5", "sm:flex-row"])}>
          <FormField
            control={form.control}
            name={"has_attachment"}
            render={({ field }) => (
              <FormItem className={cn(["w-full", "sm:w-1/3"])}>
                <FormLabel>{t("challenge:form.has_attachment._")}</FormLabel>
                <FormControl>
                  <Field>
                    <FieldIcon>
                      <FolderIcon />
                    </FieldIcon>
                    <Select
                      {...field}
                      onValueChange={(value) => {
                        field.onChange(value === "true");
                      }}
                      value={String(field.value)}
                    >
                      <SelectTrigger>
                        <SelectValue />
                      </SelectTrigger>
                      <SelectContent>
                        <SelectItem value="true">
                          {t("challenge:form.has_attachment.true")}
                        </SelectItem>
                        <SelectItem value="false">
                          {t("challenge:form.has_attachment.false")}
                        </SelectItem>
                      </SelectContent>
                    </Select>
                  </Field>
                </FormControl>
                <FormMessage />
              </FormItem>
            )}
          />
          <FormField
            control={form.control}
            name={"has_instance"}
            render={({ field }) => (
              <FormItem className={cn(["w-full", "sm:w-1/3"])}>
                <FormLabel>{t("challenge:form.has_instance._")}</FormLabel>
                <FormControl>
                  <Field>
                    <FieldIcon>
                      <ContainerIcon />
                    </FieldIcon>
                    <Select
                      {...field}
                      onValueChange={(value) => {
                        field.onChange(value === "true");
                      }}
                      value={String(field.value)}
                    >
                      <SelectTrigger>
                        <SelectValue />
                      </SelectTrigger>
                      <SelectContent>
                        <SelectItem value="true">
                          <span
                            className={cn(["flex", "gap-2", "items-center"])}
                          >
                            <ShipWheelIcon />
                            {t("challenge:form.has_instance.true")}
                          </span>
                        </SelectItem>
                        <SelectItem value="false">
                          <span
                            className={cn(["flex", "gap-2", "items-center"])}
                          >
                            <BoxIcon />
                            {t("challenge:form.has_instance.false")}
                          </span>
                        </SelectItem>
                      </SelectContent>
                    </Select>
                  </Field>
                </FormControl>
                <FormMessage />
              </FormItem>
            )}
          />
          <FormField
            control={form.control}
            name={"has_writeup"}
            render={({ field }) => (
              <FormItem className={cn(["w-full", "sm:w-1/3"])}>
                <FormLabel>{t("challenge:form.has_writeup._")}</FormLabel>
                <FormControl>
                  <Field>
                    <FieldIcon>
                      <PencilLineIcon />
                    </FieldIcon>
                    <Select
                      {...field}
                      onValueChange={(value) => {
                        field.onChange(value === "true");
                      }}
                      value={String(field.value)}
                    >
                      <SelectTrigger>
                        <SelectValue />
                      </SelectTrigger>
                      <SelectContent>
                        <SelectItem value="true">
                          {t("challenge:form.has_writeup.true")}
                        </SelectItem>
                        <SelectItem value="false">
                          {t("challenge:form.has_writeup.false")}
                        </SelectItem>
                      </SelectContent>
                    </Select>
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
              <FormLabel>{t("challenge:form.description._")}</FormLabel>
              <FormControl>
                <MarkdownEditor
                  {...field}
                  placeholder={t("challenge:form.description.placeholder")}
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
          {t("common:actions.save")}
        </Button>
      </form>
    </Form>
  );
}
