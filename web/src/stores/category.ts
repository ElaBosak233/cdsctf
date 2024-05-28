import { Category } from "@/types/category";
import { create } from "zustand";
import { createJSONStorage, persist } from "zustand/middleware";

interface CategoryState {
	categories: Array<Category>;
	setCategories: (categories: Array<Category>) => void;
	refresh: number;
	setRefresh: (refresh: number) => void;
}

export const useCategoryStore = create<CategoryState>()(
	persist(
		(set) => ({
			categories: [],
			setCategories: (categories) => set({ categories }),
			refresh: 0,
			setRefresh: (refresh) => set({ refresh }),
		}),
		{
			name: "category_storage",
			storage: createJSONStorage(() => localStorage),
		}
	)
);
