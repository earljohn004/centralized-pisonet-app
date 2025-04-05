import "./App.css";
import { useEventListeners } from "./store/Settings";
import AppRouter from "./Router";

function App() {
  useEventListeners();

  return (
    <>
      <main className="container">
        <AppRouter />
      </main>
    </>
  );
}

export default App;
