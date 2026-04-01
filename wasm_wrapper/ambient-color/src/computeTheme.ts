import init, { compute_theme_web } from "wasm_wrapper";

/**
 * Example:
 * ```ts
 * {
 *   max_temperature: 20
 *   min_temperature: 5
 *   temperature: 15
 *   day_of_year: 42
 *   sunrise_time: "06:30:00"
 *   sunset_time: "19:30:00"
 *   now: "12:30:42"
 * }
 * ```
 */
export type ComputeThemeInput = {
  max_temperature: number
  min_temperature: number
  temperature: number
  day_of_year: number
  sunrise_time: string
  sunset_time: string
  now: string
};

export type Theme = {
  background_color: string
  foreground_color: string
};

type ComputeThemeNetworkError = {
  status: "NetworkError"
};

type ComputeThemeParseError = {
  status: "ParseError"
};

type ComputeThemePaletteDataParseError = {
  status: "PaletteDataParseError"
};

type ComputeThemeInvalidInput = {
  status: "InvalidInput"
};

type ComputeThemeSuccessfulResult = {
  status: "Ok"
  data: Theme
};

type ComputeThemeOutput =
  | ComputeThemeNetworkError
  | ComputeThemeParseError
  | ComputeThemePaletteDataParseError
  | ComputeThemeInvalidInput
  | ComputeThemeSuccessfulResult;

export const computeTheme = async (input: ComputeThemeInput): Promise<Theme> => {
  await init();

  const output = await compute_theme_web(input) as ComputeThemeOutput;

  switch (output.status) {
    case "NetworkError":
      throw new Error("Network error from Rust. Try again later.");
    case "ParseError":
      throw new Error("Parse error from Rust. Run the script from the terminal to see what's up.");
    case "PaletteDataParseError":
      throw new Error("Rust failed parsing the palette. Run the script from the terminal to see what's up.");
    case "InvalidInput":
      throw new Error("Invalid input sent to Rust. Check your code.");
    case "Ok":
      return output.data;
  }
};
