import * as React from "react";
import {
    ChevronLeftIcon,
    ChevronRightIcon,
    MoreHorizontalIcon,
} from "lucide-react";

import { cn } from "@/utils";
import { Button } from "@/components/ui/button";

interface PaginationProps
    extends Partial<
        Omit<React.ComponentProps<typeof PaginationPrimitive>, "onChange">
    > {
    size?: "sm" | "md" | "lg";
    total: number;
    value: number;
    max?: number;
    onChange: (value: number) => void;
}

function Pagination(props: PaginationProps) {
    const { size = "md", total, value, max, onChange, ...rest } = props;

    const paginationNodes = React.useMemo(() => {
        return generatePaginationNodes(total, value, max);
    }, [total, value, max]);

    return (
        <PaginationPrimitive {...rest}>
            <PaginationContent>
                <PaginationItem>
                    <PaginationPrevious
                        size={size}
                        disabled={value - 1 === 0}
                        onClick={() => onChange(value - 1)}
                    />
                </PaginationItem>
                {paginationNodes?.map((node, index) => (
                    <React.Fragment key={index}>
                        {typeof node === "number" ? (
                            <PaginationItem>
                                <PaginationLink
                                    size={size}
                                    isActive={node === value}
                                    onClick={() => onChange(node)}
                                >
                                    {node}
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
                        disabled={value === total || total === 0}
                        onClick={() => onChange(value + 1)}
                    />
                </PaginationItem>
            </PaginationContent>
        </PaginationPrimitive>
    );
}

type PaginationNode = number | "...";

function generatePaginationNodes(total: number, value: number, max?: number) {
    const items: PaginationNode[] = [];
    const maxPages = max || 5;
    const halfMaxPages = Math.floor(maxPages / 2);

    if (total <= 1) {
        return [1];
    }

    const startPage = Math.max(1, value - halfMaxPages);
    const endPage = Math.min(total, value + halfMaxPages);

    if (total > maxPages) {
        if (startPage > 1) {
            items.push(1);
            if (startPage > 2) {
                items.push("...");
            }
        }

        for (let i = startPage; i <= endPage; i++) {
            items.push(i);
        }

        if (endPage < total) {
            if (endPage < total - 1) {
                items.push("...");
            }
            items.push(total);
        }
    } else {
        for (let i = 1; i <= total; i++) {
            items.push(i);
        }
    }

    return items;
}

function PaginationPrimitive({
    className,
    ...props
}: React.ComponentProps<"nav">) {
    return (
        <nav
            role="navigation"
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
            className={cn(
                ["flex", "flex-row", "items-center", "gap-1"],
                className
            )}
            {...props}
        />
    );
}

function PaginationItem({ ...props }: React.ComponentProps<"li">) {
    return <li data-slot="pagination-item" {...props} />;
}

interface PaginationLinkProps extends React.ComponentProps<typeof Button> {
    isActive?: boolean;
}

function PaginationLink({
    className,
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
            icon={ChevronLeftIcon}
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
            icon={ChevronRightIcon}
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

export { Pagination };
