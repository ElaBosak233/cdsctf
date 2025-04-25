import { Image } from "@/components/ui/image";
import { useDecryptedText } from "@/hooks/use-decrypted-text";
import { useConfigStore } from "@/storages/config";
import { cn } from "@/utils";

export default function () {
    const configStore = useConfigStore();
    const title = useDecryptedText({
        text: configStore?.config?.meta?.title || "",
        speed: 100,
    });
    const description = useDecryptedText({
        text: configStore?.config?.meta?.description || "",
        speed: 25,
    });

    return (
        <>
            <div
                className={cn([
                    "flex-1",
                    "flex",
                    "flex-col",
                    "items-center",
                    "justify-center",
                    "select-none",
                ])}
            >
                <Image
                    src={"/api/configs/logo"}
                    className={cn([
                        "drop-shadow-md",
                        "aspect-square",
                        "h-[8rem]",
                    ])}
                    alt={"logo"}
                />
                <h1
                    className={cn([
                        "text-3xl",
                        "lg:text-4xl",
                        "font-extrabold",
                        "mt-5",
                    ])}
                >
                    {title}
                </h1>
                <span className={cn(["text-secondary-foreground"])}>
                    {description}
                </span>
            </div>
            {/* <div
                className={cn([
                    "absolute",
                    "bottom-4",
                    "flex",
                    "flex-wrap",
                    "items-center",
                    "justify-center",
                    "h-auto",
                    "p-2",
                    "gap-2",
                    "w-full",
                    "text-secondary-foreground",
                ])}
            >
                <Button icon={InfoIcon} square asChild>
                    <Link to={"/about"} />
                </Button>
            </div> */}
        </>
    );
}
