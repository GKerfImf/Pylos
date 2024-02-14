import { z } from "zod";

const AvailableGames = z.object({
  AvailableGames: z.object({ game_uuids: z.array(z.string()) }),
});
const GameCreated = z.object({ GameCreated: z.any() });
const GameState = z.object({ GameState: z.object({ game_state: z.any() }) });

type TAvailableGames = z.infer<typeof AvailableGames>;
type TGameCreated = z.infer<typeof GameCreated>;
type TGameState = z.infer<typeof GameState>;

export { AvailableGames, GameCreated, GameState, TAvailableGames, TGameCreated, TGameState };
