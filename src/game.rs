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
        (GameAction::PullOrb, GameEffect::HealthReachesZero) => todo!(),
        (GameAction::PullOrb, GameEffect::PointsLtMilestone) => todo!(),
        (GameAction::PullOrb, GameEffect::PointsGteMilestone) => todo!(),
        (GameAction::PullOrb, GameEffect::OrbsReachZero) => todo!(),
        (GameAction::CashOut, GameEffect::HealthReachesZero) => todo!(),
        (GameAction::CashOut, GameEffect::PointsLtMilestone) => todo!(),
        (GameAction::CashOut, GameEffect::PointsGteMilestone) => todo!(),
        (GameAction::CashOut, GameEffect::OrbsReachZero) => todo!(),
        (GameAction::EnterShop, GameEffect::HealthReachesZero) => todo!(),
        (GameAction::EnterShop, GameEffect::PointsLtMilestone) => todo!(),
        (GameAction::EnterShop, GameEffect::PointsGteMilestone) => todo!(),
        (GameAction::EnterShop, GameEffect::OrbsReachZero) => todo!(),
        (GameAction::BuyOrb, GameEffect::HealthReachesZero) => todo!(),
        (GameAction::BuyOrb, GameEffect::PointsLtMilestone) => todo!(),
        (GameAction::BuyOrb, GameEffect::PointsGteMilestone) => todo!(),
        (GameAction::BuyOrb, GameEffect::OrbsReachZero) => todo!(),
        (GameAction::NextLevel, GameEffect::HealthReachesZero) => todo!(),
        (GameAction::NextLevel, GameEffect::PointsLtMilestone) => todo!(),
        (GameAction::NextLevel, GameEffect::PointsGteMilestone) => todo!(),
        (GameAction::NextLevel, GameEffect::OrbsReachZero) => todo!(),
    }
}

fn handle_level_complete_state(
    level_complete_data: LevelCompleteData,
    action: &GameAction,
) -> GameState {
    match action {
        GameAction::PullOrb => todo!(),
        GameAction::CashOut => todo!(),
        GameAction::EnterShop => todo!(),
        GameAction::BuyOrb => todo!(),
        GameAction::NextLevel => todo!(),
    }
}

fn handle_shop_state(shop_data: ShopData, action: &GameAction) -> GameState {
    match action {
        GameAction::PullOrb => todo!(),
        GameAction::CashOut => todo!(),
        GameAction::EnterShop => todo!(),
        GameAction::BuyOrb => todo!(),
        GameAction::NextLevel => todo!(),
    }
}

fn handle_cashed_out_state(cashed_out_data: CashedOutData, action: &GameAction) -> GameState {
    match action {
        GameAction::PullOrb => todo!(),
        GameAction::CashOut => todo!(),
        GameAction::EnterShop => todo!(),
        GameAction::BuyOrb => todo!(),
        GameAction::NextLevel => todo!(),
    }
}

fn handle_game_over_state(game_over_data: GameOverData, action: &GameAction) -> GameState {
    match action {
        GameAction::PullOrb => todo!(),
        GameAction::CashOut => todo!(),
        GameAction::EnterShop => todo!(),
        GameAction::BuyOrb => todo!(),
        GameAction::NextLevel => todo!(),
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
    }
}
