import {
    getGamePosterMetadata,
    getGames,
    saveGamePoster,
    updateGame,
} from "@/api/game";
import withGameEdit from "@/components/layouts/admin/withGameEdit";
import MDIcon from "@/components/ui/MDIcon";
import { useConfigStore } from "@/stores/config";
import { Game } from "@/types/game";
import { Metadata } from "@/types/media";
import {
    showErrNotification,
    showSuccessNotification,
} from "@/utils/notification";
import {
    Center,
    Flex,
    NumberInput,
    SimpleGrid,
    Stack,
    TextInput,
    Textarea,
    Image,
    Text,
    Button,
    Group,
    Divider,
    Checkbox,
} from "@mantine/core";
import { DateTimePicker } from "@mantine/dates";
import { Dropzone } from "@mantine/dropzone";
import { useForm } from "@mantine/form";
import { AxiosRequestConfig } from "axios";
import { useEffect, useState } from "react";
import { useParams } from "react-router-dom";

function Page() {
    const { id } = useParams<{ id: string }>();
    const configStore = useConfigStore();

    const [game, setGame] = useState<Game>();

    const [gamePosterMetadata, setGamePosterMetadata] = useState<Metadata>();

    const form = useForm({
        mode: "controlled",
        initialValues: {
            title: "",
            bio: "",
            description: "",
            is_public: false,
            started_at: 0,
            ended_at: 0,
            member_limit_min: 0,
            member_limit_max: 0,
            parallel_container_limit: 0,
            first_blood_reward_ratio: 0,
            second_blood_reward_ratio: 0,
            third_blood_reward_ratio: 0,
            is_need_write_up: false,
        },
    });

    function handleGetGame() {
        getGames({
            id: Number(id),
        }).then((res) => {
            const r = res.data;
            setGame(r.data[0]);
        });
    }

    function handleUpdateGame() {
        updateGame({
            id: Number(id),
            title: form.getValues().title,
            bio: form.getValues().bio,
            description: form.getValues().description,
            is_public: form.getValues().is_public,
            started_at: Math.ceil(form.getValues().started_at),
            ended_at: Math.ceil(form.getValues().ended_at),
            member_limit_min: form.getValues().member_limit_min,
            member_limit_max: form.getValues().member_limit_max,
            parallel_container_limit: form.getValues().parallel_container_limit,
            is_need_write_up: form.getValues().is_need_write_up,
        }).then((_) => {
            showSuccessNotification({
                message: `比赛 ${form.getValues().title} 更新成功`,
            });
        });
    }

    function handleGetGamePosterMetadata() {
        getGamePosterMetadata(Number(game?.id)).then((res) => {
            const r = res.data;
            setGamePosterMetadata(r.data);
        });
    }

    function handleSaveGamePoster(file?: File) {
        const config: AxiosRequestConfig<FormData> = {};
        saveGamePoster(Number(game?.id), file!, config)
            .then((_) => {
                showSuccessNotification({
                    message: `比赛 ${form.getValues().title} 头图更新成功`,
                });
            })
            .finally(() => {
                handleGetGame();
            });
    }

    useEffect(() => {
        handleGetGame();
    }, []);

    useEffect(() => {
        if (game) {
            form.setValues({
                title: game?.title,
                bio: game?.bio,
                description: game?.description,
                is_public: game?.is_public,
                started_at: game?.started_at,
                ended_at: game?.ended_at,
                member_limit_min: game?.member_limit_min,
                member_limit_max: game?.member_limit_max,
                parallel_container_limit: game?.parallel_container_limit,
                is_need_write_up: game?.is_need_write_up,
            });
            handleGetGamePosterMetadata();
        }
    }, [game]);

    useEffect(() => {
        document.title = `${game?.title} - ${configStore?.pltCfg?.site?.title}`;
    }, [game]);

    return (
        <>
            <Stack m={36}>
                <Stack gap={10}>
                    <Group>
                        <MDIcon>info</MDIcon>
                        <Text fw={700} size="xl">
                            基本信息
                        </Text>
                    </Group>
                    <Divider />
                </Stack>
                <form onSubmit={form.onSubmit((_) => handleUpdateGame())}>
                    <Stack mx={20}>
                        <TextInput
                            my={"auto"}
                            label="标题"
                            description="展示在最醒目位置的比赛大标题"
                            withAsterisk
                            key={form.key("title")}
                            {...form.getInputProps("title")}
                        />
                        <Textarea
                            label="简介"
                            description="展示在比赛页面的简短介绍"
                            key={form.key("bio")}
                            {...form.getInputProps("bio")}
                        />
                        <Flex gap={20}>
                            <SimpleGrid cols={4}>
                                <NumberInput
                                    my={"auto"}
                                    label="最小人数"
                                    description="一个团队所需的最少的人数"
                                    withAsterisk
                                    key={form.key("member_limit_min")}
                                    {...form.getInputProps("member_limit_min")}
                                />
                                <NumberInput
                                    my={"auto"}
                                    label="最大人数"
                                    description="一个团队所需的最多的人数"
                                    withAsterisk
                                    key={form.key("member_limit_max")}
                                    {...form.getInputProps("member_limit_max")}
                                />
                                <NumberInput
                                    my={"auto"}
                                    label="容器限制"
                                    description="一个团队最多可启用的容器数量"
                                    withAsterisk
                                    key={form.key("parallel_container_limit")}
                                    {...form.getInputProps(
                                        "parallel_container_limit"
                                    )}
                                />
                                <DateTimePicker
                                    my={"auto"}
                                    withSeconds
                                    withAsterisk
                                    label="开始时间"
                                    description="此时允许进入比赛，并且允许作答"
                                    placeholder="请选择比赛开始的时间"
                                    valueFormat="YYYY/MM/DD HH:mm:ss"
                                    value={
                                        new Date(
                                            form.getValues().started_at * 1000
                                        )
                                    }
                                    onChange={(value) => {
                                        form.setFieldValue(
                                            "started_at",
                                            Number(value?.getTime()) / 1000
                                        );
                                    }}
                                />
                                <DateTimePicker
                                    my={"auto"}
                                    withSeconds
                                    withAsterisk
                                    label="冻结时间"
                                    description="此时允许进入比赛，仅可提交 WP"
                                    placeholder="请选择比赛冻结的时间"
                                    valueFormat="YYYY/MM/DD HH:mm:ss"
                                    value={
                                        new Date(
                                            form.getValues().started_at * 1000
                                        )
                                    }
                                    onChange={(value) => {
                                        form.setFieldValue(
                                            "frozed_at",
                                            Number(value?.getTime()) / 1000
                                        );
                                    }}
                                />
                                <DateTimePicker
                                    my={"auto"}
                                    withSeconds
                                    withAsterisk
                                    label="结束时间"
                                    description="此时不允许进入比赛，封存比赛"
                                    placeholder="请选择比赛结束的时间"
                                    valueFormat="YYYY/MM/DD HH:mm:ss"
                                    value={
                                        new Date(
                                            form.getValues().ended_at * 1000
                                        )
                                    }
                                    onChange={(value) => {
                                        form.setFieldValue(
                                            "ended_at",
                                            Number(value?.getTime()) / 1000
                                        );
                                    }}
                                />
                                <Checkbox
                                    my={"auto"}
                                    label="是否公开"
                                    description="若为是，则队伍在报名参赛后自动审批"
                                    labelPosition="left"
                                    checked={form.getValues().is_public}
                                    onChange={(value) => {
                                        form.setFieldValue(
                                            "is_public",
                                            value.target.checked
                                        );
                                    }}
                                />
                                <Checkbox
                                    my={"auto"}
                                    label="是否需要 WP"
                                    description="若为是，则比赛页面中将显示提交 WP 入口"
                                    labelPosition="left"
                                    checked={form.getValues().is_need_write_up}
                                    onChange={(value) => {
                                        form.setFieldValue(
                                            "is_need_write_up",
                                            value.target.checked
                                        );
                                    }}
                                />
                            </SimpleGrid>
                        </Flex>
                        <Dropzone
                            onDrop={(files: any) =>
                                handleSaveGamePoster(files[0])
                            }
                            onReject={() => {
                                showErrNotification({
                                    message: "文件上传失败",
                                });
                            }}
                            mih={"10rem"}
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
                                {gamePosterMetadata?.filename ? (
                                    <Center>
                                        <Image
                                            mah={"20vh"}
                                            w={"15vw"}
                                            fit="contain"
                                            src={`/api/games/${game?.id}/poster`}
                                        />
                                    </Center>
                                ) : (
                                    <Center h={"20vh"}>
                                        <Stack gap={0}>
                                            <Text size="xl" inline>
                                                拖拽或点击上传头图
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
                        <Textarea
                            flex={1}
                            label="详情"
                            description="展示在比赛详情页的介绍"
                            minRows={12}
                            maxRows={12}
                            resize={"vertical"}
                            autosize
                            key={form.key("description")}
                            {...form.getInputProps("description")}
                        />
                        <Flex justify={"end"}>
                            <Button
                                type="submit"
                                size="md"
                                leftSection={<MDIcon c={"white"}>check</MDIcon>}
                            >
                                保存
                            </Button>
                        </Flex>
                    </Stack>
                </form>
            </Stack>
        </>
    );
}

export default withGameEdit(Page);
