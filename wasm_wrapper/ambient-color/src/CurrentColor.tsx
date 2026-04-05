import { createEffect, createMemo, createResource, createSignal, type Component } from "solid-js";
import { computeTheme, generateTheme } from "./wasm";
import { type ComputeThemeInput, DateFromSolarTime, makeSolarTime, type Theme } from "./wasm.schema";
import { PaletteColorVariantDisplay, ThemeDisplay } from "./ThemeDisplay";
import { Controls } from "./CurrentColorControls";
import { Schema } from "effect";

export const CurrentColor: Component = () => {
  const [theme] = createResource(() => ({}), generateTheme);

  return (
    <div>
      {(() => {
        switch (theme.state) {
          case "unresolved":
          case "pending":
          case "errored":
            return null;
          case "ready":
          case "refreshing":
            return <CurrentColorDisplay theme={theme()} />;
        }
      })()}
    </div>
  );
};

const CurrentColorDisplay: Component<{ theme: Theme }> = (props) => {
  const dayOfYearSignal = createSignal(props.theme.day_of_year);
  const temperatureSignal = createSignal(props.theme.weather_data.temperature);
  const timeSignal = createSignal(makeSolarTime(new Date()));

  const [dayOfYear] = dayOfYearSignal;
  const [temperature] = temperatureSignal;
  const [time] = timeSignal;

  const now = createMemo(() => {
    const decode = Schema.decodeSync(DateFromSolarTime);

    return decode(time());
  });

  const input = createMemo<ComputeThemeInput>(() => ({
    max_temperature: props.theme.weather_data.max_temperature,
    min_temperature: props.theme.weather_data.min_temperature,
    temperature: temperature(),
    day_of_year: dayOfYear(),
    now: now(),
    sunrise_time: props.theme.weather_data.sunrise_time,
    sunset_time: props.theme.weather_data.sunset_time,
  }));

  const [computedTheme] = createResource(input, computeTheme);
  const [currentTheme, setCurrentTheme] = createSignal(props.theme);

  createEffect(() => {
    if (computedTheme.state !== "ready") return;

    setCurrentTheme(computedTheme());
  });

  return (
    <div>
      <Controls
        dayOfYearSignal={dayOfYearSignal}
        temperatureSignal={temperatureSignal}
        timeSignal={timeSignal}
        maxTemperature={props.theme.weather_data.max_temperature}
        minTemperature={props.theme.weather_data.min_temperature}
      />
      <div class="p-4">
        <h2 class="text-xl mb-4">Current color</h2>
        <PaletteColorVariantDisplay name="CURRENT" variant={currentTheme().original_color} />
      </div>
      <ThemeDisplay theme={currentTheme()} variant="full" />
    </div>
  );
};
