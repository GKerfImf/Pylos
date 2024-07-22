import React, { ChangeEvent, useContext, useState } from "react";
import { Input } from "src/components/ui/input";
import { Label } from "src/components/ui/label";
import { Button } from "src/components/ui/button";
import { Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle } from "src/components/ui/card";
import "src/styles.css";
import { WebSocketContext } from "src/contexts/ws-context";

const ProfileTab: React.FC = () => {
  const { send } = useContext(WebSocketContext)!;

  const [name, setName] = useState<string | null>(null);
  const onInputChange = (e: ChangeEvent<HTMLInputElement>) => {
    setName(e.target.value);
  };

  const entryProfileName = "pylos_profile_name";

  // TODO: turn into hook
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

  const setProfileName = () => {
    if (name != null) {
      localStorage.setItem(entryProfileName, name);
      send({
        ChangeName: {
          new_user_name: name,
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
      <CardFooter className="flex justify-between">
        <Button onClick={setProfileName}>Save</Button>
      </CardFooter>
    </Card>
  );
};

export default ProfileTab;
