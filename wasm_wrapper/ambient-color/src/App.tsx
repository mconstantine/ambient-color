import { createSignal, type Component } from "solid-js";
import { Simulator } from "./Simulator";
import { CurrentColor } from "./CurrentColor";

type View = "simulator" | "current-color";

const App: Component = () => {
  const [currentView, setCurrentView] = createSignal<View>("current-color");

  function switchView(view: View): void {
    if (currentView() === view) return;

    setCurrentView(view);
  }

  return (
    <div>
      <nav class="navbar shadow-sm">
        <ul class="menu menu-horizontal gap-2">
          <li>
            <button
              class={currentView() === "simulator" ? "btn btn-neutral" : "btn"}
              onclick={() => { switchView("simulator"); }}
            >
              Simulator
            </button>
          </li>
          <li>
            <button
              class={currentView() === "current-color" ? "btn btn-neutral" : "btn"}
              onclick={() => { switchView("current-color"); }}
            >
              Current color
            </button>
          </li>
        </ul>
      </nav>
      {(() => {
        switch (currentView()) {
          case "simulator":
            return <Simulator />;
          case "current-color":
            return <CurrentColor />;
        }
      })()}
    </div>
  );
};

export default App;
