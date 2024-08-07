import { Stack, Text } from "@mantine/core";
import MDIcon from "./MDIcon";

interface WaterMarkProps {
    icon: string;
    text: string;
}

export default function WaterMark(props: WaterMarkProps) {
    const { icon, text } = props;

    return (
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
                {icon}
            </MDIcon>
            <Text size={"2rem"} fw={700}>
                {text}
            </Text>
        </Stack>
    );
}
