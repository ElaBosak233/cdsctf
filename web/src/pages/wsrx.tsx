import { useEffect } from "react";
import { useConfigStore } from "@/stores/config";
import {
    ActionIcon,
    Blockquote,
    Box,
    Card,
    Checkbox,
    Divider,
    Flex,
    Indicator,
    Stack,
    Text,
    TextInput,
    Tooltip,
} from "@mantine/core";
import MDIcon from "@/components/ui/MDIcon";
import { useWsrxStore } from "@/stores/wsrx";

export default function Page() {
    const configStore = useConfigStore();
    const wsrxStore = useWsrxStore();

    useEffect(() => {
        document.title = `${configStore?.pltCfg?.site?.title}`;
    }, []);

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
                <Card
                    shadow="md"
                    padding={"lg"}
                    radius={"md"}
                    withBorder
                    w={"40rem"}
                >
                    <Flex gap={10} align={"center"}>
                        <MDIcon>link</MDIcon>
                        <Text fw={600}>连接器</Text>
                    </Flex>
                    <Indicator
                        processing
                        pos={"absolute"}
                        right={25}
                        top={25}
                        color={
                            wsrxStore.status === "online"
                                ? "green"
                                : wsrxStore.status === "offline"
                                  ? "red"
                                  : "orange"
                        }
                    />
                    <Blockquote color="blue" p={10} my={20}>
                        <Flex align={"center"} gap={10}>
                            <Text fz={"sm"}>
                                本平台使用 WebSocketReflectorX （以下简称 WSRX）
                                作为启用 TCP over WebSocket
                                代理的容器的连接器，以下设置用于与 WSRX
                                交互，提升使用体验。
                            </Text>
                            <ActionIcon
                                onClick={() => {
                                    window.open(
                                        "https://github.com/XDSEC/WebSocketReflectorX",
                                        "_blank"
                                    );
                                }}
                            >
                                <MDIcon>download</MDIcon>
                            </ActionIcon>
                        </Flex>
                    </Blockquote>
                    <Stack>
                        <TextInput
                            label={"URL"}
                            value={wsrxStore?.url}
                            onChange={(e) => {
                                wsrxStore.setUrl(e.target.value);
                            }}
                            placeholder={"http://127.0.0.1:3307"}
                            rightSectionWidth={80}
                            rightSection={
                                <Flex>
                                    <Divider mx={10} orientation={"vertical"} />
                                    <Tooltip withArrow label={"重新连接"}>
                                        <ActionIcon
                                            onClick={() => wsrxStore.connect()}
                                        >
                                            <MDIcon>refresh</MDIcon>
                                        </ActionIcon>
                                    </Tooltip>
                                </Flex>
                            }
                        />
                        <Checkbox
                            label={
                                "启用连接器（若不启用，则需手动与题目环境进行连接）"
                            }
                            checked={wsrxStore?.isEnabled}
                            onChange={(e) => {
                                wsrxStore?.setIsEnabled(e.target.checked);
                            }}
                        />
                    </Stack>
                </Card>
            </Box>
        </>
    );
}
