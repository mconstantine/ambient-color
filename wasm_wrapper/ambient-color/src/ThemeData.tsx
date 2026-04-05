import type { Component } from "solid-js";
import type { Theme } from "./wasm.schema";

export type ThemeDataVariant = "partial" | "full";

type Props = {
  theme: Theme
  variant: ThemeDataVariant
};

export const ThemeData: Component<Props> = (props) => {
  return (
    <div class="p-4">
      <h2 class="text-xl">Data</h2>
      <table class="table max-w-[512px] mx-auto">
        <tbody>
          <tr>
            <td>Day of the year</td>
            <td>{props.theme.day_of_year}</td>
          </tr>
          {props.variant === "full"
            && (
              <>
                <tr>
                  <td>Min daily temperature (Celsius)</td>
                  <td>{props.theme.weather_data.min_temperature}</td>
                </tr>
                <tr>
                  <td>Max daily temperature (Celsius)</td>
                  <td>{props.theme.weather_data.max_temperature}</td>
                </tr>
                <tr>
                  <td>Current temperature (Celsius)</td>
                  <td>{props.theme.weather_data.temperature}</td>
                </tr>
                <tr>
                  <td>Sunrise time</td>
                  <td>
                    {props.theme.weather_data.sunrise_time.toLocaleTimeString("it-IT", {
                      hour: "2-digit",
                      minute: "2-digit",
                    })}
                  </td>
                </tr>
                <tr>
                  <td>Sunset time</td>
                  <td>
                    {props.theme.weather_data.sunset_time.toLocaleTimeString("it-IT", {
                      hour: "2-digit",
                      minute: "2-digit",
                    })}
                  </td>
                </tr>
                <tr>
                  <td>Current time</td>
                  <td>
                    {new Date().toLocaleTimeString("it-IT", {
                      hour: "2-digit",
                      minute: "2-digit",
                    })}
                  </td>
                </tr>
              </>
            )}
          <tr>
            <td>Current hue (degrees)</td>
            <td>{props.theme.color_data.hue.toFixed(2)}</td>
          </tr>
          {props.variant === "full" && (
            <>
              <tr>
                <td>Current chroma</td>
                <td>{(props.theme.color_data.chroma * 100).toFixed(2)}</td>
              </tr>
              <tr>
                <td>Current luma</td>
                <td>{(props.theme.color_data.luma * 100).toFixed(2)}</td>
              </tr>
            </>
          )}
        </tbody>
      </table>
    </div>
  );
};
