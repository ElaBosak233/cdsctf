import { useState } from "react";
import { Document, Page, pdfjs } from "react-pdf";
import { cn } from "@/utils";
import "react-pdf/dist/Page/AnnotationLayer.css";
import "react-pdf/dist/Page/TextLayer.css";
import { LoaderCircleIcon } from "lucide-react";

pdfjs.GlobalWorkerOptions.workerSrc = new URL(
  "pdfjs-dist/build/pdf.worker.min.mjs",
  import.meta.url
).toString();

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
      file={{
        url: url,
      }}
      onLoadSuccess={onDocumentLoadSuccess}
      onError={(err) => console.log(err)}
      className={className}
      loading={<LoaderCircleIcon className={cn(["animate-spin"])} />}
    >
      <div
        className={cn(["flex", "flex-col", "gap-4", "w-full", "max-w-full"])}
      >
        {Array.from(new Array(numPages), (_, index) => (
          <Page
            key={`page_${index + 1}`}
            pageNumber={index + 1}
            renderAnnotationLayer={false}
            renderTextLayer={false}
            className={cn([
              "w-full",
              "[&>canvas]:w-full!",
              "[&>canvas]:h-auto!",
            ])}
          />
        ))}
      </div>
    </Document>
  );
}

export { PDFViewer };
