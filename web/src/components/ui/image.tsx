import React, { useState, useEffect } from "react";
import { CircleOff, LoaderCircle } from "lucide-react";
import { cn } from "@/utils";

interface ImageProps {
    src: string;
    alt?: string;
    delay?: number;
    fallback?: React.ReactNode;
    className?: string;
    imgClassName?: string;
}

function Image(props: ImageProps) {
    const {
        src,
        alt,
        delay = 500,
        fallback = <CircleOff className={cn(["w-1/5", "h-1/5"])} />,
        className,
        imgClassName,
    } = props;

    const [isLoading, setIsLoading] = useState<boolean>(true);
    const [hasError, setHasError] = useState<boolean>(false);

    useEffect(() => {
        setIsLoading(true);
        setHasError(false);
    }, [src]);

    function handleLoad() {
        setTimeout(() => {
            setIsLoading(false);
        }, delay);
    }

    function handleError() {
        setIsLoading(false);
        setHasError(true);
    }

    return (
        <div className={cn(["relative"], className)}>
            {(isLoading || hasError) && (
                <div
                    className={cn([
                        "absolute",
                        "inset-0",
                        "flex",
                        "items-center",
                        "justify-center",
                        "backdrop-blur-sm",
                        "bg-opacity-50",
                        "text-foreground",
                    ])}
                >
                    {isLoading ? (
                        <LoaderCircle
                            className={cn(["animate-spin", "text-foreground"])}
                            size={24}
                        />
                    ) : (
                        fallback
                    )}
                </div>
            )}

            <img
                src={src}
                alt={alt}
                decoding={"async"}
                onLoad={handleLoad}
                onError={handleError}
                draggable={false}
                className={cn([
                    "w-full",
                    "h-full",
                    "object-cover",
                    isLoading || hasError ? "hidden" : "block",
                    imgClassName,
                ])}
            />
        </div>
    );
}

export { Image };
