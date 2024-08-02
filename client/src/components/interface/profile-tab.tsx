import React, { ChangeEvent, useContext, useEffect, useState } from "react";
import { useLocalStorage } from "@uidotdev/usehooks";
import { WebSocketContext } from "src/contexts/ws-context";
import { TChangeProfileInfo } from "src/types/response";
import createUUID from "src/util/uuid";
import generateDefaultName from "src/util/default-names";
import { Input } from "src/components/ui/input";
import { Label } from "src/components/ui/label";
import { Button } from "src/components/ui/button";
import { toast } from "src/components/ui/use-toast";
import { Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle } from "src/components/ui/card";
import Avatar from "src/components/interface/avatar";
import "src/styles.css";

const ProfileTab: React.FC = () => {
  const { send, subscribe, unsubscribe } = useContext(WebSocketContext)!;

  const [nameLocal, saveNameLocal] = useLocalStorage<string>("PylosProfileName", generateDefaultName());
  const [avatarLocal, saveAvatarLocal] = useLocalStorage<string>("PylosProfileAvatar", createUUID());

  const [name, setName] = useState<string>(nameLocal);
  const onInputChange = (e: ChangeEvent<HTMLInputElement>) => {
    setName(e.target.value);
  };

  const setProfileName = () => {
    saveNameLocal(name);
    send({
      ChangeProfileInfo: {
        new_user_name: name,
        new_user_avatar: avatarLocal,
      },
    });
  };

  useEffect(() => {
    subscribe("ChangeProfileInfo", "ProfileTab", (_req: TChangeProfileInfo) => {
      toast({
        title: "Success! Profile info has been changed.",
        description: "You may need to refresh the page to see the changes.",
      });
    });
    return () => {
      unsubscribe("ChangeProfileInfo", "ProfileTab");
    };
  }, []);

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
              <Input id="name" placeholder={name} onChange={onInputChange} />
            </div>
          </div>
        </form>
      </CardContent>
      <CardContent>
        <div className="grid w-full items-center gap-4">
          <div className="flex flex-col space-y-1.5 text-white">
            <Label htmlFor="name">Avatar</Label>
            <Button onClick={() => saveAvatarLocal(createUUID())} className="h-16">
              <Avatar id={avatarLocal} winner={false} />
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
