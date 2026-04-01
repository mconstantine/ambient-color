import type { Component, JSX, Signal } from "solid-js";
import { config } from "./config";
import { timeFromTimestamp } from "./timeFromTimestamp";

type Props = {
  temperature: Signal<number>
  dayOfYear: Signal<number>
  time: Signal<number>
};

export const Controls: Component<Props> = (props) => {
  const [temperature, setTemperature] = props.temperature;
  const [dayOfYear, setDayOfYear] = props.dayOfYear;
  const [time, setTime] = props.time;

  const handleTemperatureChange: JSX.ChangeEventHandler<HTMLInputElement, Event> = (event) => {
    const temperature = parseInt(event.currentTarget.value);
    if (Number.isNaN(temperature)) return;
    setTemperature(temperature);
  };

  const handleDayOfYearChange: JSX.ChangeEventHandler<HTMLInputElement, Event> = (event) => {
    const dayOfYear = parseInt(event.currentTarget.value);
    if (Number.isNaN(dayOfYear)) return;
    setDayOfYear(dayOfYear);
  };

  const handleTimeChange: JSX.ChangeEventHandler<HTMLInputElement, Event> = (event) => {
    const time = parseInt(event.currentTarget.value);
    if (Number.isNaN(time)) return;
    setTime(time);
  };

  return (
    <div class="flex flex-col max-w-[360px] mx-auto my-6 gap-6">
      <div class="flex flex-col gap-2">
        <p>Temperature</p>
        <div class="flex justify-between ">
          <input
            type="range"
            min={config.minTemperature}
            max={config.maxTemperature}
            step="1"
            value={temperature()}
            onchange={handleTemperatureChange}
            class="range"
          />
          <p>{temperature()}</p>
        </div>
        <div class="flex justify-between">
          <p>Cold</p>
          <p>Hot</p>
        </div>
      </div>
      <div class="flex flex-col gap-2">
        <p>Day of the year</p>
        <div class="flex justify-between ">
          <input
            type="range"
            min={0}
            max={365}
            step="1"
            value={dayOfYear()}
            onchange={handleDayOfYearChange}
            class="range"
          />
          <p>{dayOfYear()}</p>
        </div>
        <div class="flex justify-between">
          <p>January</p>
          <p>December</p>
        </div>
      </div>
      <div class="flex flex-col gap-2">
        <p>Time of the day</p>
        <div class="flex justify-between ">
          <input
            type="range"
            min={0}
            max={60 * 60 * 24}
            step="1"
            value={time()}
            onchange={handleTimeChange}
            class="range"
          />
          <p>{timeFromTimestamp(time()).slice(0, 5)}</p>
        </div>
        <div class="flex justify-between">
          <p>Midnight</p>
          <p>Midnight</p>
        </div>
      </div>
    </div>
  );
};
