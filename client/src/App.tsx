import React, { ReactNode, useState } from "react";
import { Canvas } from "@react-three/fiber";
import { OrbitControls, Environment } from "@react-three/drei";
import Arena from "../components/arena";
import { cn } from "../util/cn";
import * as THREE from "three";
import {
  ChromaticAberration,
  ColorAverage,
  EffectComposer,
  Noise,
  Pixelation,
  Vignette,
} from "@react-three/postprocessing";
import { BlendFunction } from "postprocessing";

interface ButtonProps {
  text: string;
  children?: ReactNode;
  onClick?: () => void;
}

const MenuHeader: React.FC = () => {
  return <p className="flex justify-center pb-2 text-blue-600 text-lg font-bold font-mono ">Pylos</p>;
};

const Rules: React.FC = () => {
  // Thanks to [https://cdn.1j1ju.com/medias/cd/27/c5-pylos-rulebook.pdf]
  return (
    <div className="tracking-tight p-1 px-2 text-gray-400 text-sm text-justify">
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

const CreateGame: React.FC<{ onClick: () => void }> = ({ onClick }) => {
  return (
    <div className="flex-row h-full overflow-scroll">
      <div className="flex justify-center text-white text-sm inset-x-0 my-2 min-w-max">Create a new game</div>

      <button
        className="w-full border border-slate-500 rounded hover:border-slate-700 bg-slate-400 hover:bg-slate-300"
        onClick={onClick}
      >
        Start
      </button>

      {/* TODO
      Name [.....]
      Opponent: [Compute / Player]
      Password [Yes / No]
      Side [White / Random / Black]
      Time Control [Yes / No]
        Minutes per side [0 -- 180]
        Increment in seconds [0 -- 180]
      */}
    </div>
  );
};

const Lobby: React.FC = () => {
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

  const Row: React.FC<{ data: { user: string; side: string; time: string } }> = ({ data }) => {
    return (
      <div className="flex hover:bg-slate-300 hover:rounded rounded bg-slate-500 transition-all duration-50 cursor-default  overflow-hidden my-1">
        <div className="flex w-8 border-y border-l border-slate-800 rounded-l justify-center items-center">
          <ColorIcon color={data.side} />
        </div>
        <div className="flex w-24 border-y border-slate-800 justify-center">{data.user}</div>
        <div className="flex w-24 border-y border-r border-slate-800 rounded-r justify-center">{data.time}</div>
      </div>
    );
  };

  const dummyData = [
    { user: "anon127", side: "White", time: "5 + 5" },
    { user: "anon311", side: "Black", time: "5 + 0" },
    { user: "anon098", side: "Random", time: "1 + 3" },
    { user: "anon912", side: "Black", time: "1 + 3" },
    { user: "anon018", side: "Random", time: "10 + 0" },
    { user: "anon031", side: "Random", time: "2 + 3" },
    { user: "anon517", side: "Black", time: "2 + 0" },
    { user: "anon101", side: "White", time: "1 + 0" },
  ];

  return (
    <div className="flex-row h-full overflow-scroll">
      <div className="flex justify-center text-white text-sm inset-x-0 my-2 min-w-max">Available games</div>
      <div className="flex-row items-center min-w-max">
        {dummyData.map((data, index) => {
          return <Row key={index} data={data} />;
        })}
      </div>
    </div>
  );
};

const Selection: React.FC<{ onClick: () => void }> = ({ onClick }) => {
  const [selectedOption, setSelectedOption] = useState<string>("left");

  return (
    <div className="flex flex-col items-center">
      <div className="flex text-sm h-6 m-1 border rounded border-slate-700">
        <button
          className={cn("w-28 bg-slate-700 rounded-l focus:outline-none transition-colors", {
            " bg-slate-800 text-white": selectedOption === "left",
            "text-black": selectedOption === "right",
          })}
          onClick={() => setSelectedOption("left")}
        >
          Create
        </button>
        <button
          className={cn("w-28 bg-slate-700 rounded-r focus:outline-none transition-colors", {
            " bg-slate-800 text-white": selectedOption === "right",
            "text-black": selectedOption === "left",
          })}
          onClick={() => setSelectedOption("right")}
        >
          Lobby
        </button>
      </div>

      <div className="flex h-96 w-full mx-6 p-2">
        <div
          className={cn(
            "overflow-hidden transition-all duration-300 rounded w-0 bg-slate-700 h-full flex justify-center items-center",
            {
              "w-full": selectedOption === "left",
            }
          )}
        >
          <CreateGame onClick={onClick} />
        </div>

        <div
          className={cn(
            "overflow-hidden transition-all duration-300 rounded w-0 bg-slate-700 h-full flex justify-center items-center",
            {
              "w-full": selectedOption === "right",
            }
          )}
        >
          <Lobby />
        </div>
      </div>
    </div>
  );
};

const Button: React.FC<ButtonProps> = ({ text, children }) => {
  const [isVisible, setIsVisible] = useState(false);

  return (
    <div className="overflow-hidden">
      <button
        className={cn(
          "text-white rounded-sm text-center text-xs h-7 w-full p-0 mb-1 border bg-blue-600 border-blue-600 hover:border-blue-300",
          { "bg-blue-900 border-blue-900 hover:border-blue-600": isVisible }
        )}
        onClick={() => setIsVisible(!isVisible)}
      >
        {text}
      </button>
      <div
        className={cn("transition-all duration-300 bg-slate-800 rounded max-h-0", {
          "max-h-screen mb-1": isVisible,
        })}
      >
        {children}
      </div>
    </div>
  );
};

const Menu: React.FC<{ onClick: () => void }> = ({ onClick }) => {
  return (
    <div className="w-72 h-4/6 p-3 border rounded-xl bg-slate-900 border-slate-900 shadow-lg shadow-black flex-row justify-center items-center overflow-scroll scrollbar-hide">
      <MenuHeader />
      <Button text={"Play"}>
        <Selection onClick={onClick} />
      </Button>
      <Button text={"How to play"}>
        <Rules />
      </Button>
      {/* Footer */}
    </div>
  );
};

export default function App() {
  const [effects, setEffects] = useState(true);

  return (
    <div className="w-full h-full">
      <div className="w-full h-full">
        <Canvas shadows camera={{ position: [-8, 8, 0] }}>
          <Environment background preset="forest" blur={1} />
          <ambientLight intensity={0.5} />
          <Arena />
          {effects ? (
            <EffectComposer>
              <Vignette eskil={false} offset={0} darkness={1.2} />

              <ColorAverage
                blendFunction={BlendFunction.NORMAL} // blend mode
              />
              <Pixelation granularity={2} />

              <ChromaticAberration
                blendFunction={BlendFunction.NORMAL}
                offset={new THREE.Vector2(0.005, 0.002)}
                radialModulation={false}
                modulationOffset={0}
              />
              <Noise premultiply blendFunction={BlendFunction.AVERAGE} />
            </EffectComposer>
          ) : (
            <EffectComposer>
              <Vignette eskil={false} offset={0} darkness={1} />
            </EffectComposer>
          )}
          <OrbitControls
            enableDamping={true}
            enableZoom={false}
            enablePan={false}
            dampingFactor={0.15}
            maxPolarAngle={Math.PI / 2 - 0.2}
            minPolarAngle={0.2}
            target={[0, 2.121, 0]}
          />
        </Canvas>
      </div>

      <div className="absolute inset-0 flex justify-center items-center">
        {/* <div className="absolute inset-0 z-10 bg-green-400"> */}
        <Menu onClick={() => setEffects(!effects)} />
      </div>
    </div>
  );
}

// // useControls(
// //   {
// //     "New Game": button(() => setEffects(!effects)),
// //     // [toggle ? "activat12ed" : "disabled"]: button(() => alert("hello"), {
// //     //   disabled: !toggle,
// //     // }),
// //     " 123": buttonGroup({
// //       "0.25x": () => alert({ Size: 0.25 }),
// //       "0.5x": () => alert({ Size: 0.5 }),
// //       "1x": () => alert({ Size: 1 }),
// //       "2x": () => alert({ Size: 2 }),
// //       "3x": () => alert({ Size: 3 }),
// //     }),
// //   },
// //   [effects]
// // );

// // autoRotate={true}
// // autoRotateSpeed={5}
// //   <EffectComposer>
// //

// {
//   /* <Noise
//     premultiply // enables or disables noise premultiplication
//     blendFunction={BlendFunction.ADD} // blend mode
// /> */
// }
// // </EffectComposer>

// // {/* <Stats /> */}
// // {/* <PixelatePass /> */}
// // {/* <DepthOfField focusDistance={0} focalLength={0.02} bokehScale={3} height={480} /> */}
// // {/* <Bloom luminanceThreshold={0} luminanceSmoothing={0.9} height={300} /> */}
// // {/* <Noise opacity={0.02} /> */}
// // {/* dpr={1} gl={{ antialias: false }} */}
// {
//   /* <Perf position="top-left" /> */
// }
