import { showNotification } from "@mantine/notifications";
import MDIcon from "@/components/ui/MDIcon";

export function showErrNotification({
	title,
	message,
}: {
	title?: string;
	message?: string;
}) {
	showNotification({
		title: title || "发生了错误",
		message: message,
		color: "red",
		icon: <MDIcon>close</MDIcon>,
	});
}

export function showSuccessNotification({
	title,
	message,
}: {
	title?: string;
	message?: string;
}) {
	showNotification({
		title: title || "成功",
		message: message,
		color: "green",
		icon: <MDIcon>check</MDIcon>,
	});
}

export function showInfoNotification({
	title,
	message,
}: {
	title?: string;
	message?: string;
}) {
	showNotification({
		title: title || "信息",
		message: message,
		color: "blue",
		icon: <MDIcon>info_i</MDIcon>,
	});
}

export function showWarnNotification({
	title,
	message,
}: {
	title?: string;
	message?: string;
}) {
	showNotification({
		title: title || "警告",
		message: message,
		color: "orange",
		icon: <MDIcon>exclamation</MDIcon>,
	});
}
