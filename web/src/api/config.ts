import { api } from "@/utils/axios";

export async function getConfig() {
    return api().get<{
        code: number;
        data: any;
    }>("/configs");
}
