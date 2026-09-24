import { useEffect } from "react";
import { useTranslation } from "react-i18next";
import { toast } from "sonner";

import { listSubmissions } from "@/api/submissions";
import { Status } from "@/models/submission";
import { useCheckerStore } from "@/storages/checker";
import { notifyApiError } from "@/utils/query";

function CheckerWatcher() {
  const { t } = useTranslation();
  const { submissions, pop } = useCheckerStore();

  useEffect(() => {
    if (submissions.length === 0) return;
    const interval = setInterval(() => {
      submissions.forEach(async (submission) => {
        try {
          const res = await listSubmissions({
            id: submission.id,
            is_desensitized: true,
          });

          const s = res.submissions?.[0];
          if (
            !s ||
            s.status === Status.Queued ||
            s.status === Status.Processing
          )
            return;

          switch (s.status) {
            case Status.Correct:
              toast.success(
                t("submission:notifications.result.correct.title"),
                {
                  id: `submission-${s.id}`,
                  description: t(
                    "submission:notifications.result.correct.description_with_id",
                    { id: s.id }
                  ),
                }
              );
              break;
            case Status.Incorrect:
              toast.error(
                t("submission:notifications.result.incorrect.title"),
                {
                  id: `submission-${s.id}`,
                  description: t(
                    "submission:notifications.result.incorrect.description_with_id",
                    { id: s.id }
                  ),
                }
              );
              break;
            case Status.Cheat:
              toast.error(t("submission:notifications.result.cheat.title"), {
                id: `submission-${s.id}`,
                description: t(
                  "submission:notifications.result.cheat.description_with_id",
                  { id: s.id }
                ),
              });
              break;
            case Status.Expired:
              toast.info(t("submission:notifications.result.expired.title"), {
                id: `submission-${s.id}`,
                description: t(
                  "submission:notifications.result.expired.description_with_id",
                  { id: s.id }
                ),
              });
              break;
            case Status.Duplicate:
              toast.success(
                t("submission:notifications.result.duplicate.title"),
                {
                  id: `submission-${s.id}`,
                  description: t(
                    "submission:notifications.result.duplicate.description_with_id",
                    { id: s.id }
                  ),
                }
              );
              break;
          }

          pop(s.id!);
        } catch (error) {
          await notifyApiError(error, {
            id: `submission-${submission.id}`,
            title: t("submission:notifications.status_check_failed"),
          });
        }
      });
    }, 2000);

    return () => clearInterval(interval);
  }, [submissions, pop, t]);

  return null;
}

export { CheckerWatcher };
