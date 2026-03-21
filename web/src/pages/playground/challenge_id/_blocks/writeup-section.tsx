import { useQuery } from "@tanstack/react-query";
import { ChevronLeftIcon, LightbulbIcon, PencilLineIcon } from "lucide-react";
import { parseAsInteger, parseAsString, useQueryState } from "nuqs";
import { useContext, useMemo } from "react";
import { useTranslation } from "react-i18next";
import { getNotes } from "@/api/notes";
import { Avatar } from "@/components/ui/avatar";
import { Button } from "@/components/ui/button";
import { MarkdownRender } from "@/components/ui/markdown-render";
import { Pagination } from "@/components/ui/pagination";
import { ScrollArea } from "@/components/ui/scroll-area";
import { Separator } from "@/components/ui/separator";
import { Typography } from "@/components/ui/typography";
import { cn } from "@/utils";
import { Context } from "../context";

function useNotesQuery(
  challengeId?: number,
  page: number = 1,
  size: number = 10
) {
  return useQuery({
    queryKey: ["note", `challenge_id=${challengeId}`],
    queryFn: () =>
      getNotes({ challenge_id: challengeId, page, size, sorts: "-updated_at" }),
    select: (response) => ({
      notes: response.items || [],
      total: response.total || 0,
    }),
    enabled: !!challengeId,
  });
}

function WriteupSection() {
  const { t } = useTranslation();
  const { challenge } = useContext(Context);

  const [page, setPage] = useQueryState("page", parseAsInteger.withDefault(1));
  const [size, _setSize] = useQueryState(
    "size",
    parseAsInteger.withDefault(10)
  );
  const [view, setView] = useQueryState(
    "writeup_view",
    parseAsString.withDefault("list")
  );
  const [noteId, setNoteId] = useQueryState("note_id", parseAsInteger);

  const { data: notesData } = useNotesQuery(challenge?.id, page, size);
  const notes = notesData?.notes || [];
  const total = notesData?.total || 0;

  const selectedNote = useMemo(() => {
    if (!notes || notes.length === 0) return undefined;
    const matched = notes.find((note) => note.id === noteId);
    return matched ?? notes[0];
  }, [notes, noteId]);

  const isReading = view === "writeup" || view === "note";
  const content =
    view === "writeup" ? challenge?.writeup : selectedNote?.content;

  return (
    <div className={cn(["flex", "flex-1", "min-h-0", "flex-col", "gap-2"])}>
      {isReading ? (
        <>
          <div className={cn(["flex", "items-center", "gap-2"])}>
            <Button
              size={"sm"}
              square
              icon={<ChevronLeftIcon />}
              onClick={() => setView("list")}
            />
            <span className={cn(["text-sm", "text-muted-foreground"])}>
              {view === "writeup" ? (
                t("challenge:writeup_official")
              ) : (
                <span className={cn(["flex", "items-center", "gap-1"])}>
                  {t("challenge:note")}
                  <Separator
                    orientation="vertical"
                    className={cn(["mx-1", "h-4"])}
                  />
                  {selectedNote?.user_name}
                </span>
              )}
            </span>
          </div>
          {content ? (
            <div className={cn(["flex-1", "min-h-0", "flex", "flex-col"])}>
              <ScrollArea
                className={cn(["h-full", "min-h-0", "overflow-hidden"])}
              >
                <div className={cn(["space-y-4", "pr-3", "pb-6"])}>
                  <Typography>
                    <MarkdownRender src={content} />
                  </Typography>
                </div>
              </ScrollArea>
            </div>
          ) : (
            <span className={cn(["text-muted", "text-center", "select-none"])}>
              暂无内容
            </span>
          )}
        </>
      ) : (
        <>
          <div className={cn(["flex", "flex-col", "gap-2"])}>
            {challenge?.has_writeup && (
              <>
                <Button
                  variant={"solid"}
                  icon={<LightbulbIcon />}
                  onClick={() => {
                    setView("writeup");
                    setNoteId(null);
                  }}
                >
                  Official Writeup
                </Button>
                <Separator />
              </>
            )}
            {notes && notes.length > 0 ? (
              <div className={cn(["flex", "flex-col", "gap-2", "mx-1"])}>
                {notes.map((note) => (
                  <Button
                    key={note.id}
                    variant={"tonal"}
                    className={cn([
                      "h-auto",
                      "w-full",
                      "flex",
                      "gap-4",
                      "justify-between",
                    ])}
                    onClick={() => {
                      setView("note");
                      setNoteId(note.id);
                    }}
                  >
                    <div className={cn(["flex", "items-center", "gap-2"])}>
                      <Avatar
                        src={
                          note.user_has_avatar &&
                          `/api/users/${note.user_id}/avatar`
                        }
                        fallback={note.user_name?.charAt(0)}
                        className={cn(["size-8"])}
                      />
                      <span>{note.user_name}</span>
                      <span
                        className={cn([
                          "text-xs",
                          "text-muted-foreground",
                          "flex-1",
                          "max-w-xs",
                          "truncate",
                        ])}
                      >
                        {note.content?.slice(0, 100)}
                      </span>
                    </div>
                    <div
                      className={cn([
                        "flex",
                        "flex-col",
                        "gap-0",
                        "items-start",
                      ])}
                    >
                      <div
                        className={cn([
                          "flex",
                          "gap-1",
                          "items-center",
                          "text-muted-foreground",
                        ])}
                      >
                        <PencilLineIcon className={cn(["size-3!"])} />
                        <span className={cn(["text-xs"])}>
                          {new Date(
                            Number(note.updated_at) * 1000
                          ).toLocaleDateString()}
                        </span>
                      </div>
                    </div>
                  </Button>
                ))}
              </div>
            ) : (
              <span
                className={cn(["text-muted", "text-center", "select-none"])}
              >
                暂无笔记
              </span>
            )}
          </div>
          <div className={cn(["flex-1"])} />
          <Pagination
            size={"sm"}
            total={Math.ceil(total / size)}
            value={page}
            onChange={setPage}
            className={cn(["self-center"])}
          />
        </>
      )}
    </div>
  );
}

export { WriteupSection };
