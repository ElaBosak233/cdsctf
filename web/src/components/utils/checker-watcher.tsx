import { useEffect } from "react";
import { toast } from "sonner";

import { getSubmission } from "@/api/submissions";
import { Status } from "@/models/submission";
import { useCheckerStore } from "@/storages/checker";

function CheckerWatcher() {
  const { submissions, pop } = useCheckerStore();

  useEffect(() => {
    if (submissions.length === 0) return;
    const interval = setInterval(() => {
      submissions.forEach(async (submission) => {
        const res = await getSubmission({
          id: submission.id,
          is_desensitized: true,
        });

        const s = res.data?.[0];
        if (!s || s.status === 0) return;

        switch (s.status) {
          case Status.Correct:
            toast.success(`#${s.id} 正确`, {
              id: `submission-${s.id}`,
              description: "恭喜你，提交成功！",
            });
            break;
          case Status.Incorrect:
            toast.error(`#${s.id} 错误`, {
              id: `submission-${s.id}`,
              description: "再检查一下？",
            });
            break;
          case Status.Cheat:
            toast.error(`#${s.id} 作弊`, {
              id: `submission-${s.id}`,
              description: "你存在作弊的可能，已记录。",
            });
            break;
          case Status.Expired:
            toast.info(`#${s.id} 超时`, {
              id: `submission-${s.id}`,
              description: "提交超时。",
            });
            break;
          case Status.Duplicate:
            toast.success(`#${s.id} 正确，又一次`, {
              id: `submission-${s.id}`,
              description: "你已经做出过这道题了。",
            });
            break;
        }

        pop(s.id!);
      });
    }, 2000);

    return () => clearInterval(interval);
  }, [submissions]);

  return null;
}

export { CheckerWatcher };
