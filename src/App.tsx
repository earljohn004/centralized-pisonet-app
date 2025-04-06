import "./App.css";
import { useEventListeners } from "./hooks/useEventListeners";
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
