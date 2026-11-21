export function splitEmailDomainRules(value: string) {
  return value
    .split(/\r?\n/)
    .map((rule) => rule.trim())
    .filter(Boolean);
}

export function isEmailDomainRule(rule: string) {
  const domain = rule.startsWith("*.") ? rule.slice(2) : rule;
  const labelPattern = /^[a-z0-9](?:[a-z0-9-]{0,61}[a-z0-9])?$/i;
  return (
    domain.length <= 253 &&
    domain.split(".").every((label) => labelPattern.test(label))
  );
}

export function isEmailDomainAllowed(address: string, rules?: string[]) {
  if (!rules?.length) return true;

  const domain = address.slice(address.lastIndexOf("@") + 1).toLowerCase();
  return rules.some((value) => {
    const rule = value.trim().toLowerCase();
    return rule.startsWith("*.")
      ? domain.endsWith(rule.slice(1))
      : domain === rule;
  });
}
