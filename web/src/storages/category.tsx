import { create } from "zustand";
import { createJSONStorage, persist } from "zustand/middleware";
import {
    Star,
    MessageCircle,
    Globe,
    Rewind,
    Hash,
    Code,
    Image,
    ImagePlus,
    MessageSquareCode,
    Search,
    Cloud,
    Cpu,
    Smartphone,
    Database,
    Circle,
    HardDrive,
    Settings,
    Lightbulb,
    Shield,
    Brush,
    LucideIcon,
} from "lucide-react";

export interface Category {
    id?: number;
    name?: string;
    color?: string;
    icon?: LucideIcon;
}

interface CategoryState {
    categories: Array<Category>;
    setCategories: (categories: Array<Category>) => void;
    getCategory: (id?: number) => Category;
}

export const useCategoryStore = create<CategoryState>()(
    persist(
        (set, get) => ({
            categories: [
                {
                    id: 1,
                    name: "misc",
                    color: "#3F51B5",
                    icon: Star,
                },
                {
                    id: 2,
                    name: "web",
                    color: "#009688",
                    icon: Globe,
                },
                {
                    id: 3,
                    name: "reverse",
                    color: "#E64A19",
                    icon: Rewind,
                },
                {
                    id: 4,
                    name: "crypto",
                    color: "#607D8B",
                    icon: Hash,
                },
                {
                    id: 5,
                    name: "pwn",
                    color: "#D32F2F",
                    icon: Code,
                },
                {
                    id: 6,
                    name: "forensics",
                    color: "#9C27B0",
                    icon: Search,
                },
                {
                    id: 7,
                    name: "ds",
                    color: "#0D47A1",
                    icon: Database,
                },
                {
                    id: 8,
                    name: "mobile",
                    color: "#1976D2",
                    icon: Smartphone,
                },
                {
                    id: 9,
                    name: "steg",
                    color: "#795548",
                    icon: Circle,
                },
                {
                    id: 10,
                    name: "osint",
                    color: "#4CAF50",
                    icon: Image,
                },
                {
                    id: 11,
                    name: "hardware",
                    color: "#673AB7",
                    icon: Cpu,
                },
                {
                    id: 12,
                    name: "cloud",
                    color: "#FF9800",
                    icon: Cloud,
                },
                {
                    id: 13,
                    name: "societal",
                    color: "#BF360C",
                    icon: Settings,
                },
                {
                    id: 14,
                    name: "ai",
                    color: "#1565C0",
                    icon: Lightbulb,
                },
                {
                    id: 15,
                    name: "blockchain",
                    color: "#009688",
                    icon: Shield,
                },
                {
                    id: 16,
                    name: "art",
                    color: "#F57F17",
                    icon: Brush,
                },
                {
                    id: 17,
                    name: "dev",
                    color: "#37474F",
                    icon: Code,
                },
            ],
            setCategories: (categories: Array<Category>) => set({ categories }),
            getCategory: (id?: number) => {
                const category = get().categories.find(
                    (category) => category.id === id
                );
                if (category) return category;
                return get().categories[0];
            },
        }),
        {
            name: "category",
            storage: createJSONStorage(() => localStorage),
        }
    )
);
