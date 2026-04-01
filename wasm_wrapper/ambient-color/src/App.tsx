import type { Component } from "solid-js";

import styles from "./App.module.css";

const App: Component = () => {
  return (
    <div class={styles["App"]}>
      <p>Hello World!</p>
    </div>
  );
};

export default App;
