import {
    Pod,
    PodCreateRequest,
    PodFindRequest,
    PodRemoveRequest,
    PodRenewRequest,
} from "@/types/pod";
import { api } from "@/utils/axios";

export async function getPods(request: PodFindRequest) {
    return api().get<{
        code: number;
        data: Array<Pod>;
    }>("/pods", { params: request });
}

export async function createPod(request: PodCreateRequest) {
    return api().post<{
        code: number;
        data: Pod;
    }>("/pods", request);
}

export async function renewPod(request: PodRenewRequest) {
    return api().post<{
        code: number;
        data: Pod;
    }>(`/pods/${request.id}/renew`, request);
}

export async function stopPod(request: PodRemoveRequest) {
    return api().post<{
        code: number;
        data: Pod;
    }>(`/pods/${request.id}/stop`, { data: request });
}
