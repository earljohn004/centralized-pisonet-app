import { createBrowserRouter, RouterProvider } from "react-router-dom";
import { MainWindow, Settings, SmallWindow } from "./window";

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
    element: <Settings />,
  },
]);

export default function AppRouter() {
  return <RouterProvider router={router} />;
}
