import { mount } from "nsh";
import icon from "./assets/icon.png";
import { Store } from "./src/store";
import { AppView } from "./src/views";

// One store for the app's lifetime; its signals drive every dynamic area of
// the view. The native scanner is imported directly by src/scan.ts.
function App() {
  const store = new Store();
  store.loadAppearance();
  store.search();
  store.refreshProcesses();
  return <AppView store={store} icon={icon} />;
}

mount(App);
