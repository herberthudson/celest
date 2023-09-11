import { invoke } from "@tauri-apps/api/tauri";
import { Show, createEffect, createSignal } from "solid-js";
import "./App.css";
// import { listen } from "@tauri-apps/api/event";

function App() {
  const [isConfigured, setIsConfigured] = createSignal(false);

  // const cargoEvent = listen("CargoEvent", (event) => {
  //   console.log(event);
  // });

  createEffect(async () => {
    setIsConfigured(await invoke("is_configured"));
  })


  return (
    <div class="container">
      <Show when={isConfigured()} fallback={<h1>Not configured</h1>}>
        <h1>Configured</h1>
      </Show>
    </div>
  );
}

export default App;
