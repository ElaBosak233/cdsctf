import { useState } from "react";
import { Document, Page, pdfjs } from "react-pdf";
import { cn } from "@/utils";
import "react-pdf/dist/Page/AnnotationLayer.css";
import "react-pdf/dist/Page/TextLayer.css";
import { LoaderCircleIcon } from "lucide-react";

pdfjs.GlobalWorkerOptions.workerSrc = `//unpkg.com/pdfjs-dist@${pdfjs.version}/build/pdf.worker.min.mjs`;

export interface PDFViewerProps {
  url: string;
  className?: string;
}

function PDFViewer(props: PDFViewerProps) {
  const { url, className } = props;
  const [numPages, setNumPages] = useState<number>(0);
  const onDocumentLoadSuccess = ({ numPages }: { numPages: number }): void => {
    setNumPages(numPages);
  };

  return (
    <Document
      key={`${url}-${numPages}`}
      file={url}
      onLoadSuccess={onDocumentLoadSuccess}
      onError={(err) => console.log(err)}
      className={className}
      loading={
        <div
          className={cn(["flex", "justify-center", "items-center", "gap-5"])}
        >
          <LoaderCircleIcon className={cn(["animate-spin"])} />
          <span>加载中...</span>
        </div>
      }
    >
      <div
        className={cn(["flex", "flex-col", "gap-4", "w-full", "max-w-full"])}
      >
        {Array.from(new Array(numPages), (_, index) => (
          <Page
            key={`page-${index + 1}`}
            pageNumber={index + 1}
            renderAnnotationLayer={false}
            renderTextLayer={false}
            className={cn([
              "w-full",
              "[&>canvas]:w-full!",
              "[&>canvas]:h-auto!",
              "rounded-lg",
              "overflow-hidden",
            ])}
          />
        ))}
      </div>
    </Document>
  );
}

export { PDFViewer };
