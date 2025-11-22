enum GameState {
    Level(LevelData),
    LevelComplete(LevelCompleteData),
    Shop(ShopData),
    CashedOut(CashedOutData),
    GameOver(GameOverData),
    Invalid,
}

struct LevelData;
struct ShopData;
struct LevelCompleteData;
struct CashedOutData;
struct GameOverData;

enum GameAction {
    PullOrb,
    CashOut,
    EnterShop,
    BuyOrb,
    NextLevel,
}

enum GameEffect {
    HealthReachesZero,
    PointsLtMilestone,
    PointsGteMilestone,
    OrbsReachZero,
}

fn handle_level_state(
    level_data: LevelData,
    action: &GameAction,
    effect: &GameEffect,
) -> GameState {
    match (action, effect) {
        (GameAction::PullOrb, GameEffect::HealthReachesZero) => GameState::GameOver(GameOverData),
        (GameAction::PullOrb, GameEffect::OrbsReachZero) => GameState::GameOver(GameOverData),
        (GameAction::PullOrb, GameEffect::PointsLtMilestone) => GameState::Level(level_data),
        (GameAction::PullOrb, GameEffect::PointsGteMilestone) => {
            GameState::LevelComplete(LevelCompleteData)
        }
        (GameAction::CashOut, GameEffect::PointsLtMilestone) => GameState::CashedOut(CashedOutData),
        (_, _) => GameState::Invalid,
    }
}

fn handle_level_complete_state(
    level_complete_data: LevelCompleteData,
    action: &GameAction,
) -> GameState {
    match action {
        GameAction::CashOut => GameState::CashedOut(CashedOutData),
        GameAction::EnterShop => GameState::Shop(ShopData),
        _ => GameState::Invalid,
    }
}

fn handle_shop_state(shop_data: ShopData, action: &GameAction) -> GameState {
    match action {
        GameAction::BuyOrb => GameState::Shop(shop_data),
        GameAction::NextLevel => GameState::Level(LevelData),
        _ => GameState::Invalid,
    }
}

fn handle_cashed_out_state(cashed_out_data: CashedOutData, action: &GameAction) -> GameState {
    match action {
        _ => GameState::Invalid,
    }
}

fn handle_game_over_state(game_over_data: GameOverData, action: &GameAction) -> GameState {
    match action {
        _ => GameState::Invalid,
    }
}

fn stf_game(state: GameState, action: &GameAction, effect: &GameEffect) -> GameState {
    match state {
        GameState::Level(level_data) => handle_level_state(level_data, action, effect),
        GameState::LevelComplete(level_complete_data) => {
            handle_level_complete_state(level_complete_data, action)
        }
        GameState::Shop(shop_data) => handle_shop_state(shop_data, action),
        GameState::CashedOut(cashed_out_data) => handle_cashed_out_state(cashed_out_data, action),
        GameState::GameOver(game_over_data) => handle_game_over_state(game_over_data, action),
        GameState::Invalid => panic!("Invalid Game State"),
    }
}
