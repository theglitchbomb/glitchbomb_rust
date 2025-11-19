enum GamePack {
    GameActive(ActiveGameData),
    GameComplete(CompletedGameData),
    PackComplete(PackResults),
}

struct ActiveGameData;
struct CompletedGameData;
struct PackResults;

enum GamePackAction {
    Win,
    Lose,
    CashOut,
    NextGame,
}

enum PackProgressEffect {
    MoreGamesRemaining,
    AllGamesComplete,
}

fn stf_gamepack(
    pack: GamePack,
    action: &GamePackAction,
    progress: &PackProgressEffect,
) -> GamePack {
    match (pack, action, progress) {
        (
            GamePack::GameActive(_),
            GamePackAction::Win | GamePackAction::Lose | GamePackAction::CashOut,
            PackProgressEffect::MoreGamesRemaining,
        ) => GamePack::GameComplete(CompletedGameData),
        (
            GamePack::GameActive(_),
            GamePackAction::Win | GamePackAction::Lose | GamePackAction::CashOut,
            PackProgressEffect::AllGamesComplete,
        ) => GamePack::PackComplete(PackResults),
        (GamePack::GameComplete(_), GamePackAction::NextGame, _) => {
            GamePack::GameActive(ActiveGameData)
        }
        (state, _, _) => state,
    }
}
