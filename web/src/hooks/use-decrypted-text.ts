import { useState, useEffect } from "react";

type RevealDirection = "start" | "end" | "center";

interface UseDecryptedTextProps {
    text: string;
    speed?: number;
    maxIterations?: number;
    sequential?: boolean;
    revealDirection?: RevealDirection;
    useOriginalCharsOnly?: boolean;
    characters?: string;
}

const useDecryptedText = ({
    text,
    speed = 50,
    maxIterations = 10,
    sequential = true,
    revealDirection = "start",
    useOriginalCharsOnly = false,
    characters = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz!@#$%^&*()_+",
}: UseDecryptedTextProps): string => {
    const [displayText, setDisplayText] = useState<string>(text);
    const [revealedIndices, setRevealedIndices] = useState<Set<number>>(
        new Set()
    );

    const getNextIndex = (revealedSet: Set<number>): number => {
        const textLength = text.length;
        switch (revealDirection) {
            case "start":
                return revealedSet.size;
            case "end":
                return textLength - 1 - revealedSet.size;
            case "center": {
                const middle = Math.floor(textLength / 2);
                const offset = Math.floor(revealedSet.size / 2);
                const nextIndex =
                    revealedSet.size % 2 === 0
                        ? middle + offset
                        : middle - offset - 1;

                if (
                    nextIndex >= 0 &&
                    nextIndex < textLength &&
                    !revealedSet.has(nextIndex)
                ) {
                    return nextIndex;
                }

                for (let i = 0; i < textLength; i++) {
                    if (!revealedSet.has(i)) return i;
                }
                return 0;
            }
            default:
                return revealedSet.size;
        }
    };

    const shuffleText = (currentRevealed: Set<number>): string => {
        const availableChars = useOriginalCharsOnly
            ? Array.from(new Set(text.split(""))).filter((char) => char !== " ")
            : characters.split("");

        return text
            .split("")
            .map((char, i) => {
                if (char === " ") return " ";
                if (currentRevealed.has(i)) return text[i];
                return availableChars[
                    Math.floor(Math.random() * availableChars.length)
                ];
            })
            .join("");
    };

    useEffect(() => {
        let interval: NodeJS.Timeout | undefined;
        let currentIteration = 0;

        interval = setInterval(() => {
            setRevealedIndices((prevRevealed) => {
                if (sequential) {
                    if (prevRevealed.size < text.length) {
                        const nextIndex = getNextIndex(prevRevealed);
                        const newRevealed = new Set(prevRevealed);
                        newRevealed.add(nextIndex);
                        setDisplayText(shuffleText(newRevealed));
                        return newRevealed;
                    } else {
                        clearInterval(interval);
                        return prevRevealed;
                    }
                } else {
                    setDisplayText(shuffleText(prevRevealed));
                    currentIteration++;
                    if (currentIteration >= maxIterations) {
                        clearInterval(interval);
                        setDisplayText(text);
                    }
                    return prevRevealed;
                }
            });
        }, speed);

        return () => {
            if (interval) clearInterval(interval);
        };
    }, [
        text,
        speed,
        maxIterations,
        sequential,
        revealDirection,
        characters,
        useOriginalCharsOnly,
    ]);

    return displayText;
};

export { useDecryptedText };
