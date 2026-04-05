import { type Accessor, createMemo, createResource, createSignal, type Component } from "solid-js";
import { type DayOfYear, makeDayOfYear, type ComputeThemeInput } from "./wasm.schema";
import { computeTheme } from "./wasm";
import { ThemeDisplay } from "./ThemeDisplay";
import { Controls } from "./SimulatorControls";

export const Simulator: Component = () => {
  const dayOfYearSignal = createSignal(getDayOfYear());
  const [dayOfYear] = dayOfYearSignal;

  const input: Accessor<ComputeThemeInput> = createMemo(() => ({
    now: new Date(),
    day_of_year: dayOfYear(),
    min_temperature: 0,
    max_temperature: 0,
    sunrise_time: new Date(),
    sunset_time: new Date(),
    temperature: 0,
  }));

  const [theme] = createResource(input, computeTheme);

  return (
    <div>
      <Controls dayOfYearSignal={dayOfYearSignal} />
      {(() => {
        switch (theme.state) {
          case "unresolved":
          case "pending":
          case "errored":
            return null;
          case "refreshing":
          case "ready":
            return <ThemeDisplay theme={theme()} variant="partial" />;
        }
      })()}
    </div>
  );
};

function getDayOfYear(): DayOfYear {
  const now = new Date();
  const startOfYear = new Date(now.getFullYear(), 0, 1);
  const differenceMs = now.getTime() - startOfYear.getTime();
  const differenceDays = differenceMs / (1000 * 60 * 60 * 24);

  return makeDayOfYear(Math.floor(differenceDays) + 1);
}
