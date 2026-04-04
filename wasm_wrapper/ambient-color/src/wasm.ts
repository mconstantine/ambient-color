import init, { compute_theme_web, generate_theme_web } from "wasm_wrapper";
import { ComputeThemeInput, ThemeResponse, type Theme } from "./wasm.schema";
import { Schema } from "effect";

export async function generateTheme(): Promise<Theme> {
  await init();

  const response = await generate_theme_web() as unknown;

  try {
    const decode = Schema.decodeUnknownSync(ThemeResponse);
    const themeResponse = decode(response);

    return handleThemeResponse(themeResponse);
  }
  catch (e) {
    if (e instanceof Error) {
      console.log(e.message);
    }

    throw e;
  }
}

export async function computeTheme(input: ComputeThemeInput): Promise<Theme> {
  await init();

  const encode = Schema.encodeSync(ComputeThemeInput);
  const encoded = encode(input);
  const response = await compute_theme_web(encoded) as unknown;

  try {
    const decode = Schema.decodeUnknownSync(ThemeResponse);
    const themeResponse = decode(response);

    return handleThemeResponse(themeResponse);
  }
  catch (e) {
    if (e instanceof Error) {
      console.log(e.message);
    }

    throw e;
  }
}

function handleThemeResponse(response: ThemeResponse): Theme {
  switch (response.status) {
    case "NetworkError":
      throw new Error("Rust could not connect to the network. Check your connection, and try reloading the page");
    case "ParseError":
      throw new Error("Rust could not parse the weather data. Did wttr.in change their data format?");
    case "InvalidInput":
      throw new Error("Rust could not parse the input. Check your schemas.");
    case "Ok":
      return response.data;
  }
}
