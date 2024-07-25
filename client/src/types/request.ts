import Ball from "src/types/ball";

type TRequest =
  | {
      ChangeName: {
        new_user_name: string;
        new_user_avatar: string;
      };
    }
  | {
      GetClientName: {
        client_uuid: string;
      };
    }
  | {
      CreateGame: {
        game_description: {
          game_uuid: string | null;
          opponent: "Human" | "Computer";
          creator_name: string;
          side_selection: "Random" | "AlwaysWhite" | "AlwaysBlack";
          time_control: string | null;
        };
      };
    }
  | { JoinGame: { game_uuid: string } }
  | { GetAvailableGames: {} }
  | { GetGameState: { game_uuid: string } }
  | { MakeMove: { game_uuid: string; mv: { from: Ball; to: Ball } } };

export { TRequest };
