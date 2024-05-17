import { useAuth } from "@/utils/axios";

export function useMediaApi() {
	const auth = useAuth();

	const getChallengeAttachmentInfoByChallengeID = (id: number) => {
		return auth.get(`/media/challenges/attachments/${id}/info`);
	};

	const getChallengeAttachmentByChallengeID = (id: number) => {
		return auth.get(`/media/challenges/attachments/${id}`);
	};

	const setChallengeAttachmentByChallengeID = (id: number, data: any) => {
		return auth.post(`/media/challenges/attachments/${id}`, data);
	};

	const deleteChallengeAttachmentByChallengeID = (id: number) => {
		return auth.delete(`/media/challenges/attachments/${id}`);
	};

	return {
		getChallengeAttachmentInfoByChallengeID,
		getChallengeAttachmentByChallengeID,
		setChallengeAttachmentByChallengeID,
		deleteChallengeAttachmentByChallengeID,
	};
}
