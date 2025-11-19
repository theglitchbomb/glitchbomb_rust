enum GameState {
    InGame(ActiveGameData),
    Won(VictoryData),
    Lost(DefeatData),
}

struct ActiveGameData;
struct VictoryData;
struct DefeatData;

enum GameAction {
    PullOrb,
    CashOut,
}

enum PullExecution {
    PullsBomb,
    PullsNonBomb,
}

fn stf_game(state: GameState, action: &GameAction, execution: &PullExecution) -> GameState {
    match (state, action, execution) {
        (GameState::InGame(data), GameAction::PullOrb, PullExecution::PullsBomb) => GameState::Lost(DefeatData),
        (GameState::InGame(data), GameAction::PullOrb, PullExecution::PullsNonBomb) => GameState::InGame(data),
        (GameState::InGame(data), GameAction::CashOut, _) => GameState::Won(VictoryData),
        (state, _, _) => state,
    }
}
