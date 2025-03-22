import { createBrowserRouter, RouterProvider } from "react-router-dom";
import { MainWindow, SmallWindow } from "./window";

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
]);

export default function AppRouter() {
  return <RouterProvider router={router} />;
}
