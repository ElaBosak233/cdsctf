import { Outlet } from "react-router";

import { getConfigs, getVersion } from "@/api/configs";
import { getUserProfile } from "@/api/users/profile";
import { Navbar } from "@/components/widgets/navbar";
import { useAuthStore } from "@/storages/auth";
import { useConfigStore } from "@/storages/config";
import { cn } from "@/utils";
import { useQuery } from "@tanstack/react-query";
import { useEffect } from "react";

export default function () {
  const { setUser } = useAuthStore();
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

  const { data: profileData } = useQuery({
    queryKey: ["profile"],
    queryFn: getUserProfile,
    retry: false,
    select: (response) => response.data,
  });

  useEffect(() => {
    if (!configData) return;

    setConfig(configData);
  }, [configData, setConfig]);

  useEffect(() => {
    if (!versionData) return;

    setVersion(versionData);
  }, [versionData, setVersion]);

  useEffect(() => {
    if (!profileData) return;

    setUser(profileData);
  }, [profileData, setUser]);

  return (
    <>
      <Navbar />
      <main className={cn(["flex", "flex-col", "min-h-[calc(100vh-64px)]"])}>
        <Outlet />
      </main>
    </>
  );
}
