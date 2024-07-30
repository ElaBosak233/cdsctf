import { useUserApi } from "@/api/user";
import MDIcon from "@/components/ui/MDIcon";
import { useAuthStore } from "@/stores/auth";
import { useConfigStore } from "@/stores/config";
import { Box, Button, Flex, Group, Stack, TextInput } from "@mantine/core";
import { useForm, zodResolver } from "@mantine/form";
import { useEffect, useState } from "react";
import { useNavigate } from "react-router-dom";
import { User } from "@/types/user";
import {
    showErrNotification,
    showSuccessNotification,
} from "@/utils/notification";
import Turnstile from "react-turnstile";
import ReCAPTCHA from "react-google-recaptcha";
import { z } from "zod";

export default function Page() {
    const configStore = useConfigStore();
    const navigate = useNavigate();
    const userApi = useUserApi();
    const authStore = useAuthStore();

    useEffect(() => {
        document.title = `注册 - ${configStore?.pltCfg?.site?.title}`;
    }, []);

    const [registerLoading, setRegisterLoading] = useState(false);

    const form = useForm({
        mode: "controlled",
        initialValues: {
            username: "",
            nickname: "",
            password: "",
            email: "",
            token: "",
        },

        validate: zodResolver(
            z.object({
                username: z.string().regex(/^[a-z0-9_]{4,16}$/, {
                    message:
                        "用户名只能包含小写字母、数字和下划线，长度为 4-16 位",
                }),
                nickname: z.string().min(1, { message: "昵称不能为空" }),
                email: z.string().email({ message: "邮箱格式不正确" }),
                password: z.string().min(6, { message: "密码长度至少为 6 位" }),
            })
        ),
    });

    function register() {
        if (
            configStore?.pltCfg?.user?.register?.captcha?.enabled &&
            !form.getValues().token
        ) {
            showErrNotification({
                title: "注册失败",
                message: "请完成验证码验证",
            });
            return;
        }
        setRegisterLoading(true);
        userApi
            .register({
                username: form.getValues().username?.toLocaleLowerCase(),
                nickname: form.getValues().nickname,
                password: form.getValues().password,
                email: form.getValues().email,
                token: form.getValues().token,
            })
            .then((res) => {
                const r = res.data;
                authStore.setPgsToken(r.token as string);
                authStore.setUser(r.data as User);
                showSuccessNotification({
                    title: "注册成功",
                    message: "请登录",
                });
                navigate("/login");
            })
            .catch((err) => {
                switch (err.response?.status) {
                    case 400:
                        showErrNotification({
                            title: "注册失败",
                            message: "用户名或邮箱已被注册",
                        });
                        break;
                }
            })
            .finally(() => {
                setRegisterLoading(false);
            });
    }

    return (
        <>
            <Box
                sx={{
                    position: "fixed",
                    top: "50%",
                    left: "50%",
                    transform: "translate(-50%, -50%)",
                }}
                className={"no-select"}
            >
                <Stack>
                    <form onSubmit={form.onSubmit((_) => register())}>
                        <Stack>
                            <Group>
                                <TextInput
                                    label="用户名"
                                    size="lg"
                                    leftSection={<MDIcon>person</MDIcon>}
                                    key={form.key("username")}
                                    {...form.getInputProps("username")}
                                />
                                <TextInput
                                    label="昵称"
                                    size="lg"
                                    leftSection={<MDIcon>person</MDIcon>}
                                    key={form.key("nickname")}
                                    {...form.getInputProps("nickname")}
                                />
                            </Group>
                            <TextInput
                                label="邮箱"
                                size="lg"
                                leftSection={<MDIcon>email</MDIcon>}
                                key={form.key("email")}
                                {...form.getInputProps("email")}
                            />
                            <TextInput
                                label="密码"
                                type="password"
                                size="lg"
                                leftSection={<MDIcon>lock</MDIcon>}
                                key={form.key("password")}
                                {...form.getInputProps("password")}
                            />
                            <Flex justify={"center"}>
                                {configStore?.pltCfg?.user?.register?.captcha
                                    ?.enabled && (
                                    <>
                                        {configStore?.captchaCfg?.provider ===
                                            "turnstile" && (
                                            <Turnstile
                                                sitekey={String(
                                                    configStore?.captchaCfg
                                                        ?.turnstile?.site_key
                                                )}
                                                onVerify={(token) => {
                                                    form.setValues({
                                                        ...form.values,
                                                        token: token,
                                                    });
                                                }}
                                            />
                                        )}
                                        {configStore?.captchaCfg?.provider ===
                                            "recaptcha" && (
                                            <ReCAPTCHA
                                                sitekey={String(
                                                    configStore?.captchaCfg
                                                        ?.recaptcha?.site_key
                                                )}
                                                onChange={(token) => {
                                                    form.setValues({
                                                        ...form.values,
                                                        token: String(token),
                                                    });
                                                }}
                                            />
                                        )}
                                    </>
                                )}
                            </Flex>
                            <Button
                                loading={registerLoading}
                                size={"lg"}
                                fullWidth
                                sx={{ bgcolor: "primary.700" }}
                                type="submit"
                            >
                                注册
                            </Button>
                        </Stack>
                    </form>
                    <Box
                        sx={{
                            display: "flex",
                            marginTop: "1rem",
                            justifyContent: "end",
                        }}
                    >
                        已有帐号？
                        <Box
                            onClick={() => navigate("/login")}
                            sx={{
                                fontStyle: "italic",
                                ":hover": {
                                    cursor: "pointer",
                                },
                            }}
                        >
                            登录
                        </Box>
                    </Box>
                </Stack>
            </Box>
        </>
    );
}
