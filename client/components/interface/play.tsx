import React, { useContext, useEffect, useState } from "react";
import { Card, CardContent, CardFooter, CardHeader, CardTitle } from "../ui/card";

import { Button } from "../ui/button";
import { Label } from "../ui/label";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "../ui/select";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "../ui/tabs";
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "../ui/table";
import { Slider } from "../ui/slider";

import "../../src/styles.css";
import { WebSocketContext } from "../../contexts/ws-context";

const CreateGameTab: React.FC = () => {
  const [time, setTime] = useState(5);
  const [increment, setIncrement] = useState(0);

  const { sendJsonMessage } = useContext(WebSocketContext)!;

  return (
    <Card>
      <CardHeader>
        <CardTitle>Create a new game</CardTitle>
        {/* <CardDescription>Make changes to your account here. Click save when you're done.</CardDescription> */}
      </CardHeader>
      <CardContent className="space-y-2">
        {/* <div className="space-y-1">
          <Label htmlFor="name">Name</Label>
          <Input id="name" defaultValue="Pedro Duarte" />
        </div> */}
        <div className="flex flex-col space-y-1.5">
          <Label htmlFor="opponent">Opponent</Label>
          <Select>
            <SelectTrigger id="opponent">
              <SelectValue placeholder="Computer" />
            </SelectTrigger>
            <SelectContent position="popper">
              <SelectItem value="computer">Computer</SelectItem>
              <SelectItem value="player">Player</SelectItem>
            </SelectContent>
          </Select>
        </div>
        <div className="flex flex-col space-y-1.5">
          <Label htmlFor="side">Side</Label>
          <Select>
            <SelectTrigger id="side">
              <SelectValue placeholder="Random" />
            </SelectTrigger>
            <SelectContent position="popper">
              <SelectItem value="random">Random</SelectItem>
              <SelectItem value="white">White</SelectItem>
              <SelectItem value="black">Black</SelectItem>
            </SelectContent>
          </Select>
        </div>
        <div className="flex flex-col space-y-1.5">
          <Label htmlFor="time">Time : {time} mins</Label>
          <Slider onValueChange={(e) => setTime(e[0])} defaultValue={[5]} max={10} min={1} step={1} />
        </div>
        <div className="flex flex-col space-y-1.5">
          <Label htmlFor="increment">Increment: {increment}</Label>
          <Slider onValueChange={(e) => setIncrement(e[0])} defaultValue={[0]} max={60} step={1} />
        </div>
        {/*  */}
      </CardContent>
      <CardFooter>
        <Button onClick={() => sendJsonMessage({ CreateGame: {} })} size="sm">
          Start
        </Button>
      </CardFooter>
    </Card>
  );
};

const JoinGameTab: React.FC = () => {
  console.log("[JoinGameTab]");

  const ColorIcon: React.FC<{ color: string }> = ({ color }) => {
    switch (color) {
      case "White": {
        return (
          <svg className="h-4 w-4" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512">
            <path d="M464 256A208 208 0 1 0 48 256a208 208 0 1 0 416 0zM0 256a256 256 0 1 1 512 0A256 256 0 1 1 0 256z" />
          </svg>
        );
      }
      case "Black": {
        return (
          <svg className="h-4 w-4" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512">
            <path d="M256 512A256 256 0 1 0 256 0a256 256 0 1 0 0 512z" />
          </svg>
        );
      }
      case "Random": {
        return (
          <svg className="h-4 w-4" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512">
            <path d="M448 256c0-106-86-192-192-192V448c106 0 192-86 192-192zM0 256a256 256 0 1 1 512 0A256 256 0 1 1 0 256z" />
          </svg>
        );
      }
    }
    console.warn("Unknown color:", color);
    return null;
  };

  const { sendJsonMessage, lastMessage } = useContext(WebSocketContext)!;

  type TGame = {
    user: String;
    side: "White" | "Black" | "Random";
    time: String;
  };
  const [games, setGames] = useState<TGame[]>([]);
  useEffect(() => {
    sendJsonMessage({ GetAvailableGames: {} });
  }, []);

  useEffect(() => {
    if (lastMessage != null) {
      const res = JSON.parse(lastMessage.data);
      if (res.hasOwnProperty("AvailableGames")) {
        setGames(
          res.AvailableGames.available_games.map((game: any, index: number) => ({
            user: index,
            side: "Random",
            time: "0 + 1",
          }))
        );
      }

      // .has();
      // console.log(JSON.parse(lastMessage.data).AvailableGames.available_games);
      // const newGame: TGame = ;
      // setGames((prev: TGame[]) => [...prev, newGame]);
    }
  }, [lastMessage]);

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
        <TableBody>
          {games.length > 0
            ? games.map((data, index) => {
                return (
                  <TableRow key={index}>
                    <TableCell>
                      <ColorIcon color={data.side} />
                    </TableCell>
                    <TableCell>{data.user}</TableCell>
                    <TableCell>{data.time}</TableCell>
                    {/* <TableCell className="text-right">$250.00</TableCell> */}
                  </TableRow>
                );
              })
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

  const { connectionStatus } = useContext(WebSocketContext)!;

  return (
    <Card>
      <CardHeader>
        <CardTitle>Invite a player {connectionStatus}</CardTitle>
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
