import {
    SubmissionCreateRequest,
    SubmissionDeleteRequest,
    SubmissionFindRequest,
} from "@/types/submission";
import { useAuth } from "@/utils/axios";

export function useSubmissionApi() {
    const auth = useAuth();

    const createSubmission = (request: SubmissionCreateRequest) => {
        return auth.post("/submissions", { ...request });
    };

    const getSubmissions = (request: SubmissionFindRequest) => {
        return auth.get("/submissions", { params: request });
    };

    const getSubmissionByID = (id: number) => {
        return auth.get(`/submissions/${id}`);
    };

    const deleteSubmission = (request: SubmissionDeleteRequest) => {
        return auth.delete(`/submissions/${request.id}`);
    };

    return {
        getSubmissions,
        createSubmission,
        deleteSubmission,
        getSubmissionByID,
    };
}
