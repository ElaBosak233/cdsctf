import { Field, FieldIcon } from "@/components/ui/field";
import { TextField } from "@/components/ui/text-field";
import { ChallengeCard } from "@/components/widgets/challenge-card";
import { Challenge } from "@/models/challenge";
import {
    SearchIcon,
    LibraryIcon,
    PackageOpenIcon,
    ListOrderedIcon,
} from "lucide-react";
import { cn } from "@/utils";
import { Button } from "@/components/ui/button";
import { Pagination } from "@/components/ui/pagination";
import { useEffect, useState } from "react";
import {
    ChallengeStatus,
    getChallengeStatus,
    getPlaygroundChallenges,
} from "@/api/challenges";
import { useAuthStore } from "@/storages/auth";
import { Dialog, DialogContent, DialogTrigger } from "@/components/ui/dialog";
import { ChallengeDialog } from "@/components/widgets/challenge-dialog";
import { useSearchParams } from "react-router";
import { Select } from "@/components/ui/select";
import { useCategoryStore } from "@/storages/category";

export default function Index() {
    const authStore = useAuthStore();
    const categoryStore = useCategoryStore();
    const [searchParams, setSearchParams] = useSearchParams();

    const [total, setTotal] = useState<number>(0);
    const [challenges, setChallenges] = useState<Array<Challenge>>();
    const [challengeStatus, setChallengeStatus] =
        useState<Record<string, ChallengeStatus>>();

    const [doSearch, setDoSearch] = useState<number>(0);
    const [title, setTitle] = useState<string>(searchParams.get("title") || "");
    const [category, setCategory] = useState<string | "all">("all");
    const [page, setPage] = useState<number>(
        Number(searchParams.get("page")) || 1
    );
    const [size, setSize] = useState<number>(
        Number(searchParams.get("size")) || 20
    );

    useEffect(() => {
        setSearchParams({
            title: title,
            page: String(page),
            size: String(size),
            category: category,
        });
    }, [title, page, size, category]);

    useEffect(() => {
        getPlaygroundChallenges({
            title,
            page,
            size,
            category: category !== "all" ? Number(category) : undefined,
        }).then((res) => {
            if (res.code === 200) {
                setTotal(res.total || 0);
                setChallenges(res.data);
            }
        });
    }, [doSearch, page, category]);

    useEffect(() => {
        if (!challenges?.length) return;

        getChallengeStatus({
            challenge_ids: challenges?.map((challenge) => challenge.id!),
            user_id: authStore?.user?.id,
        }).then((res) => {
            setChallengeStatus(res.data);
        });
    }, [challenges]);

    return (
        <>
            <div
                className={cn([
                    "flex-1",
                    "p-7",
                    "mx-auto",
                    "flex",
                    "flex-col",
                    "gap-7",
                ])}
            >
                <div className={cn(["flex", "items-center", "gap-3"])}>
                    <Field className={cn(["flex-1"])}>
                        <FieldIcon>
                            <LibraryIcon />
                        </FieldIcon>
                        <TextField
                            placeholder={"题目名"}
                            value={title}
                            onChange={(e) => setTitle(e.target.value)}
                        />
                    </Field>
                    <Button
                        size={"lg"}
                        className={cn(["h-12"])}
                        icon={SearchIcon}
                        variant={"solid"}
                        onClick={() => setDoSearch((prev) => prev + 1)}
                    >
                        搜索
                    </Button>
                </div>
                <div
                    className={cn([
                        "flex",
                        "items-center",
                        "md:justify-between",
                        "justify-center",
                    ])}
                >
                    <Pagination
                        size={"sm"}
                        total={Math.ceil(total / size)}
                        value={page}
                        onChange={setPage}
                    />
                    <div
                        className={cn([
                            "hidden",
                            "md:flex",
                            "gap-5",
                            "flex-1",
                            "justify-end",
                        ])}
                    >
                        <Field size={"sm"} className={cn(["w-48"])}>
                            <FieldIcon>
                                <LibraryIcon />
                            </FieldIcon>
                            <Select
                                options={[
                                    {
                                        value: "all",
                                        content: (
                                            <div
                                                className={cn([
                                                    "flex",
                                                    "gap-2",
                                                    "items-center",
                                                ])}
                                            >
                                                全部
                                            </div>
                                        ),
                                    },
                                    ...categoryStore.categories?.map(
                                        (category) => {
                                            const Icon = category?.icon!;

                                            return {
                                                value: String(category?.id),
                                                content: (
                                                    <div
                                                        className={cn([
                                                            "flex",
                                                            "gap-2",
                                                            "items-center",
                                                        ])}
                                                    >
                                                        <Icon />
                                                        {category?.name?.toUpperCase()}
                                                    </div>
                                                ),
                                            };
                                        }
                                    ),
                                ]}
                                onValueChange={(value) => setCategory(value)}
                                value={category}
                            />
                        </Field>
                        <Field size={"sm"} className={cn(["w-48"])}>
                            <FieldIcon>
                                <ListOrderedIcon />
                            </FieldIcon>
                            <Select
                                placeholder={"每页显示"}
                                options={[
                                    { value: "10" },
                                    { value: "20" },
                                    { value: "40" },
                                    { value: "60" },
                                ]}
                                value={String(size)}
                                onValueChange={(value) =>
                                    setSize(Number(value))
                                }
                            />
                        </Field>
                    </div>
                </div>
                <div className={cn(["flex-1"])}>
                    <div
                        className={cn([
                            "w-[60vw]",
                            "grid",
                            "sm:grid-cols-2",
                            "xl:grid-cols-4",
                            "gap-4",
                        ])}
                    >
                        {challenges?.map((challenge, index) => (
                            <Dialog key={index}>
                                <DialogTrigger>
                                    <ChallengeCard
                                        challenge={challenge}
                                        status={
                                            challengeStatus?.[challenge?.id!]
                                        }
                                    />
                                </DialogTrigger>
                                <DialogContent>
                                    <ChallengeDialog challenge={challenge} />
                                </DialogContent>
                            </Dialog>
                        ))}
                    </div>
                    {!challenges?.length && (
                        <div
                            className={cn([
                                "text-secondary-foreground",
                                "flex",
                                "justify-center",
                                "gap-3",
                                "select-none",
                            ])}
                        >
                            <PackageOpenIcon />
                            好像还没有题目哦。
                        </div>
                    )}
                </div>
            </div>
        </>
    );
}
