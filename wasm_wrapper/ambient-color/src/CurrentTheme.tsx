import styles from "./CurrentTheme.module.css";
import type { Component } from "solid-js";
import type { Theme } from "./computeTheme";

type Props = {
  theme: Theme
};

export const CurrentTheme: Component<Props> = (props) => {
  return (
    <div class={styles["CurrentTheme"]}>
      <h1 class="text-2xl">Current theme</h1>
      <div style={{ "background-color": props.theme.background_color }}>
        <p style={{ color: props.theme.foreground_color }}>A</p>
      </div>
    </div>
  );
};
