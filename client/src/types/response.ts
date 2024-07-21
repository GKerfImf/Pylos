import { z } from "zod";

// TODO: proper type for [client_role]
const JoinGame = z.object({
  JoinGame: z.object({ client_uuid: z.string(), client_role: z.string(), game_uuid: z.string() }),
});
type TJoinGame = z.infer<typeof JoinGame>;

const GameParticipants = z.object({
  GameParticipants: z.object({
    game_uuid: z.string(),
    participants: z.array(z.array(z.string())),
  }),
});
type TGameParticipants = z.infer<typeof GameParticipants>;

const AvailableGames = z.object({
  AvailableGames: z.object({
    game_descriptions: z.array(
      z.object({
        creator_name: z.string(),
        game_uuid: z.string(),
        side_selection: z.string(),
        time_control: z.union([
          z.null(),
          z.object({
            increment: z.object({ nanos: z.number(), secs: z.number() }),
            time: z.object({ nanos: z.number(), secs: z.number() }),
          }),
        ]),
      })
    ),
  }),
});
type TAvailableGames = z.infer<typeof AvailableGames>;

const CreateGame = z.object({ CreateGame: z.any() });
type TCreateGame = z.infer<typeof CreateGame>;

const GameState = z.object({ GameState: z.object({ game_state: z.any() }) });
type TGameState = z.infer<typeof GameState>;

const Response = z.union([GameParticipants, AvailableGames, JoinGame, GameState, CreateGame]);

type TResponse = z.infer<typeof Response>;

export { Response };
export { TGameState, TResponse, TJoinGame, TAvailableGames, TGameParticipants, TCreateGame };
