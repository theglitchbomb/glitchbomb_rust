enum GameState {
    Level(LevelData),
    LevelComplete(LevelCompleteData),
    Shop(ShopData),
    CashedOut(CashedOutData),
    GameOver(GameOverData),
}

struct LevelData;
struct ShopData;
struct LevelCompleteData;
struct CashedOutData;
struct GameOverData;

enum GameAction {
    StartLevel,
    EnterShop,
    BuyOrb,
    PullOrb,
    CashOut,
}

enum PullEffect {
    HealthReachesZero,
    PointsReachMilestone,
    OrbsRunOut,
    Continues,
}

fn stf_game(state: GameState, action: &GameAction, effect: &PullEffect) -> GameState {
    match (state, action, effect) {
        (GameState::Shop(data), GameAction::BuyOrb, _) => GameState::Shop(data),
        (GameState::Shop(data), GameAction::StartLevel, _) => GameState::Level(LevelData),
        (GameState::Level(data), GameAction::PullOrb, PullEffect::HealthReachesZero) => {
            GameState::GameOver(GameOverData)
        }
        (GameState::Level(data), GameAction::PullOrb, PullEffect::OrbsRunOut) => {
            GameState::GameOver(GameOverData)
        }
        (GameState::Level(data), GameAction::PullOrb, PullEffect::Continues) => {
            GameState::Level(data)
        }
        (GameState::Level(data), GameAction::PullOrb, PullEffect::PointsReachMilestone) => {
            GameState::LevelComplete(LevelCompleteData)
        }
        (GameState::LevelComplete(data), GameAction::EnterShop, _) => GameState::Shop(ShopData),
        (GameState::LevelComplete(data), GameAction::CashOut, _) => {
            GameState::CashedOut(CashedOutData)
        }
        (state, _, _) => state,
    }
}
