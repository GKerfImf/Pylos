import React from "react";
import Menu from "./components/interface/menu";
import PylosCanvas from "./components/canvas";
import { WebSocketProvider } from "./contexts/ws-context";

import { createBrowserRouter, RouterProvider } from "react-router-dom";

const ActiveGame: React.FC = () => {
  return (
    <div className="w-72 h-4/6 p-3 border rounded-xl bg-slate-900 border-slate-900 shadow-lg shadow-black flex-row justify-center items-center overflow-scroll scrollbar-hide">
      {/*  */}

      <div className=" flex  space-x-4 rounded-md p-2 mb-1 bg-slate-800 ">
        <div className="flex-1 space-y-1">
          <p className="text-sm font-bold leading-none text-muted-foreground">anon304</p>
          <p className="text-sm text-muted-foreground">Send notifications to device.</p>
          <p className="text-sm text-muted-foreground">Send notifications to device.</p>
          <p className="text-sm text-muted-foreground">Send notifications to device.</p>
        </div>
        <div className="h-10 w-10 rounded-full bg-slate-100" />
      </div>

      <div className=" flex space-x-4 rounded-md p-2 bg-slate-200">
        <div className="flex-1 space-y-1">
          <p className="text-sm font-bold leading-none text-muted-foreground">anon534</p>
          <p className="text-sm text-muted-foreground">Send notifications to device.</p>
          <p className="text-sm text-muted-foreground">Send notifications to device.</p>
          <p className="text-sm text-muted-foreground">Send notifications to device.</p>
          <p className="text-sm text-muted-foreground">Send notifications to device.</p>
          <p className="text-sm text-muted-foreground">Send notifications to device.</p>
        </div>
        <div className="h-10 w-10 rounded-full bg-slate-900" />
      </div>

      {/*  */}
    </div>
  );
};

export default function App() {
  const router = createBrowserRouter([
    {
      path: "/",
      element: (
        <div className="absolute inset-0 flex justify-center items-center">
          <Menu />
        </div>
      ),
    },
    // other pages....
    {
      path: "/game",
      element: <PylosCanvas />,
    },
  ]);

  return (
    <div className="w-full h-full">
      <WebSocketProvider>
        <RouterProvider router={router} />
        {/* <PylosCanvas />
        <div className="absolute inset-0 flex justify-center items-center">
          <Menu />
        </div> */}
        {/* <div className="absolute right-0 bottom-0">
        <ActiveGame />
      </div> */}
      </WebSocketProvider>
    </div>
  );
}
