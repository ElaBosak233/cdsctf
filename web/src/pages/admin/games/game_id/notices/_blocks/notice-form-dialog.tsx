import { zodResolver } from "@hookform/resolvers/zod";
import { useQuery, useQueryClient } from "@tanstack/react-query";
import {
  LibraryIcon,
  MessageCircleIcon,
  SaveIcon,
  TypeIcon,
} from "lucide-react";
import { useContext, useMemo, useState } from "react";
import { useForm } from "react-hook-form";
import { useTranslation } from "react-i18next";
import { useParams } from "react-router";
import { toast } from "sonner";
import { z } from "zod";
import { getGameChallenges } from "@/api/management/games/game_id/challenges";
import {
  createGameNotice,
  updateGameNotice,
} from "@/api/management/games/game_id/notices";
import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";
import {
  Combobox,
  ComboboxContent,
  ComboboxEmpty,
  ComboboxInput,
  ComboboxItem,
  ComboboxList,
} from "@/components/ui/combobox";
import { DialogBody, DialogFooter, DialogHeader } from "@/components/ui/dialog";
import { Field, FieldIcon } from "@/components/ui/field";
import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "@/components/ui/form";
import { MarkdownEditor } from "@/components/ui/markdown-editor";
import { TextField } from "@/components/ui/text-field";
import type { AdminGameNoticeView } from "@/models/game_notice";
import { useSharedStore } from "@/storages/shared";
import { cn } from "@/utils";
import { getCategory } from "@/utils/category";
import { notifyApiError, parseRouteNumericId } from "@/utils/query";
import { Context } from "../../context";

interface NoticeFormDialogProps {
  notice?: AdminGameNoticeView;
  onClose: () => void;
}

type ChallengeOption = {
  challenge_id: number;
  challenge_title: string;
  challenge_category: number;
  enabled: boolean;
};

function ChallengeCategoryIcon({ categoryId }: { categoryId?: number | null }) {
  const category = getCategory(categoryId ?? 1);
  const CategoryIcon = category.icon!;
  return (
    <span
      className="flex size-4 shrink-0 items-center justify-center"
      style={{ color: category.color }}
    >
      <CategoryIcon className="size-4" />
    </span>
  );
}

function NoticeFormDialog({ notice, onClose }: NoticeFormDialogProps) {
  const { t } = useTranslation();
  const isEditing = notice != null;

  const { game_id } = useParams<{ game_id: string }>();
  const routeGameId = parseRouteNumericId(game_id);
  const { game } = useContext(Context);
  const sharedStore = useSharedStore();
  const queryClient = useQueryClient();

  const gid = routeGameId ?? game?.id ?? notice?.game_id;
  const [challengeSearch, setChallengeSearch] = useState("");

  const { data: gameChallenges } = useQuery({
    queryKey: ["admin", "game-challenges-options", gid],
    queryFn: () => getGameChallenges({ game_id: gid! }),
    select: (response) =>
      [...(response.challenges ?? [])]
        .map<ChallengeOption>((gc) => ({
          challenge_id: gc.challenge_id,
          challenge_title: gc.challenge_title,
          challenge_category: gc.challenge_category ?? 1,
          enabled: gc.enabled,
        }))
        .sort((a, b) => {
          if (a.challenge_category !== b.challenge_category) {
            return a.challenge_category - b.challenge_category;
          }
          return a.challenge_title.localeCompare(b.challenge_title);
        }),
    enabled: gid != null,
  });

  const allChallenges = useMemo(() => {
    const list = gameChallenges ?? [];
    if (
      notice?.challenge_id != null &&
      !list.some((item) => item.challenge_id === notice.challenge_id)
    ) {
      return [
        {
          challenge_id: notice.challenge_id,
          challenge_title: notice.challenge_title ?? `#${notice.challenge_id}`,
          challenge_category: notice.challenge_category ?? 1,
          enabled: true,
        },
        ...list,
      ];
    }
    return list;
  }, [
    gameChallenges,
    notice?.challenge_category,
    notice?.challenge_id,
    notice?.challenge_title,
  ]);

  const filteredChallenges = useMemo(() => {
    const keyword = challengeSearch.trim().toLowerCase();
    if (!keyword) return allChallenges;
    return allChallenges.filter((item) => {
      const catName = (
        getCategory(item.challenge_category).name ?? ""
      ).toLowerCase();
      return (
        item.challenge_title.toLowerCase().includes(keyword) ||
        catName.includes(keyword) ||
        `#${item.challenge_id}`.includes(keyword)
      );
    });
  }, [allChallenges, challengeSearch]);

  const formSchema = useMemo(
    () =>
      z.object({
        title: z.string().trim().min(1, t("game:notice.form.title.message")),
        challenge_id: z.number().nullable(),
        content: z
          .string()
          .trim()
          .min(1, t("game:notice.form.content.message")),
      }),
    [t]
  );

  const form = useForm<z.infer<typeof formSchema>>({
    resolver: zodResolver(formSchema),
    defaultValues: {
      title: notice?.title ?? "",
      challenge_id: notice?.challenge_id ?? null,
      content: notice?.content ?? "",
    },
  });

  async function onSubmit(values: z.infer<typeof formSchema>) {
    if (gid == null) return;

    try {
      if (isEditing) {
        const res = await updateGameNotice({
          id: notice.id,
          game_id: gid,
          title: values.title,
          content: values.content,
          challenge_id: values.challenge_id,
        });
        toast.success(
          t("game:notice.actions.edit.success", { title: res.notice.title })
        );
      } else {
        const res = await createGameNotice({
          game_id: gid,
          title: values.title,
          content: values.content,
          challenge_id: values.challenge_id,
        });
        toast.success(
          t("game:notice.actions.create.success", { title: res.notice.title })
        );
      }

      await queryClient.invalidateQueries({
        queryKey: ["admin", "game-notices", gid],
      });
      await queryClient.invalidateQueries({
        queryKey: ["game_notices", gid],
      });
      sharedStore?.setRefresh();
      onClose();
    } catch (error) {
      await notifyApiError(error);
    }
  }

  return (
    <Card
      className={cn([
        "w-full",
        "max-w-2xl",
        "min-h-64",
        "rounded-elevated",
        "shadow-lg",
        "overflow-hidden",
        "flex",
        "flex-col",
      ])}
    >
      <DialogHeader
        className="p-5 pb-0"
        icon={<MessageCircleIcon />}
        title={
          isEditing
            ? t("game:notice.actions.edit._")
            : t("game:notice.actions.create._")
        }
      />
      <DialogBody className="p-5 pt-4">
        <Form {...form}>
          <form
            onSubmit={form.handleSubmit(onSubmit)}
            autoComplete="off"
            className={cn(["flex", "flex-col", "flex-1", "gap-5"])}
          >
            <FormField
              control={form.control}
              name="title"
              render={({ field }) => (
                <FormItem className="w-full">
                  <FormLabel>{t("game:notice.form.title._")}</FormLabel>
                  <Field size="sm">
                    <FieldIcon>
                      <TypeIcon />
                    </FieldIcon>
                    <FormControl>
                      <TextField
                        {...field}
                        placeholder={t("game:notice.form.title.placeholder")}
                        value={field.value || ""}
                        onChange={field.onChange}
                      />
                    </FormControl>
                  </Field>
                  <FormMessage />
                </FormItem>
              )}
            />

            <FormField
              control={form.control}
              name="challenge_id"
              render={({ field }) => {
                const selectedChallenge =
                  allChallenges.find(
                    (item) => item.challenge_id === field.value
                  ) ?? null;

                return (
                  <FormItem className="w-full">
                    <FormLabel>{t("game:notice.form.challenge._")}</FormLabel>
                    <Field size="sm">
                      <FieldIcon>
                        <LibraryIcon />
                      </FieldIcon>
                      <Combobox<ChallengeOption>
                        value={selectedChallenge}
                        options={filteredChallenges.map((challenge) => ({
                          value: challenge,
                          content: `#${challenge.challenge_id} ${challenge.challenge_title}`,
                        }))}
                        itemToStringLabel={(item) =>
                          `#${item.challenge_id} ${item.challenge_title}`
                        }
                        isItemEqualToValue={(item, value) =>
                          item.challenge_id === value.challenge_id
                        }
                        filter={null}
                        onInputValueChange={(value, details) => {
                          if (
                            details.reason === "input-change" ||
                            value === ""
                          ) {
                            setChallengeSearch(value);
                          }
                        }}
                        onValueChange={(item) => {
                          field.onChange(item?.challenge_id ?? null);
                          setChallengeSearch("");
                        }}
                      >
                        <ComboboxInput
                          showClear
                          placeholder={t("game:notice.challenge_none")}
                          startContent={
                            selectedChallenge ? (
                              <ChallengeCategoryIcon
                                categoryId={
                                  selectedChallenge.challenge_category
                                }
                              />
                            ) : undefined
                          }
                        />
                        <ComboboxContent>
                          <ComboboxEmpty>{t("challenge:empty")}</ComboboxEmpty>
                          <ComboboxList>
                            {filteredChallenges.map((gc) => (
                              <ComboboxItem key={gc.challenge_id} value={gc}>
                                <ChallengeCategoryIcon
                                  categoryId={gc.challenge_category}
                                />
                                <span className="shrink-0 font-mono text-xs text-muted-foreground">
                                  #{gc.challenge_id}
                                </span>
                                <span className="min-w-0 truncate">
                                  {gc.challenge_title}
                                </span>
                              </ComboboxItem>
                            ))}
                          </ComboboxList>
                        </ComboboxContent>
                      </Combobox>
                    </Field>
                    <FormMessage />
                  </FormItem>
                );
              }}
            />

            <FormField
              control={form.control}
              name="content"
              render={({ field }) => (
                <FormItem className="w-full">
                  <FormLabel>{t("game:notice.form.content._")}</FormLabel>
                  <FormControl>
                    <MarkdownEditor
                      {...field}
                      className={cn(["h-full", "min-h-64"])}
                    />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />

            <DialogFooter>
              <Button
                icon={<SaveIcon />}
                variant="solid"
                type="submit"
                loading={form.formState.isSubmitting}
              >
                {t("common:actions.save")}
              </Button>
            </DialogFooter>
          </form>
        </Form>
      </DialogBody>
    </Card>
  );
}

export { NoticeFormDialog };
