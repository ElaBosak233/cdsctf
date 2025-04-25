import { ChevronDown, ChevronUp } from "lucide-react";
import React, { useCallback, useEffect, useState } from "react";
import { NumericFormat, NumericFormatProps } from "react-number-format";
import { TextField } from "./text-field";
import { FieldContext } from "./field";
import { cn } from "@/utils";

export interface NumberInputProps
    extends Omit<React.ComponentProps<"input">, "type">,
        Omit<NumericFormatProps, "value" | "onValueChange"> {
    stepper?: number;
    thousandSeparator?: string;
    placeholder?: string;
    defaultValue?: number;
    min?: number;
    max?: number;
    value?: number;
    suffix?: string;
    prefix?: string;
    onValueChange?: (value: number | undefined) => void;
    fixedDecimalScale?: boolean;
    decimalScale?: number;
}

function NumberField(props: NumberInputProps) {
    const {
        stepper,
        thousandSeparator,
        placeholder,
        defaultValue,
        min = -Infinity,
        max = Infinity,
        onValueChange,
        fixedDecimalScale = false,
        decimalScale = 0,
        suffix,
        prefix,
        value: controlledValue,
        ref,
        ...rest
    } = props;

    const context = React.useContext(FieldContext);

    const { hasIcon, hasExtraButton } = context;

    const [value, setValue] = useState<number | undefined>(
        controlledValue ?? defaultValue
    );

    const handleIncrement = useCallback(() => {
        setValue((prev) =>
            prev === undefined
                ? (stepper ?? 1)
                : Math.min(prev + (stepper ?? 1), max)
        );
    }, [stepper, max]);

    const handleDecrement = useCallback(() => {
        setValue((prev) =>
            prev === undefined
                ? -(stepper ?? 1)
                : Math.max(prev - (stepper ?? 1), min)
        );
    }, [stepper, min]);

    useEffect(() => {
        const handleKeyDown = (e: KeyboardEvent) => {
            if (
                document.activeElement ===
                (ref as React.RefObject<HTMLInputElement>).current
            ) {
                if (e.key === "ArrowUp") {
                    handleIncrement();
                } else if (e.key === "ArrowDown") {
                    handleDecrement();
                }
            }
        };

        window.addEventListener("keydown", handleKeyDown);

        return () => {
            window.removeEventListener("keydown", handleKeyDown);
        };
    }, [handleIncrement, handleDecrement, ref]);

    useEffect(() => {
        if (controlledValue !== undefined) {
            setValue(controlledValue);
        }
    }, [controlledValue]);

    const handleChange = (values: {
        value: string;
        floatValue: number | undefined;
    }) => {
        const newValue =
            values.floatValue === undefined ? undefined : values.floatValue;
        setValue(newValue);
        if (onValueChange) {
            onValueChange(newValue);
        }
    };

    const handleBlur = () => {
        if (value !== undefined) {
            if (value < min) {
                setValue(min);
                (ref as React.RefObject<HTMLInputElement>).current!.value =
                    String(min);
            } else if (value > max) {
                setValue(max);
                (ref as React.RefObject<HTMLInputElement>).current!.value =
                    String(max);
            }
        }
    };

    return (
        <div
            className={cn([
                "flex",
                "items-center",
                "flex-1",
                "w-0",
                "relative",
            ])}
        >
            <NumericFormat
                value={value}
                onValueChange={handleChange}
                thousandSeparator={thousandSeparator}
                decimalScale={decimalScale}
                fixedDecimalScale={fixedDecimalScale}
                allowNegative={min < 0}
                valueIsNumericString
                onBlur={handleBlur}
                max={max}
                min={min}
                suffix={suffix}
                prefix={prefix}
                customInput={TextField}
                placeholder={placeholder}
                className={cn([
                    "[appearance:textfield]",
                    "[&::-webkit-outer-spin-button]:appearance-none",
                    "[&::-webkit-inner-spin-button]:appearance-none",
                    "relative",
                    hasExtraButton && "rounded-r-none",
                ])}
                getInputRef={ref}
                type={"text"}
                {...rest}
            />

            <div
                className={cn([
                    "absolute",
                    "right-1",
                    "top-1/2",
                    "-translate-y-1/2",
                    "flex",
                    "flex-col",
                ])}
            >
                <button
                    aria-label="Increase value"
                    className={cn([
                        "px-2",
                        "h-4",
                        "rounded-l-none",
                        "rounded-br-none",
                        "border-input",
                        "border-l-0",
                        "border-b-[0.5px]",
                        "focus-visible:relative",
                        "cursor-pointer",
                    ])}
                    onClick={handleIncrement}
                    disabled={value === max}
                    type={"button"}
                >
                    <ChevronUp size={15} />
                </button>
                <button
                    aria-label="Decrease value"
                    className={cn([
                        "px-2",
                        "h-4",
                        "rounded-l-none",
                        "rounded-tr-none",
                        "border-input",
                        "border-l-0",
                        "border-t-[0.5px]",
                        "focus-visible:relative",
                        "cursor-pointer",
                    ])}
                    onClick={handleDecrement}
                    disabled={value === min}
                    type={"button"}
                >
                    <ChevronDown size={15} />
                </button>
            </div>
        </div>
    );
}

export { NumberField };
