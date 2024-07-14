import Ball from "src/types/ball";

type TRequest =
  | {
      ChangeName: {
        new_user_name: string;
      };
    }
  | {
      GetClientName: {
        client_uuid: string;
      };
    }
  | { CreateGame: {} }
  | { JoinGame: { game_uuid: string } }
  | { GetAvailableGames: {} }
  | { GetGameState: { game_uuid: string } }
  | { MakeMove: { game_uuid: string; mv: { from: Ball; to: Ball } } };

export { TRequest };
