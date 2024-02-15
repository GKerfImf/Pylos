import { z } from "zod";

const JoinGame = z.object({
  JoinGame: z.object({ status: z.number(), client_uuid: z.string(), game_uuid: z.string() }),
});

const AvailableGames = z.object({
  AvailableGames: z.object({ game_uuids: z.array(z.string()) }),
});
const CreateGame = z.object({ CreateGame: z.any() });
const GameState = z.object({ GameState: z.object({ game_state: z.any() }) });

type TJoinGame = z.infer<typeof JoinGame>;
type TAvailableGames = z.infer<typeof AvailableGames>;
type TCreateGame = z.infer<typeof CreateGame>;
type TGameState = z.infer<typeof GameState>;

export { JoinGame, AvailableGames, CreateGame, GameState, TJoinGame, TAvailableGames, TCreateGame, TGameState };
