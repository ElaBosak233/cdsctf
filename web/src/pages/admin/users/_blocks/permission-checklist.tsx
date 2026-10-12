import { useTranslation } from "react-i18next";
import { Checkbox } from "@/components/ui/checkbox";
import { PERMISSION_OPTIONS, type Permission } from "@/models/user";
import { cn } from "@/utils";

interface PermissionChecklistProps {
  value: Permission[];
  onChange: (value: Permission[]) => void;
}

function PermissionChecklist({ value, onChange }: PermissionChecklistProps) {
  const { t } = useTranslation();

  function togglePermission(permission: Permission, checked: boolean) {
    const next = checked
      ? [...new Set([...value, permission])]
      : value.filter((current) => current !== permission);
    onChange(next);
  }

  return (
    <div className="grid gap-2 sm:grid-cols-2">
      {PERMISSION_OPTIONS.map(({ value: permission, key }) => {
        const checked = value.includes(permission);
        const checkboxId = `permission-${key}`;
        return (
          <label
            key={permission}
            htmlFor={checkboxId}
            className={cn(
              "flex cursor-pointer items-start gap-3 rounded-md border border-input/70 bg-input/30 p-3 transition-colors",
              "hover:bg-input/70 select-none",
              checked && "border-primary/50 bg-primary/10 hover:bg-primary/15"
            )}
          >
            <Checkbox
              id={checkboxId}
              checked={checked}
              onCheckedChange={(nextChecked) =>
                togglePermission(permission, nextChecked)
              }
              aria-label={t(`user:permissions.options.${key}._`)}
              className="mt-0.5"
            />
            <span className="min-w-0">
              <span className="block text-sm font-medium">
                {t(`user:permissions.options.${key}._`)}
              </span>
              <span className="mt-0.5 block text-xs text-muted-foreground">
                {t(`user:permissions.options.${key}.description`)}
              </span>
            </span>
          </label>
        );
      })}
    </div>
  );
}

export { PermissionChecklist };
