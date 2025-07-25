import { zodResolver } from "@hookform/resolvers/zod";
import { StatusCodes } from "http-status-codes";
import { HTTPError } from "ky";
import {
  CheckIcon,
  LockIcon,
  MailIcon,
  TypeIcon,
  UserRoundIcon,
} from "lucide-react";
import { useState } from "react";
import { useForm } from "react-hook-form";
import { useNavigate } from "react-router";
import { toast } from "sonner";
import { z } from "zod";
import { register } from "@/api/users";
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
import { TextField } from "@/components/ui/text-field";
import { Captcha } from "@/components/widgets/captcha";
import { useConfigStore } from "@/storages/config";
import { cn } from "@/utils";
import { parseErrorResponse } from "@/utils/query";

function RegisterForm() {
  const configStore = useConfigStore();
  const navigate = useNavigate();

  const [loading, setLoading] = useState<boolean>(false);

  const formSchema = z
    .object({
      username: z
        .string({
          message: "请输入用户名",
        })
        .regex(/^[a-z]/, "用户名必须以小写字母开头")
        .regex(/^[a-z0-9]*$/, "用户名只能包含小写字母和数字"),
      name: z.string({
        message: "请输入昵称",
      }),
      email: z.email("邮箱不合法"),
      password: z
        .string({
          message: "请输入密码",
        })
        .min(6, "密码最少需要 6 个字符"),
      confirm_password: z.string({
        message: "请重新输入新密码",
      }),
    })
    .refine((data) => data.password === data.confirm_password, {
      message: "新密码与确认密码不一致",
      path: ["confirm_password"],
    });

  const [captcha, setCaptcha] = useState<{
    id?: string;
    content?: string;
  }>();

  const form = useForm<z.infer<typeof formSchema>>({
    resolver: zodResolver(formSchema),
  });

  async function onSubmit(values: z.infer<typeof formSchema>) {
    setLoading(true);
    try {
      const res = await register({
        captcha,
        ...values,
      });

      if (res.code === StatusCodes.OK) {
        toast.success("注册成功", {
          id: "register-success",
          description: "注册成功，请登录",
        });
        navigate("/account/login");
      }
    } catch (error) {
      if (!(error instanceof HTTPError)) throw error;
      const res = await parseErrorResponse(error);

      if (res.code === StatusCodes.BAD_REQUEST) {
        toast.success("注册失败", {
          id: "register-error",
          description: res.msg,
        });
      }

      if (res.code === StatusCodes.CONFLICT) {
        toast.success("注册失败", {
          id: "register-error",
          description: "用户名或邮箱重复",
        });
      }
    } finally {
      setLoading(false);
    }
  }

  return (
    <Form {...form}>
      <form
        onSubmit={form.handleSubmit(onSubmit)}
        autoComplete={"off"}
        className={cn(["flex", "flex-col", "h-full", "gap-8"])}
      >
        <div className={cn("space-y-3", "flex-1")}>
          <div className={cn(["flex", "gap-3", "items-center"])}>
            <FormField
              control={form.control}
              name={"username"}
              render={({ field }) => (
                <FormItem className={cn(["flex-1"])}>
                  <FormLabel>用户名</FormLabel>
                  <FormControl>
                    <Field>
                      <FieldIcon>
                        <UserRoundIcon />
                      </FieldIcon>
                      <TextField {...field} />
                    </Field>
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />
            <FormField
              control={form.control}
              name={"name"}
              render={({ field }) => (
                <FormItem className={cn(["flex-1"])}>
                  <FormLabel>昵称</FormLabel>
                  <FormControl>
                    <Field>
                      <FieldIcon>
                        <TypeIcon />
                      </FieldIcon>
                      <TextField {...field} />
                    </Field>
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />
          </div>
          <FormField
            control={form.control}
            name={"email"}
            render={({ field }) => (
              <FormItem>
                <FormLabel>邮箱</FormLabel>
                <FormControl>
                  <Field>
                    <FieldIcon>
                      <MailIcon />
                    </FieldIcon>
                    <TextField {...field} />
                  </Field>
                </FormControl>
                <FormMessage />
              </FormItem>
            )}
          />
          <FormField
            control={form.control}
            name={"password"}
            render={({ field }) => (
              <FormItem>
                <FormLabel>密码</FormLabel>
                <FormControl>
                  <Field>
                    <FieldIcon>
                      <LockIcon />
                    </FieldIcon>
                    <TextField type={"password"} {...field} />
                  </Field>
                </FormControl>
                <FormMessage />
              </FormItem>
            )}
          />
          <FormField
            control={form.control}
            name={"confirm_password"}
            render={({ field }) => (
              <FormItem>
                <FormLabel>确认密码</FormLabel>
                <FormControl>
                  <Field>
                    <FieldIcon>
                      <LockIcon />
                    </FieldIcon>
                    <TextField type={"password"} {...field} />
                  </Field>
                </FormControl>
                <FormMessage />
              </FormItem>
            )}
          />
          {configStore?.config?.captcha?.provider !== "none" && (
            <FormField
              name={"captcha"}
              render={() => (
                <FormItem>
                  <FormLabel>验证码</FormLabel>
                  <Captcha onChange={setCaptcha} />
                </FormItem>
              )}
            />
          )}
        </div>
        <Button
          variant={"solid"}
          level={"info"}
          type={"submit"}
          size={"lg"}
          className={cn(["w-full"])}
          icon={<CheckIcon />}
          loading={loading}
        >
          注册
        </Button>
      </form>
    </Form>
  );
}

export { RegisterForm };
