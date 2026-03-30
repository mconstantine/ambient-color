const colors = require("tailwindcss/colors")
const { Schema } = require("effect")
const fs = require("fs");

const Color = Schema.Struct({
  50: Schema.NonEmptyTrimmedString,
  100: Schema.NonEmptyTrimmedString,
  200: Schema.NonEmptyTrimmedString,
  300: Schema.NonEmptyTrimmedString,
  400: Schema.NonEmptyTrimmedString,
  500: Schema.NonEmptyTrimmedString,
  600: Schema.NonEmptyTrimmedString,
  700: Schema.NonEmptyTrimmedString,
  800: Schema.NonEmptyTrimmedString,
  900: Schema.NonEmptyTrimmedString,
  950: Schema.NonEmptyTrimmedString,
});

const decode = Schema.decodeUnknownSync(Schema.Record({
  key: Schema.NonEmptyTrimmedString,
  value: Color,
}));

const parsed = decode(Object.fromEntries(
  Object.entries(colors)
    .filter(([, value]) => typeof value === "object" && value !== null)
));

fs.writeFileSync("./palette.json", JSON.stringify(parsed, null, 2), "utf-8");
