import React, { useContext, useEffect, useState } from "react";
import Menu from "./components/interface/menu";
import PylosCanvas from "./components/canvas";
import { WebSocketContext, WebSocketProvider } from "./contexts/ws-context";

import { createBrowserRouter, RouterProvider } from "react-router-dom";
import Player from "./types/player";
import { cn } from "./util/cn";
import { TGameState, TJoinGame } from "./types/response";
import { Toaster } from "./components/ui/toaster";

const ActiveGame: React.FC = () => {
  const [currentTurn, setCurrentTurn] = useState(Player.White);
  const { subscribe, unsubscribe, send } = useContext(WebSocketContext)!;

  // TODO: check who is playing white/black

  // Listen to the game state to know whose turn it is
  useEffect(() => {
    subscribe("GameState", "ActiveGame", (req: TGameState) => {
      setCurrentTurn(req.GameState.game_state.turn);
    });
    return () => {
      unsubscribe("GameState", "ActiveGame");
    };
  }, []);

  // Listen to the clients that join the game to display names
  useEffect(() => {
    subscribe("JoinGame", "ActiveGame", (req: TJoinGame) => {
      send({
        GetClientName: {
          client_uuid: req.JoinGame.client_uuid,
        },
      });
    });
    return () => {
      unsubscribe("JoinGame", "ActiveGame");
    };
  }, []);

  // TODO: listen to [ChangeName] to keep track of players' names
  // TODO: useEffect( () => {} )

  return (
    <div className="w-72 h-4/6 p-3 border rounded-xl bg-slate-900 border-slate-900 shadow-lg shadow-black flex-row justify-center items-center overflow-scroll scrollbar-hide">
      {/*  */}
      <div
        className={cn("flex space-x-4 rounded-md p-2 bg-slate-200 border-2 border-black", {
          "border-cyan-400": currentTurn == Player.White,
        })}
      >
        <div className="flex-1 space-y-1">
          <p className="text-sm font-bold leading-none text-muted-foreground">anon534</p>
          {/* <p className="text-sm text-muted-foreground">Send notifications to device.</p> */}
        </div>
        <div className="h-10 w-10 rounded-full bg-slate-300" />
      </div>

      <div
        className={cn("flex space-x-4 rounded-md p-2 mb-1 bg-slate-800 border-2 border-black", {
          " border-cyan-400": currentTurn == Player.Black,
        })}
      >
        <div className="flex-1 space-y-1">
          <p className="text-sm font-bold leading-none text-muted-foreground">anon304</p>
          {/* <p className="text-sm text-muted-foreground">Send notifications to device.</p> */}
        </div>
        <div className="h-10 w-10 rounded-full bg-slate-700" />
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
          <Toaster />
        </div>
      ),
    },
    // other pages....
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
        {/* <PylosCanvas />
        <div className="absolute inset-0 flex justify-center items-center">
          <Menu />
        </div> */}
      </WebSocketProvider>
    </div>
  );
}
