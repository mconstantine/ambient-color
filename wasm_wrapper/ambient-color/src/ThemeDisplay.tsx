import type { Component } from "solid-js";
import type { PaletteColor, PaletteColorVariant, Theme } from "./wasm.schema";
import { ThemeData, type ThemeDataVariant } from "./ThemeData";

type Props = {
  theme: Theme
  variant: ThemeDataVariant
};

export const ThemeDisplay: Component<Props> = (props) => {
  return (
    <div>
      <ThemeData theme={props.theme} variant={props.variant} />
      <div
        class="p-4 min-h-[100vh]"
        style={{
          "background-color": props.theme.neutral_palette.w950.bg,
          color: props.theme.primary_palette.w950.fg,
        }}
      >
        <h2 class="text-xl mb-5">Generated theme</h2>
        <PaletteColorDisplay palette={props.theme.primary_palette} name="Primary" />
        <PaletteColorDisplay palette={props.theme.opposite_palette} name="Opposite" />
        <PaletteColorDisplay palette={props.theme.secondary_palette} name="Secondary" />
        <PaletteColorDisplay palette={props.theme.tertiary_palette} name="Tertiary" />
        <PaletteColorDisplay palette={props.theme.neutral_palette} name="Neutral" />
      </div>
    </div>
  );
};

type PaletteColorDisplayProps = {
  name: string
  palette: PaletteColor
};

const PaletteColorDisplay: Component<PaletteColorDisplayProps> = (props) => {
  return (
    <div class="mb-5">
      <h3 class="text-lg mb-2">{props.name}</h3>
      <div class="flex">
        <PaletteColorVariantDisplay variant={props.palette.w50} name="50" />
        <PaletteColorVariantDisplay variant={props.palette.w100} name="100" />
        <PaletteColorVariantDisplay variant={props.palette.w200} name="200" />
        <PaletteColorVariantDisplay variant={props.palette.w300} name="300" />
        <PaletteColorVariantDisplay variant={props.palette.w400} name="400" />
        <PaletteColorVariantDisplay variant={props.palette.w500} name="500" />
        <PaletteColorVariantDisplay variant={props.palette.w600} name="600" />
        <PaletteColorVariantDisplay variant={props.palette.w700} name="700" />
        <PaletteColorVariantDisplay variant={props.palette.w800} name="800" />
        <PaletteColorVariantDisplay variant={props.palette.w900} name="900" />
        <PaletteColorVariantDisplay variant={props.palette.w950} name="950" />
      </div>
    </div>
  );
};

type PaletteColorVariantDisplayProps = {
  name: string
  variant: PaletteColorVariant
};

export const PaletteColorVariantDisplay: Component<PaletteColorVariantDisplayProps> = (props) => {
  return (
    <div
      class="w-[96px] h-[96px] p-2"
      style={{
        "background-color": props.variant.bg,
        color: props.variant.fg,
      }}
    >
      <p>{props.name}</p>
    </div>
  );
};
