import { zodResolver } from "@hookform/resolvers/zod";
import { CheckIcon, IdCardIcon, TypeIcon } from "lucide-react";
import { useState } from "react";
import { useForm } from "react-hook-form";
import { useTranslation } from "react-i18next";
import { useNavigate } from "react-router";
import { toast } from "sonner";
import { z } from "zod";
import { createAdminIdp } from "@/api/admin/idps";
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
import { TextField } from "@/components/ui/text-field";
import { useSharedStore } from "@/storages/shared";
import { cn } from "@/utils";

import defaultScript from "../idp_id/_blocks/examples/default.cdsx?raw";

interface CreateDialogProps {
  onClose: () => void;
}

function CreateDialog(props: CreateDialogProps) {
  const { onClose } = props;
  const { t } = useTranslation();
  const navigate = useNavigate();
  const sharedStore = useSharedStore();
  const [loading, setLoading] = useState(false);

  const formSchema = z.object({
    name: z.string().min(1, {
      message: t("admin:idp.form.name.message"),
    }),
  });

  const form = useForm<z.infer<typeof formSchema>>({
    resolver: zodResolver(formSchema),
    defaultValues: {
      name: "",
    },
  });

  function onSubmit(values: z.infer<typeof formSchema>) {
    setLoading(true);
    createAdminIdp({
      name: values.name,
      enabled: false,
      portal: null,
      script: defaultScript,
    })
      .then((res) => {
        toast.success(
          t("admin:idp.actions.create.success", { name: res.idp?.name })
        );
        onClose();
        if (res.idp?.id != null) {
          navigate(`/admin/idps/${res.idp.id}`);
        }
      })
      .finally(() => {
        sharedStore.setRefresh();
        setLoading(false);
      });
  }

  return (
    <Card
      className={cn(["w-lg", "min-h-48", "p-5", "flex", "flex-col", "gap-5"])}
    >
      <h3 className={cn(["flex", "gap-3", "items-center", "text-md"])}>
        <IdCardIcon className={cn(["size-4"])} />
        {t("admin:idp.actions.create._")}
      </h3>
      <Form {...form}>
        <form
          onSubmit={form.handleSubmit(onSubmit)}
          autoComplete="off"
          className={cn(["flex", "flex-col", "flex-1", "gap-5"])}
        >
          <FormField
            control={form.control}
            name="name"
            render={({ field }) => (
              <FormItem>
                <FormLabel>{t("admin:idp.form.name._")}</FormLabel>
                <FormControl>
                  <Field size="sm">
                    <FieldIcon>
                      <TypeIcon />
                    </FieldIcon>
                    <TextField
                      {...field}
                      placeholder={t("admin:idp.form.name.placeholder")}
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
            variant="solid"
            icon={<CheckIcon />}
            level="success"
            loading={loading}
            type="submit"
          >
            {t("common:actions.confirm")}
          </Button>
        </form>
      </Form>
    </Card>
  );
}

export { CreateDialog };
