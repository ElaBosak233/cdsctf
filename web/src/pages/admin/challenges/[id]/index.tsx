import { useChallengeApi } from "@/api/challenge";
import withChallengeEdit from "@/components/layouts/admin/withChallengeEdit";
import MDIcon from "@/components/ui/MDIcon";
import { useCategoryStore } from "@/stores/category";
import { useConfigStore } from "@/stores/config";
import { Challenge } from "@/types/challenge";
import { Metadata } from "@/types/media";
import {
    showLoadingNotification,
    showSuccessNotification,
} from "@/utils/notification";
import {
    ActionIcon,
    Button,
    Text,
    Divider,
    FileInput,
    Flex,
    Group,
    NumberInput,
    Select,
    SimpleGrid,
    Stack,
    Switch,
    TextInput,
    Textarea,
    Tooltip,
} from "@mantine/core";
import { useForm, zodResolver } from "@mantine/form";
import { AxiosRequestConfig } from "axios";
import { useEffect, useState } from "react";
import { useParams } from "react-router-dom";
import { z } from "zod";

function Page() {
    const configStore = useConfigStore();
    const challengeApi = useChallengeApi();
    const categoryStore = useCategoryStore();

    const { id } = useParams<{ id: string }>();

    const [refresh, setRefresh] = useState<number>(0);

    const [challenge, setChallenge] = useState<Challenge>();
    const [attachmentMetadata, setAttachmentMetadata] = useState<Metadata>();

    const [attachment, setAttachment] = useState<File | null>(null);

    function getChallenge() {
        challengeApi
            .getChallenges({
                id: Number(id),
                is_detailed: true,
            })
            .then((res) => {
                const r = res.data;
                setChallenge(r.data[0]);
            });
    }

    function getAttachmentMetadata() {
        challengeApi.getChallengeAttachmentMetadata(Number(id)).then((res) => {
            const r = res.data;
            setAttachmentMetadata(r.data);
        });
    }

    function saveAttachment() {
        showLoadingNotification({
            id: "upload-attachment",
            message: "正在上传附件",
        });
        const config: AxiosRequestConfig<FormData> = {};
        challengeApi
            .saveChallengeAttachment(Number(id), attachment!, config)
            .then((_) => {
                showSuccessNotification({
                    id: "upload-attachment",
                    message: "附件上传成功",
                    update: true,
                });
                setRefresh((prev) => prev + 1);
            });
    }

    function deleteAttachment() {
        challengeApi.deleteChallengeAttachment(Number(id)).then((_) => {
            showSuccessNotification({
                message: "附件删除成功",
            });
            setRefresh((prev) => prev + 1);
        });
    }

    useEffect(() => {
        if (attachment) {
            saveAttachment();
        }
    }, [attachment]);

    const form = useForm({
        mode: "controlled",
        initialValues: {
            title: "",
            description: "",
            category_id: 0,
            is_dynamic: false,
            duration: 0,
        },
        validate: zodResolver(
            z.object({
                title: z.string({
                    required_error: "标题不能为空",
                }),
            })
        ),
    });

    function updateChallenge() {
        challengeApi
            .updateChallenge({
                id: Number(id),
                title: form.getValues().title,
                description: form.getValues().description,
                category_id: form.getValues().category_id,
                is_dynamic: form.getValues().is_dynamic,
                duration: form.getValues().duration,
            })
            .then((_) => {
                showSuccessNotification({
                    message: `题目 ${form.getValues().title} 更新成功`,
                });
                setRefresh((prev) => prev + 1);
            });
    }

    useEffect(() => {
        setAttachment(null);
        getChallenge();
    }, [refresh]);

    useEffect(() => {
        if (challenge) {
            form.setValues({
                title: challenge.title,
                description: challenge.description,
                category_id: challenge.category_id,
                is_dynamic: challenge.is_dynamic,
                duration: challenge.duration,
            });
            getAttachmentMetadata();
        }
    }, [challenge]);

    useEffect(() => {
        document.title = `${challenge?.title} - ${configStore?.pltCfg?.site?.title}`;
    }, [challenge]);

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
                <form onSubmit={form.onSubmit((_) => updateChallenge())}>
                    <Stack mx={20}>
                        <Group>
                            <TextInput
                                label="标题"
                                withAsterisk
                                description="题目大标题"
                                sx={{
                                    flexGrow: 1,
                                }}
                                key={form.key("title")}
                                {...form.getInputProps("title")}
                            />
                            <Select
                                label="分类"
                                withAsterisk
                                w={"20%"}
                                description="题目分类"
                                data={categoryStore?.categories?.map(
                                    (category) => {
                                        return {
                                            value: String(category.id),
                                            label: String(category.name),
                                        };
                                    }
                                )}
                                allowDeselect={false}
                                value={String(form.getValues().category_id)}
                                onChange={(value) => {
                                    form.setFieldValue(
                                        "category_id",
                                        Number(value)
                                    );
                                }}
                            />
                        </Group>
                        <Textarea
                            label="描述"
                            description="题目的描述，支持 Markdown"
                            autosize
                            minRows={9}
                            maxRows={9}
                            resize="vertical"
                            key={form.key("description")}
                            {...form.getInputProps("description")}
                        />
                        <Group align={"end"} gap={10}>
                            <TextInput
                                label="附件名/大小"
                                disabled
                                sx={{
                                    flexGrow: 1,
                                }}
                                value={
                                    attachmentMetadata?.filename
                                        ? `${attachmentMetadata?.filename} / ${attachmentMetadata?.size} bytes`
                                        : ""
                                }
                            />
                            <FileInput
                                label="上传附件"
                                description="上传题目附件"
                                placeholder="点击此处上传附件"
                                value={attachment}
                                onChange={setAttachment}
                            />
                            <Tooltip label="清除附件" withArrow>
                                <ActionIcon
                                    my={7}
                                    onClick={() => deleteAttachment()}
                                >
                                    <MDIcon color={"red"}>delete</MDIcon>
                                </ActionIcon>
                            </Tooltip>
                        </Group>
                        <SimpleGrid cols={2}>
                            <Switch
                                my={"auto"}
                                label="是否需要动态容器"
                                description={
                                    "题目是否需要启用容器环境进行题目分发"
                                }
                                checked={form.getValues().is_dynamic}
                                onChange={(e) =>
                                    form.setFieldValue(
                                        "is_dynamic",
                                        e.currentTarget.checked
                                    )
                                }
                            />
                            <NumberInput
                                label="持续时间"
                                description="动态容器持续时间（秒）"
                                key={form.key("duration")}
                                {...form.getInputProps("duration")}
                            />
                        </SimpleGrid>

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

export default withChallengeEdit(Page);
