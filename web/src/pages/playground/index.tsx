import { keepPreviousData, useQuery } from "@tanstack/react-query";
import {
  LibraryIcon,
  ListOrderedIcon,
  PackageOpenIcon,
  SearchIcon,
  TagIcon,
  TypeIcon,
} from "lucide-react";
import { parseAsInteger, useQueryState } from "nuqs";
import { useEffect, useState } from "react";
import { useTranslation } from "react-i18next";
import { useNavigate } from "react-router";
import {
  type GetPlaygroundChallengesRequest,
  getChallengeStatus,
  getPlaygroundChallenges,
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
import type { ChallengeMini } from "@/models/challenge";
import { useAuthStore } from "@/storages/auth";
import { useConfigStore } from "@/storages/config";
import { cn } from "@/utils";
import { categories } from "@/utils/category";

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
  const { config } = useConfigStore();
  const { t } = useTranslation();
  const navigate = useNavigate();

  useEffect(() => {
    if (useAuthStore.getState().user) return;

    navigate(`/account/login?redirect=/playground`, { replace: true });
  }, [navigate]);

  const [doSearch, setDoSearch] = useState<number>(0);
  const [title, setTitle] = useQueryState("title");
  const [tag, setTag] = useQueryState("tag");
  const [category, setCategory] = useQueryState("category", {
    defaultValue: "all",
  });
  const [page, setPage] = useQueryState("page", parseAsInteger.withDefault(1));
  const [size, setSize] = useQueryState("size", parseAsInteger.withDefault(20));

  const {
    data: { challenges, total } = { challenges: [], total: 0 },
    isLoading: isChallengeFetching,
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

  const { data: challengeStatus, isLoading: isChallengeStatusFetching } =
    useChallengeStatusQuery(challenges, authStore?.user?.id);

  const loading = isChallengeFetching || isChallengeStatusFetching;

  return (
    <>
      <title>{`${t("challenge.playground")} - ${config?.meta?.title}`}</title>
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
              placeholder={t("challenge.search.title")}
              value={title || undefined}
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
            {t("common.search")}
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
                placeholder={t("challenge.search.tag")}
                value={tag || undefined}
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
                        {t("common.all")}
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
                "absolute",
                "top-1/2",
                "left-1/2",
                "-translate-x-1/2",
                "-translate-y-1/2",
                "text-secondary-foreground",
                "flex",
                "flex-col",
                "flex-1",
                "justify-center",
                "items-center",
                "gap-3",
                "select-none",
              ])}
            >
              <PackageOpenIcon className={cn(["size-8"])} />
              {t("challenge.empty")}
            </div>
          )}
        </div>
      </div>
    </>
  );
}
