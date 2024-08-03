import MDIcon from "@/components/ui/MDIcon";
import { Flag, Type } from "@/types/flag";
import {
    Box,
    Button,
    Card,
    Divider,
    Flex,
    Modal,
    ModalProps,
    Stack,
    TextInput,
    Text,
    Switch,
    Select,
} from "@mantine/core";
import { useForm } from "@mantine/form";
import { useEffect } from "react";

interface ChallengeFlagCreateModalProps extends ModalProps {
    addFlag: (flag: Flag) => void;
}

export default function ChallengeFlagCreateModal(
    props: ChallengeFlagCreateModalProps
) {
    const { addFlag, ...modalProps } = props;

    const form = useForm({
        mode: "controlled",
        initialValues: {
            value: "",
            env: "",
            banned: false,
            type: Type.Pattern,
        },
    });

    useEffect(() => {
        form.reset();
    }, [modalProps.opened]);

    return (
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
                        <MDIcon>flag</MDIcon>
                        <Text fw={600}>创建 Flag</Text>
                    </Flex>
                    <Divider my={10} />
                    <Box p={10}>
                        <form
                            onSubmit={form.onSubmit((_) => {
                                addFlag(form.values);
                                modalProps.onClose();
                            })}
                        >
                            <Stack gap={10}>
                                <TextInput
                                    label="Flag 值"
                                    withAsterisk
                                    description="使用正则时，请注意使用转义符"
                                    key={form.key("value")}
                                    {...form.getInputProps("value")}
                                />
                                <Select
                                    label="Flag 类型"
                                    description="不同的 Flag 类型，适用于不同的情境"
                                    withAsterisk
                                    data={[
                                        {
                                            label: "正则表达式",
                                            value: Type.Pattern.toString(),
                                        },
                                        {
                                            label: "动态",
                                            value: Type.Dynamic.toString(),
                                        },
                                    ]}
                                    key={form.key("type")}
                                    value={form.values.type.toString()}
                                    onChange={(value) =>
                                        form.setFieldValue(
                                            "type",
                                            Number(value)
                                        )
                                    }
                                    allowDeselect={false}
                                />
                                <Flex gap={20} align={"center"}>
                                    <TextInput
                                        label="环境变量"
                                        description="当题目启用动态容器时，可设置将 Flag 以容器环境变量的形式注入容器"
                                        key={form.key("env")}
                                        {...form.getInputProps("env")}
                                    />
                                    <Switch
                                        label="是否封禁此 Flag"
                                        description="当用户提交此 Flag 时，直接判定为作弊"
                                        key={form.key("banned")}
                                        {...form.getInputProps("banned")}
                                    />
                                </Flex>
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
    );
}
