import React, { useContext, useEffect, useState } from "react";
import { useNavigate, useParams } from "react-router-dom";
import { Label } from "src/components/ui/label";
import { Slider } from "src/components/ui/slider";
import { Button } from "src/components/ui/button";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "src/components/ui/tabs";
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "src/components/ui/table";
import { Card, CardContent, CardFooter, CardHeader, CardTitle } from "src/components/ui/card";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "src/components/ui/select";
import "src/styles.css";
import { WebSocketContext } from "src/contexts/ws-context";
import { TAvailableGames, TCreateGame } from "src/types/response";
import { TRequest } from "src/types/request";

const OpponentSelect: React.FC<{ opponent: any; setOpponent: any }> = ({ opponent, setOpponent }) => {
  return (
    <div className="flex flex-col space-y-1.5">
      <Label htmlFor="opponent">Opponent</Label>
      <Select onValueChange={setOpponent} defaultValue="computer" disabled={true}>
        <SelectTrigger id="opponent">
          <SelectValue placeholder="Player" />
        </SelectTrigger>
        <SelectContent position="popper">
          <SelectItem value="player">Player</SelectItem>
          <SelectItem value="computer">Computer</SelectItem>
        </SelectContent>
      </Select>
    </div>
  );
};

const SideSelect: React.FC<{
  side: "Random" | "AlwaysWhite" | "AlwaysBlack";
  setSide: (newSide: "Random" | "AlwaysWhite" | "AlwaysBlack") => void;
}> = ({ side, setSide }) => {
  return (
    <div className="flex flex-col space-y-1.5">
      <Label htmlFor="side">Side</Label>
      <Select onValueChange={setSide} defaultValue="Random">
        <SelectTrigger id="side">
          <SelectValue placeholder="Random" />
        </SelectTrigger>
        <SelectContent position="popper">
          <SelectItem value="Random">Random</SelectItem>
          <SelectItem value="AlwaysWhite">White</SelectItem>
          <SelectItem value="AlwaysBlack">Black</SelectItem>
        </SelectContent>
      </Select>
    </div>
  );
};

const TimeControlSelect: React.FC<{ timeControl: any; setTimeControl: any }> = ({ timeControl, setTimeControl }) => {
  return (
    <div className="flex flex-col space-y-1.5">
      <Label htmlFor="time-control">Time control</Label>
      <Select onValueChange={setTimeControl} defaultValue="unlimited" disabled>
        <SelectTrigger id="unlimited">
          <SelectValue placeholder="Unlimited" />
        </SelectTrigger>
        <SelectContent position="popper">
          <SelectItem value="unlimited">Unlimited</SelectItem>
          <SelectItem value="real-time">Real-time</SelectItem>
        </SelectContent>
      </Select>
    </div>
  );
};

const TimeSelect: React.FC<{ time: any; setTime: any }> = ({ time, setTime }) => {
  return (
    <div className="flex flex-col space-y-1.5">
      <Label htmlFor="time">Time : {time} mins</Label>
      <Slider onValueChange={(e) => setTime(e[0])} defaultValue={[5]} max={10} min={1} step={1} />
    </div>
  );
};

const Increment: React.FC<{ increment: any; setIncrement: any }> = ({ increment, setIncrement }) => {
  return (
    <div className="flex flex-col space-y-1.5">
      <Label htmlFor="increment">Increment: {increment}</Label>
      <Slider onValueChange={(e) => setIncrement(e[0])} defaultValue={[0]} max={60} step={1} />
    </div>
  );
};

const CreateGameTab: React.FC = () => {
  const navigate = useNavigate();

  const [opponent, setOpponent] = useState<"player" | "computer">("computer");
  const [side, setSide] = useState<"Random" | "AlwaysWhite" | "AlwaysBlack">("Random");
  const [timeControl, setTimeControl] = useState<"unlimited" | "real-time">("unlimited");
  const [time, setTime] = useState(5);
  const [increment, setIncrement] = useState(0);

  const { subscribe, unsubscribe, send } = useContext(WebSocketContext)!;

  useEffect(() => {
    subscribe("CreateGame", "CreateGameTab", (req: TCreateGame) => {
      navigate("/games/" + req.CreateGame.game_uuid);
    });
    return () => {
      unsubscribe("CreateGame", "JoinGameTab");
    };
  });

  // TODO: turn into hook
  // TODO: delete duplicate
  const entryProfileName = "pylos_profile_name";
  const getProfileName = () => {
    const entryID = "pylos_uuid";
    if (!localStorage.getItem(entryID)) {
      console.warn("[ProfileTab]: [pylos_uuid] does not exist");
    }

    if (!localStorage.getItem(entryProfileName)) {
      return `anon-${localStorage.getItem(entryID)!.slice(0, 8)}`;
    } else {
      return localStorage.getItem(entryProfileName)!;
    }
  };

  const createGame = () => {
    // TODO: implement the time control
    const time_control = timeControl == "unlimited" ? null : "not implemented";

    const req: TRequest = {
      CreateGame: {
        game_description: {
          game_uuid: null,
          creator_name: getProfileName(),
          side_selection: side,
          time_control: time_control,
        },
      },
    };

    send(req);
  };

  return (
    <Card>
      <CardHeader>
        <CardTitle>Create a new game</CardTitle>
      </CardHeader>

      <CardContent className="space-y-2">
        <OpponentSelect opponent={opponent} setOpponent={setOpponent} />
        <SideSelect side={side} setSide={setSide} />
        <TimeControlSelect timeControl={timeControl} setTimeControl={setTimeControl} />
        {timeControl == "real-time" ? (
          <div>
            <TimeSelect time={time} setTime={setTime} />
            <Increment increment={increment} setIncrement={setIncrement} />
          </div>
        ) : null}
      </CardContent>

      <CardFooter>
        <Button onClick={createGame} size="sm">
          Start
        </Button>
      </CardFooter>
    </Card>
  );
};

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
        req.AvailableGames.game_descriptions.map((description: any, index: number) => {
          let time =
            description.time_control == null
              ? "∞"
              : (description.time_control.time.secs / 60).toFixed(0) + "+" + description.time_control.increment.secs;
          return {
            game_uuid: description.game_uuid,
            user: description.creator_name,
            side: description.side_selection,
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

const InviteTab: React.FC = () => {
  const dummyData = [
    { user: "anon127", last: "online" },
    { user: "anon311", last: "5 min ago" },
    { user: "anon098", last: "5 min ago" },
    { user: "anon912", last: "5 min ago" },
    { user: "anon018", last: "5 min ago" },
    { user: "anon031", last: "5 min ago" },
    { user: "anon517", last: "5 min ago" },
    { user: "anon101", last: "5 min ago" },
  ];

  return (
    <Card>
      <CardHeader>
        <CardTitle>Invite a player</CardTitle>
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
            <TableHead>User</TableHead>
            <TableHead>Last seen</TableHead>
            {/* <TableHead className="text-right">Amount</TableHead> */}
          </TableRow>
        </TableHeader>
        <TableBody>
          {dummyData.map((data, index) => {
            return (
              <TableRow key={index}>
                <TableCell>{data.user}</TableCell>
                <TableCell>{data.last}</TableCell>
                {/* <TableCell className="text-right">$250.00</TableCell> */}
              </TableRow>
            );
          })}
        </TableBody>
      </Table>
      <CardFooter className="pb-0">{/* <Button>Save password</Button> */}</CardFooter>
    </Card>
  );
};

const Play: React.FC = () => {
  return (
    <Tabs defaultValue="create" className="">
      {/* <TabsList className="grid w-full grid-cols-3 p-0"> */}
      <TabsList className="grid w-full grid-cols-2 p-0">
        <TabsTrigger value="create">Create</TabsTrigger>
        <TabsTrigger value="join">Join</TabsTrigger>
        {/* <TabsTrigger value="invite">Invite</TabsTrigger> */}
      </TabsList>
      <TabsContent value="create">
        <CreateGameTab />
      </TabsContent>
      <TabsContent value="join">
        <JoinGameTab />
      </TabsContent>
      {/* <TabsContent value="invite">
        <InviteTab />
      </TabsContent> */}
    </Tabs>
  );
};

export default Play;
