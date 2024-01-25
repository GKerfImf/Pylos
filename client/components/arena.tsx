import React, { useEffect, useState } from "react";
import Platform from "../components/platform";
//@ts-ignore
//@ts-nocheck

import _ from "lodash";
import {
  Sphere,
  BlackSphere,
  WhiteSphere,
  GhostSphere,
  BlackGhostSphere,
  WhiteGhostSphere,
} from "../components/spheres";
import cartesian from "../util/cart";
import { Index2D, Index3D } from "../types/index";
import { Coord3D } from "../types/coord";

// TODO: move
const range = (start: number, end: number): number[] =>
  Array.from({ length: end - start }, (v, k) => k + start);

enum Player {
  White,
  Black,
}

interface PlayerBoardProps {
  player: Player;
}

function PlayerBoard({ player }: PlayerBoardProps) {
  console.log("[PlayerBoard]");
  const SIZE_X = 5;
  const SIZE_Y = 3;
  const indices: number[][] = cartesian(range(0, SIZE_X), range(0, SIZE_Y));

  const initCoordinates = (xs: number[], ys: number[]) => {
    const coords = new Map<string, Coord3D>(); /* Index2D*/
    for (let i = 0; i < 15; i++) {
      const x = indices[i][0];
      const y = indices[i][1];
      // Ewwwwwwwww,
      // TODO: remove JSON?
      // TODO: why [x] and [y] are swapped?
      coords.set(JSON.stringify({ x: x, y: y }), {
        cX: xs[x],
        cY: ys[y],
        cZ: 0.45,
      });
    }
    return coords;
  };
  const coords = // new Map<string /* Index2D*/, Coord3D>();
    player == Player.White
      ? initCoordinates([-2, -1, 0, 1, 2], [3.5, 4.5, 5.5])
      : initCoordinates([2, 1, 0, -1, -2], [-3.5, -4.5, -5.5]);

  const getColor = () => {
    return player == Player.White ? "white" : "black";
  };

  // TODO: optimize if needed
  const deleteFromSet = (set, el) => {
    return new Set([...set].filter((x) => !_.isEqual(JSON.parse(x), el)));
  };

  const [balls, setBalls] = useState<Set<string>>(new Set()); // Index2D
  const [ghostBalls, setGhostBalls] = useState<Set<Index2D>>(new Set());
  const [selectedBall, setSelectedBall] = useState<Index2D | null>(null);

  useEffect(() => {
    for (let x = 0; x < SIZE_X; x++) {
      for (let y = 0; y < SIZE_Y; y++) {
        setBalls((prev) => new Set(prev.add(JSON.stringify({ x: x, y: y }))));
      }
    }
    // setBalls((prev) => new Set(prev.add(JSON.stringify({ x: 3, y: 2 }))));
    // setGhostBalls((prev) => new Set(prev.add({ x: 1, y: 1 })));
  }, []);

  return (
    <group>
      {Array.from(balls).map((i: string, key) => {
        const index: Index2D = JSON.parse(i);
        const { cX, cY, cZ } = coords.get(i)!;
        return (
          <Sphere
            key={key}
            isClicked={_.isEqual(index, selectedBall)}
            isClickable={true}
            signalClick={() => {
              if (_.isEqual(selectedBall, index)) {
                setSelectedBall(null);
              } else {
                setSelectedBall(index);
              }
            }}
            color={getColor()}
            position={[cX, cZ, cY]}
          />
        );
      })}

      {Array.from(ghostBalls).map((i: Index2D, index) => {
        const { cX, cY, cZ } = coords.get(JSON.stringify(i))!;
        return (
          <GhostSphere
            key={index}
            position={[cX, cZ, cY]}
            color={getColor()}
            signalClick={() => {
              if (selectedBall) {
                setBalls((prev) => deleteFromSet(prev, selectedBall));
              }
            }}
          />
        );
      })}
    </group>
  );
}

class MainBoard {
  SIZE_Z = 4; // Z goes first
  SIZE_X = 4;
  SIZE_Y = 4;

  indices: number[][] = [
    ...cartesian(range(0, 4), range(0, 4)).map((el) => [...el, 0]),
    ...cartesian(range(0, 3), range(0, 3)).map((el) => [...el, 1]),
    ...cartesian(range(0, 2), range(0, 2)).map((el) => [...el, 2]),
    ...cartesian(range(0, 1), range(0, 1)).map((el) => [...el, 3]),
  ];

  whiteBalls = new Set<Index3D>();
  blackBalls = new Set<Index3D>();
  whiteGhostBalls = new Set<Index3D>();
  blackGhostBalls = new Set<Index3D>();

  coords = new Map<string /* Index3D*/, Coord3D>();
  #initCoordinates() {
    const xs = [[1.5, 0.5, -0.5, -1.5], [1, 0, -1], [0.5, -0.5], [0]];
    const ys = [[-1.5, -0.5, 0.5, 1.5], [-1, 0, 1], [-0.5, 0.5], [0]];

    for (let i = 0; i < 30; i++) {
      const x = this.indices[i][0];
      const y = this.indices[i][1];
      const z = this.indices[i][2];
      // TODO: remove JSON?
      // TODO: why [x] and [y] are swapped?
      this.coords.set(JSON.stringify({ x: x, y: y, z: z }), {
        cX: xs[z][y],
        cY: ys[z][x],
        cZ: 0.6 + z * 0.707,
      });
    }
  }

  constructor() {
    this.#initCoordinates();
    // this.blackBalls.add({ x: 0, y: 0, z: 0 });
    // this.blackBalls.add({ x: 0, y: 1, z: 0 });
    // this.blackBalls.add({ x: 0, y: 2, z: 0 });
    this.blackBalls.add({ x: 0, y: 0, z: 1 });

    // this.whiteBalls.add({ x: 1, y: 0, z: 0 });
    this.whiteBalls.add({ x: 1, y: 1, z: 0 });

    this.whiteGhostBalls.add({ x: 1, y: 0, z: 0 });
    this.whiteGhostBalls.add({ x: 1, y: 1, z: 0 });
    this.whiteGhostBalls.add({ x: 1, y: 2, z: 0 });
    this.whiteGhostBalls.add({ x: 1, y: 3, z: 0 });

    this.blackGhostBalls.add({ x: 0, y: 0, z: 0 });
    this.blackGhostBalls.add({ x: 0, y: 1, z: 0 });
    this.blackGhostBalls.add({ x: 0, y: 2, z: 0 });
    this.blackGhostBalls.add({ x: 0, y: 3, z: 0 });
  }

  #onClick(i: Index3D) {
    console.log(i);
  }

  render() {
    console.log("[MainBoard]");
    return (
      <group>
        {Array.from(this.whiteBalls).map((i: Index3D) => {
          const { cX, cY, cZ } = this.coords.get(JSON.stringify(i))!;
          return (
            <WhiteSphere
              key={Math.floor(Math.random() * 1000)}
              isClicked={false}
              isClickable={true}
              signalClick={(e) => {
                this.#onClick(i);
              }}
              position={[cX, cZ, cY]}
            />
          );
        })}
        {Array.from(this.blackBalls).map((i: Index3D) => {
          const { cX, cY, cZ } = this.coords.get(JSON.stringify(i))!;
          return (
            <BlackSphere
              key={Math.floor(Math.random() * 1000)}
              isClicked={false}
              isClickable={true}
              signalClick={(e) => {
                this.#onClick(i);
              }}
              position={[cX, cZ, cY]}
            />
          );
        })}

        {Array.from(this.whiteGhostBalls).map((i: Index3D) => {
          const { cX, cY, cZ } = this.coords.get(JSON.stringify(i))!;
          return (
            <WhiteGhostSphere
              key={Math.floor(Math.random() * 1000)}
              position={[cX, cZ, cY]}
            />
          );
        })}
        {Array.from(this.blackGhostBalls).map((i: Index3D) => {
          const { cX, cY, cZ } = this.coords.get(JSON.stringify(i))!;
          return (
            <BlackGhostSphere
              key={Math.floor(Math.random() * 1000)}
              position={[cX, cZ, cY]}
            />
          );
        })}
      </group>
    );
  }
}

export default function App() {
  // const white = new PlayerBoardClass(Player.White, 1);
  // const black = new PlayerBoardClass(Player.Black, 1);

  const main = new MainBoard();

  return (
    <group>
      <PlayerBoard player={Player.White} />

      {/* {white.render()} */}
      {/* {black.render()} */}
      {main.render()}
      <Platform />
    </group>
  );
}
