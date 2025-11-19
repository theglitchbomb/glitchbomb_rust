enum GameState {
    InGame(ActiveGame),
    Won(VictoryData),
    Lost(DefeatData),
}

struct ActiveGame;
struct VictoryData;
struct DefeatData;

enum OrbType {
    PowerUp(PowerUpOrb),
    Benefit(BenefitOrb),
    Bomb(BombOrb),
    Multiplier(MultiplierOrb),
}

struct PowerUpOrb;
struct BenefitOrb;
struct BombOrb;
struct MultiplierOrb;

enum GameAction {
    PullOrb,
    StartGame,
    EndTurn,
    CashOut,
}

enum PullExecution {
    TriggersWin,
    TriggersBomb,
    Continues,
}

fn stf_game(state: GameState, action: &GameAction, execution: &PullExecution) -> GameState {
    match (state, action, execution) {
        (GameState::InGame(data), GameAction::PullOrb, PullExecution::TriggersWin) => GameState::Won(VictoryData),
        (GameState::InGame(data), GameAction::PullOrb, PullExecution::TriggersBomb) => GameState::Lost(DefeatData),
        (GameState::InGame(data), GameAction::PullOrb, PullExecution::Continues) => GameState::InGame(data),
        (GameState::InGame(data), GameAction::CashOut, _) => GameState::Won(VictoryData),
        (state, _, _) => state,
    }
}
