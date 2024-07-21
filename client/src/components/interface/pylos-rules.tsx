import React from "react";
import "src/styles.css";

const Rules: React.FC = () => {
  // Thanks to [https://cdn.1j1ju.com/medias/cd/27/c5-pylos-rulebook.pdf]
  return (
    <div className="tracking-tight p-1 px-2 text-gray-400 text-sm text-justify bg-slate-800 rounded-md">
      <p>
        Each player, at the beginning of the game, places one of their balls in one of the hollows on the board. Play
        continues in this order, placing one ball each turn.
      </p>
      <br />
      <p>
        When a square made of four spheres exists on the board or at higher levels at the beginning of a player's turn,
        a player may choose to stack one of his spheres on it. They may use a sphere from their reserve or they may use
        a sphere from the board and stack it on top of the square, thus limiting the number of balls they take from
        their reserve. Of course, a player can only take a ball from the board as long as it is not supporting any other
        balls on top of it.
      </p>
      <br />
      <p>
        A player who makes a square completely out of their own colour (4 balls) immediately takes back one or two of
        his spheres from the board and places them back into their reserve. They may take back the ball that they just
        played which completed the square.
      </p>
      <br />
      <p>The winner is the player who places their last sphere on top of the pyramid.</p>
    </div>
  );
};

export default Rules;
