import MDIcon from "@/components/ui/MDIcon";
import WaterMark from "@/components/ui/WaterMark";
import { Box, Stack, Text } from "@mantine/core";
import { text } from "stream/consumers";

export default function Page() {
    return (
        <>
            <Stack
                pos={"absolute"}
                opacity={0.15}
                align={"center"}
                justify={"center"}
                top={"50%"}
                left={"50%"}
                sx={{
                    transform: "translate(-50%, -50%)",
                }}
                className={"no-select"}
            >
                <MDIcon color={"gray"} size={150}>
                    settings
                </MDIcon>
                <Text size={"3rem"} fw={700}>
                    管理面板
                </Text>
            </Stack>
        </>
    );
}
