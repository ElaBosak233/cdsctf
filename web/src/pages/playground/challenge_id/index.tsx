import { useQuery } from "@tanstack/react-query";
import { motion } from "framer-motion";
import { ChevronLeftIcon, LightbulbIcon, LightbulbOffIcon } from "lucide-react";
import { useQueryState } from "nuqs";
import { useMemo, useState } from "react";
import { useNavigate, useParams } from "react-router";
import { getChallenge } from "@/api/challenges/challenge_id";
import { Button } from "@/components/ui/button";
import { LoadingOverlay } from "@/components/ui/loading-overlay";
import { MarkdownRender } from "@/components/ui/markdown-render";
import {
  ResizableHandle,
  ResizablePanel,
  ResizablePanelGroup,
} from "@/components/ui/resizable";
import { ScrollArea } from "@/components/ui/scroll-area";
import { Separator } from "@/components/ui/separator";
import { Typography } from "@/components/ui/typography";
import { AttachmentSection } from "@/components/widgets/challenge-dialog/attachment-section";
import { Context as ChallengeDialogContext } from "@/components/widgets/challenge-dialog/context";
import { InstanceSection } from "@/components/widgets/challenge-dialog/instance-section";
import { SubmitSection } from "@/components/widgets/challenge-dialog/submit-section";
import { useConfigStore } from "@/storages/config";
import { cn } from "@/utils";
import { getCategory } from "@/utils/category";
import { NoteSection } from "./_blocks/note-section";
import { WriteupSection } from "./_blocks/writeup-section";
import { Context } from "./context";

function useChallengeQuery(challengeId?: number) {
  return useQuery({
    queryKey: ["challenge", challengeId],
    queryFn: () => getChallenge({ id: challengeId! }),
    select: (response) => response.challenge,
    enabled: !!challengeId,
  });
}

export default function Index() {
  const { config } = useConfigStore();
  const { challenge_id } = useParams<{ challenge_id: string }>();
  const navigate = useNavigate();
  const [isExiting, setIsExiting] = useState(false);

  const { data: challenge, isLoading } = useChallengeQuery(
    Number(challenge_id)
  );

  const category = useMemo(
    () => getCategory(challenge?.category || 1),
    [challenge?.category]
  );
  const CategoryIcon = category.icon!;

  const [display, setDisplay] = useQueryState("display", {
    defaultValue: "description",
  });

  const content =
    display === "description" ? challenge?.description : challenge?.writeup;

  const problemHeader = (
    <div className={cn(["flex", "items-center", "justify-between", "gap-3"])}>
      <div className={cn(["flex", "gap-3", "items-center", "flex-wrap"])}>
        <Button
          size={"sm"}
          square
          onClick={() => {
            setIsExiting(true);
            setTimeout(() => {
              navigate("/playground", { replace: true });
            }, 220);
          }}
        >
          <ChevronLeftIcon />
        </Button>

        <CategoryIcon color={category?.color} className={cn(["size-5"])} />
        <h2 className={cn(["text-lg", "font-semibold"])}>
          {challenge?.title || "-"}
        </h2>
      </div>
      <Button
        size={"sm"}
        square
        icon={
          display === "description" ? <LightbulbIcon /> : <LightbulbOffIcon />
        }
        onClick={() =>
          setDisplay((prev) =>
            prev === "description" ? "writeup" : "description"
          )
        }
      />
    </div>
  );

  const problemBody = (
    <div className={cn(["relative", "flex", "flex-col", "flex-1", "min-h-0"])}>
      <LoadingOverlay loading={isLoading} />
      {display === "description" ? (
        <div className={cn(["flex-1", "min-h-0", "flex", "flex-col"])}>
          <ScrollArea className={cn(["h-full", "min-h-0", "overflow-hidden"])}>
            <div className={cn(["space-y-4", "pr-3", "pb-6"])}>
              <Typography>
                <MarkdownRender src={content} />
              </Typography>
            </div>
          </ScrollArea>
        </div>
      ) : (
        <WriteupSection />
      )}
    </div>
  );

  return (
    <>
      <title>{`${challenge?.title} - ${config?.meta?.title}`}</title>
      <Context.Provider value={{ challenge }}>
        <div
          className={cn([
            "flex",
            "flex-col",
            "min-h-0",
            "overflow-hidden",
            "h-(--app-content-height)",
          ])}
        >
          <motion.div
            className={cn([
              "flex",
              "flex-col",
              "gap-4",
              "p-4",
              "md:p-6",
              "min-h-0",
              "flex-1",
              "overflow-hidden",
            ])}
            initial={{ opacity: 0, y: 8 }}
            animate={isExiting ? { opacity: 0, y: 8 } : { opacity: 1, y: 0 }}
            transition={{ duration: 0.2, ease: "easeInOut" }}
          >
            <div className={cn(["md:hidden", "flex", "flex-1", "min-h-0"])}>
              <div
                className={cn([
                  "flex",
                  "flex-col",
                  "w-full",
                  "gap-4",
                  "flex-1",
                  "min-h-0",
                ])}
              >
                {problemHeader}
                <Separator />
                {problemBody}
              </div>
            </div>

            <ChallengeDialogContext.Provider
              value={{ challenge, team: undefined, debug: false }}
            >
              <div
                className={cn([
                  "hidden",
                  "md:flex",
                  "flex-1",
                  "min-h-0",
                  "flex-col",
                ])}
              >
                <ResizablePanelGroup
                  orientation="horizontal"
                  className={cn([
                    "flex-1",
                    "min-h-0",
                    "rounded-lg",
                    "border",
                    "bg-card",
                  ])}
                >
                  <ResizablePanel defaultSize={"30%"} minSize={"30%"}>
                    <div
                      className={cn(["flex", "flex-col", "h-full", "min-h-0"])}
                    >
                      <div className={cn(["p-4", "space-y-3"])}>
                        {problemHeader}
                        <Separator />
                      </div>
                      <div
                        className={cn([
                          "flex-1",
                          "min-h-0",
                          "px-4",
                          "pb-4",
                          "flex",
                          "flex-col",
                          "gap-4",
                        ])}
                      >
                        {problemBody}
                        {!!challenge && (
                          <>
                            {challenge?.has_attachment && <AttachmentSection />}
                            {challenge?.has_instance && <InstanceSection />}
                            <div className={cn(["flex", "flex-col", "gap-3"])}>
                              <Separator />
                              <SubmitSection />
                            </div>
                          </>
                        )}
                      </div>
                    </div>
                  </ResizablePanel>
                  <ResizableHandle withHandle />
                  <ResizablePanel defaultSize={"45%"} minSize={"25%"}>
                    <div
                      className={cn(["flex", "flex-col", "h-full", "min-h-0"])}
                    >
                      <NoteSection />
                    </div>
                  </ResizablePanel>
                </ResizablePanelGroup>
              </div>
            </ChallengeDialogContext.Provider>
          </motion.div>
        </div>
      </Context.Provider>
    </>
  );
}
