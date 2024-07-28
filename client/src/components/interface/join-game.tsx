import React, { useContext, useEffect, useState } from "react";
import { useNavigate } from "react-router-dom";
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "src/components/ui/table";
import { Card, CardFooter, CardHeader, CardTitle } from "src/components/ui/card";
import { WebSocketContext } from "src/contexts/ws-context";
import { TAvailableGames } from "src/types/response";
import "src/styles.css";

const JoinGameTab: React.FC = () => {
  console.debug("[JoinGameTab]");

  const navigate = useNavigate();

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
    return null;
  };

  const { send, subscribe, unsubscribe } = useContext(WebSocketContext)!;

  type TGame = {
    game_uuid: String;
    user: String;
    side: "AlwaysWhite" | "AlwaysBlack" | "Random";
    time: String;
  };
  const [games, setGames] = useState<TGame[]>([]);
  useEffect(() => {
    send({ GetAvailableGames: {} });
  }, []);

  useEffect(() => {
    subscribe("AvailableGames", "JoinGameTab", (req: TAvailableGames) => {
      setGames(
        req.AvailableGames.available_games.map((description: any, index: number) => {
          const [game_uuid, game_spec] = description;

          let time =
            game_spec.time_control == null
              ? "∞"
              : (game_spec.time_control.time.secs / 60).toFixed(0) + "+" + game_spec.time_control.increment.secs;
          return {
            game_uuid: game_uuid,
            user: game_spec.creator_name,
            side: game_spec.side_selection,
            time: time,
          };
        })
      );
    });
    return () => {
      unsubscribe("AvailableGames", "JoinGameTab");
    };
  });

  return (
    <Card>
      <CardHeader>
        <CardTitle>Join a game</CardTitle>
        {/* <CardDescription>Change your password here. After saving, you'll be logged out.</CardDescription> */}
      </CardHeader>
      {/* <CardContent className="space-y-2">
          <div className="space-y-1">
            <Label htmlFor="current">Current password</Label>
            <Input id="current" type="password" />
          </div>
          <div className="space-y-1">
            <Label htmlFor="new">New password</Label>
            <Input id="new" type="password" />
          </div>
        </CardContent> */}
      <Table>
        {/* <TableCaption>A list of your recent invoices.</TableCaption> */}
        <TableHeader>
          <TableRow>
            <TableHead>Side</TableHead>
            <TableHead>User</TableHead>
            <TableHead>Time</TableHead>
            {/* <TableHead className="text-right">Amount</TableHead> */}
          </TableRow>
        </TableHeader>
        <TableBody className=" cursor-default">
          {games.length > 0
            ? games.map((data, index) => (
                <TableRow onClick={() => navigate(`/games/${data.game_uuid}`)} key={index}>
                  <TableCell>
                    <ColorIcon color={data.side} />
                  </TableCell>
                  <TableCell>{data.user}</TableCell>
                  <TableCell>{data.time}</TableCell>
                </TableRow>
              ))
            : null}
        </TableBody>
      </Table>
      <CardFooter className="pb-0">{/* <Button>Save password</Button> */}</CardFooter>
    </Card>
  );
};

export default JoinGameTab;
