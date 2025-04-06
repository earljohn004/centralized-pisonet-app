import { createBrowserRouter, RouterProvider } from "react-router-dom";
import { MainWindow, SettingsWindow, SmallWindow } from "./window";

export const router = createBrowserRouter([
  {
    path: "/",
    element: <MainWindow />,
  },
  {
    path: "/show_main",
    element: <MainWindow />,
  },
  {
    path: "/show_small",
    element: <SmallWindow />,
  },
  {
    path: "/settings",
    element: <SettingsWindow />,
  },
]);

export default function AppRouter() {
  return <RouterProvider router={router} />;
}
