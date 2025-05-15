import {
  BrushIcon,
  CircleIcon,
  CloudIcon,
  CodeIcon,
  CpuIcon,
  DatabaseIcon,
  GlobeIcon,
  HashIcon,
  ImageIcon,
  LightbulbIcon,
  LucideIcon,
  RewindIcon,
  SearchIcon,
  SettingsIcon,
  ShieldIcon,
  SmartphoneIcon,
  StarIcon,
} from "lucide-react";

type Category = {
  id?: number;
  name?: string;
  color?: string;
  icon?: LucideIcon;
};

const categories = [
  {
    id: 1,
    name: "misc",
    color: "#3F51B5",
    icon: StarIcon,
  },
  {
    id: 2,
    name: "web",
    color: "#009688",
    icon: GlobeIcon,
  },
  {
    id: 3,
    name: "reverse",
    color: "#E64A19",
    icon: RewindIcon,
  },
  {
    id: 4,
    name: "crypto",
    color: "#607D8B",
    icon: HashIcon,
  },
  {
    id: 5,
    name: "pwn",
    color: "#D32F2F",
    icon: CodeIcon,
  },
  {
    id: 6,
    name: "forensics",
    color: "#9C27B0",
    icon: SearchIcon,
  },
  {
    id: 7,
    name: "ds",
    color: "#0D47A1",
    icon: DatabaseIcon,
  },
  {
    id: 8,
    name: "mobile",
    color: "#1976D2",
    icon: SmartphoneIcon,
  },
  {
    id: 9,
    name: "steg",
    color: "#795548",
    icon: CircleIcon,
  },
  {
    id: 10,
    name: "osint",
    color: "#4CAF50",
    icon: ImageIcon,
  },
  {
    id: 11,
    name: "hardware",
    color: "#673AB7",
    icon: CpuIcon,
  },
  {
    id: 12,
    name: "cloud",
    color: "#FF9800",
    icon: CloudIcon,
  },
  {
    id: 13,
    name: "societal",
    color: "#BF360C",
    icon: SettingsIcon,
  },
  {
    id: 14,
    name: "ai",
    color: "#1565C0",
    icon: LightbulbIcon,
  },
  {
    id: 15,
    name: "blockchain",
    color: "#009688",
    icon: ShieldIcon,
  },
  {
    id: 16,
    name: "art",
    color: "#F57F17",
    icon: BrushIcon,
  },
  {
    id: 17,
    name: "dev",
    color: "#37474F",
    icon: CodeIcon,
  },
];

function getCategory(id: number): Category {
  const category = categories.find((category) => category.id === id);
  if (category) return category;
  return categories[0];
}

export { type Category, categories, getCategory };
