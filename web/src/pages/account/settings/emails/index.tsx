import { useQuery } from "@tanstack/react-query";
import {
  CheckIcon,
  MailCheckIcon,
  MailPlusIcon,
  MailsIcon,
  TrashIcon,
} from "lucide-react";
import React, { useState } from "react";
import { useTranslation } from "react-i18next";
import { getEmails } from "@/api/users/me/emails";
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
import { useRefresh } from "@/hooks/use-refresh";
import type { EmailView } from "@/models/email";
import { BASIC_PERMISSION, hasPermission } from "@/models/user";
import { useAuthStore } from "@/storages/auth";
import { useConfigStore } from "@/storages/config";
import { cn } from "@/utils";

import { CreateDialog } from "./_blocks/create-dialog";
import { DeleteDialog } from "./_blocks/delete-dialog";
import { VerifyDialog } from "./_blocks/verify-dialog";

export default function Index() {
  const { t } = useTranslation();

  const configStore = useConfigStore();
  const emailEnabled = configStore.config?.email?.enabled === true;
  const hasBasic = useAuthStore((state) =>
    hasPermission(state.user?.permissions, BASIC_PERMISSION)
  );
  const { tick, bump } = useRefresh();

  const [createDialogOpen, setCreateDialogOpen] = useState<boolean>(false);

  const [verifyDialogOpen, setVerifyDialogOpen] = useState<boolean>(false);
  const [verifyEmail, setVerifyEmail] = useState<string>("");

  const [deleteDialogOpen, setDeleteDialogOpen] = useState<boolean>(false);
  const [deleteEmail, setDeleteEmail] = useState<string>("");

  const { data: emails = [] } = useQuery<Array<EmailView>>({
    queryKey: ["account", "emails", tick],
    queryFn: async () => (await getEmails()).emails ?? [],
  });

  const verifiedCount = emails.filter((email) => email.verified).length;

  return (
    <>
      <title>{`${t("user:settings.email")} - ${configStore?.config?.meta?.title}`}</title>
      <div
        className={cn([
          "flex",
          "flex-col",
          "flex-1",
          "p-4",
          "sm:p-6",
          "lg:p-10",
          "xl:mx-50",
          "lg:mx-30",
          "gap-5",
        ])}
      >
        <div className={cn(["flex", "justify-between", "items-center"])}>
          <div className={cn(["flex", "items-center", "gap-3"])}>
            <div
              className={cn([
                "flex items-center justify-center",
                "size-10 rounded-badge",
                "bg-primary/10",
                "shrink-0",
              ])}
            >
              <MailsIcon className={cn(["size-5"])} />
            </div>
            <h2 className={cn(["text-base", "font-semibold"])}>
              {t("user:settings.email")}
            </h2>
          </div>
          <Button
            size={"md"}
            variant={"solid"}
            icon={<MailPlusIcon />}
            onClick={() => setCreateDialogOpen(true)}
          >
            {t("common:actions.add")}
          </Button>
        </div>
        <Separator />
        {!hasBasic && (
          <p className="text-sm text-muted-foreground">
            {t(
              emailEnabled && verifiedCount === 0
                ? "user:emails.basic_verification"
                : "user:emails.basic_contact_admin"
            )}
          </p>
        )}
        <ItemGroup
          className={cn([
            "overflow-hidden",
            "rounded-xl",
            "border",
            "bg-card",
            "text-card-foreground",
          ])}
        >
          {emails.map((email, index) => {
            const isLastVerified = email.verified && verifiedCount === 1;
            return (
              <React.Fragment key={email.email}>
                <Item className={cn(["bg-card", "text-card-foreground"])}>
                  <ItemContent className="min-w-0 gap-1">
                    <ItemTitle className="break-all">{email.email}</ItemTitle>
                    <ItemDescription>
                      {isLastVerified ? (
                        t("user:emails.last_verified.status")
                      ) : email.verified ? (
                        <span className={cn(["flex", "gap-1", "items-center"])}>
                          <span>{t("user:emails.verified.true._")}</span>
                          <CheckIcon
                            className={cn(["size-4", "text-success"])}
                          />
                        </span>
                      ) : (
                        t("user:emails.verified.false._")
                      )}
                    </ItemDescription>
                  </ItemContent>
                  <ItemActions>
                    {emailEnabled && !email.verified && (
                      <Button
                        variant="ghost"
                        size="sm"
                        icon={<MailCheckIcon />}
                        onClick={() => {
                          setVerifyEmail(email.email);
                          setVerifyDialogOpen(true);
                        }}
                      >
                        {t("user:emails.actions.verify._")}
                      </Button>
                    )}
                    <Button
                      level={"error"}
                      variant="ghost"
                      size={"sm"}
                      square
                      icon={<TrashIcon />}
                      aria-label={t("user:emails.actions.delete._")}
                      disabled={isLastVerified}
                      onClick={() => {
                        setDeleteEmail(email.email);
                        setDeleteDialogOpen(true);
                      }}
                    />
                  </ItemActions>
                </Item>
                {index !== emails.length - 1 && <ItemSeparator />}
              </React.Fragment>
            );
          })}
        </ItemGroup>
      </div>
      <Dialog open={createDialogOpen} onOpenChange={setCreateDialogOpen}>
        <DialogContent>
          <CreateDialog
            onClose={() => setCreateDialogOpen(false)}
            bump={bump}
          />
        </DialogContent>
      </Dialog>
      <Dialog open={verifyDialogOpen} onOpenChange={setVerifyDialogOpen}>
        <DialogContent>
          <VerifyDialog
            key={verifyEmail}
            email={verifyEmail}
            onClose={() => setVerifyDialogOpen(false)}
            bump={bump}
          />
        </DialogContent>
      </Dialog>
      <Dialog open={deleteDialogOpen} onOpenChange={setDeleteDialogOpen}>
        <DialogContent>
          <DeleteDialog
            email={deleteEmail}
            onClose={() => setDeleteDialogOpen(false)}
            bump={bump}
          />
        </DialogContent>
      </Dialog>
    </>
  );
}
