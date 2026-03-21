import { useQuery } from "@tanstack/react-query";
import { DownloadIcon } from "lucide-react";
import { useContext } from "react";
import { getChallengeAttachments as getChallengeAttachmentsDebug } from "@/api/admin/challenges/challenge_id/attachments";
import { getChallengeAttachments } from "@/api/challenges/challenge_id/attachments";
import { Button } from "@/components/ui/button";
import { cn } from "@/utils";
import { Context } from "./context";

function useChallengeAttachmentsQuery(
  challengeId?: number,
  hasAttachment?: boolean,
  debug: boolean = false
) {
  return useQuery({
    queryKey: ["challenge_attachments", challengeId],
    queryFn: () =>
      debug
        ? getChallengeAttachmentsDebug(challengeId!)
        : getChallengeAttachments(challengeId!),
    select: (response) => response.items,
    enabled: !!challengeId && hasAttachment,
  });
}

export function AttachmentSection() {
  const { challenge, debug } = useContext(Context);

  const { data: metadata } = useChallengeAttachmentsQuery(
    challenge?.id,
    challenge?.has_attachment,
    debug
  );

  return (
    <div className={cn(["flex", "gap-3", "flex-wrap"])}>
      {metadata?.map((m) => (
        <Button asChild icon={<DownloadIcon />} size={"sm"} key={m.filename}>
          <a
            target={"_blank"}
            href={
              debug
                ? `/api/admin/challenges/${challenge?.id}/attachments/${m.filename}`
                : `/api/challenges/${challenge?.id}/attachments/${m.filename}`
            }
          >
            {m.filename}
          </a>
        </Button>
      ))}
    </div>
  );
}
