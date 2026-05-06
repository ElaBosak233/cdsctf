import { HTTPError } from "ky";
import { ChevronsLeftRightEllipsisIcon, LoaderCircleIcon } from "lucide-react";
import { useEffect } from "react";
import { useNavigate, useParams, useSearchParams } from "react-router";
import { toast } from "sonner";
import { useQuery } from "@tanstack/react-query";
import { Avatar } from "@/components/ui/avatar";
import { bindWithIdp, getIdp, loginWithIdp } from "@/api/idps";
import { useAuthStore } from "@/storages/auth";
import { cn } from "@/utils";
import { parseRouteNumericId } from "@/utils/query";
import { DefaultLogo } from "@/components/widgets/default-logo";

function searchParamsToRecord(searchParams: URLSearchParams) {
  const params: Record<string, string> = {};
  for (const [key, value] of searchParams.entries()) {
    if (key !== "redirect") params[key] = value;
  }
  return params;
}

export default function Index() {
  const { idp_id } = useParams();
  const idpId = parseRouteNumericId(idp_id);
  const [searchParams] = useSearchParams();
  const navigate = useNavigate();

  const { data: idp, isLoading } = useQuery({
    queryKey: ["idp", idpId],
    queryFn: () => getIdp(idpId!).then((res) => res.idp),
    enabled: idpId != null,
  });

  useEffect(() => {
    if (idpId == null || !idp) return;

    const resolvedIdpId = idpId;
    const redirect = searchParams.get("redirect") || "/";
    const params = searchParamsToRecord(searchParams);
    const user = useAuthStore.getState().user;

    async function run() {
      try {
        if (user) {
          await bindWithIdp(resolvedIdpId, { params });
          toast.success("IdP bound");
          navigate("/account/settings/idp", { replace: true });
          return;
        }

        const res = await loginWithIdp(resolvedIdpId, { params });
        useAuthStore.getState().setUser(res.user);
        toast.success("Signed in");
        navigate(redirect, { replace: true });
      } catch (error) {
        if (error instanceof HTTPError) {
          toast.error("IdP request failed");
          navigate(user ? "/account/settings/idp" : "/account/login", {
            replace: true,
          });
        } else {
          throw error;
        }
      }
    }

    run();
  }, [idpId, idp, searchParams, navigate]);

  return (
    <div className={cn(["flex", "flex-col", "flex-1", "items-center", "justify-center", "gap-5"])}>
      {idp && (
        <div className={cn(["flex", "items-center", "gap-4"])}>
          <Avatar
            square
            className={cn(["size-24", "bg-transparent"])}
            src={idp.avatar_hash && `/api/media?hash=${idp.avatar_hash}`}
            fallback={idp.name?.charAt(0)}
          />
        </div>
      )}
      <LoaderCircleIcon
        className={cn(["size-8", "animate-spin", "text-primary"])}
      />
    </div>
  );
}
