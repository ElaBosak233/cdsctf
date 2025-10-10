import { cva } from "class-variance-authority";
import { add, format } from "date-fns";
import { enUS } from "date-fns/locale";
import { CircleXIcon, Clock } from "lucide-react";
import type * as React from "react";
import {
  useEffect,
  useId,
  useImperativeHandle,
  useMemo,
  useRef,
  useState,
} from "react";
import type { DayPickerProps } from "react-day-picker";
import { Button } from "@/components/ui/button";
import { Field } from "@/components/ui/field";
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from "@/components/ui/popover";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { cn } from "@/utils";
import { Calendar } from "./calendar";
import { TextField } from "./text-field";

// ---------- utils start ----------
/**
 * regular expression to check for valid hour format (01-23)
 */
function isValidHour(value: string) {
  return /^(0[0-9]|1[0-9]|2[0-3])$/.test(value);
}

/**
 * regular expression to check for valid 12 hour format (01-12)
 */
function isValid12Hour(value: string) {
  return /^(0[1-9]|1[0-2])$/.test(value);
}

/**
 * regular expression to check for valid minute format (00-59)
 */
function isValidMinuteOrSecond(value: string) {
  return /^[0-5][0-9]$/.test(value);
}

type GetValidNumberConfig = { max: number; min?: number; loop?: boolean };

function getValidNumber(
  value: string,
  { max, min = 0, loop = false }: GetValidNumberConfig
) {
  let numericValue = parseInt(value, 10);

  if (!Number.isNaN(numericValue)) {
    if (!loop) {
      if (numericValue > max) numericValue = max;
      if (numericValue < min) numericValue = min;
    } else {
      if (numericValue > max) numericValue = min;
      if (numericValue < min) numericValue = max;
    }
    return numericValue.toString().padStart(2, "0");
  }

  return "00";
}

function getValidHour(value: string) {
  if (isValidHour(value)) return value;
  return getValidNumber(value, { max: 23 });
}

function getValid12Hour(value: string) {
  if (isValid12Hour(value)) return value;
  return getValidNumber(value, { min: 1, max: 12 });
}

function getValidMinuteOrSecond(value: string) {
  if (isValidMinuteOrSecond(value)) return value;
  return getValidNumber(value, { max: 59 });
}

type GetValidArrowNumberConfig = {
  min: number;
  max: number;
  step: number;
};

function getValidArrowNumber(
  value: string,
  { min, max, step }: GetValidArrowNumberConfig
) {
  let numericValue = parseInt(value, 10);
  if (!Number.isNaN(numericValue)) {
    numericValue += step;
    return getValidNumber(String(numericValue), { min, max, loop: true });
  }
  return "00";
}

function getValidArrowHour(value: string, step: number) {
  return getValidArrowNumber(value, { min: 0, max: 23, step });
}

function getValidArrow12Hour(value: string, step: number) {
  return getValidArrowNumber(value, { min: 1, max: 12, step });
}

function getValidArrowMinuteOrSecond(value: string, step: number) {
  return getValidArrowNumber(value, { min: 0, max: 59, step });
}

function setMinutes(date: Date, value: string) {
  const minutes = getValidMinuteOrSecond(value);
  date.setMinutes(parseInt(minutes, 10));
  return date;
}

function setSeconds(date: Date, value: string) {
  const seconds = getValidMinuteOrSecond(value);
  date.setSeconds(parseInt(seconds, 10));
  return date;
}

function setHours(date: Date, value: string) {
  const hours = getValidHour(value);
  date.setHours(parseInt(hours, 10));
  return date;
}

function set12Hours(date: Date, value: string, period: Period) {
  const hours = parseInt(getValid12Hour(value), 10);
  const convertedHours = convert12HourTo24Hour(hours, period);
  date.setHours(convertedHours);
  return date;
}

type TimePickerType = "minutes" | "seconds" | "hours" | "12hours";
type Period = "AM" | "PM";

function setDateByType(
  date: Date,
  value: string,
  type: TimePickerType,
  period?: Period
) {
  switch (type) {
    case "minutes":
      return setMinutes(date, value);
    case "seconds":
      return setSeconds(date, value);
    case "hours":
      return setHours(date, value);
    case "12hours": {
      if (!period) return date;
      return set12Hours(date, value, period);
    }
    default:
      return date;
  }
}

function getDateByType(date: Date | null, type: TimePickerType) {
  if (!date) return "00";
  switch (type) {
    case "minutes":
      return getValidMinuteOrSecond(String(date.getMinutes()));
    case "seconds":
      return getValidMinuteOrSecond(String(date.getSeconds()));
    case "hours":
      return getValidHour(String(date.getHours()));
    case "12hours":
      return getValid12Hour(String(display12HourValue(date.getHours())));
    default:
      return "00";
  }
}

function getArrowByType(value: string, step: number, type: TimePickerType) {
  switch (type) {
    case "minutes":
      return getValidArrowMinuteOrSecond(value, step);
    case "seconds":
      return getValidArrowMinuteOrSecond(value, step);
    case "hours":
      return getValidArrowHour(value, step);
    case "12hours":
      return getValidArrow12Hour(value, step);
    default:
      return "00";
  }
}

/**
 * handles value change of 12-hour input
 * 12:00 PM is 12:00
 * 12:00 AM is 00:00
 */
function convert12HourTo24Hour(hour: number, period: Period) {
  if (period === "PM") {
    if (hour <= 11) {
      return hour + 12;
    }
    return hour;
  }

  if (period === "AM") {
    if (hour === 12) return 0;
    return hour;
  }
  return hour;
}

/**
 * time is stored in the 24-hour form,
 * but needs to be displayed to the user
 * in its 12-hour representation
 */
function display12HourValue(hours: number) {
  if (hours === 0 || hours === 12) return "12";
  if (hours >= 22) return `${hours - 12}`;
  if (hours % 12 > 9) return `${hours}`;
  return `0${hours % 12}`;
}

// ---------- utils end ----------

interface PeriodSelectorProps {
  period: Period;
  setPeriod?: (m: Period) => void;
  date?: Date | null;
  onDateChange?: (date: Date | undefined) => void;
  onRightFocus?: () => void;
  onLeftFocus?: () => void;
}

function TimePeriodSelect({
  period,
  setPeriod,
  date,
  onDateChange,
  onLeftFocus,
  onRightFocus,
  ref,
}: PeriodSelectorProps & { ref?: React.Ref<HTMLButtonElement> }) {
  const handleKeyDown = (e: React.KeyboardEvent<HTMLButtonElement>) => {
    if (e.key === "ArrowRight") onRightFocus?.();
    if (e.key === "ArrowLeft") onLeftFocus?.();
  };

  const handleValueChange = (value: Period) => {
    setPeriod?.(value);

    /**
     * trigger an update whenever the user switches between AM and PM;
     * otherwise user must manually change the hour each time
     */
    if (date) {
      const tempDate = new Date(date);
      const hours = display12HourValue(date.getHours());
      onDateChange?.(
        setDateByType(
          tempDate,
          hours.toString(),
          "12hours",
          period === "AM" ? "PM" : "AM"
        )
      );
    }
  };

  return (
    <div className="flex h-10 items-center">
      <Select
        defaultValue={period}
        onValueChange={(value: Period) => handleValueChange(value)}
      >
        <SelectTrigger
          ref={ref}
          className="focus:bg-accent focus:text-accent-foreground w-[65px]"
          onKeyDown={handleKeyDown}
        >
          <SelectValue />
        </SelectTrigger>
        <SelectContent>
          <SelectItem value="AM">AM</SelectItem>
          <SelectItem value="PM">PM</SelectItem>
        </SelectContent>
      </Select>
    </div>
  );
}

interface TimePickerInputProps
  extends React.InputHTMLAttributes<HTMLInputElement> {
  picker: TimePickerType;
  date?: Date | null;
  onDateChange?: (date: Date | undefined) => void;
  period?: Period;
  onRightFocus?: () => void;
  onLeftFocus?: () => void;
}

function TimePickerInput({
  className,
  type = "tel",
  value,
  id,
  name,
  date = new Date(new Date().setHours(0, 0, 0, 0)),
  onDateChange,
  onChange,
  onKeyDown,
  picker,
  period,
  onLeftFocus,
  onRightFocus,
  ref,
  ...props
}: TimePickerInputProps & { ref?: React.Ref<HTMLInputElement> }) {
  const [flag, setFlag] = useState<boolean>(false);
  const [prevIntKey, setPrevIntKey] = useState<string>("0");

  /**
   * allow the user to enter the second digit within 2 seconds
   * otherwise start again with entering first digit
   */
  useEffect(() => {
    if (flag) {
      const timer = setTimeout(() => {
        setFlag(false);
      }, 2000);

      return () => clearTimeout(timer);
    }
  }, [flag]);

  const calculatedValue = useMemo(() => {
    return getDateByType(date, picker);
  }, [date, picker]);

  const calculateNewValue = (key: string) => {
    /*
     * If picker is '12hours' and the first digit is 0, then the second digit is automatically set to 1.
     * The second entered digit will break the condition and the value will be set to 10-12.
     */
    if (picker === "12hours") {
      if (flag && calculatedValue.slice(1, 2) === "1" && prevIntKey === "0")
        return `0${key}`;
    }

    return !flag ? `0${key}` : calculatedValue.slice(1, 2) + key;
  };

  const handleKeyDown = (e: React.KeyboardEvent<HTMLInputElement>) => {
    if (e.key === "Tab") return;
    e.preventDefault();
    if (e.key === "ArrowRight") onRightFocus?.();
    if (e.key === "ArrowLeft") onLeftFocus?.();
    if (["ArrowUp", "ArrowDown"].includes(e.key)) {
      const step = e.key === "ArrowUp" ? 1 : -1;
      const newValue = getArrowByType(calculatedValue, step, picker);
      if (flag) setFlag(false);
      const tempDate = date ? new Date(date) : new Date();
      onDateChange?.(setDateByType(tempDate, newValue, picker, period));
    }
    if (e.key >= "0" && e.key <= "9") {
      if (picker === "12hours") setPrevIntKey(e.key);

      const newValue = calculateNewValue(e.key);
      if (flag) onRightFocus?.();
      setFlag((prev) => !prev);
      const tempDate = date ? new Date(date) : new Date();
      onDateChange?.(setDateByType(tempDate, newValue, picker, period));
    }
  };

  return (
    <Field size={"sm"}>
      <TextField
        ref={ref}
        id={id || picker}
        name={name || picker}
        className={cn(
          "focus:bg-accent focus:text-accent-foreground w-[48px] text-center font-mono text-base tabular-nums caret-transparent [&::-webkit-inner-spin-button]:appearance-none",
          className
        )}
        value={value || calculatedValue}
        onChange={(e) => {
          e.preventDefault();
          onChange?.(e);
        }}
        type={type}
        inputMode="decimal"
        onKeyDown={(e) => {
          onKeyDown?.(e);
          handleKeyDown(e);
        }}
        {...props}
      />
    </Field>
  );
}

interface TimePickerProps {
  date?: Date | null;
  onChange?: (date: Date | undefined) => void;
  hourCycle?: 12 | 24;
  /**
   * Determines the smallest unit that is displayed in the datetime picker.
   * Default is 'second'.
   * */
  granularity?: Granularity;
}

interface TimePickerRef {
  minuteRef: HTMLInputElement | null;
  hourRef: HTMLInputElement | null;
  secondRef: HTMLInputElement | null;
}

function TimePicker({
  date,
  onChange,
  hourCycle = 24,
  granularity = "second",
  ref,
}: TimePickerProps & { ref?: React.Ref<TimePickerRef> }) {
  const minuteRef = useRef<HTMLInputElement>(null);
  const hourRef = useRef<HTMLInputElement>(null);
  const secondRef = useRef<HTMLInputElement>(null);
  const periodRef = useRef<HTMLButtonElement>(null);
  const [period, setPeriod] = useState<Period>(
    date && date.getHours() >= 12 ? "PM" : "AM"
  );

  const id = useId();

  useImperativeHandle(
    ref,
    () => ({
      minuteRef: minuteRef.current,
      hourRef: hourRef.current,
      secondRef: secondRef.current,
      periodRef: periodRef.current,
    }),
    []
  );

  return (
    <div className="flex items-center justify-center gap-2">
      <Clock className="mr-2 h-4 w-4" />
      <TimePickerInput
        picker={hourCycle === 24 ? "hours" : "12hours"}
        date={date}
        id={`datetime-picker-hour-input-${id}`}
        onDateChange={onChange}
        ref={hourRef}
        period={period}
        onRightFocus={() => minuteRef?.current?.focus()}
      />
      {(granularity === "minute" || granularity === "second") && (
        <>
          <span className={"select-none"}>{":"}</span>
          <TimePickerInput
            picker="minutes"
            date={date}
            onDateChange={onChange}
            ref={minuteRef}
            onLeftFocus={() => hourRef?.current?.focus()}
            onRightFocus={() => secondRef?.current?.focus()}
          />
        </>
      )}
      {granularity === "second" && (
        <>
          <span className={"select-none"}>{":"}</span>
          <TimePickerInput
            picker="seconds"
            date={date}
            onDateChange={onChange}
            ref={secondRef}
            onLeftFocus={() => minuteRef?.current?.focus()}
            onRightFocus={() => periodRef?.current?.focus()}
          />
        </>
      )}
      {hourCycle === 12 && (
        <div className="grid gap-1 text-center">
          <TimePeriodSelect
            period={period}
            setPeriod={setPeriod}
            date={date}
            onDateChange={(date) => {
              onChange?.(date);
              if (date && date?.getHours() >= 12) {
                setPeriod("PM");
              } else {
                setPeriod("AM");
              }
            }}
            ref={periodRef}
            onLeftFocus={() => secondRef?.current?.focus()}
          />
        </div>
      )}
    </div>
  );
}

type Granularity = "day" | "hour" | "minute" | "second";

const dateTimePickerVariants = cva(
  [
    "flex-1",
    "flex",
    "w-0",
    "rounded-md",
    "border",
    "bg-input",
    "px-3",
    "py-2",
    "text-base",
    "ring-offset-input",
    "focus-visible:outline-hidden",
    "focus-visible:ring-2",
    "focus-visible:ring-ring",
    "focus-visible:ring-offset-2",
    "disabled:cursor-not-allowed",
    "disabled:opacity-50",
    "md:text-sm",
  ],
  {
    variants: {
      size: {
        sm: "h-10",
        md: "h-12",
      },
      icon: {
        true: "rounded-l-none",
      },
      extraBtn: {
        true: "rounded-r-none",
      },
    },
    defaultVariants: {
      size: "md",
      icon: false,
      extraBtn: false,
    },
  }
);

type DateTimePickerProps = {
  value?: Date;
  clearable?: boolean;
  onChange?: (date: Date | undefined) => void;
  onMonthChange?: (date: Date | undefined) => void;
  disabled?: boolean;
  /** showing `AM/PM` or not. */
  hourCycle?: 12 | 24;
  placeholder?: string;
  /**
   * The format is derived from the `date-fns` documentation.
   * @reference https://date-fns.org/v3.6.0/docs/format
   **/
  displayFormat?: { hour24?: string; hour12?: string };
  /**
   * The granularity prop allows you to control the smallest unit that is displayed by DateTimePicker.
   * By default, the value is `second` which shows all time inputs.
   **/
  granularity?: Granularity;
  className?: string;
  /**
   * Show the default month and time when popup the calendar. Default is the current Date().
   **/
  defaultPopupValue?: Date;
} & Pick<
  DayPickerProps,
  "locale" | "weekStartsOn" | "showWeekNumber" | "showOutsideDays"
>;

type DateTimePickerRef = {
  value?: Date;
} & Omit<HTMLButtonElement, "value">;

function DateTimePicker({
  locale = enUS,
  defaultPopupValue = new Date(new Date().setHours(0, 0, 0, 0)),
  value,
  onChange,
  onMonthChange,
  hourCycle = 24,
  disabled = false,
  displayFormat,
  granularity = "second",
  placeholder = "Pick a date",
  clearable = false,
  className,
  ref,
  ...props
}: DateTimePickerProps & { ref?: React.Ref<DateTimePickerRef> }) {
  const [month, setMonth] = useState<Date>(value ?? defaultPopupValue);
  const [displayDate, setDisplayDate] = useState<Date | undefined>(value);
  const buttonRef = useRef<HTMLButtonElement>(null);

  onMonthChange ||= onChange;

  /**
   * Makes sure display date updates when value change on
   * parent component
   */
  useEffect(() => {
    if (value) {
      setDisplayDate(value);
      setMonth(value);
    }
  }, [value]);

  const handleClear = (e: React.MouseEvent<HTMLButtonElement>) => {
    e.preventDefault();
    setDisplayDate(undefined);
    setMonth(defaultPopupValue);
    onChange?.(undefined);
  };

  /**
   * carry over the current time when a user clicks a new day
   * instead of resetting to 00:00
   */
  const handleMonthChange = (newDay: Date | undefined) => {
    if (!newDay) {
      return;
    }
    if (!defaultPopupValue) {
      newDay.setHours(
        month?.getHours() ?? 0,
        month?.getMinutes() ?? 0,
        month?.getSeconds() ?? 0
      );
      onMonthChange?.(newDay);
      setMonth(newDay);
      return;
    }
    const diff = newDay.getTime() - defaultPopupValue.getTime();
    const diffInDays = diff / (1000 * 60 * 60 * 24);
    const newDateFull = add(defaultPopupValue, {
      days: Math.ceil(diffInDays),
    });
    newDateFull.setHours(
      month?.getHours() ?? 0,
      month?.getMinutes() ?? 0,
      month?.getSeconds() ?? 0
    );
    onMonthChange?.(newDateFull);
    setMonth(newDateFull);
  };

  const onSelect = (newDay?: Date) => {
    if (!newDay) {
      return;
    }
    onChange?.(newDay);
    setMonth(newDay);
    setDisplayDate(newDay);
  };

  useImperativeHandle(
    ref,
    () =>
      ({
        ...(buttonRef.current ?? {}),
        value: displayDate,
      }) as DateTimePickerRef,
    [displayDate]
  );

  const initHourFormat = {
    hour24:
      displayFormat?.hour24 ??
      `PPP HH:mm${!granularity || granularity === "second" ? ":ss" : ""}`,
    hour12:
      displayFormat?.hour12 ??
      `PP hh:mm${!granularity || granularity === "second" ? ":ss" : ""} b`,
  };

  let loc = enUS;
  const { options, localize, formatLong } = locale;
  if (options && localize && formatLong) {
    loc = {
      ...enUS,
      options,
      localize,
      formatLong,
    };
  }

  return (
    <Popover>
      <PopoverTrigger asChild disabled={disabled}>
        <div className="relative flex flex-1 w-0 items-center">
          <Button
            type="button"
            disabled={disabled}
            className={cn(
              dateTimePickerVariants({ icon: true }),
              "justify-start w-full",
              className
            )}
            ref={buttonRef}
          >
            {displayDate ? (
              format(
                displayDate,
                hourCycle === 24
                  ? initHourFormat.hour24
                  : initHourFormat.hour12,
                { locale: loc }
              )
            ) : (
              <span>{placeholder}</span>
            )}
          </Button>

          {clearable && displayDate && (
            <Button
              variant="ghost"
              size={"sm"}
              square
              className={cn([
                "absolute right-1 opacity-70 hover:opacity-100 hover:bg-transparent transition-opacity",
              ])}
              onClick={handleClear}
            >
              <CircleXIcon className="size-4" />
            </Button>
          )}
        </div>
      </PopoverTrigger>
      <PopoverContent className="w-auto p-0">
        <Calendar
          mode="single"
          selected={displayDate}
          month={month}
          onSelect={(newDate) => {
            if (newDate) {
              newDate.setHours(
                month?.getHours() ?? 0,
                month?.getMinutes() ?? 0,
                month?.getSeconds() ?? 0
              );
              onSelect(newDate);
            }
          }}
          onMonthChange={handleMonthChange}
          locale={locale}
          {...props}
        />
        {granularity !== "day" && (
          <div className="border-border border-t p-3">
            <TimePicker
              onChange={(newTime) => {
                if (!newTime) return;
                const current = displayDate ?? month ?? new Date();
                const merged = new Date(current);
                merged.setHours(
                  newTime.getHours(),
                  newTime.getMinutes(),
                  newTime.getSeconds()
                );
                setDisplayDate(merged);
                setMonth(merged);
                onChange?.(merged);
              }}
              date={displayDate ?? month}
              hourCycle={hourCycle}
              granularity={granularity}
            />
          </div>
        )}
      </PopoverContent>
    </Popover>
  );
}

export { DateTimePicker, TimePickerInput, TimePicker };
export type { TimePickerType, DateTimePickerProps, DateTimePickerRef };
