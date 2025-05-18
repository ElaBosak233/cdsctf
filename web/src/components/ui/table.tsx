import * as React from "react";

import { cn } from "@/utils";

type TableProps = React.ComponentProps<"table"> & {};

function Table(props: TableProps) {
  const { className, ref, ...rest } = props;
  return (
    <table
      ref={ref}
      className={cn("w-full caption-bottom text-sm", className)}
      {...rest}
    />
  );
}

type TableHeaderProps = React.ComponentProps<"thead"> & {};

function TableHeader(props: TableHeaderProps) {
  const { className, ref, ...rest } = props;

  return (
    <thead ref={ref} className={cn("[&_tr]:border-b", className)} {...rest} />
  );
}

type TableBodyProps = React.ComponentProps<"tbody"> & {};

function TableBody(props: TableBodyProps) {
  const { className, ref, ...rest } = props;

  return (
    <tbody
      ref={ref}
      className={cn("[&_tr:last-child]:border-0", className)}
      {...rest}
    />
  );
}

type TableFooterProps = React.ComponentProps<"tfoot"> & {};

function TableFooter(props: TableFooterProps) {
  const { className, ref, ...rest } = props;

  return (
    <tfoot
      ref={ref}
      className={cn(
        "border-t bg-muted/50 font-medium last:[&>tr]:border-b-0",
        className
      )}
      {...rest}
    />
  );
}

type TableRowProps = React.ComponentProps<"tr"> & {};

function TableRow(props: TableRowProps) {
  const { className, ref, ...rest } = props;

  return (
    <tr
      ref={ref}
      className={cn(
        "border-b transition-colors hover:bg-muted/50 data-[state=selected]:bg-muted",
        className
      )}
      {...rest}
    />
  );
}

type TableHeadProps = React.ComponentProps<"th"> & {};

function TableHead(props: TableHeadProps) {
  const { className, ref, ...rest } = props;

  return (
    <th
      ref={ref}
      className={cn(
        "h-12 px-4 text-left align-middle font-medium text-muted-foreground [&:has([role=checkbox])]:pr-0",
        className
      )}
      {...rest}
    />
  );
}

type TableCellProps = React.ComponentProps<"td"> & {};

function TableCell(props: TableCellProps) {
  const { className, ref, ...rest } = props;

  return (
    <td
      ref={ref}
      className={cn(
        "p-4 align-middle [&:has([role=checkbox])]:pr-0",
        className
      )}
      {...rest}
    />
  );
}

type TableCaptionProps = React.ComponentProps<"caption"> & {};

function TableCaption(props: TableCaptionProps) {
  const { className, ref, ...rest } = props;

  return (
    <caption
      ref={ref}
      className={cn("mt-4 text-sm text-muted-foreground", className)}
      {...rest}
    />
  );
}

export {
  Table,
  TableBody,
  TableCaption,
  TableCell,
  TableFooter,
  TableHead,
  TableHeader,
  TableRow,
};
