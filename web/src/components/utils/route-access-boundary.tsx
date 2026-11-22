import {
  LoaderCircleIcon,
  ShieldQuestionIcon,
  ShieldXIcon,
} from "lucide-react";
import type { PropsWithChildren } from "react";
import { useTranslation } from "react-i18next";
import { Link, Navigate, useLocation, useMatches } from "react-router";
import { Button } from "@/components/ui/button";
import { useSession } from "@/hooks/use-session";
import {
  BASIC_PERMISSION,
  hasPermission,
  type Permission,
} from "@/models/user";
import { useAuthStore } from "@/storages/auth";
import { cn } from "@/utils";
import {
  emailSettingsPath,
  getLocationTarget,
  getLoginUrl,
} from "@/utils/redirect";

export type AccessPolicy = {
  authenticated?: boolean;
  permissions?: Permission[];
  anyPermissions?: Permission[];
};

type RouteHandle = {
  access?: AccessPolicy;
};

function RouteAccessBoundary({ children }: PropsWithChildren) {
  const { t } = useTranslation();
  const location = useLocation();
  const matches = useMatches();
  useSession();
  const { status, user } = useAuthStore();

  const policy = matches.reduce<AccessPolicy>((result, match) => {
    const access = (match.handle as RouteHandle | undefined)?.access;
    const permissions = access?.permissions ?? [];
    const anyPermissions = access?.anyPermissions ?? [];

    return {
      authenticated: result.authenticated || access?.authenticated,
      permissions: [
        ...new Set([...(result.permissions ?? []), ...permissions]),
      ],
      anyPermissions: [
        ...new Set([...(result.anyPermissions ?? []), ...anyPermissions]),
      ],
    };
  }, {});

  const protectedRoute =
    policy.authenticated ||
    (policy.permissions?.length ?? 0) > 0 ||
    (policy.anyPermissions?.length ?? 0) > 0;

  if (protectedRoute && status === "checking") {
    return (
      <div className={cn(["flex", "flex-1", "items-center", "justify-center"])}>
        <LoaderCircleIcon className={cn(["size-8", "animate-spin"])} />
      </div>
    );
  }

  if (protectedRoute && status === "unavailable") {
    return (
      <div
        className={cn([
          "flex",
          "flex-1",
          "flex-col",
          "items-center",
          "justify-center",
          "gap-4",
          "px-6",
          "text-center",
          "select-none",
        ])}
      >
        <ShieldQuestionIcon className="size-16 text-muted-foreground" />
        <p className="text-sm text-muted-foreground">
          {t("account:guard.unavailable")}
        </p>
      </div>
    );
  }

  if (protectedRoute && !user) {
    return <Navigate replace to={getLoginUrl(getLocationTarget(location))} />;
  }

  const permissionForbidden = (policy.permissions ?? []).some(
    (permission) => !hasPermission(user?.permissions, permission)
  );
  const anyPermissionForbidden =
    (policy.anyPermissions?.length ?? 0) > 0 &&
    !(policy.anyPermissions ?? []).some((permission) =>
      hasPermission(user?.permissions, permission)
    );

  if (permissionForbidden || anyPermissionForbidden) {
    const needsBasic =
      policy.permissions?.includes(BASIC_PERMISSION) &&
      !hasPermission(user?.permissions, BASIC_PERMISSION);
    return (
      <div
        className={cn([
          "flex",
          "flex-1",
          "flex-col",
          "items-center",
          "justify-center",
          "gap-4",
          "px-6",
          "text-center",
          "select-none",
        ])}
      >
        <ShieldXIcon className={cn(["size-16"])} strokeWidth={1.2} />
        <p className="text-sm text-muted-foreground">
          {t(
            needsBasic
              ? "account:guard.basic_required"
              : "account:guard.forbidden"
          )}
        </p>
        {needsBasic && (
          <Button
            size="sm"
            variant="outline"
            render={<Link to={emailSettingsPath} />}
          >
            {t("user:settings.email")}
          </Button>
        )}
      </div>
    );
  }

  return children;
}

export { RouteAccessBoundary };
