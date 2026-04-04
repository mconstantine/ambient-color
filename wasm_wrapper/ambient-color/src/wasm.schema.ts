import { ParseResult, Schema } from "effect";

const DayOfYear = Schema.Int
  .pipe(Schema.positive())
  .pipe(Schema.lessThanOrEqualTo(366))
  .pipe(Schema.brand("DayOfYear"));

export type DayOfYear = typeof DayOfYear.Type;

export function makeDayOfYear(n: number): DayOfYear {
  const decode = Schema.decodeSync(DayOfYear);
  const result = decode(n);

  return result;
}

const timePattern = /^([0-2]\d):([0-5]\d):([0-5]\d)$/;

const SolarTime = Schema.NonEmptyTrimmedString
  .pipe(Schema.pattern(timePattern))
  .pipe(Schema.brand("SolarTime"));

export type SolarTime = typeof SolarTime.Type;

export function makeSolarTime(date: Date): SolarTime {
  const timeString = date.toLocaleTimeString("it-IT", {
    hour: "2-digit",
    minute: "2-digit",
    second: "2-digit",
  });

  const decode = Schema.decodeSync(SolarTime);
  const result = decode(timeString);

  return result;
}

export const DateFromSolarTime = Schema.transformOrFail(
  SolarTime,
  Schema.DateFromSelf,
  {
    decode: (from: string, _options, ast) => {
      const match = from.match(timePattern);

      if (match === null) {
        return ParseResult.fail(new ParseResult.Type(ast, from, "Expected a string in the format HH:mm:ss"));
      }

      const [, hours, minutes, seconds] = match;

      if (hours === undefined || minutes === undefined || seconds === undefined) {
        return ParseResult.fail(new ParseResult.Type(ast, from, "Expected a string in the format HH:mm:ss"));
      }

      const hoursNum = parseInt(hours);
      const minutesNum = parseInt(minutes);
      const secondsNum = parseInt(seconds);

      if (Number.isNaN(hoursNum)) return ParseResult.fail(new ParseResult.Type(ast, hours, "Expected an integer"));
      if (Number.isNaN(minutesNum)) return ParseResult.fail(new ParseResult.Type(ast, minutes, "Expected an integer"));
      if (Number.isNaN(secondsNum)) return ParseResult.fail(new ParseResult.Type(ast, seconds, "Expected an integer"));

      const now = new Date();

      return ParseResult.succeed(new Date(Date.UTC(
        now.getUTCFullYear(),
        now.getUTCMonth(),
        now.getUTCDate(),
        hoursNum,
        minutesNum,
        secondsNum,
      )));
    },
    encode: (to: Date) => ParseResult.succeed(to.toLocaleTimeString("it-IT", {
      hour: "2-digit",
      minute: "2-digit",
      second: "2-digit",
    }) as SolarTime),
    strict: true,
  },
);

export const ComputeThemeInput = Schema.Struct({
  max_temperature: Schema.Int,
  min_temperature: Schema.Int,
  temperature: Schema.Int,
  sunrise_time: DateFromSolarTime,
  sunset_time: DateFromSolarTime,
  day_of_year: DayOfYear,
  now: DateFromSolarTime,
});
export type ComputeThemeInput = typeof ComputeThemeInput.Type;

const Color = Schema.NonEmptyTrimmedString.pipe(Schema.pattern(/^#[A-F0-9]{6}$/)).pipe(Schema.brand("Color"));

const PaletteColorVariant = Schema.Struct({
  bg: Color,
  fg: Color,
});
export type PaletteColorVariant = typeof PaletteColorVariant.Type;

const PaletteColor = Schema.Struct({
  w50: PaletteColorVariant,
  w100: PaletteColorVariant,
  w200: PaletteColorVariant,
  w300: PaletteColorVariant,
  w400: PaletteColorVariant,
  w500: PaletteColorVariant,
  w600: PaletteColorVariant,
  w700: PaletteColorVariant,
  w800: PaletteColorVariant,
  w900: PaletteColorVariant,
  w950: PaletteColorVariant,
});
export type PaletteColor = typeof PaletteColor.Type;

const WeatherData = ComputeThemeInput.pipe(Schema.pick(
  "max_temperature",
  "min_temperature",
  "temperature",
  "sunrise_time",
  "sunset_time",
));

export const Theme = Schema.Struct({
  weather_data: WeatherData,
  original_color: PaletteColorVariant,
  primary_palette: PaletteColor,
  opposite_palette: PaletteColor,
  secondary_palette: PaletteColor,
  tertiary_palette: PaletteColor,
  neutral_palette: PaletteColor,
});
export type Theme = typeof Theme.Type;

const ThemeResponseFailureStatus = Schema.Literal("NetworkError", "ParseError", "InvalidInput");
const ThemeResponseFailure = Schema.Struct({ status: ThemeResponseFailureStatus });

const ThemeResponseSuccess = Schema.Struct({
  status: Schema.Literal("Ok"),
  data: Theme,
});

export const ThemeResponse = Schema.Union(ThemeResponseFailure, ThemeResponseSuccess);
export type ThemeResponse = typeof ThemeResponse.Type;
