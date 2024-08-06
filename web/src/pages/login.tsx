import MDIcon from "@/components/ui/MDIcon";
import { useAuthStore } from "@/stores/auth";
import { useConfigStore } from "@/stores/config";
import { Box, Button, TextInput } from "@mantine/core";
import { useForm } from "@mantine/form";
import { useEffect, useState } from "react";
import { useNavigate } from "react-router-dom";
import {
    showErrNotification,
    showSuccessNotification,
} from "@/utils/notification";
import { login } from "@/api/user";

export default function Page() {
    const configStore = useConfigStore();
    const navigate = useNavigate();
    const authStore = useAuthStore();

    useEffect(() => {
        document.title = `登录 - ${configStore?.pltCfg?.site?.title}`;
    }, []);

    const [loginLoading, setLoginLoading] = useState(false);

    const form = useForm({
        mode: "controlled",
        initialValues: {
            account: "",
            password: "",
        },

        validate: {
            account: (value) => {
                if (value === "") {
                    return "账号不能为空";
                }
                return null;
            },
            password: (value) => {
                if (value === "") {
                    return "密码不能为空";
                }
                return null;
            },
        },
    });

    function handleLogin() {
        setLoginLoading(true);
        login({
            account: form.getValues().account?.toLocaleLowerCase(),
            password: form.getValues().password,
        })
            .then((res) => {
                const r = res.data;
                authStore.setPgsToken(r?.token);
                authStore.setUser(r?.data);
                showSuccessNotification({
                    title: "登录成功",
                    message: `欢迎进入 ${configStore?.pltCfg?.site?.title}`,
                });
                navigate("/");
                console.log(res);
            })
            .catch((err) => {
                showErrNotification({
                    title: "发生了错误",
                    message: `登录失败 ${err}`,
                });
            })
            .finally(() => {
                setLoginLoading(false);
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
                <Box
                    sx={{
                        display: "flex",
                        flexDirection: "column",
                        marginTop: "2rem",
                    }}
                >
                    <form onSubmit={form.onSubmit((_) => handleLogin())}>
                        <TextInput
                            label="用户名/邮箱"
                            size="lg"
                            leftSection={<MDIcon>person</MDIcon>}
                            key={form.key("account")}
                            {...form.getInputProps("account")}
                        />
                        <TextInput
                            label="密码"
                            type="password"
                            size="lg"
                            leftSection={<MDIcon>lock</MDIcon>}
                            mt={10}
                            key={form.key("password")}
                            {...form.getInputProps("password")}
                        />
                        <Button
                            loading={loginLoading}
                            size={"lg"}
                            fullWidth
                            sx={{ marginTop: "2rem", bgcolor: "primary.700" }}
                            type="submit"
                        >
                            登录
                        </Button>
                    </form>
                    {configStore?.pltCfg?.user?.register?.enabled && (
                        <Box
                            sx={{
                                display: "flex",
                                marginTop: "1rem",
                                justifyContent: "end",
                            }}
                        >
                            没有帐号？
                            <Box
                                onClick={() => navigate("/register")}
                                sx={{
                                    fontStyle: "italic",
                                    ":hover": {
                                        cursor: "pointer",
                                    },
                                }}
                            >
                                注册
                            </Box>
                        </Box>
                    )}
                </Box>
            </Box>
        </>
    );
}
