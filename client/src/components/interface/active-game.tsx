import React, { useContext, useEffect, useState } from "react";
import { WebSocketContext } from "../../contexts/ws-context";
import { cva } from "class-variance-authority";

import Player from "../../types/player";
import { cn } from "../../util/cn";
import { TGameState, TGameParticipants } from "../../types/response";
import { Badge } from "../../components/ui/badge";
import { Button } from "../ui/button";
import { useNavigate } from "react-router-dom";
import Avatar from "./avatar";

const PlayerCard: React.FC<{
  color: "white" | "black";
  state: "active" | "passive";
  winner: boolean;
  connected: boolean;
  playerName: string;
  avatarID: string;
}> = ({ playerName, avatarID, color, winner, state, connected }) => {
  const badgeVariants = cva("rounded-sm mr-1 px-2 items-center text-xs", {
    variants: {
      color: {
        white: "border-slate-400 text-slate-900 blur",
        black: "border-slate-600 text-slate-100 blur",
      },
    },
  });

  const cardVariants = cva("rounded-sm items-center text-xs mb-1", {
    variants: {
      color: {
        white: "bg-slate-100 text-slate-900 hover:brightness-110",
        black: "bg-slate-900 text-slate-100 hover:brightness-150",
      },
      state: {
        // Thanks to [https://cssf1.com/snippet/create-a-neon-effect-with-tailwindcss]
        active: "shadow-[0_0_2px_#fff,inset_0_0_1px_#fff,0_0_3px_#08f,0_0_9px_#08f,0_0_18px_#08f]",
        passive: "border-black",
      },
      connected: {
        true: "",
        false: "blur-sm hover:brightness-100",
      },
    },
  });

  const Main = () => {
    return (
      <div className="flex-1 space-y-1">
        <p className="text-sm font-bold leading-none">{playerName}</p>
        <Badge variant="outline" className={cn(badgeVariants({ color }), "")}>
          Time: 5:00
        </Badge>
        <Badge variant="outline" className={cn(badgeVariants({ color }), "")}>
          Ping: 140ms
        </Badge>
      </div>
    );
  };

  return (
    <div
      className={cn(cardVariants({ color, state, connected }), "flex space-x-4 rounded-md p-2 border-2 cursor-default")}
    >
      <Main />
      <Avatar id={avatarID} winner={winner} />
    </div>
  );
};

const WaitingWhitePlayer: React.FC = () => {
  return (
    <PlayerCard
      playerName={"Waiting for opponent..."}
      avatarID={"magic-constant"}
      color="white"
      state="passive"
      connected={false}
      winner={false}
    />
  );
};

const WaitingBlackPlayer: React.FC = () => {
  return (
    <PlayerCard
      playerName={"Waiting for opponent..."}
      avatarID={"magic-constant"}
      color="black"
      state="passive"
      connected={false}
      winner={false}
    />
  );
};

const WhitePlayer: React.FC<{
  whitePlayerAvatar: string;
  whitePlayer: string | null;
  blackPlayer: string | null;
  currentTurn: Player | null;
  winner: Player | null;
}> = ({ whitePlayer, blackPlayer, currentTurn, whitePlayerAvatar, winner }) => {
  let state: "active" | "passive" =
    whitePlayer != null && blackPlayer != null && currentTurn == Player.White ? "active" : "passive";
  return (
    <PlayerCard
      playerName={whitePlayer!}
      avatarID={whitePlayerAvatar}
      color="white"
      state={state}
      connected={true}
      winner={winner == Player.White}
    />
  );
};

const BlackPlayer: React.FC<{
  blackPlayerAvatar: string;
  whitePlayer: string | null;
  blackPlayer: string | null;
  currentTurn: Player | null;
  winner: Player | null;
}> = ({ whitePlayer, blackPlayer, currentTurn, blackPlayerAvatar, winner }) => {
  let state: "active" | "passive" =
    whitePlayer != null && blackPlayer != null && currentTurn == Player.Black ? "active" : "passive";
  return (
    <PlayerCard
      playerName={blackPlayer!}
      avatarID={blackPlayerAvatar}
      color="black"
      state={state}
      connected={true}
      winner={winner == Player.Black}
    />
  );
};

const HotKey: React.FC<{ value: string }> = ({ value }) => {
  return (
    <kbd className="pointer-events-none inline-flex h-5 select-none items-center gap-1 px-1.5 font-mono text-[12px] font-medium opacity-100">
      <span className="">{value}</span>
    </kbd>
  );
};

const Controls = () => {
  const navigate = useNavigate();

  const redirectToLobby = () => {
    navigate(`/`);
  };

  const common = "border-primary bg-primary text-slate-400 text-xs font-medium px-1 py-1 hover:bg-secondary h-6";
  return (
    <div className="flex justify-center mt-2 ">
      <div className="inline-flex shadow-sm rounded-sm mr-2 text-muted-foreground" role="group">
        <Button className={cn("rounded-r-none border", common)} disabled>
          <HotKey value={"←"} />
        </Button>
        <Button className={cn("rounded-l-none border", common)} disabled>
          <HotKey value={"→"} />
        </Button>
      </div>
      <div className="inline-flex shadow-sm rounded-sm " role="group">
        <Button className={cn("rounded-r-none border", common)} disabled>
          Draw
        </Button>
        <Button className={cn("rounded-none border-t border-b", common)} disabled>
          Resign
        </Button>
        <Button className={cn("rounded-l-none border", common)} onClick={redirectToLobby}>
          Lobby
        </Button>
      </div>
    </div>
  );
};

const ActiveGame: React.FC = () => {
  const [whitePlayer, setWhitePlayer] = useState<string | null>(null);
  const [blackPlayer, setBlackPlayer] = useState<string | null>(null);

  const [whitePlayerAvatar, setWhitePlayerAvatar] = useState<string>("000");
  const [blackPlayerAvatar, setBlackPlayerAvatar] = useState<string>("000");

  const [currentTurn, setCurrentTurn] = useState<Player | null>(null);
  const [winner, setWinner] = useState<Player | null>(null);

  const { subscribe, unsubscribe } = useContext(WebSocketContext)!;

  useEffect(() => {
    subscribe("GameState", "ActiveGame", (req: TGameState) => {
      setCurrentTurn(req.GameState.game_state.turn);
      if (req.GameState.game_state.winner != null) {
        setWinner(req.GameState.game_state.winner);
      }
    });
    return () => {
      unsubscribe("GameState", "ActiveGame");
    };
  }, []);

  useEffect(() => {
    const magic_constant_ai_uuid = "Min-Max AI";
    subscribe("GameParticipants", "ActiveGame", (req: TGameParticipants) => {
      if (req.GameParticipants.player_white != null) {
        let pw = req.GameParticipants.player_white;
        if (pw[2].player_type == "Human") {
          setWhitePlayer(pw[0]);
          setWhitePlayerAvatar(pw[1]);
        } else {
          setWhitePlayer(magic_constant_ai_uuid);
          setWhitePlayerAvatar(magic_constant_ai_uuid);
        }
      }

      if (req.GameParticipants.player_black != null) {
        let pw = req.GameParticipants.player_black;
        if (pw[2].player_type == "Human") {
          setBlackPlayer(pw[0]);
          setBlackPlayerAvatar(pw[1]);
        } else {
          setBlackPlayer(magic_constant_ai_uuid);
          setBlackPlayerAvatar(magic_constant_ai_uuid);
        }
      }
    });
    return () => {
      unsubscribe("GameParticipants", "ActiveGame");
    };
  }, []);

  return (
    <div className="w-72 h-4/6 p-2 border rounded-xl bg-slate-800 border-slate-800 shadow-lg shadow-black flex-row justify-center items-center overflow-scroll scrollbar-hide">
      {whitePlayer ? (
        <WhitePlayer
          whitePlayer={whitePlayer}
          blackPlayer={blackPlayer}
          currentTurn={currentTurn}
          whitePlayerAvatar={whitePlayerAvatar}
          winner={winner}
        />
      ) : (
        <WaitingWhitePlayer />
      )}
      {blackPlayer ? (
        <BlackPlayer
          whitePlayer={whitePlayer}
          blackPlayer={blackPlayer}
          currentTurn={currentTurn}
          blackPlayerAvatar={blackPlayerAvatar}
          winner={winner}
        />
      ) : (
        <WaitingBlackPlayer />
      )}
      <Controls />
    </div>
  );
};

export default ActiveGame;
