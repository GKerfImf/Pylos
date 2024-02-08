import React from "react";
import { Skeleton } from "src/components/ui/skeleton";
import "src/styles.css";

const ProfileTab: React.FC = () => {
  return (
    <Skeleton className="h-[125px] w-full rounded-xl" />
    // <Card className="w-full ">
    //   <CardHeader>
    //     <CardTitle>Create project</CardTitle>
    //     <CardDescription>Deploy your new project in one-click.</CardDescription>
    //   </CardHeader>
    //   <CardContent>
    //     <form>
    //       <div className="grid w-full items-center gap-4">
    //         <div className="flex flex-col space-y-1.5 text-white">
    //           <Label htmlFor="name">Name</Label>
    //           <Input id="name" placeholder="Name of your project" />
    //         </div>
    //       </div>
    //     </form>
    //   </CardContent>
    //   <CardFooter className="flex justify-between">
    //     <Button>Save</Button>
    //   </CardFooter>
    // </Card>
  );
};

export default ProfileTab;
