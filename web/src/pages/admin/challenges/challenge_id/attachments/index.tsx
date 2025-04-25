import {
    Dropzone,
    DropZoneArea,
    DropzoneTrigger,
    useDropzone,
} from "@/components/ui/dropzone";
import { Metadata } from "@/models/media";
import {
    CloudUploadIcon,
    HardDrive,
    HardDriveIcon,
    TextIcon,
    TrashIcon,
} from "lucide-react";
import { useContext, useEffect, useState } from "react";
import { Context } from "../context";
import { deleteChallengeAttachment } from "@/api/admin/challenges/challenge_id/attachment";
import { Field, FieldIcon } from "@/components/ui/field";
import { TextField } from "@/components/ui/text-field";
import { cn } from "@/utils";
import { Button } from "@/components/ui/button";
import { toast } from "sonner";
import { getChallengeAttachmentMetadata } from "@/api/challenges/challenge_id/attachment";

export default function Index() {
    const { challenge } = useContext(Context);
    const [metadata, setMetadata] = useState<Metadata>();
    const [refresh, setRefresh] = useState<number>(0);

    useEffect(() => {
        if (!challenge?.id) return;
        getChallengeAttachmentMetadata(challenge?.id!).then((res) => {
            setMetadata(res.data);
        });
    }, [challenge?.id, refresh]);

    const dropzone = useDropzone({
        onDropFile: async (file) => {
            const formData = new FormData();
            formData.append("file", file);
            const xhr = new XMLHttpRequest();
            xhr.open(
                "POST",
                `/api/admin/challenges/${challenge?.id}/attachment`,
                true
            );
            xhr.upload.onprogress = (event) => {
                if (event.lengthComputable) {
                    const percentComplete = (event.loaded / event.total) * 100;
                    toast.loading(`上传进度 ${percentComplete}%`, {
                        id: "attachment-upload",
                    });
                }
            };
            xhr.onload = () => {
                if (xhr.status === 200) {
                    toast.success("文件上传成功", {
                        id: "attachment-upload",
                    });
                    setRefresh((prev) => prev + 1);
                } else {
                    toast.error("文件上传失败", {
                        id: "attachment-upload",
                        description: xhr.responseText,
                    });
                }
            };
            xhr.onerror = () => {
                return {
                    status: "error",
                };
            };

            xhr.send(formData);

            return {
                status: "success",
                result: "",
            };
        },
    });

    function handleDeleteAttachment() {
        deleteChallengeAttachment(challenge?.id!)
            .then((res) => {
                if (res.code === 200) {
                    toast.success(`题目 ${challenge?.title} 附件删除成功`);
                }
            })
            .finally(() => {
                setRefresh((prev) => prev + 1);
            });
    }

    return (
        <div className={cn(["flex", "flex-col", "gap-5"])}>
            <Dropzone {...dropzone}>
                <DropZoneArea>
                    <DropzoneTrigger className="h-fit flex flex-col items-center gap-4 bg-transparent p-10 text-center text-sm">
                        <CloudUploadIcon className="size-16" />
                        <p className="font-semibold">上传附件</p>
                        <p className="text-sm text-muted-foreground">
                            附件将直接由服务器托管，建议充分考虑存储空间、流量等因素。
                        </p>
                    </DropzoneTrigger>
                </DropZoneArea>
            </Dropzone>
            <div className={cn(["flex", "gap-3", "items-center"])}>
                {metadata && (
                    <>
                        <Field className={cn(["w-1/2"])}>
                            <FieldIcon>
                                <TextIcon />
                            </FieldIcon>
                            <TextField
                                disabled
                                placeholder={"文件名"}
                                value={metadata?.filename}
                                onChange={() => {}}
                            />
                        </Field>
                        <Field className={cn(["flex-1"])}>
                            <FieldIcon>
                                <HardDriveIcon />
                            </FieldIcon>
                            <TextField
                                disabled
                                placeholder={"文件大小"}
                                value={`${metadata?.size} Bytes`}
                                onChange={() => {}}
                            />
                        </Field>
                        <Button
                            variant={"solid"}
                            level={"error"}
                            icon={TrashIcon}
                            onClick={handleDeleteAttachment}
                        >
                            删除附件
                        </Button>
                    </>
                )}
            </div>
        </div>
    );
}
