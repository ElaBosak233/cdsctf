import { ModalProps } from "@mantine/core";

interface ChallengeCreateModalProps extends ModalProps {
	setRefresh: () => void;
}

export default function ChallengeCreateModal(props: ChallengeCreateModalProps) {
	const { setRefresh, ...modalProps } = props;
}
