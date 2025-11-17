import {
  CheckIcon,
  MailCheckIcon,
  MailPlusIcon,
  TrashIcon,
} from "lucide-react";
import React, { useEffect, useState } from "react";
import { useTranslation } from "react-i18next";
import { getEmails } from "@/api/users/profile/emails";
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
import type { Email } from "@/models/email";
import { useConfigStore } from "@/storages/config";
import { cn } from "@/utils";
import { CreateDialog } from "./create-dialog";
import { DeleteDialog } from "./delete-dialog";
import { VerifyDialog } from "./verify-dialog";

export default function Index() {
  const { t } = useTranslation();

  const configStore = useConfigStore();
  const { tick, bump } = useRefresh();

  const [emails, setEmails] = useState<Array<Email>>();

  const [createDialogOpen, setCreateDialogOpen] = useState<boolean>(false);

  const [verifyDialogOpen, setVerifyDialogOpen] = useState<boolean>(false);
  const [verifyEmail, setVerifyEmail] = useState<string>("");

  const [deleteDialogOpen, setDeleteDialogOpen] = useState<boolean>(false);
  const [deleteEmail, setDeleteEmail] = useState<string>("");

  useEffect(() => {
    void tick;

    getEmails().then((res) => {
      setEmails(res.data);
    });
  }, [tick]);

  return (
    <>
      <title>{`${t("user.settings.email")} - ${configStore?.config?.meta?.title}`}</title>
      <div
        className={cn([
          "flex",
          "flex-col",
          "flex-1",
          "p-10",
          "xl:mx-50",
          "lg:mx-30",
          "gap-8",
        ])}
      >
        <div className={cn(["flex", "justify-end"])}>
          <Button
            size={"md"}
            variant={"solid"}
            icon={<MailPlusIcon />}
            onClick={() => setCreateDialogOpen(true)}
          >
            {t("common.actions.add")}
          </Button>
        </div>
        <Separator />
        <ItemGroup>
          {emails?.map((email, index) => (
            <React.Fragment key={email.email}>
              <Item>
                <ItemContent className="gap-1">
                  <ItemTitle>{email.email}</ItemTitle>
                  <ItemDescription>
                    {email.is_verified ? (
                      <div className={cn(["flex", "gap-1", "items-center"])}>
                        <span>{t("user.emails.is_verified.true._")}</span>
                        <CheckIcon className={cn(["size-4", "text-success"])} />
                      </div>
                    ) : (
                      t("user.emails.is_verified.false._")
                    )}
                  </ItemDescription>
                </ItemContent>
                <ItemActions>
                  <Button
                    variant="ghost"
                    size={"sm"}
                    square
                    disabled={email.is_verified}
                    onClick={() => {
                      setVerifyEmail(email.email);
                      setVerifyDialogOpen(true);
                    }}
                  >
                    <MailCheckIcon />
                  </Button>
                  <Button
                    variant="ghost"
                    size={"sm"}
                    square
                    onClick={() => {
                      setDeleteEmail(email.email);
                      setDeleteDialogOpen(true);
                    }}
                  >
                    <TrashIcon className={cn(["text-error"])} />
                  </Button>
                </ItemActions>
              </Item>
              {index !== emails.length - 1 && <ItemSeparator />}
            </React.Fragment>
          ))}
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
