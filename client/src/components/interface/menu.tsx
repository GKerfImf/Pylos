import React from "react";
import { Accordion, AccordionContent, AccordionItem, AccordionTrigger } from "src/components/ui/accordion";
import MenuHeader from "src/components/interface/header";
import Rules from "src/components/interface/pylos_rules";
import ProfileTab from "src/components/interface/profile-tab";
import Play from "src/components/interface/play";
import "src/styles.css";

const Menu: React.FC = () => {
  return (
    <div className="w-72 h-4/6 p-3 border rounded-xl bg-slate-900 border-slate-900 shadow-lg shadow-black flex-row justify-center items-center overflow-scroll no-scrollbar">
      <MenuHeader />

      <Accordion type="single" collapsible>
        <AccordionItem value="Play">
          <AccordionTrigger>
            <div className="flex w-full justify-center text-xs font-mono text-white">Play</div>
          </AccordionTrigger>
          <AccordionContent>
            <Play />
          </AccordionContent>
        </AccordionItem>
        <AccordionItem value="Profile">
          <AccordionTrigger>
            <div className="flex w-full justify-center text-xs font-mono text-white">Profile</div>
          </AccordionTrigger>
          <AccordionContent>
            <ProfileTab />
          </AccordionContent>
        </AccordionItem>
        {/* <AccordionItem value="Your games">
          <AccordionTrigger>
            <div className="flex w-full justify-center text-xs font-mono text-white">Your games</div>
          </AccordionTrigger>
          <AccordionContent>
            <Rules />
          </AccordionContent>
        </AccordionItem> */}
        <AccordionItem value="How to play">
          <AccordionTrigger>
            <div className="flex w-full justify-center text-xs font-mono text-white">How to play</div>
          </AccordionTrigger>
          <AccordionContent>
            <Rules />
          </AccordionContent>
        </AccordionItem>
      </Accordion>

      {/* Footer */}
    </div>
  );
};

export default Menu;
