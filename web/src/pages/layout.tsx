import { Outlet, useNavigate } from "react-router";

import { getConfigs, getVersion } from "@/api/configs";
import { Navbar } from "@/components/widgets/navbar";
import { useConfigStore } from "@/storages/config";
import { cn } from "@/utils";
import globalRouter from "@/utils/global-router";
import { useQuery } from "@tanstack/react-query";
import { useEffect } from "react";

export default function () {
  const navigate = useNavigate();
  globalRouter.navigate = navigate;
  const { setConfig, setVersion } = useConfigStore();

  const { data: configData } = useQuery({
    queryKey: ["configs"],
    queryFn: getConfigs,
    select: (response) => response.data,
  });

  const { data: versionData } = useQuery({
    queryKey: ["version"],
    queryFn: getVersion,
    select: (response) => response.data,
  });

  useEffect(() => {
    if (configData) {
      setConfig(configData);
    }
  }, [configData, setConfig]);

  useEffect(() => {
    if (versionData) {
      setVersion(versionData);
    }
  }, [versionData, setVersion]);

  return (
    <>
      <Navbar />
      <main className={cn(["flex", "flex-col", "min-h-[calc(100vh-64px)]"])}>
        <Outlet />
      </main>
    </>
  );
}
