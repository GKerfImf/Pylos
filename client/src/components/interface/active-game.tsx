import React, { useContext, useEffect, useState } from "react";
import Menu from "../../components/interface/menu";
import PylosCanvas from "../../components/canvas";
import { WebSocketContext, WebSocketProvider } from "../../contexts/ws-context";

import { createBrowserRouter, RouterProvider } from "react-router-dom";
import Player from "../../types/player";
import { cn } from "../../util/cn";
import { TGameState, TGameParticipants } from "../../types/response";
import { Toaster } from "../../components/ui/toaster";
import { Badge } from "../../components/ui/badge";

const ActiveGame: React.FC = () => {
  const [whitePlayer, setWhitePlayer] = useState<string>("");
  const [blackPlayer, setBlackPlayer] = useState<string>("");
  const [viewers, setViewers] = useState<string[]>([]);

  const [currentTurn, setCurrentTurn] = useState(Player.White);
  const { subscribe, unsubscribe } = useContext(WebSocketContext)!;

  // Listen to the game state to know whose turn it is
  useEffect(() => {
    subscribe("GameState", "ActiveGame", (req: TGameState) => {
      setCurrentTurn(req.GameState.game_state.turn);
    });
    return () => {
      unsubscribe("GameState", "ActiveGame");
    };
  }, []);

  //
  useEffect(() => {
    subscribe("GameParticipants", "ActiveGame", (req: TGameParticipants) => {
      setViewers([]);
      req.GameParticipants.participants.map((participant) => {
        let name = participant[0];
        let role = participant[1];

        if (role == "PlayerWhite") {
          setWhitePlayer(name);
        } else if (role == "PlayerBlack") {
          setBlackPlayer(name);
        } else {
          setViewers((val) => [...val, name]);
        }
      });
    });
    return () => {
      unsubscribe("GameParticipants", "ActiveGame");
    };
  }, []);

  const WhitePlayer = () => {
    return (
      <div
        className={cn("flex space-x-4 rounded-md p-2 bg-slate-200 border-2 border-black", {
          "border-cyan-400": currentTurn == Player.White,
        })}
      >
        <div className="flex-1 space-y-1">
          <p className="text-sm font-bold leading-none text-muted-foreground">{whitePlayer}</p>
          {/* <p className="text-sm text-muted-foreground">Send notifications to device.</p> */}
        </div>
        <div className="h-10 w-10 rounded-full bg-slate-300" />
      </div>
    );
  };

  const BlackPlayer = () => {
    return (
      <div
        className={cn("flex space-x-4 rounded-md p-2 mb-1 bg-slate-800 border-2 border-black", {
          " border-cyan-400": currentTurn == Player.Black,
        })}
      >
        <div className="flex-1 space-y-1">
          <p className="text-sm font-bold leading-none text-muted-foreground">{blackPlayer}</p>
          {/* <p className="text-sm text-muted-foreground">Send notifications to device.</p> */}
        </div>
        <div className="h-10 w-10 rounded-full bg-slate-700" />
      </div>
    );
  };

  const Viewers = () => {
    return (
      <div className="space-x-4 rounded-md p-2 mb-1 bg-slate-800 border-2 border-black ">
        {viewers.map((v, key) => {
          return (
            <Badge key={key} variant="secondary" className="rounded-sm m-1 p-1">
              {v}
            </Badge>
          );
        })}
      </div>
    );
  };

  return (
    <div className="w-72 h-4/6 p-3 border rounded-xl bg-slate-900 border-slate-900 shadow-lg shadow-black flex-row justify-center items-center overflow-scroll scrollbar-hide">
      <WhitePlayer />
      <BlackPlayer />
      {viewers.length > 0 ? <Viewers /> : null}
    </div>
  );
};

export default ActiveGame;
