import { z } from "zod";

const JoinGame = z.object({
  JoinGame: z.object({ status: z.number(), client_uuid: z.string(), game_uuid: z.string() }),
});
type TJoinGame = z.infer<typeof JoinGame>;

const AvailableGames = z.object({
  AvailableGames: z.object({ game_uuids: z.array(z.string()) }),
});
const CreateGame = z.object({ CreateGame: z.any() });

const GameState = z.object({ GameState: z.object({ game_state: z.any() }) });
type TGameState = z.infer<typeof GameState>;

const ClientName = z.object({ ClientName: z.object({ user_name: z.string(), client_uuid: z.string() }) });
const Response = z.union([AvailableGames, JoinGame, GameState, ClientName, CreateGame]);

type TResponse = z.infer<typeof Response>;

export { Response };
export { TGameState, TResponse, TJoinGame };
