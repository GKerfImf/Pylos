import { z } from "zod";

const GameParticipants = z.object({
  GameParticipants: z.object({
    game_uuid: z.string(),
    player_white: z.union([z.null(), z.any()]),
    player_black: z.union([z.null(), z.any()]),
  }),
});
type TGameParticipants = z.infer<typeof GameParticipants>;

const AvailableGames = z.object({
  AvailableGames: z.object({
    available_games: z.array(
      z.tuple([
        z.string(),
        z.object({ status: z.string(), created_at: z.any(), last_move_at: z.any() }),
        z.object({
          creator_name: z.string(),
          side_selection: z.string(),
          time_control: z.union([
            z.null(),
            z.object({
              increment: z.object({ nanos: z.number(), secs: z.number() }),
              time: z.object({ nanos: z.number(), secs: z.number() }),
            }),
          ]),
        }),
      ])
    ),
  }),
});
type TAvailableGames = z.infer<typeof AvailableGames>;

const CreateGame = z.object({ CreateGame: z.any() });
type TCreateGame = z.infer<typeof CreateGame>;

const GameState = z.object({
  GameState: z.object({
    game_state: z.object({
      balls: z.array(z.any()),
      nmove: z.number(),
      takeDownRule: z.number(),
      turn: z.number(),
      winner: z.union([z.null(), z.number()]),
    }),
  }),
});
type TGameState = z.infer<typeof GameState>;

const Response = z.union([GameParticipants, AvailableGames, GameState, CreateGame]);

type TResponse = z.infer<typeof Response>;

export { Response };
export { TGameState, TResponse, TAvailableGames, TGameParticipants, TCreateGame };
