import { HTTPError } from "ky";
import { LoaderCircleIcon } from "lucide-react";
import { useEffect } from "react";
import { useNavigate, useParams, useSearchParams } from "react-router";
import { toast } from "sonner";
import { bindWithIdp, loginWithIdp } from "@/api/idps";
import { useAuthStore } from "@/storages/auth";
import { cn } from "@/utils";
import { parseRouteNumericId } from "@/utils/query";

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

  useEffect(() => {
    if (idpId == null) return;

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
  }, [idpId, searchParams, navigate]);

  return (
    <div className={cn(["flex", "flex-1", "items-center", "justify-center"])}>
      <LoaderCircleIcon
        className={cn(["size-8", "animate-spin", "text-primary"])}
      />
    </div>
  );
}
