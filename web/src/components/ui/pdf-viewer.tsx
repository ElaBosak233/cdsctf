import { createPluginRegistration } from "@embedpdf/core";
import { EmbedPDF } from "@embedpdf/core/react";
import { usePdfiumEngine } from "@embedpdf/engines/react";
import { useTranslation } from "react-i18next";
import { cn } from "@/utils";
import { LoaderCircleIcon } from "lucide-react";

import {
  Viewport,
  ViewportPluginPackage,
} from "@embedpdf/plugin-viewport/react";
import { Scroller, ScrollPluginPackage } from "@embedpdf/plugin-scroll/react";
import {
  DocumentContent,
  DocumentManagerPlugin,
  DocumentManagerPluginPackage,
} from "@embedpdf/plugin-document-manager/react";
import {
  RenderLayer,
  RenderPluginPackage,
} from "@embedpdf/plugin-render/react";
import { ScrollArea } from "./scroll-area";

const plugins = [
  createPluginRegistration(DocumentManagerPluginPackage),
  createPluginRegistration(ViewportPluginPackage),
  createPluginRegistration(ScrollPluginPackage),
  createPluginRegistration(RenderPluginPackage),
];

export interface PDFViewerProps {
  url: string;
  className?: string;
}

function PDFViewer(props: PDFViewerProps) {
  const { url, className } = props;
  const { t } = useTranslation();
  const { engine, isLoading } = usePdfiumEngine();
  if (isLoading || !engine) {
    return (
      <div className={cn(["flex", "justify-center", "items-center", "gap-5"])}>
        <LoaderCircleIcon className={cn(["animate-spin"])} />
        <span>{t("common:loading")}</span>
      </div>
    );
  }

  return (
    <EmbedPDF
      engine={engine}
      plugins={plugins}
      onInitialized={async (registry) => {
        registry
          ?.getPlugin<DocumentManagerPlugin>(DocumentManagerPlugin.id)
          ?.provides()
          ?.openDocumentUrl({ url })
          .toPromise();
      }}
    >
      {({ activeDocumentId }) =>
        activeDocumentId && (
          <DocumentContent documentId={activeDocumentId}>
            {({ isLoaded }) =>
              isLoaded && (
                <div
                  className={cn(
                    "h-full w-full min-h-0 flex flex-col",
                    className
                  )}
                >
                  <ScrollArea className="h-full min-h-0 overflow-hidden">
                    <Viewport
                      documentId={activeDocumentId}
                      className="w-full min-h-0"
                    >
                      <Scroller
                        documentId={activeDocumentId}
                        className="h-auto! w-auto!"
                        renderPage={({ pageIndex }) => (
                          <RenderLayer
                            documentId={activeDocumentId}
                            pageIndex={pageIndex}
                            draggable={false}
                          />
                        )}
                      />
                    </Viewport>
                  </ScrollArea>
                </div>
              )
            }
          </DocumentContent>
        )
      }
    </EmbedPDF>
  );
}

export { PDFViewer };
