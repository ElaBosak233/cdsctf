import {
    Card,
    Flex,
    Modal,
    ModalProps,
    Text,
    Divider,
    TextInput,
    Stack,
    Button,
    Box,
    Select,
} from "@mantine/core";
import MDIcon from "@/components/ui/MDIcon";
import { useForm } from "@mantine/form";
import { zodResolver } from "mantine-form-zod-resolver";
import { z } from "zod";
import {
    showErrNotification,
    showSuccessNotification,
} from "@/utils/notification";
import { useEffect } from "react";
import { createUser } from "@/api/user";
import { Group as UGroup } from "@/types/user";

interface UserCreateModalProps extends ModalProps {
    setRefresh: () => void;
}

export default function UserCreateModal(props: UserCreateModalProps) {
    const { setRefresh, ...modalProps } = props;

    const form = useForm({
        mode: "controlled",
        initialValues: {
            username: "",
            nickname: "",
            email: "",
            password: "",
            group: UGroup.User,
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

    function handleCreateUser() {
        createUser({
            username: form.getValues().username,
            nickname: form.getValues().nickname,
            email: form.getValues().email,
            password: form.getValues().password,
            group: form.getValues().group,
        })
            .then((_) => {
                showSuccessNotification({
                    message: `用户 ${form.getValues().username} 创建成功`,
                });
                setRefresh();
            })
            .catch((e) => {
                showErrNotification({
                    message: e.response.data.error || "创建用户失败",
                });
            })
            .finally(() => {
                form.reset();
                modalProps.onClose();
            });
    }

    useEffect(() => {
        form.reset();
    }, [modalProps.opened]);

    return (
        <>
            <Modal.Root {...modalProps}>
                <Modal.Overlay />
                <Modal.Content
                    sx={{
                        flex: "none",
                        backgroundColor: "transparent",
                    }}
                >
                    <Card
                        shadow="md"
                        padding={"lg"}
                        radius={"md"}
                        withBorder
                        w={"40rem"}
                    >
                        <Flex gap={10} align={"center"}>
                            <MDIcon>person_add</MDIcon>
                            <Text fw={600}>创建用户</Text>
                        </Flex>
                        <Divider my={10} />
                        <Box p={10}>
                            <form
                                onSubmit={form.onSubmit((_) =>
                                    handleCreateUser()
                                )}
                            >
                                <Stack gap={10}>
                                    <Flex gap={10} w={"100%"}>
                                        <TextInput
                                            label="用户名"
                                            size="md"
                                            w={"40%"}
                                            leftSection={
                                                <MDIcon>person</MDIcon>
                                            }
                                            key={form.key("username")}
                                            {...form.getInputProps("username")}
                                        />
                                        <TextInput
                                            label="昵称"
                                            size="md"
                                            w={"60%"}
                                            key={form.key("nickname")}
                                            {...form.getInputProps("nickname")}
                                        />
                                    </Flex>
                                    <Select
                                        label="权限组"
                                        data={[
                                            {
                                                label: "管理员",
                                                value: UGroup.Admin.toString(),
                                            },
                                            {
                                                label: "普通用户",
                                                value: UGroup.User.toString(),
                                            },
                                        ]}
                                        allowDeselect={false}
                                        key={form.key("group")}
                                        value={form
                                            .getValues()
                                            .group.toString()}
                                        onChange={(value) => {
                                            form.setFieldValue(
                                                "group",
                                                Number(value)
                                            );
                                        }}
                                    />
                                    <TextInput
                                        label="邮箱"
                                        size="md"
                                        leftSection={<MDIcon>email</MDIcon>}
                                        key={form.key("email")}
                                        {...form.getInputProps("email")}
                                    />
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
                                        创建
                                    </Button>
                                </Flex>
                            </form>
                        </Box>
                    </Card>
                </Modal.Content>
            </Modal.Root>
        </>
    );
}
