import { useChallengeApi } from "@/api/challenge";
import withChallengeEdit from "@/components/layouts/admin/withChallengeEdit";
import ChallengeFlagCreateModal from "@/components/modals/admin/ChallengeFlagCreateModal";
import MDIcon from "@/components/ui/MDIcon";
import ChallengeFlagAccordion from "@/components/widgets/admin/ChallengeFlagAccordion";
import { Challenge } from "@/types/challenge";
import { Flag } from "@/types/flag";
import { showSuccessNotification } from "@/utils/notification";
import {
    Accordion,
    Flex,
    Group,
    Stack,
    Text,
    Divider,
    ActionIcon,
    Tooltip,
    Button,
    Select,
    SimpleGrid,
    Switch,
    TextInput,
    Card,
    Badge,
    Center,
} from "@mantine/core";
import { useDisclosure } from "@mantine/hooks";
import { useEffect, useState } from "react";
import { useParams } from "react-router-dom";

function Page() {
    const { id } = useParams<{ id: string }>();
    const challengeApi = useChallengeApi();

    const [challenge, setChallenge] = useState<Challenge>();
    const [flags, setFlags] = useState<Array<Flag>>();

    const [createOpened, { open: createOpen, close: createClose }] =
        useDisclosure(false);

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

    function updateChallengeFlag() {
        challengeApi
            .updateChallenge({
                id: Number(id),
                flags: flags,
            })
            .then((_) => {
                showSuccessNotification({
                    message: "题目 Flag 更新成功",
                });
            });
    }

    useEffect(() => {
        getChallenge();
    }, []);

    useEffect(() => {
        setFlags(challenge?.flags);
    }, [challenge]);

    useEffect(() => {
        document.title = `Flags - ${challenge?.title}`;
    }, [challenge]);

    return (
        <>
            <Stack m={36}>
                <Stack gap={10}>
                    <Flex justify={"space-between"} align={"center"}>
                        <Group>
                            <MDIcon>flag</MDIcon>
                            <Text fw={700} size="xl">
                                Flags
                            </Text>
                        </Group>
                        <Tooltip label="创建 Flag" withArrow>
                            <ActionIcon onClick={() => createOpen()}>
                                <MDIcon>add</MDIcon>
                            </ActionIcon>
                        </Tooltip>
                    </Flex>
                    <Divider />
                </Stack>
                <Stack mx={20}>
                    {flags?.map((flag, index) => (
                        <Card shadow={"xs"} key={index}>
                            <Flex gap={15}>
                                <Center>
                                    <Badge
                                        color={flag?.banned ? "red" : "brand"}
                                    >
                                        {index + 1}
                                    </Badge>
                                </Center>
                                <TextInput
                                    label="Flag 值"
                                    disabled
                                    value={flag.value}
                                />
                                <Select
                                    label="Flag 类型"
                                    disabled
                                    data={[
                                        {
                                            label: "正则表达式",
                                            value: "pattern",
                                        },
                                        {
                                            label: "动态",
                                            value: "dynamic",
                                        },
                                    ]}
                                    allowDeselect={false}
                                    value={flag.type}
                                />
                                <TextInput
                                    label="环境变量"
                                    disabled
                                    value={flag.env}
                                />
                                <Flex justify={"end"} align={"center"} flex={1}>
                                    <ActionIcon
                                        onClick={() => {
                                            const newFlags = flags?.filter(
                                                (_, i) => i !== index
                                            );
                                            setFlags(newFlags);
                                        }}
                                    >
                                        <MDIcon c={"red"}>delete</MDIcon>
                                    </ActionIcon>
                                </Flex>
                            </Flex>
                        </Card>
                    ))}
                </Stack>
                <Flex justify="end">
                    <Button
                        leftSection={<MDIcon c={"white"}>check</MDIcon>}
                        onClick={() => updateChallengeFlag()}
                    >
                        保存
                    </Button>
                </Flex>
            </Stack>
            <ChallengeFlagCreateModal
                centered
                opened={createOpened}
                onClose={createClose}
                addFlag={(flag) => {
                    const newFlags = flags?.concat(flag);
                    setFlags(newFlags);
                }}
            />
        </>
    );
}

export default withChallengeEdit(Page);
