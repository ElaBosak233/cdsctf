import {
  LibraryIcon,
  ListOrderedIcon,
  PackageOpenIcon,
  SearchIcon,
  TagIcon,
  TypeIcon,
} from "lucide-react";
import { useEffect, useState } from "react";
import { useNavigate, useSearchParams } from "react-router";

import {
  getChallengeStatus,
  getPlaygroundChallenges,
  GetPlaygroundChallengesRequest,
} from "@/api/challenges";
import { Button } from "@/components/ui/button";
import { Dialog, DialogContent, DialogTrigger } from "@/components/ui/dialog";
import { Field, FieldIcon } from "@/components/ui/field";
import { LoadingOverlay } from "@/components/ui/loading-overlay";
import { Pagination } from "@/components/ui/pagination";
import { Select } from "@/components/ui/select";
import { TextField } from "@/components/ui/text-field";
import { ChallengeCard } from "@/components/widgets/challenge-card";
import { ChallengeDialog } from "@/components/widgets/challenge-dialog";
import { ChallengeMini } from "@/models/challenge";
import { useAuthStore } from "@/storages/auth";
import { useConfigStore } from "@/storages/config";
import { cn } from "@/utils";
import { categories } from "@/utils/category";
import { keepPreviousData, useQuery } from "@tanstack/react-query";

function usePlaygroundChallengeQuery(
  params: GetPlaygroundChallengesRequest,
  trigger: number = 0
) {
  return useQuery({
    queryKey: [
      "playground",
      trigger,
      params.size,
      params.page,
      params.category,
    ],
    queryFn: () => getPlaygroundChallenges(params),
    select: (response) => ({
      challenges: response.data || [],
      total: response.total || 0,
    }),
    enabled: !!params,
    placeholderData: keepPreviousData,
  });
}

function useChallengeStatusQuery(
  challenges: ChallengeMini[] | undefined,
  userId?: number
) {
  return useQuery({
    queryKey: ["challenge_status", challenges?.map((c) => c.id), userId],
    queryFn: () =>
      getChallengeStatus({
        challenge_ids: challenges?.map((challenge) => challenge.id!) || [],
        user_id: userId,
      }),
    select: (response) => response.data || {},
    enabled: !!challenges?.length && !!userId,
  });
}

export default function Index() {
  const authStore = useAuthStore();
  const configStore = useConfigStore();
  const navigate = useNavigate();

  useEffect(() => {
    if (authStore?.user) return;

    navigate(`/account/login?redirect=/playground`, { replace: true });
  }, []);

  const [searchParams, setSearchParams] = useSearchParams();

  const [doSearch, setDoSearch] = useState<number>(0);
  const [title, setTitle] = useState<string>(searchParams.get("title") || "");
  const [tag, setTag] = useState<string>(searchParams.get("tag") || "");
  const [category, setCategory] = useState<string | "all">(
    searchParams.get("category") || "all"
  );
  const [page, setPage] = useState<number>(
    Number(searchParams.get("page")) || 1
  );
  const [size, setSize] = useState<number>(
    Number(searchParams.get("size")) || 20
  );

  useEffect(() => {
    if (doSearch < 1) return;

    const searchParams: {
      page: string;
      size: string;
      category?: string;
      title?: string;
      tag?: string;
    } = {
      page: String(page),
      size: String(size),
    };
    if (title) searchParams.title = title;
    if (tag) searchParams.tag = tag;
    if (category) searchParams.category = String(category);
    setSearchParams(searchParams);
  }, [doSearch]);

  const {
    data: { challenges, total } = { challenges: [], total: 0 },
    isFetching: loading,
  } = usePlaygroundChallengeQuery(
    {
      page,
      size,
      category: category !== "all" ? Number(category) : undefined,
      title: title || undefined,
      tag: tag || undefined,
      sorts: "-created_at",
    },
    doSearch
  );

  const { data: challengeStatus } = useChallengeStatusQuery(
    challenges,
    authStore?.user?.id
  );

  return (
    <>
      <title>{`练习场 - ${configStore?.config?.meta?.title}`}</title>
      <div
        className={cn([
          "flex-1",
          "p-7",
          "xl:mx-auto",
          "flex",
          "flex-col",
          "gap-7",
        ])}
      >
        <div className={cn(["flex", "items-center", "gap-3"])}>
          <Field className={cn(["flex-1"])}>
            <FieldIcon>
              <TypeIcon />
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
            icon={<SearchIcon />}
            variant={"solid"}
            onClick={() => setDoSearch((prev) => prev + 1)}
          >
            搜索
          </Button>
        </div>
        <div
          className={cn([
            "flex",
            "flex-wrap",
            "items-center",
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
                <TagIcon />
              </FieldIcon>
              <TextField
                placeholder={"标签"}
                value={tag}
                onChange={(e) => setTag(e.target.value)}
              />
            </Field>
            <Field size={"sm"} className={cn(["w-48"])}>
              <FieldIcon>
                <LibraryIcon />
              </FieldIcon>
              <Select
                options={[
                  {
                    value: "all",
                    content: (
                      <div className={cn(["flex", "gap-2", "items-center"])}>
                        全部
                      </div>
                    ),
                  },
                  ...(categories || []).map((category) => {
                    const Icon = category.icon!;

                    return {
                      value: String(category?.id),
                      content: (
                        <div className={cn(["flex", "gap-2", "items-center"])}>
                          <Icon />
                          {category?.name?.toUpperCase()}
                        </div>
                      ),
                    };
                  }),
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
                onValueChange={(value) => setSize(Number(value))}
              />
            </Field>
          </div>
        </div>
        <div className={cn(["flex-1", "relative"])}>
          <LoadingOverlay loading={loading} />
          <div
            className={cn([
              "grid",
              "w-full",
              "sm:grid-cols-2",
              "md:grid-cols-3",
              "xl:grid-cols-4",
              "xl:w-[60vw]",
              "gap-4",
            ])}
          >
            {challenges?.map((challenge) => (
              <Dialog key={challenge?.id}>
                <DialogTrigger>
                  <ChallengeCard
                    digest={challenge}
                    status={challengeStatus?.[challenge.id!]}
                  />
                </DialogTrigger>
                <DialogContent>
                  <ChallengeDialog digest={challenge} />
                </DialogContent>
              </Dialog>
            ))}
          </div>
          {!challenges?.length && !loading && (
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
