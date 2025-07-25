import { EyeIcon } from "lucide-react";
import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";
import { Dialog, DialogContent, DialogTrigger } from "@/components/ui/dialog";
import { Typography } from "@/components/ui/typography";
import { cn } from "@/utils";
import { MarkdownRender } from "../utils/markdown-render";

type ContentDialogProps = {
  title: string;
  content: string;
  triggerText?: string;
  maxPreviewLength?: number;
  showPreview?: boolean;
};

export function ContentDialog({
  content,
  maxPreviewLength = 10,
  showPreview = true,
}: ContentDialogProps) {
  const contentString =
    typeof content === "string" ? content : JSON.stringify(content);
  const preview =
    contentString.length > maxPreviewLength
      ? `${contentString.substring(0, maxPreviewLength)}...`
      : contentString;

  return (
    <div className="flex items-center">
      {showPreview && <span className="truncate max-w-xs">{preview}</span>}

      <Dialog>
        <DialogTrigger>
          <Button
            variant="ghost"
            size="sm"
            className={showPreview ? "ml-2 h-8 px-2" : "h-8 w-8 p-0"}
          >
            <EyeIcon className="h-4 w-4" />
          </Button>
        </DialogTrigger>
        <DialogContent>
          <Card className={cn(["sm:max-w-2xl", "p-5", "w-128", "min-h-64"])}>
            <Typography>
              <MarkdownRender src={content} />
            </Typography>
          </Card>
        </DialogContent>
      </Dialog>
    </div>
  );
}
