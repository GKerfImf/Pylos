import React from "react";
import Menu from "./components/interface/menu";
import ActiveGame from "./components/interface/active-game";
import PylosCanvas from "./components/game/canvas";
import { WebSocketProvider } from "./contexts/ws-context";

import { createBrowserRouter, RouterProvider } from "react-router-dom";
import { Toaster } from "./components/ui/toaster";

export default function App() {
  const router = createBrowserRouter([
    {
      path: "/",
      element: (
        <div className="absolute inset-0 flex justify-center items-center">
          <Menu />
          <Toaster />
        </div>
      ),
    },
    {
      path: "/games/:id",
      element: (
        <div className="w-full h-full">
          <PylosCanvas />
          <div className="absolute right-0 top-0">
            <ActiveGame />
          </div>
          <Toaster />
        </div>
      ),
    },
  ]);

  return (
    <div className="w-full h-full">
      <WebSocketProvider>
        <RouterProvider router={router} />
      </WebSocketProvider>
    </div>
  );
}
