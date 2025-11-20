enum GamePack {
    InGame(InGameData),
    GameComplete(GameCompleteData),
    GamePackComplete,
}

struct InGameData;
struct GameCompleteData;

enum GamePackAction {
    PlayGame,
    EndGame,
    NextGame,
}

enum GamePackEffect {
    SomeNewGames,
    NoNewGames,
    GameContinues,
    GameConcludes,
}

fn stf_gamepack(gamepack: GamePack, action: &GamePackAction, effect: &GamePackEffect) -> GamePack {
    match (gamepack, action, effect) {
        (GamePack::InGame(data), GamePackAction::PlayGame, GamePackEffect::GameContinues) => {
            GamePack::InGame(data)
        }
        (GamePack::InGame(data), GamePackAction::PlayGame, GamePackEffect::GameConcludes) => {
            GamePack::GameComplete(GameCompleteData)
        }
        (GamePack::InGame(data), GamePackAction::EndGame, GamePackEffect::SomeNewGames) => {
            GamePack::GameComplete(GameCompleteData)
        }
        (GamePack::InGame(data), GamePackAction::EndGame, GamePackEffect::NoNewGames) => {
            GamePack::GamePackComplete
        }
        (GamePack::GameComplete(data), GamePackAction::NextGame, _) => GamePack::InGame(InGameData),
        (state, _, _) => state,
    }
}
