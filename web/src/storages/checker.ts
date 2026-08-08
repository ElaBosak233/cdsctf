import { create } from "zustand";

import type { SubmissionSummary } from "@/models/submission";

type CheckerState = {
  loading: boolean;
  setLoading: (loading: boolean) => void;

  submissions: Array<SubmissionSummary>;
  add: (submission: SubmissionSummary) => void;
  pop: (id: number) => void;
};

export const useCheckerStore = create<CheckerState>()((set, get) => ({
  loading: false,
  setLoading: (loading) => set({ loading }),

  submissions: [],
  add: (submission) => set({ submissions: [...get().submissions, submission] }),
  pop: (id) =>
    set({
      submissions: get().submissions.filter(
        (submission) => submission.id !== id
      ),
    }),
}));
