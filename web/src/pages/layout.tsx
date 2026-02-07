import { useQuery } from "@tanstack/react-query";
import { useEffect } from "react";
import { Outlet } from "react-router";
import { getConfigs, getVersion } from "@/api/configs";
import { getUserProfile } from "@/api/users/me";
import { useAuthStore } from "@/storages/auth";
import { useConfigStore } from "@/storages/config";
import { cn, stripIndent } from "@/utils";
import { Background } from "./_blocks/background";
import { Navbar } from "./_blocks/navbar";

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

  useEffect(() => {
    if (!versionData?.tag) return;

    console.log(
      stripIndent(`\n
        %cCds%cCTF %cv%c${versionData?.tag}\n
        %cOriginally developed by ElaBosak233.\n
        %cAttacks on the platform are generally not part of CTF challenges.\n
        %cIf you run into any issues, please consider opening an issue on GitHub: https://github.com/elabosak233/cdsctf.
        `),
      "color: #44b2fc; font-weight: bold; font-size: 1.75rem; font-style: italic; font-family: consolas;",
      "color: #ffda5c; font-weight: bold; font-size: 1.75rem; font-style: italic; font-family: consolas;",
      "color: #44b2fc",
      "color: currentColor",
      "color: #ababab; font-wight: semibold; font-size: 0.9em; font-style: italic;",
      "color: #d96a42",
      "color: currentColor;"
    );
  }, [versionData]);

  return (
    <div className={cn(["flex", "flex-col", "min-h-screen"])}>
      <Background />
      <Navbar />
      <div className={cn(["min-h-(--app-content-height)", "flex", "flex-col"])}>
        <Outlet />
      </div>
    </div>
  );
}
