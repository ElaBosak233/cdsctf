import { keepPreviousData, useQuery } from "@tanstack/react-query";
import { StatusCodes } from "http-status-codes";
import { CheckIcon, MailIcon, MailPlusIcon, TrashIcon } from "lucide-react";
import { Fragment, useMemo, useState } from "react";
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
import { CreateEmailDialog } from "./create-dialog";
import { DeleteEmailDialog } from "./delete-dialog";

export default function Emails() {
  const { user_id } = useParams<{ user_id: string }>();
  const userId = Number(user_id);
  const sharedStore = useSharedStore();

  const [createDialogOpen, setCreateDialogOpen] = useState(false);
  const [deleteDialogOpen, setDeleteDialogOpen] = useState(false);
  const [deleteTarget, setDeleteTarget] = useState<string>();
  const [updatingEmail, setUpdatingEmail] = useState<string>();

  const { data: emails = [], refetch } = useQuery<Array<Email>>({
    queryKey: ["admin", "users", user_id, "emails", sharedStore.refresh],
    queryFn: async () => {
      if (!user_id) return [];
      const res = await getEmails({
        user_id: userId,
      });

      return res.data ?? [];
    },
    enabled: !!user_id,
    placeholderData: keepPreviousData,
  });

  const hasEmails = useMemo(() => emails.length > 0, [emails.length]);

  function handleRefresh() {
    void refetch();
    sharedStore.setRefresh();
  }

  function handleToggle(email: string, is_verified: boolean) {
    if (!user_id) return;
    setUpdatingEmail(email);
    updateEmail({
      user_id: userId,
      email,
      is_verified,
    })
      .then((res) => {
        if (res.code === StatusCodes.OK) {
          toast.success(`邮箱 ${email} 状态已更新`);
          handleRefresh();
        }
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
            邮箱管理
          </h1>
          <p className={cn(["text-muted-foreground", "text-sm"])}>
            添加、编辑该用户的绑定邮箱
          </p>
        </div>
        <div className={cn(["flex", "items-center", "gap-2"])}>
          <Button
            size={"sm"}
            variant={"solid"}
            icon={<MailPlusIcon />}
            onClick={() => setCreateDialogOpen(true)}
          >
            添加邮箱
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
                    {email.is_verified && (
                      <Badge className={cn(["bg-success/15", "text-success"])}>
                        <CheckIcon className={cn(["size-3.5"])} />
                        已验证
                      </Badge>
                    )}
                  </ItemTitle>
                  <ItemDescription>
                    {email.is_verified
                      ? "该邮箱已通过验证"
                      : "该邮箱尚未完成验证"}
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
                    <span>已验证</span>
                    <Switch
                      checked={email.is_verified}
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
          <div>该用户暂无绑定邮箱</div>
        </div>
      )}

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
    </div>
  );
}
