import React, { useContext, useEffect, useReducer } from "react";
import _ from "lodash";
import range from "src/util/range";
import cartesian from "src/util/cart";
import Ball from "src/types/ball";
import Board from "src/types/board";
import Player from "src/types/player";
import Index3D from "src/types/index";
import Coord3D from "src/types/coord";
import TypedMap from "src/types/typed_map";
import { Sphere, GhostSphere } from "src/components/game/spheres";
import { WebSocketContext } from "src/contexts/ws-context";
import { useParams } from "react-router-dom";
import { TGameState } from "src/types/response";
const Platform = React.lazy(() => import("src/components/game/platform"));

function findParents(index: Index3D): Index3D[] {
  if (index.b != Board.Main) {
    return [];
  }
  const { b, x, y, z } = index;
  const parents = [
    { b: b, x: x, y: y, z: z + 1 },
    { b: b, x: x - 1, y: y, z: z + 1 },
    { b: b, x: x, y: y - 1, z: z + 1 },
    { b: b, x: x - 1, y: y - 1, z: z + 1 },
  ]
    .filter((e) => 0 <= e.x && e.x < 3 - z)
    .filter((e) => 0 <= e.y && e.y < 3 - z)
    .filter((e) => 0 <= e.z && e.z < 4);
  return parents;
}

function findChildren(index: Index3D): Index3D[] {
  if (index.b != Board.Main) {
    return [];
  }
  const { b, x, y, z } = index;
  const children = [
    { b: b, x: x, y: y, z: z - 1 },
    { b: b, x: x + 1, y: y, z: z - 1 },
    { b: b, x: x, y: y + 1, z: z - 1 },
    { b: b, x: x + 1, y: y + 1, z: z - 1 },
  ]
    .filter((e) => 0 <= e.x && e.x <= 4 - z)
    .filter((e) => 0 <= e.y && e.y <= 4 - z)
    .filter((e) => 0 <= e.z && e.z < 4);
  return children;
}

function isParent(child: Index3D, parent: Index3D): boolean {
  return findParents(child).filter((i) => _.isEqual(i, parent)).length > 0;
}

function findBall(state: any, index: Index3D): Ball | null {
  const balls: Ball[] = state.balls;
  const ball = balls.filter((i) => _.isEqual(i.index, index));
  return ball.length == 0 ? null : ball[0];
}

const initCoordinates = () => {
  const coords = new TypedMap<Index3D, Coord3D>();

  const xs_side = [-2, -1, 0, 1, 2];
  const ys_side = [3.5, 4.5, 5.5];
  const indices_side: number[][] = cartesian(range(0, xs_side.length), range(0, ys_side.length));
  for (let i = 0; i < 15; i++) {
    const x = indices_side[i][0];
    const y = indices_side[i][1];
    coords.set({ b: Board.White, x: x, y: y, z: 0 }, { cX: xs_side[x], cY: ys_side[y], cZ: 0.45 });
    coords.set({ b: Board.Black, x: x, y: y, z: 0 }, { cX: -xs_side[x], cY: -ys_side[y], cZ: 0.45 });
  }

  const indices_mid: number[][] = [4, 3, 2, 1].flatMap((z) =>
    cartesian(range(0, z), range(0, z)).map((el) => [...el, 4 - z])
  );
  const xs_mid = [[1.5, 0.5, -0.5, -1.5], [1, 0, -1], [0.5, -0.5], [0]];
  const ys_mid = [[-1.5, -0.5, 0.5, 1.5], [-1, 0, 1], [-0.5, 0.5], [0]];
  for (let i = 0; i < 30; i++) {
    const x = indices_mid[i][0];
    const y = indices_mid[i][1];
    const z = indices_mid[i][2];
    // [x] and [y] are swapped because I look at this board table from the side
    coords.set({ b: Board.Main, x: x, y: y, z: z }, { cX: xs_mid[z][y], cY: ys_mid[z][x], cZ: 0.6 + z * 0.707 });
  }

  return coords;
};
const coords: TypedMap<Index3D, Coord3D> = initCoordinates();

function isBall(state: any, index: Index3D): boolean {
  return findBall(state, index) == null ? false : true;
}

function getGhostBalls(state: any, selectedBall: Ball): Ball[] {
  if (selectedBall == null) {
    return [];
  }
  if (state?.takeDownRule > 0) {
    const indices: number[][] = cartesian(range(0, 5), range(0, 3));
    const balls = indices
      .map(([x, y, z]: number[]) => {
        return { b: selectedBall.player == Player.White ? Board.White : Board.Black, x: x, y: y, z: 0 };
      })
      .map((index: Index3D) => ({ player: selectedBall.player, index: index }));
    return balls;
  }

  const indices: number[][] = [4, 3, 2, 1]
    .filter((e) => e < 4 - selectedBall.index.z || selectedBall.index.b != Board.Main)
    .flatMap((z) => cartesian(range(0, z), range(0, z)).map((el) => [...el, 4 - z]));
  return indices
    .map(([x, y, z]: number[]) => {
      return { b: Board.Main, x: x, y: y, z: z };
    })
    .filter((index: Index3D) => !isBall(state, index))
    .filter((index: Index3D) => findChildren(index).every((index: Index3D) => isBall(state, index)))
    .filter((index: Index3D) => !isParent(selectedBall.index, index))
    .map((index: Index3D) => ({ player: selectedBall.player, index: index }));
}

const isClickable = (state: any, ball: Ball) => {
  if (ball.player != state.turn) {
    return false;
  }

  if (state.takeDownRule > 0 && ball.index.b != Board.Main) {
    return false;
  }

  if (ball.index.b == Board.White) {
    return true;
  }
  if (ball.index.b == Board.Black) {
    return true;
  }
  return findParents(ball.index).every((index: Index3D) => !isBall(state, index));
};

function ballsReducer(state: any, action: any) {
  console.debug("[ballsReducer]", action);

  function removeBall(balls: Ball[], ball: Ball) {
    return balls.filter((i) => !_.isEqual(i, ball));
  }

  function addBall(balls: any, ball: any) {
    return [...balls, ball];
  }

  function moveBall(from: Ball, to: Ball) {
    console.debug("[moveBall]");

    const balls: Ball[] = state.balls;

    const newSelectedBall = null;
    const newSelectedGhostBall = null;
    const newBalls: Ball[] = addBall(removeBall(balls, from), to);

    return {
      turn: null,
      takeDownRule: state.takeDownRule,
      nmove: state.nmove + 1,
      selectedBall: newSelectedBall,
      selectedGhostBall: newSelectedGhostBall,
      balls: newBalls,
    };
  }

  switch (action.type) {
    case "SelectBall": {
      return { ...state, selectedBall: action.ball };
    }
    case "SelectGhostBall": {
      if (state.selectedBall == null) {
        console.error("[selectedBall] should not be null");
      }
      return moveBall(state.selectedBall, action.ball);
    }
    case "SetGameState": {
      return action.new_state;
    }
    default: {
      console.error("Unknown type of action");
    }
  }
  return state;
}

function Arena() {
  console.debug("[Arena]");
  let { id } = useParams();

  const [state, dispatch] = useReducer(ballsReducer, {
    nmove: null,
    turn: null,
    takeDownRule: null,
    selectedBall: null,
    selectedGhostBall: null,
    balls: [],
  });

  const { send, subscribe, unsubscribe } = useContext(WebSocketContext)!;

  useEffect(() => {
    send({ JoinGame: { game_uuid: id! } });
    send({ GetGameState: { game_uuid: id! } });
  }, []);

  useEffect(() => {
    subscribe("GameState", "Arena", (req: TGameState) => {
      dispatch({ type: "SetGameState", new_state: req.GameState.game_state });
    });
    return () => {
      unsubscribe("GameState", "Arena");
    };
  }, []);

  return (
    <group>
      {state.balls.map((ball: Ball) => {
        const { cX, cY, cZ }: Coord3D = coords.get(ball.index)!;
        return (
          <Sphere
            key={JSON.stringify(ball)}
            id={ball}
            isClicked={_.isEqual(ball, state.selectedBall)}
            isClickable={isClickable(state, ball)}
            color={ball.player == Player.White ? "white" : "black"}
            position={[cX, cZ, cY]}
            onClick={(e: any) => {
              e.stopPropagation();
              if (isClickable(state, ball)) {
                if (_.isEqual(state.selectedBall, ball)) {
                  dispatch({ type: "SelectBall", ball: null });
                } else {
                  dispatch({ type: "SelectBall", ball: ball });
                }
              }
            }}
          />
        );
      })}
      {state.selectedBall != null
        ? getGhostBalls(state, state.selectedBall).map((ball: Ball) => {
            const { cX, cY, cZ }: Coord3D = coords.get(ball.index)!;
            return (
              <GhostSphere
                key={JSON.stringify(ball)}
                id={ball}
                color={state.selectedBall.player == Player.White ? "white" : "black"}
                position={[cX, cZ, cY]}
                onClick={(e: any) => {
                  e.stopPropagation();
                  dispatch({ type: "SelectGhostBall", ball: ball });
                  send({ MakeMove: { game_uuid: id!, mv: { from: state.selectedBall, to: ball } } });
                }}
              />
            );
          })
        : null}
    </group>
  );
}

export default function App() {
  return (
    <group>
      <Arena />
      <Platform />
    </group>
  );
}
