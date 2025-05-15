import { Submission } from "@/models/submission";
import { create } from "zustand";

interface CheckerState {
  loading: boolean;
  setLoading: (loading: boolean) => void;

  submissions: Array<Submission>;
  add: (submission: Submission) => void;
  pop: (id: number) => void;
}

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
