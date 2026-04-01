import { createEffect, createMemo, createResource, createSignal, type Component } from "solid-js";
import { computeTheme, type ComputeThemeInput } from "./computeTheme";
import { CurrentTheme } from "./CurrentTheme";
import { Controls } from "./Controls";
import { timeFromTimestamp } from "./timeFromTimestamp";
import { config } from "./config";

const App: Component = () => {
  const temperatureSignal = createSignal(config.initialTemperature); // degrees Celsius
  const timeSignal = createSignal(60 * 60 * 12); // seconds from midnight
  const dayOfYearSignal = createSignal(150); // 0 to 365

  const [temperature] = temperatureSignal;
  const [time] = timeSignal;
  const [dayOfYear] = dayOfYearSignal;

  const input = createMemo<ComputeThemeInput>(() => ({
    max_temperature: config.maxTemperature,
    min_temperature: config.minTemperature,
    temperature: temperature(),
    day_of_year: dayOfYear(),
    now: timeFromTimestamp(time()),
    sunrise_time: "06:30:15",
    sunset_time: "19:24:12",
  }));

  const [theme] = createResource(input, computeTheme);

  createEffect(() => {
    console.log(temperature());
  });

  return (
    <>
      <Controls
        temperature={temperatureSignal}
        time={timeSignal}
        dayOfYear={dayOfYearSignal}
      />
      {() => {
        switch (theme.state) {
          case "unresolved":
            return null;
          case "pending":
          case "refreshing":
            return (
              <div class="flex justify-center mt-4">
                <span class="loading loading-infinity loading-xl" />
              </div>
            );
          case "errored":
            return (
              <div role="alert" class="alert alert-error">
                <ion-icon name="bug-outline"></ion-icon>
                <span>
                  There's been an error during the generation of the theme.
                  The console may contain additional information.
                </span>
              </div>
            );
          case "ready":
            return <CurrentTheme theme={theme()} />;
        }
      }}
    </>
  );
};

export default App;
