import { useEffect } from "react";
import { useTranslation } from "react-i18next";
import { toast } from "sonner";

import { getSubmission } from "@/api/submissions";
import { Status } from "@/models/submission";
import { useCheckerStore } from "@/storages/checker";

function CheckerWatcher() {
  const { t } = useTranslation();
  const { submissions, pop } = useCheckerStore();

  useEffect(() => {
    if (submissions.length === 0) return;
    const interval = setInterval(() => {
      submissions.forEach(async (submission) => {
        const res = await getSubmission({
          id: submission.id,
          is_desensitized: true,
        });

        const s = res.items?.[0];
        if (!s || s.status === 0) return;

        switch (s.status) {
          case Status.Correct:
            toast.success(`#${s.id} ${t("submission:solved")}`, {
              id: `submission-${s.id}`,
              description: t("submission:result.correct"),
            });
            break;
          case Status.Incorrect:
            toast.error(`#${s.id}`, {
              id: `submission-${s.id}`,
              description: t("submission:result.incorrect"),
            });
            break;
          case Status.Cheat:
            toast.error(`#${s.id}`, {
              id: `submission-${s.id}`,
              description: t("submission:result.cheat"),
            });
            break;
          case Status.Expired:
            toast.info(`#${s.id}`, {
              id: `submission-${s.id}`,
              description: t("submission:result.timeout"),
            });
            break;
          case Status.Duplicate:
            toast.success(`#${s.id} ${t("submission:solved")}`, {
              id: `submission-${s.id}`,
              description: t("submission:result.duplicate"),
            });
            break;
        }

        pop(s.id!);
      });
    }, 2000);

    return () => clearInterval(interval);
  }, [submissions, pop, t]);

  return null;
}

export { CheckerWatcher };
