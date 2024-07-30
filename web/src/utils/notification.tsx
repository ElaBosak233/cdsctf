import {
    NotificationData,
    showNotification,
    updateNotification,
} from "@mantine/notifications";
import MDIcon from "@/components/ui/MDIcon";

export function showErrNotification({
    id,
    title,
    message,
    update,
}: {
    id?: string;
    title?: string;
    message?: string;
    update?: boolean;
}) {
    const notificationData: NotificationData = {
        id: id,
        title: title || "错误",
        message: message,
        color: "red",
        icon: <MDIcon c={"white"}>exclamation</MDIcon>,
    };
    if (update) {
        updateNotification({
            ...notificationData,
            autoClose: 2000,
            withCloseButton: true,
            loading: false,
        });
    } else {
        showNotification(notificationData);
    }
}

export function showSuccessNotification({
    id,
    title,
    message,
    update,
}: {
    id?: string;
    title?: string;
    message?: string;
    update?: boolean;
}) {
    const notificationData: NotificationData = {
        id: id,
        title: title || "成功",
        message: message,
        color: "green",
        icon: <MDIcon c={"white"}>check</MDIcon>,
    };
    if (update) {
        updateNotification({
            ...notificationData,
            autoClose: 2000,
            withCloseButton: true,
            loading: false,
        });
    } else {
        showNotification(notificationData);
    }
}

export function showInfoNotification({
    id,
    title,
    message,
    update,
}: {
    id?: string;
    title?: string;
    message?: string;
    update?: boolean;
}) {
    const notificationData: NotificationData = {
        id: id,
        title: title || "信息",
        message: message,
        color: "blue",
        icon: <MDIcon c={"white"}>info_i</MDIcon>,
    };
    if (update) {
        updateNotification({
            ...notificationData,
            autoClose: 2000,
            withCloseButton: true,
            loading: false,
        });
    } else {
        showNotification(notificationData);
    }
}

export function showWarnNotification({
    id,
    title,
    message,
    update,
}: {
    id?: string;
    title?: string;
    message?: string;
    update?: boolean;
}) {
    const notificationData: NotificationData = {
        id: id,
        title: title || "警告",
        message: message,
        color: "orange",
        icon: <MDIcon c={"white"}>exclamation</MDIcon>,
    };
    if (update) {
        updateNotification({
            ...notificationData,
            autoClose: 2000,
            withCloseButton: true,
            loading: false,
        });
    } else {
        showNotification(notificationData);
    }
}

export function showLoadingNotification({
    id,
    title,
    message,
}: {
    id?: string;
    title?: string;
    message?: string;
}) {
    showNotification({
        id: id,
        title: title || "请稍后",
        loading: true,
        message: message,
        color: "blue",
        autoClose: false,
        withCloseButton: false,
    });
}
