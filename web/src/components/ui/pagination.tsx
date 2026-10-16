import {
  ChevronLeftIcon,
  ChevronRightIcon,
  MoreHorizontalIcon,
} from "lucide-react";
import * as React from "react";

import { Button } from "@/components/ui/button";
import { cn } from "@/utils";

type PaginationProps = Omit<
  React.ComponentProps<typeof PaginationPrimitive>,
  "onChange"
> & {
  size?: "sm" | "md" | "lg";
  total: number;
  value: number;
  max?: number;
  onChange: (value: number) => void;
};

function Pagination(props: PaginationProps) {
  const { size = "md", total, value, max, onChange, ...rest } = props;
  const currentPage = Math.min(Math.max(value, 1), Math.max(total, 1));

  const paginationNodes = React.useMemo(() => {
    return generatePaginationNodes(total, currentPage, max);
  }, [total, currentPage, max]);

  return (
    <PaginationPrimitive {...rest}>
      <PaginationContent>
        <PaginationItem>
          <PaginationPrevious
            size={size}
            disabled={currentPage <= 1}
            onClick={() => onChange(currentPage - 1)}
          />
        </PaginationItem>
        {paginationNodes.map((node) => (
          <React.Fragment
            key={
              node.type === "page"
                ? `page-${node.value}`
                : `ellipsis-${node.position}`
            }
          >
            {node.type === "page" ? (
              <PaginationItem>
                <PaginationLink
                  size={size}
                  isActive={node.value === currentPage}
                  onClick={() => onChange(node.value)}
                >
                  {node.value}
                </PaginationLink>
              </PaginationItem>
            ) : (
              <PaginationEllipsis />
            )}
          </React.Fragment>
        ))}
        <PaginationItem>
          <PaginationNext
            size={size}
            disabled={currentPage >= total || total === 0}
            onClick={() => onChange(currentPage + 1)}
          />
        </PaginationItem>
      </PaginationContent>
    </PaginationPrimitive>
  );
}

type PaginationNode =
  | { type: "page"; value: number }
  | { type: "ellipsis"; position: "start" | "end" };

function generatePaginationNodes(
  total: number,
  value: number,
  max?: number
): PaginationNode[] {
  const items: Array<PaginationNode> = [];
  const requestedMaxPages = Math.max(max || 5, 5);
  const maxPages =
    requestedMaxPages % 2 === 0 ? requestedMaxPages + 1 : requestedMaxPages;
  const currentPage = Math.min(Math.max(value, 1), total);

  if (total <= 1) {
    return [{ type: "page", value: 1 }];
  }

  if (total <= maxPages + 2) {
    for (let i = 1; i <= total; i++) {
      items.push({ type: "page", value: i });
    }
    return items;
  }

  const siblingCount = Math.floor((maxPages - 3) / 2);
  const firstPageThreshold = siblingCount + 2;
  const lastPageThreshold = total - siblingCount - 1;

  if (currentPage <= firstPageThreshold) {
    for (let i = 1; i <= maxPages; i++) {
      items.push({ type: "page", value: i });
    }
    items.push({ type: "ellipsis", position: "end" });
    items.push({ type: "page", value: total });
    return items;
  }

  if (currentPage >= lastPageThreshold) {
    items.push({ type: "page", value: 1 });
    items.push({ type: "ellipsis", position: "start" });
    for (let i = total - maxPages + 1; i <= total; i++) {
      items.push({ type: "page", value: i });
    }
    return items;
  }

  items.push({ type: "page", value: 1 });
  items.push({ type: "ellipsis", position: "start" });
  for (
    let i = currentPage - siblingCount;
    i <= currentPage + siblingCount;
    i++
  ) {
    items.push({ type: "page", value: i });
  }
  items.push({ type: "ellipsis", position: "end" });
  items.push({ type: "page", value: total });

  return items;
}

function PaginationPrimitive({
  className,
  ...props
}: React.ComponentProps<"nav">) {
  return (
    <nav
      aria-label="pagination"
      data-slot="pagination"
      className={cn(["flex"], className)}
      {...props}
    />
  );
}

function PaginationContent({
  className,
  ...props
}: React.ComponentProps<"ul">) {
  return (
    <ul
      data-slot="pagination-content"
      className={cn(["flex", "flex-row", "items-center", "gap-1"], className)}
      {...props}
    />
  );
}

function PaginationItem({ ...props }: React.ComponentProps<"li">) {
  return <li data-slot="pagination-item" {...props} />;
}

type PaginationLinkProps = React.ComponentProps<typeof Button> & {
  isActive?: boolean;
};

function PaginationLink({
  isActive,
  disabled = false,
  size = "md",
  ...props
}: PaginationLinkProps) {
  return (
    <Button
      size={size}
      disabled={disabled}
      variant={isActive ? "tonal" : "ghost"}
      square
      {...props}
    />
  );
}

function PaginationPrevious({
  size,
  disabled,
  className,
  ...props
}: React.ComponentProps<typeof PaginationLink>) {
  return (
    <PaginationLink
      size={size}
      disabled={disabled}
      className={cn(["gap-1", "px-2.5", "sm:pl-2.5"], className)}
      icon={<ChevronLeftIcon />}
      {...props}
    />
  );
}

function PaginationNext({
  size,
  disabled,
  className,
  ...props
}: React.ComponentProps<typeof PaginationLink>) {
  return (
    <PaginationLink
      size={size}
      disabled={disabled}
      className={cn(["gap-1", "px-2.5", "sm:pr-2.5"], className)}
      icon={<ChevronRightIcon />}
      {...props}
    />
  );
}

function PaginationEllipsis({
  className,
  ...props
}: React.ComponentProps<"span">) {
  return (
    <span
      aria-hidden
      data-slot="pagination-ellipsis"
      className={cn(
        ["flex", "size-9", "items-center", "justify-center"],
        className
      )}
      {...props}
    >
      <MoreHorizontalIcon className="size-4" />
      <span className="sr-only">More pages</span>
    </span>
  );
}

export { Pagination, type PaginationProps };
