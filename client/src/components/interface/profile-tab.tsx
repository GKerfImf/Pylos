import React, { ChangeEvent, useContext, useState } from "react";
import { Input } from "src/components/ui/input";
import { Label } from "src/components/ui/label";
import { Button } from "src/components/ui/button";
import { Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle } from "src/components/ui/card";
import "src/styles.css";
import { WebSocketContext } from "src/contexts/ws-context";
import Avatar from "./avatar";
import useLocalState from "src/hooks/local-storage";

const ProfileTab: React.FC = () => {
  const { send } = useContext(WebSocketContext)!;

  const [name, setName] = useState<string | null>(null);
  const onInputChange = (e: ChangeEvent<HTMLInputElement>) => {
    setName(e.target.value);
  };

  const { setState: setProfileNameLocal, getState: getProfileName } = useLocalState(
    "pylos_profile_name",
    (uuid: string) => `anon-${uuid.slice(0, 8)}`
  );
  const { getState: getAvatarState, setRandom: setRandomAvatar } = useLocalState("pylos_profile_avatar");

  const setProfileName = () => {
    if (name != null) {
      send({
        ChangeName: {
          new_user_name: setProfileNameLocal(name),
          new_user_avatar: getAvatarState(),
        },
      });
    }
  };

  return (
    <Card className="w-full ">
      <CardHeader>
        <CardTitle>Edit profile</CardTitle>
        <CardDescription>Make changes to your profile here. Click save when you're done.</CardDescription>
      </CardHeader>
      <CardContent>
        <form>
          <div className="grid w-full items-center gap-4">
            <div className="flex flex-col space-y-1.5 text-white">
              <Label htmlFor="name">Name</Label>
              <Input id="name" placeholder={getProfileName()} onChange={onInputChange} />
            </div>
          </div>
        </form>
      </CardContent>
      <CardContent>
        <div className="grid w-full items-center gap-4">
          <div className="flex flex-col space-y-1.5 text-white">
            <Label htmlFor="name">Avatar</Label>
            <Button onClick={setRandomAvatar} className="h-16">
              <Avatar id={getAvatarState()} />
              <p className="m-2">Change avatar</p>
            </Button>
          </div>
        </div>
      </CardContent>
      <CardFooter className="flex justify-between">
        <Button onClick={setProfileName}>Save</Button>
      </CardFooter>
    </Card>
  );
};

export default ProfileTab;
