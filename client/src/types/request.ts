type TRequest =
  | { CreateGame: {} }
  | { JoinGame: { game_uuid: string } }
  | { GetAvailableGames: {} }
  | { GetGameState: { game_uuid: string } }
  | { SetGameState: { game_uuid: string; game_state: any } };

export { TRequest };
