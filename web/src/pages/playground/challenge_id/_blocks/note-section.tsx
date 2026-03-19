import { zodResolver } from "@hookform/resolvers/zod";
import { useQuery } from "@tanstack/react-query";
import { StatusCodes } from "http-status-codes";
import {
  EyeIcon,
  MegaphoneIcon,
  MegaphoneOffIcon,
  PencilLineIcon,
  SaveIcon,
} from "lucide-react";
import { useContext, useEffect, useState } from "react";
import { useForm } from "react-hook-form";
import { useTranslation } from "react-i18next";
import { toast } from "sonner";
import z from "zod";
import { getMyNotes, saveMyNote } from "@/api/users/me/notes";
import { Button } from "@/components/ui/button";
import { MarkdownEditor } from "@/components/ui/markdown-editor";
import { MarkdownRender } from "@/components/ui/markdown-render";
import { ScrollArea } from "@/components/ui/scroll-area";
import { Typography } from "@/components/ui/typography";
import { useAuthStore } from "@/storages/auth";
import { cn } from "@/utils";
import { Context } from "../context";

function useNoteQuery(userId?: number, challengeId?: number) {
  return useQuery({
    queryKey: ["note", `user_id=${userId}`, `challenge_id=${challengeId}`],
    queryFn: () => getMyNotes({ challenge_id: challengeId }),
    select: (response) => response.data,
    enabled: !!challengeId && !!userId,
  });
}

function NoteSection() {
  const { t } = useTranslation();
  const authStore = useAuthStore();
  const { challenge } = useContext(Context);

  const { data: notes } = useNoteQuery(authStore?.user?.id || 0, challenge?.id);

  const note = notes && notes.length > 0 ? notes[0] : null;

  const formSchema = z.object({
    content: z.string(),
    public: z.boolean(),
  });

  const form = useForm<z.infer<typeof formSchema>>({
    resolver: zodResolver(formSchema),
    defaultValues: {
      content: "",
      public: false,
    },
  });

  useEffect(() => {
    form.reset(note || { content: "", public: false }, {
      keepDefaultValues: false,
    });
  }, [note, form]);

  const [mode, setMode] = useState<"view" | "edit">("edit");

  async function handleSaveMyNote() {
    const res = await saveMyNote({
      content: form.getValues("content"),
      challenge_id: Number(challenge?.id),
      public: form.getValues("public"),
    });

    if (res.code === StatusCodes.OK) {
      toast.success(t("challenge:note_save_success"));
    }
  }

  return (
    <div className={cn(["flex-1", "min-h-0", "p-4", "relative"])}>
      {mode === "edit" && (
        <MarkdownEditor
          className={cn(["h-full"])}
          placeholder={t("challenge:note_placeholder")}
          value={form.getValues("content")}
          onChange={(value) => form.setValue("content", value)}
        />
      )}
      {mode === "view" && (
        <ScrollArea className={cn(["h-full", "min-h-0", "overflow-hidden"])}>
          <Typography className={cn(["p-5"])}>
            <MarkdownRender src={form.getValues("content")} />
          </Typography>
        </ScrollArea>
      )}
      <div className={cn(["absolute", "top-5", "right-5"])}>
        <div className={cn(["flex", "gap-1"])}>
          {mode === "edit" && (
            <>
              <Button
                size={"sm"}
                square
                icon={
                  form.watch("public") ? (
                    <MegaphoneOffIcon />
                  ) : (
                    <MegaphoneIcon />
                  )
                }
                onClick={() =>
                  form.setValue("public", !form.getValues("public"))
                }
              />
              <Button
                size={"sm"}
                square
                icon={<SaveIcon />}
                onClick={handleSaveMyNote}
              />
            </>
          )}
          <Button
            size={"sm"}
            square
            icon={mode === "edit" ? <EyeIcon /> : <PencilLineIcon />}
            onClick={() => setMode(mode === "edit" ? "view" : "edit")}
          />
        </div>
      </div>
    </div>
  );
}

export { NoteSection };
