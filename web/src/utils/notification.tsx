import { notifications, showNotification } from "@mantine/notifications";
import MDIcon from "@/components/ui/MDIcon";

export function showErrNotification({
	id,
	title,
	message,
}: {
	id?: string;
	title?: string;
	message?: string;
}) {
	if (id) {
		notifications.update({
			id: id,
			title: title || "发生了错误",
			message: message,
			color: "red",
			icon: <MDIcon c={"white"}>close</MDIcon>,
			autoClose: 2000,
			withCloseButton: true,
			loading: false,
		});
		return;
	}
	showNotification({
		title: title || "发生了错误",
		message: message,
		color: "red",
		icon: <MDIcon c={"white"}>close</MDIcon>,
	});
}

export function showSuccessNotification({
	id,
	title,
	message,
}: {
	id?: string;
	title?: string;
	message?: string;
}) {
	if (id) {
		notifications.update({
			id: id,
			title: title || "成功",
			message: message,
			color: "green",
			icon: <MDIcon c={"white"}>check</MDIcon>,
			autoClose: 2000,
			withCloseButton: true,
			loading: false,
		});
		return;
	}
	showNotification({
		title: title || "成功",
		message: message,
		color: "green",
		icon: <MDIcon c={"white"}>check</MDIcon>,
	});
}

export function showInfoNotification({
	id,
	title,
	message,
}: {
	id?: string;
	title?: string;
	message?: string;
}) {
	if (id) {
		notifications.update({
			id: id,
			title: title || "信息",
			message: message,
			color: "blue",
			icon: <MDIcon c={"white"}>info_i</MDIcon>,
			autoClose: 2000,
			withCloseButton: true,
			loading: false,
		});
		return;
	}
	showNotification({
		title: title || "信息",
		message: message,
		color: "blue",
		icon: <MDIcon c={"white"}>info_i</MDIcon>,
	});
}

export function showWarnNotification({
	id,
	title,
	message,
}: {
	id?: string;
	title?: string;
	message?: string;
}) {
	if (id) {
		notifications.update({
			id: id,
			title: title || "警告",
			message: message,
			color: "orange",
			icon: <MDIcon c={"white"}>exclamation</MDIcon>,
			autoClose: 2000,
			withCloseButton: true,
			loading: false,
		});
		return;
	}
	showNotification({
		title: title || "警告",
		message: message,
		color: "orange",
		icon: <MDIcon c={"white"}>exclamation</MDIcon>,
	});
}

export function showLoadingNotification({
	title,
	message,
}: {
	title?: string;
	message?: string;
}): string {
	const id = notifications.show({
		title: title || "请稍后",
		loading: true,
		message: message,
		color: "blue",
		autoClose: false,
		withCloseButton: false,
	});
	return id;
}
