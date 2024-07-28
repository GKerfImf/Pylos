import React, { useContext, useEffect, useState } from "react";
import { useNavigate } from "react-router-dom";
import { Label } from "src/components/ui/label";
import { Slider } from "src/components/ui/slider";
import { Button } from "src/components/ui/button";
import { Card, CardContent, CardFooter, CardHeader, CardTitle } from "src/components/ui/card";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "src/components/ui/select";
import { WebSocketContext } from "src/contexts/ws-context";
import { TCreateGame } from "src/types/response";
import { TRequest } from "src/types/request";
import "src/styles.css";

const OpponentSelect: React.FC<{ opponent: "Human" | "Computer"; setOpponent: any }> = ({ opponent, setOpponent }) => {
  return (
    <div className="flex flex-col space-y-1.5">
      <Label htmlFor="opponent">Opponent</Label>
      <Select onValueChange={setOpponent} defaultValue="Computer">
        <SelectTrigger id="opponent">
          <SelectValue placeholder="Human" />
        </SelectTrigger>
        <SelectContent position="popper">
          <SelectItem value="Human">Human</SelectItem>
          <SelectItem value="Computer">Computer</SelectItem>
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

  const [opponent, setOpponent] = useState<"Human" | "Computer">("Computer");
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
        game_configuration: {
          game_uuid: null,
          opponent: opponent,
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

export default CreateGameTab;
