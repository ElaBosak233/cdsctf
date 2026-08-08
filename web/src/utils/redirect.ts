import type { Location } from "react-router";

const IDP_REDIRECT_KEY = "idp_auth_redirect";

export function getLocationTarget(
  location: Pick<Location, "pathname" | "search" | "hash">
) {
  return `${location.pathname}${location.search}${location.hash}`;
}

export function getLoginTarget(
  location: Pick<Location, "pathname" | "search" | "hash">
) {
  if (location.pathname !== "/account/login") {
    return getLocationTarget(location);
  }

  return getSafeRedirect(new URLSearchParams(location.search).get("redirect"));
}

export function getSafeRedirect(value: string | null | undefined) {
  if (!value?.startsWith("/") || value.startsWith("//")) return "/";
  if (value.includes("\\") || value.includes("://")) return "/";

  return value;
}

export function getLoginUrl(target: string) {
  return `/account/login?redirect=${encodeURIComponent(target)}`;
}

export function withRedirect(path: string, target: string) {
  if (target === "/") return path;

  return `${path}?redirect=${encodeURIComponent(target)}`;
}

export function rememberIdpRedirect(target: string) {
  const redirect = getSafeRedirect(target);

  if (redirect === "/") {
    sessionStorage.removeItem(IDP_REDIRECT_KEY);
    return;
  }

  sessionStorage.setItem(IDP_REDIRECT_KEY, redirect);
}

export function getIdpRedirect(value: string | null | undefined) {
  if (value) return getSafeRedirect(value);

  return getSafeRedirect(sessionStorage.getItem(IDP_REDIRECT_KEY));
}

export function clearIdpRedirect() {
  sessionStorage.removeItem(IDP_REDIRECT_KEY);
}
