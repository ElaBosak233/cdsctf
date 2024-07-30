import { Category } from "@/types/category";
import { create } from "zustand";
import { createJSONStorage, persist } from "zustand/middleware";

interface CategoryState {
    categories: Array<Category>;
    setCategories: (categories: Array<Category>) => void;
    getCategory: (id: number) => Category | undefined;
    refresh: number;
    setRefresh: (refresh: number) => void;
}

export const useCategoryStore = create<CategoryState>()(
    persist(
        (set, get) => ({
            categories: [],
            setCategories: (categories) => set({ categories }),
            getCategory: (id) =>
                get().categories.find((category) => category.id === id),
            refresh: 0,
            setRefresh: (refresh) => set({ refresh }),
        }),
        {
            name: "category_storage",
            storage: createJSONStorage(() => localStorage),
        }
    )
);
