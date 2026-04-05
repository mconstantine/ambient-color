import { createMemo, type Component, type JSX, type Signal } from "solid-js";
import { makeDayOfYear, type DayOfYear } from "./wasm.schema";

type Props = {
  dayOfYearSignal: Signal<DayOfYear>
};

export const Controls: Component<Props> = (props) => {
  const [dayOfYear, setDayOfYear] = props.dayOfYearSignal;

  const currentDate = createMemo(() => {
    const now = new Date();

    return new Date(
      new Date(now.getFullYear(), 0, 1)
        .getTime() + 1000 * 60 * 60 * 24 * dayOfYear(),
    )
      .toLocaleDateString("en-US", {
        month: "long",
        day: "2-digit",
      });
  });

  const handleDayOfYearChange: JSX.InputEventHandler<HTMLInputElement, InputEvent> = (e) => {
    try {
      const dayOfYear = makeDayOfYear(parseInt(e.currentTarget.value));

      setDayOfYear(dayOfYear);
    }
    catch {}
  };

  return (
    <div class="p-4">
      <h2 class="text-xl mb-5">Controls</h2>
      <div class="grid grid-cols-[auto_1fr_10em] gap-3">
        <h3>Day of year</h3>
        <input
          type="range"
          min={1}
          max={366}
          value={dayOfYear().toString(10)}
          oninput={handleDayOfYearChange}
          class="range w-[100%]"
        />
        <p>
          {dayOfYear().toString(10)}
          {" "}
          (
          {currentDate()}
          )
        </p>
      </div>
    </div>
  );
};
