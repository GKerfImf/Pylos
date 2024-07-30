import React, { useContext, useEffect, useState } from "react";
import { useNavigate } from "react-router-dom";
import { Table, TableBody, TableCaption, TableCell, TableHead, TableHeader, TableRow } from "src/components/ui/table";
import { Card, CardFooter, CardHeader, CardTitle } from "src/components/ui/card";
import { WebSocketContext } from "src/contexts/ws-context";
import { TAvailableGames } from "src/types/response";
import "src/styles.css";

type TGame = {
  game_uuid: String;
  user: String;
  side: "AlwaysWhite" | "AlwaysBlack" | "Random";
  time: String;
  status: "Pending" | "InProgress" | "Completed";
};

const ColorIcon: React.FC<{ color: "AlwaysWhite" | "AlwaysBlack" | "Random" }> = ({ color }) => {
  switch (color) {
    case "AlwaysWhite": {
      return (
        <svg className="h-4 flex w-full" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512">
          <path d="M464 256A208 208 0 1 0 48 256a208 208 0 1 0 416 0zM0 256a256 256 0 1 1 512 0A256 256 0 1 1 0 256z" />
        </svg>
      );
    }
    case "AlwaysBlack": {
      return (
        <svg className="h-4 flex w-full" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512">
          <path d="M256 512A256 256 0 1 0 256 0a256 256 0 1 0 0 512z" />
        </svg>
      );
    }
    case "Random": {
      return (
        <svg className="h-4 flex w-full" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512">
          <path d="M448 256c0-106-86-192-192-192V448c106 0 192-86 192-192zM0 256a256 256 0 1 1 512 0A256 256 0 1 1 0 256z" />
        </svg>
      );
    }
  }
};

const Header: React.FC = () => {
  return (
    <TableHeader>
      <TableRow>
        <TableHead>Side</TableHead>
        <TableHead>User</TableHead>
        <TableHead>Time</TableHead>
      </TableRow>
    </TableHeader>
  );
};

const Row: React.FC<{ data: TGame; key: number }> = ({ data, key }) => {
  const navigate = useNavigate();

  return (
    <TableRow onClick={() => navigate(`/games/${data.game_uuid}`)} key={key}>
      <TableCell>
        <ColorIcon color={data.side} />
      </TableCell>
      <TableCell>{data.user}</TableCell>
      <TableCell>{data.time}</TableCell>
    </TableRow>
  );
};

const AvailableGames: React.FC<{ games: TGame[] }> = ({ games }) => {
  return (
    <Table>
      <Header />
      <TableBody className="cursor-default">
        {games.length > 0 ? games.map((data, index) => <Row data={data} key={index} />) : null}
      </TableBody>
      {games.length == 0 ? (
        <TableCaption className="px-8 py-2">
          There are no available games. You can create one by going to the 'Create' tab.
        </TableCaption>
      ) : null}
    </Table>
  );
};

const PendingGames: React.FC<{ games: TGame[] }> = ({ games }) => {
  return (
    <Card className="mb-2">
      <CardHeader>
        <CardTitle>Join a game</CardTitle>
      </CardHeader>
      <AvailableGames games={games.filter((data) => data.status == "Pending")} />
    </Card>
  );
};

const InProgressGames: React.FC<{ games: TGame[] }> = ({ games }) => {
  const gamesInProgress = games.filter((data) => data.status == "InProgress");

  if (gamesInProgress.length == 0) {
    return null;
  }

  return (
    <Card className="mb-2">
      <CardHeader>
        <CardTitle>Watch a game</CardTitle>
      </CardHeader>
      <AvailableGames games={gamesInProgress} />
    </Card>
  );
};

const CompletedGames: React.FC<{ games: TGame[] }> = ({ games }) => {
  const completedFames = games.filter((data) => data.status == "Completed");

  if (completedFames.length == 0) {
    return null;
  }

  return (
    <Card>
      <CardHeader>
        <CardTitle>Review a game</CardTitle>
      </CardHeader>
      <AvailableGames games={completedFames} />
    </Card>
  );
};

const JoinGameTab: React.FC = () => {
  const { send, subscribe, unsubscribe } = useContext(WebSocketContext)!;

  const [games, setGames] = useState<TGame[]>([]);

  useEffect(() => {
    send({ GetAvailableGames: {} });
  }, []);

  useEffect(() => {
    subscribe("AvailableGames", "JoinGameTab", (req: TAvailableGames) => {
      setGames(
        req.AvailableGames.available_games.map((description: any, index: number) => {
          const [game_uuid, game_meta, game_spec] = description;

          let time =
            game_spec.time_control == null
              ? "∞"
              : (game_spec.time_control.time.secs / 60).toFixed(0) + "+" + game_spec.time_control.increment.secs;
          return {
            game_uuid: game_uuid,
            user: game_spec.creator_name,
            side: game_spec.side_selection,
            time: time,
            status: game_meta.status,
          };
        })
      );
    });
    return () => {
      unsubscribe("AvailableGames", "JoinGameTab");
    };
  });

  return (
    <div>
      <PendingGames games={games} />
      <InProgressGames games={games} />
      <CompletedGames games={games} />
    </div>
  );
};

export default JoinGameTab;
