import { keepPreviousData, useQuery } from "@tanstack/react-query";
import { CheckIcon, MailIcon, MailPlusIcon, TrashIcon } from "lucide-react";
import { Fragment, useMemo, useState } from "react";
import { useTranslation } from "react-i18next";
import { useParams } from "react-router";
import { toast } from "sonner";
import { getEmails, updateEmail } from "@/api/admin/users/user_id/emails";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Dialog, DialogContent } from "@/components/ui/dialog";
import {
  Item,
  ItemActions,
  ItemContent,
  ItemDescription,
  ItemGroup,
  ItemSeparator,
  ItemTitle,
} from "@/components/ui/item";
import { Separator } from "@/components/ui/separator";
import { Switch } from "@/components/ui/switch";
import type { Email } from "@/models/email";
import { useSharedStore } from "@/storages/shared";
import { cn } from "@/utils";
import { parseRouteNumericId } from "@/utils/query";
import { CreateEmailDialog } from "./_blocks/create-dialog";
import { DeleteEmailDialog } from "./_blocks/delete-dialog";

export default function Emails() {
  const { t } = useTranslation();

  const { user_id } = useParams<{ user_id: string }>();
  const userId = parseRouteNumericId(user_id);
  const sharedStore = useSharedStore();

  const [createDialogOpen, setCreateDialogOpen] = useState(false);
  const [deleteDialogOpen, setDeleteDialogOpen] = useState(false);
  const [deleteTarget, setDeleteTarget] = useState<string>();
  const [updatingEmail, setUpdatingEmail] = useState<string>();

  const { data: emails = [], refetch } = useQuery<Array<Email>>({
    queryKey: ["admin", "users", userId, "emails", sharedStore.refresh],
    queryFn: async () => {
      const res = await getEmails({
        user_id: userId!,
      });

      return res.emails ?? [];
    },
    enabled: userId != null,
    placeholderData: keepPreviousData,
  });

  const hasEmails = useMemo(() => emails.length > 0, [emails.length]);

  function handleRefresh() {
    void refetch();
    sharedStore.setRefresh();
  }

  function handleToggle(email: string, verified: boolean) {
    if (userId == null) return;
    setUpdatingEmail(email);
    updateEmail({
      user_id: userId,
      email,
      verified,
    })
      .then(() => {
        toast.success(t("user:emails.actions.update.success", { email }));
        handleRefresh();
      })
      .finally(() => setUpdatingEmail(undefined));
  }

  return (
    <div className={cn(["flex", "flex-col", "gap-8", "flex-1"])}>
      <div
        className={cn([
          "flex",
          "gap-2",
          "flex-row",
          "items-center",
          "justify-between",
        ])}
      >
        <div className={cn(["space-y-1"])}>
          <h1
            className={cn([
              "text-2xl",
              "font-bold",
              "flex",
              "gap-2",
              "items-center",
            ])}
          >
            <MailIcon />
            {t("user:emails._")}
          </h1>
          <p className={cn(["text-muted-foreground", "text-sm"])}>
            {t("user:emails.brief")}
          </p>
        </div>
        <div className={cn(["flex", "items-center", "gap-2"])}>
          <Button
            size={"sm"}
            variant={"solid"}
            icon={<MailPlusIcon />}
            disabled={userId == null}
            onClick={() => setCreateDialogOpen(true)}
          >
            {t("common:actions.add")}
          </Button>
        </div>
      </div>
      <Separator />
      {hasEmails ? (
        <ItemGroup className={cn(["rounded-xl", "border"])}>
          {emails.map((email, index) => (
            <Fragment key={email.email}>
              <Item
                className={cn([
                  "flex",
                  "flex-col",
                  "gap-4",
                  "md:flex-row",
                  "md:items-center",
                ])}
              >
                <ItemContent>
                  <ItemTitle className={cn(["text-base"])}>
                    {email.email}
                    {email.verified && (
                      <Badge className={cn(["bg-success/15", "text-success"])}>
                        <CheckIcon className={cn(["size-3.5"])} />
                        {t("user:emails.verified.true._")}
                      </Badge>
                    )}
                  </ItemTitle>
                  <ItemDescription>
                    {email.verified
                      ? t("user:emails.verified.true.long")
                      : t("user:emails.verified.false.long")}
                  </ItemDescription>
                </ItemContent>
                <ItemActions className={cn(["flex", "flex-wrap", "gap-3"])}>
                  <div
                    className={cn([
                      "flex",
                      "items-center",
                      "gap-2",
                      "text-sm",
                      "text-muted-foreground",
                    ])}
                  >
                    <span>{t("user:emails.verified.true._")}</span>
                    <Switch
                      checked={email.verified}
                      onCheckedChange={(checked) =>
                        handleToggle(email.email, checked === true)
                      }
                      disabled={updatingEmail === email.email}
                    />
                  </div>
                  <Button
                    variant={"ghost"}
                    size={"sm"}
                    level={"error"}
                    square
                    icon={<TrashIcon />}
                    onClick={() => {
                      setDeleteTarget(email.email);
                      setDeleteDialogOpen(true);
                    }}
                  />
                </ItemActions>
              </Item>
              {index !== emails.length - 1 && <ItemSeparator />}
            </Fragment>
          ))}
        </ItemGroup>
      ) : (
        <div
          className={cn([
            "flex",
            "flex-col",
            "items-center",
            "justify-center",
            "gap-2",
            "py-20",
            "text-muted-foreground",
          ])}
        >
          <MailIcon className={cn(["size-8"])} />
          <div>{t("user:emails.empty")}</div>
        </div>
      )}

      {userId != null && (
        <>
          <Dialog open={createDialogOpen} onOpenChange={setCreateDialogOpen}>
            <DialogContent
              className={cn(["border-none", "bg-transparent", "shadow-none"])}
            >
              <CreateEmailDialog
                userId={userId}
                onClose={() => setCreateDialogOpen(false)}
                onSuccess={handleRefresh}
              />
            </DialogContent>
          </Dialog>

          <Dialog open={deleteDialogOpen} onOpenChange={setDeleteDialogOpen}>
            <DialogContent
              className={cn(["border-none", "bg-transparent", "shadow-none"])}
            >
              <DeleteEmailDialog
                userId={userId}
                email={deleteTarget}
                onClose={() => setDeleteDialogOpen(false)}
                onSuccess={handleRefresh}
              />
            </DialogContent>
          </Dialog>
        </>
      )}
    </div>
  );
}
