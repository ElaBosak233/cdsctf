import {
    Submission,
    SubmissionCreateRequest,
    SubmissionDeleteRequest,
    SubmissionFindRequest,
} from "@/types/submission";
import { api } from "@/utils/axios";

export async function createSubmission(request: SubmissionCreateRequest) {
    return api().post<{
        code: number;
        data: Submission;
    }>("/submissions", request);
}

export async function getSubmissions(request: SubmissionFindRequest) {
    return api().get<{
        code: number;
        data: Array<Submission>;
        total: number;
    }>("/submissions", { params: request });
}

export async function getSubmissionByID(id: number) {
    return api().get<{
        code: number;
        data: Submission;
    }>(`/submissions/${id}`);
}

export async function deleteSubmission(request: SubmissionDeleteRequest) {
    return api().delete<{
        code: number;
    }>(`/submissions/${request.id}`);
}
