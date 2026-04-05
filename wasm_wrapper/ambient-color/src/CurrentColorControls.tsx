import { createMemo, type Component, type JSX, type Signal } from "solid-js";
import { makeDayOfYear, makeSolarTime, type DayOfYear, type SolarTime } from "./wasm.schema";

type Props = {
  dayOfYearSignal: Signal<DayOfYear>
  temperatureSignal: Signal<number>
  timeSignal: Signal<SolarTime>
  maxTemperature: number
  minTemperature: number
};

export const Controls: Component<Props> = (props) => {
  const [dayOfYear, setDayOfYear] = props.dayOfYearSignal;
  const [temperature, setTemperature] = props.temperatureSignal;
  const [time, setTime] = props.timeSignal;

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

  const currentTimeMs = createMemo(() => {
    const [hours, minutes, seconds] = time().split(":").map(s => parseInt(s));

    if (hours === undefined || minutes === undefined || seconds === undefined) {
      throw new Error("Failed to parse time signal into date");
    }

    const ms = hours * 60 * 60 * 1000 + minutes * 60 * 1000 + seconds * 1000;

    return ms;
  });

  const currentTimeString = createMemo(() => {
    const now = new Date();
    const date = new Date(now.getFullYear(), now.getMonth(), now.getDate(), 0, 0, 0, currentTimeMs());

    return date.toLocaleTimeString("it-IT", {
      hour: "2-digit",
      minute: "2-digit",
    });
  });

  const handleDayOfYearChange: JSX.InputEventHandler<HTMLInputElement, InputEvent> = (e) => {
    try {
      const dayOfYear = makeDayOfYear(parseInt(e.currentTarget.value));

      setDayOfYear(dayOfYear);
    }
    catch {}
  };

  const handleTemperatureChange: JSX.InputEventHandler<HTMLInputElement, InputEvent> = (e) => {
    const temperature = parseInt(e.currentTarget.value);

    if (Number.isNaN(temperature)) return;

    setTemperature(temperature);
  };

  const handleTimeChange: JSX.InputEventHandler<HTMLInputElement, InputEvent> = (e) => {
    const now = new Date();
    const ms = parseInt(e.currentTarget.value);
    const date = new Date(now.getFullYear(), now.getMonth(), now.getDate(), 0, 0, 0, ms);
    const time = makeSolarTime(date);

    setTime(time);
  };

  return (
    <div class="p-4 flex flex-col gap-5">
      <h2 class="text-xl">Controls</h2>
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
      <div class="grid grid-cols-[auto_1fr_10em] gap-3">
        <h3>Temperature</h3>
        <input
          type="range"
          min={props.minTemperature}
          max={props.maxTemperature}
          step={1}
          value={Math.max(Math.min(temperature(), props.maxTemperature), props.minTemperature)}
          oninput={handleTemperatureChange}
          class="range w-[100%]"
        />
        <p>{temperature()}</p>
      </div>
      <div class="grid grid-cols-[auto_1fr_10em] gap-3">
        <h3>Time</h3>
        <input
          type="range"
          min={0}
          max={(1000 * 60 * 60 * 24) - 1}
          step={60000}
          value={currentTimeMs()}
          oninput={handleTimeChange}
          class="range w-[100%]"
        />
        <p>{currentTimeString()}</p>
      </div>
    </div>
  );
};
