import {
    getUserAvatarMetadata,
    getUsers,
    saveUserAvatar,
    updateUser,
} from "@/api/user";
import MDIcon from "@/components/ui/MDIcon";
import { useAuthStore } from "@/stores/auth";
import { Metadata } from "@/types/media";
import { User } from "@/types/user";
import {
    showErrNotification,
    showSuccessNotification,
} from "@/utils/notification";
import {
    Box,
    Button,
    Card,
    Center,
    Divider,
    Flex,
    Stack,
    Text,
    TextInput,
    Image,
} from "@mantine/core";
import { Dropzone } from "@mantine/dropzone";
import { useForm, zodResolver } from "@mantine/form";
import { AxiosRequestConfig } from "axios";
import { useEffect, useState } from "react";
import { z } from "zod";

export default function Page() {
    const authStore = useAuthStore();

    const [user, setUser] = useState<User>();
    const [avatarMetadata, setAvatarMetadata] = useState<Metadata>();

    const [refresh, setRefresh] = useState<number>(0);

    const form = useForm({
        initialValues: {
            username: "",
            email: "",
            nickname: "",
            password: "",
        },
        validate: zodResolver(
            z.object({
                nickname: z.string().min(1, { message: "昵称不能为空" }),
                email: z.string().email({ message: "邮箱格式不正确" }),
                password: z
                    .string()
                    .optional()
                    .refine((val) => val === "" || Number(val?.length) >= 6, {
                        message: "密码长度至少为 6 位",
                    }),
            })
        ),
    });

    function handleGetUser() {
        getUsers({
            id: authStore.user?.id,
        }).then((res) => {
            const r = res.data;
            setUser(r.data?.[0]);
            authStore.setUser(r.data?.[0]);
        });
    }

    function handleUpdateUser() {
        updateUser({
            id: Number(user?.id),
            nickname: form.getValues().nickname,
            email: form.getValues().email,
            password: form.getValues().password
                ? form.getValues().password
                : undefined,
        })
            .then((_) => {
                showSuccessNotification({
                    message: `个人资料更新成功`,
                });
            })
            .catch((e) => {
                showErrNotification({
                    message: e.response.data.error || "更新用户失败",
                });
            })
            .finally(() => {
                setRefresh((prev) => prev + 1);
            });
    }

    function handleGetUserAvatarMetadata() {
        getUserAvatarMetadata(Number(user?.id)).then((res) => {
            const r = res.data;
            setAvatarMetadata(r.data);
        });
    }

    function handleSaveUserAvatar(file?: File) {
        const config: AxiosRequestConfig<FormData> = {};
        saveUserAvatar(Number(user?.id), file!, config)
            .then((_) => {
                showSuccessNotification({
                    message: `用户 ${form.getValues().nickname} 头像更新成功`,
                });
            })
            .finally(() => {
                setRefresh((prev) => prev + 1);
            });
    }

    useEffect(() => {
        handleGetUser();
    }, [refresh]);

    useEffect(() => {
        if (user) {
            form.setValues({
                username: user.username,
                email: user.email,
                nickname: user.nickname,
                password: "",
            });
            handleGetUserAvatarMetadata();
        }
    }, [user]);

    return (
        <>
            <Box
                sx={{
                    position: "fixed",
                    top: "50%",
                    left: "50%",
                    transform: "translate(-50%, -50%)",
                }}
            >
                <Card
                    shadow="md"
                    padding={"lg"}
                    radius={"md"}
                    withBorder
                    w={"50rem"}
                >
                    <Flex gap={10} align={"center"}>
                        <MDIcon>person</MDIcon>
                        <Text fw={600}>个人资料</Text>
                    </Flex>
                    <Divider my={10} />
                    <Box p={10}>
                        <form
                            onSubmit={form.onSubmit((_) => handleUpdateUser())}
                        >
                            <Stack gap={10}>
                                <Flex gap={10}>
                                    <Stack flex={1}>
                                        <Flex gap={10} w={"100%"}>
                                            <TextInput
                                                label="用户名"
                                                size="md"
                                                w={"40%"}
                                                disabled
                                                leftSection={
                                                    <MDIcon>person</MDIcon>
                                                }
                                                key={form.key("username")}
                                                {...form.getInputProps(
                                                    "username"
                                                )}
                                            />
                                            <TextInput
                                                label="昵称"
                                                size="md"
                                                w={"60%"}
                                                key={form.key("nickname")}
                                                {...form.getInputProps(
                                                    "nickname"
                                                )}
                                            />
                                        </Flex>
                                        <TextInput
                                            label="邮箱"
                                            size="md"
                                            leftSection={<MDIcon>email</MDIcon>}
                                            key={form.key("email")}
                                            {...form.getInputProps("email")}
                                        />
                                    </Stack>
                                    <Dropzone
                                        onDrop={(files: any) =>
                                            handleSaveUserAvatar(files[0])
                                        }
                                        onReject={() => {
                                            showErrNotification({
                                                message: "文件上传失败",
                                            });
                                        }}
                                        h={150}
                                        w={150}
                                        accept={[
                                            "image/png",
                                            "image/gif",
                                            "image/jpeg",
                                            "image/webp",
                                            "image/avif",
                                            "image/heic",
                                        ]}
                                    >
                                        <Center
                                            style={{
                                                pointerEvents: "none",
                                            }}
                                        >
                                            {avatarMetadata ? (
                                                <Center>
                                                    <Image
                                                        w={120}
                                                        h={120}
                                                        fit="contain"
                                                        src={`/api/users/${user?.id}/avatar?refresh=${refresh}`}
                                                    />
                                                </Center>
                                            ) : (
                                                <Center>
                                                    <Stack gap={0}>
                                                        <Text size="xl" inline>
                                                            拖拽或点击上传头像
                                                        </Text>
                                                        <Text
                                                            size="sm"
                                                            c="dimmed"
                                                            inline
                                                            mt={7}
                                                        >
                                                            图片大小不超过 3MB
                                                        </Text>
                                                    </Stack>
                                                </Center>
                                            )}
                                        </Center>
                                    </Dropzone>
                                </Flex>
                                <TextInput
                                    label="密码"
                                    size="md"
                                    leftSection={<MDIcon>lock</MDIcon>}
                                    key={form.key("password")}
                                    {...form.getInputProps("password")}
                                />
                            </Stack>
                            <Flex mt={20} justify={"end"}>
                                <Button
                                    type="submit"
                                    leftSection={
                                        <MDIcon c={"white"}>check</MDIcon>
                                    }
                                >
                                    更新
                                </Button>
                            </Flex>
                        </form>
                    </Box>
                </Card>
            </Box>
        </>
    );
}
