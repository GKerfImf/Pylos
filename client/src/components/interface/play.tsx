import React from "react";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "src/components/ui/tabs";
import CreateGameTab from "src/components/interface/create-game";
import JoinGameTab from "src/components/interface/join-game";
import "src/styles.css";

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
