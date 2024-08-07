import { Team } from "@/types/team";
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
import {
    deleteTeamUser,
    getTeamAvatarMetadata,
    getTeamInviteToken,
    getTeams,
    saveTeamAvatar,
    updateTeam,
    updateTeamInviteToken,
} from "@/api/team";
import { Metadata } from "@/types/media";

interface TeamEditModalProps extends ModalProps {
    setRefresh: () => void;
    teamID: number;
}

export default function TeamEditModal(props: TeamEditModalProps) {
    const { setRefresh, teamID, ...modalProps } = props;

    const [team, setTeam] = useState<Team>();
    const [inviteToken, setInviteToken] = useState<string>("");
    const [avatarMetadata, setAvatarMetadata] = useState<Metadata>();

    const form = useForm({
        mode: "controlled",
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

    function handleGetTeam() {
        getTeams({
            id: teamID,
        }).then((res) => {
            const r = res.data;
            setTeam(r.data[0]);
        });
    }

    function handleGetTeamAvatarMetadata() {
        getTeamAvatarMetadata(team?.id!).then((res) => {
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
                handleGetTeam();
                setRefresh();
            });
    }

    function handleGetTeamInviteToken() {
        getTeamInviteToken({
            id: Number(teamID),
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

    function handleUpdateTeam() {
        updateTeam({
            id: Number(team?.id),
            name: form.getValues().name,
            description: form.getValues().description,
            email: form.getValues().email,
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

    function handleTransferCaptain(user?: User) {
        updateTeam({
            id: Number(team?.id),
            captain_id: Number(user?.id),
        })
            .then((_) => {
                showSuccessNotification({
                    message: `团队 ${form.values.name} 转让成功`,
                });
                setRefresh();
            })
            .catch((e) => {
                showErrNotification({
                    message: e.response.data.error || "转让团队失败",
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
            handleGetTeam();
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
                        <MDIcon color={"orange"}>star</MDIcon>
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
                handleTransferCaptain(user);
            },
        });

    useEffect(() => {
        if (team) {
            form.setValues({
                name: team?.name,
                description: team?.description,
                email: team?.email,
            });
        }
    }, [team]);

    useEffect(() => {
        if (modalProps.opened && teamID) {
            handleGetTeam();
            handleGetTeamAvatarMetadata();
            handleGetTeamInviteToken();
        }
        if (!modalProps.opened) {
            setTimeout(() => {
                setTeam(undefined);
                form.reset();
            }, 100);
        }
    }, [teamID, modalProps.opened]);

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
                                onSubmit={form.onSubmit((_) =>
                                    handleUpdateTeam()
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
                                            />
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
                                                        <MDIcon>refresh</MDIcon>
                                                    </ActionIcon>
                                                }
                                                value={`${team?.id}:${inviteToken}`}
                                            />
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
                                    />
                                    <TextInput
                                        label="邮箱"
                                        size="md"
                                        leftSection={<MDIcon>email</MDIcon>}
                                        key={form.key("email")}
                                        {...form.getInputProps("email")}
                                    />
                                </Stack>
                                <Stack mt={10}>
                                    <Text>成员</Text>
                                    <Group gap={20}>
                                        {team?.users?.map((user) => (
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
                                                {user?.id !==
                                                    team?.captain_id && (
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
                                <Flex mt={20} justify={"end"} gap={10}>
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
                </Modal.Content>
            </Modal.Root>
        </>
    );
}
