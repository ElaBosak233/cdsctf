import { useAuthStore } from "@/stores/auth";
import { Team, TeamUpdateRequest } from "@/types/team";
import {
    showErrNotification,
    showSuccessNotification,
} from "@/utils/notification";
import {
    Card,
    Flex,
    Modal,
    ModalProps,
    Text,
    Divider,
    Box,
    Stack,
    TextInput,
    Textarea,
    Button,
    ActionIcon,
    Avatar,
    Tooltip,
    Group,
    Center,
    Image,
} from "@mantine/core";
import { isEmail, useForm } from "@mantine/form";
import MDIcon from "@/components/ui/MDIcon";
import { useEffect, useState } from "react";
import { modals } from "@mantine/modals";
import { User } from "@/types/user";
import { AxiosRequestConfig } from "axios";
import { Dropzone } from "@mantine/dropzone";
import { Metadata } from "@/types/media";
import {
    deleteTeam,
    deleteTeamUser,
    getTeamAvatarMetadata,
    getTeamInviteToken,
    saveTeamAvatar,
    updateTeam,
    updateTeamInviteToken,
} from "@/api/team";

interface TeamEditModalProps extends ModalProps {
    setRefresh: () => void;
    team?: Team;
}

export default function TeamEditModal(props: TeamEditModalProps) {
    const { setRefresh, team, ...modalProps } = props;

    const authStore = useAuthStore();

    const [isCaptain, setIsCaptain] = useState<boolean>(false);
    const [inviteToken, setInviteToken] = useState<string>("");
    const [users, setUsers] = useState<Array<User> | undefined>([]);

    const [avatarMetadata, setAvatarMetadata] = useState<Metadata>();

    const form = useForm({
        mode: "uncontrolled",
        initialValues: {
            name: "",
            description: "",
            email: "",
        },
        validate: {
            name: (value) => {
                if (value === "") {
                    return "团队名称不能为空";
                }
                return null;
            },
            description: (value) => {
                if (value === "") {
                    return "团队简介不能为空";
                }
                return null;
            },
            email: isEmail("邮箱格式不正确"),
        },
    });

    function handleGetTeamAvatarMetadata() {
        getTeamAvatarMetadata(Number(team?.id)).then((res) => {
            const r = res.data;
            setAvatarMetadata(r.data);
        });
    }

    function handleSaveTeamAvatar(file?: File) {
        const config: AxiosRequestConfig<FormData> = {};
        saveTeamAvatar(Number(team?.id), file!, config)
            .then((_) => {
                showSuccessNotification({
                    message: `团队 ${form.getValues().name} 头像更新成功`,
                });
            })
            .finally(() => {
                setRefresh();
                modalProps.onClose();
            });
    }

    function handleGetTeamInviteToken() {
        getTeamInviteToken({
            id: Number(team?.id),
        }).then((res) => {
            const r = res.data;
            setInviteToken(r.token);
        });
    }

    function handleUpdateTeamInviteToken() {
        updateTeamInviteToken({
            id: Number(team?.id),
        }).then((res) => {
            const r = res.data;
            setInviteToken(r.token);
            showSuccessNotification({
                message: `团队 ${team?.name} 邀请码更新成功`,
            });
        });
    }

    function handleUpdateTeam(request: TeamUpdateRequest) {
        updateTeam({
            id: Number(team?.id),
            name: request?.name,
            description: request?.description,
            email: request?.email,
            captain_id: request?.captain_id || Number(authStore.user?.id),
        })
            .then((_) => {
                showSuccessNotification({
                    message: `团队 ${form.values.name} 更新成功`,
                });
                setRefresh();
            })
            .catch((e) => {
                showErrNotification({
                    message: e.response.data.error || "更新团队失败",
                });
            })
            .finally(() => {
                form.reset();
                modalProps.onClose();
            });
    }

    function handleDeleteTeamUser(user?: User) {
        deleteTeamUser({
            id: Number(team?.id),
            user_id: Number(user?.id),
        }).then((_) => {
            showSuccessNotification({
                message: `用户 ${user?.nickname} 已被踢出`,
            });
            setRefresh();
            setUsers((prevUsers) =>
                prevUsers?.filter((u) => u?.id !== user?.id)
            );
        });
    }

    function handleDeleteTeam() {
        deleteTeam({
            id: Number(team?.id),
        })
            .then((_) => {
                showSuccessNotification({
                    message: `团队 ${team?.name} 已被解散`,
                });
            })
            .finally(() => {
                setRefresh();
                modalProps.onClose();
            });
    }

    const openDeleteTeamUserModal = (user?: User) =>
        modals.openConfirmModal({
            centered: true,
            children: (
                <>
                    <Flex gap={10} align={"center"}>
                        <MDIcon>person_remove</MDIcon>
                        <Text fw={600}>踢出成员</Text>
                    </Flex>
                    <Divider my={10} />
                    <Text>你确定要踢出成员 {user?.nickname} 吗？</Text>
                </>
            ),
            withCloseButton: false,
            labels: {
                confirm: "确定",
                cancel: "取消",
            },
            confirmProps: {
                color: "red",
            },
            onConfirm: () => {
                handleDeleteTeamUser(user);
            },
        });

    const openTransferCaptainModal = (user?: User) =>
        modals.openConfirmModal({
            centered: true,
            children: (
                <>
                    <Flex gap={10} align={"center"}>
                        <MDIcon>star</MDIcon>
                        <Text fw={600}>转让队长</Text>
                    </Flex>
                    <Divider my={10} />
                    <Text>你确定要将队长转让给 {user?.nickname} 吗？</Text>
                </>
            ),
            withCloseButton: false,
            labels: {
                confirm: "确定",
                cancel: "取消",
            },
            onConfirm: () => {
                handleUpdateTeam({
                    captain_id: user?.id,
                });
            },
        });

    const openDeleteTeamModal = () =>
        modals.openConfirmModal({
            centered: true,
            children: (
                <>
                    <Flex gap={10} align={"center"}>
                        <MDIcon color={"red"}>swap_horiz</MDIcon>
                        <Text fw={600}>解散团队</Text>
                    </Flex>
                    <Divider my={10} />
                    <Text>你确定要解散团队 {team?.name} 吗？</Text>
                </>
            ),
            withCloseButton: false,
            labels: {
                confirm: "确定",
                cancel: "取消",
            },
            confirmProps: {
                color: "red",
            },
            onConfirm: () => {
                handleDeleteTeam();
            },
        });

    useEffect(() => {
        if (team) {
            setIsCaptain(authStore?.user?.id === team?.captain_id);
            if (authStore?.user?.id === team?.captain_id) {
                handleGetTeamInviteToken();
            }
            setUsers(team?.users);
            form.setValues({
                name: team?.name,
                description: team?.description,
                email: team?.email,
            });
            handleGetTeamAvatarMetadata();
        }
    }, [team]);

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
                            <MDIcon>group_add</MDIcon>
                            <Text fw={600}>团队详情</Text>
                        </Flex>
                        <Divider my={10} />
                        <Box p={10}>
                            <form
                                onSubmit={form.onSubmit((values) =>
                                    handleUpdateTeam({
                                        name: values.name,
                                        description: values.description,
                                        email: values.email,
                                    })
                                )}
                            >
                                <Stack gap={10}>
                                    <Flex gap={10}>
                                        <Stack gap={10} flex={1}>
                                            <TextInput
                                                label="团队名称"
                                                size="md"
                                                leftSection={
                                                    <MDIcon>people</MDIcon>
                                                }
                                                key={form.key("name")}
                                                {...form.getInputProps("name")}
                                                readOnly={!isCaptain}
                                            />
                                            {isCaptain && (
                                                <TextInput
                                                    label="邀请码"
                                                    size="md"
                                                    readOnly
                                                    rightSection={
                                                        <ActionIcon
                                                            onClick={
                                                                handleUpdateTeamInviteToken
                                                            }
                                                        >
                                                            <MDIcon>
                                                                refresh
                                                            </MDIcon>
                                                        </ActionIcon>
                                                    }
                                                    value={`${team?.id}:${inviteToken}`}
                                                />
                                            )}
                                        </Stack>
                                        <Dropzone
                                            onDrop={(files: any) =>
                                                handleSaveTeamAvatar(files[0])
                                            }
                                            onReject={() => {
                                                showErrNotification({
                                                    message: "文件上传失败",
                                                });
                                            }}
                                            disabled={!isCaptain}
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
                                                {avatarMetadata?.filename ? (
                                                    <Center>
                                                        <Image
                                                            w={120}
                                                            h={120}
                                                            fit="contain"
                                                            src={`/api/teams/${team?.id}/avatar`}
                                                        />
                                                    </Center>
                                                ) : (
                                                    <Center>
                                                        <Stack gap={0}>
                                                            <Text
                                                                size="xl"
                                                                inline
                                                            >
                                                                拖拽或点击上传头像
                                                            </Text>
                                                            <Text
                                                                size="sm"
                                                                c="dimmed"
                                                                inline
                                                                mt={7}
                                                            >
                                                                图片大小不超过
                                                                3MB
                                                            </Text>
                                                        </Stack>
                                                    </Center>
                                                )}
                                            </Center>
                                        </Dropzone>
                                    </Flex>
                                    <Textarea
                                        label="团队简介"
                                        size="md"
                                        key={form.key("description")}
                                        {...form.getInputProps("description")}
                                        readOnly={!isCaptain}
                                    />
                                    <TextInput
                                        label="邮箱"
                                        size="md"
                                        leftSection={<MDIcon>email</MDIcon>}
                                        key={form.key("email")}
                                        {...form.getInputProps("email")}
                                        readOnly={!isCaptain}
                                    />
                                </Stack>
                                <Stack mt={10}>
                                    <Text>成员</Text>
                                    <Group gap={20}>
                                        {users?.map((user) => (
                                            <Flex
                                                key={user?.id}
                                                align={"center"}
                                                gap={15}
                                            >
                                                <Flex align={"center"} gap={10}>
                                                    <Avatar
                                                        color="brand"
                                                        src={`/api/users/${user?.id}/avatar`}
                                                        radius="xl"
                                                    >
                                                        <MDIcon>person</MDIcon>
                                                    </Avatar>
                                                    <Text>
                                                        {user?.nickname}
                                                    </Text>
                                                </Flex>
                                                {user?.id ===
                                                    team?.captain_id && (
                                                    <Tooltip
                                                        label="队长"
                                                        withArrow
                                                    >
                                                        <MDIcon
                                                            color={"yellow"}
                                                        >
                                                            star
                                                        </MDIcon>
                                                    </Tooltip>
                                                )}
                                                {isCaptain &&
                                                    user?.id !==
                                                        authStore?.user?.id && (
                                                        <Flex>
                                                            <Tooltip
                                                                label="转让队长"
                                                                withArrow
                                                            >
                                                                <ActionIcon
                                                                    color="grey"
                                                                    onClick={() => {
                                                                        openTransferCaptainModal(
                                                                            user
                                                                        );
                                                                    }}
                                                                >
                                                                    <MDIcon>
                                                                        star
                                                                    </MDIcon>
                                                                </ActionIcon>
                                                            </Tooltip>
                                                            <Tooltip
                                                                label="踢出"
                                                                withArrow
                                                            >
                                                                <ActionIcon
                                                                    color="red"
                                                                    onClick={() => {
                                                                        openDeleteTeamUserModal(
                                                                            user
                                                                        );
                                                                    }}
                                                                >
                                                                    <MDIcon>
                                                                        close
                                                                    </MDIcon>
                                                                </ActionIcon>
                                                            </Tooltip>
                                                        </Flex>
                                                    )}
                                            </Flex>
                                        ))}
                                    </Group>
                                </Stack>
                                {isCaptain && (
                                    <Flex mt={20} justify={"end"} gap={10}>
                                        <Button
                                            leftSection={
                                                <MDIcon c={"white"}>
                                                    swap_horiz
                                                </MDIcon>
                                            }
                                            onClick={openDeleteTeamModal}
                                            color="red"
                                        >
                                            解散
                                        </Button>
                                        <Button
                                            type="submit"
                                            leftSection={
                                                <MDIcon c={"white"}>
                                                    check
                                                </MDIcon>
                                            }
                                        >
                                            更新
                                        </Button>
                                    </Flex>
                                )}
                            </form>
                        </Box>
                    </Card>
                </Modal.Content>
            </Modal.Root>
        </>
    );
}
