export function isSubRoute(
  parentPath: string,
  childPath: string,
  basePath: string = "/"
): boolean {
  const normalize = (path: string) => path.replace(/\/+$/, "");

  const p = normalize(parentPath);
  const c = normalize(childPath);
  const b = normalize(basePath);

  if (p === b) {
    return p === c;
  }

  if (c === p || c.startsWith(`${p}/`)) {
    return true;
  }

  return false;
}
