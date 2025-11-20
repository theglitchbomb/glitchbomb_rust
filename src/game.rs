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

enum PullEffect {
    HealthReachesZero,
    PointsLtMilestone,
    PointsGteMilestone,
    OrbsReachZero,
}

fn stf_game(state: GameState, action: &GameAction, effect: &PullEffect) -> GameState {
    match (state, action, effect) {
        (GameState::Level(level_data), GameAction::PullOrb, PullEffect::HealthReachesZero) => {
            todo!()
        }
        (GameState::Level(level_data), GameAction::PullOrb, PullEffect::PointsLtMilestone) => {
            todo!()
        }
        (GameState::Level(level_data), GameAction::PullOrb, PullEffect::PointsGteMilestone) => {
            todo!()
        }
        (GameState::Level(level_data), GameAction::PullOrb, PullEffect::OrbsReachZero) => todo!(),
        (GameState::Level(level_data), GameAction::CashOut, PullEffect::HealthReachesZero) => {
            todo!()
        }
        (GameState::Level(level_data), GameAction::CashOut, PullEffect::PointsLtMilestone) => {
            todo!()
        }
        (GameState::Level(level_data), GameAction::CashOut, PullEffect::PointsGteMilestone) => {
            todo!()
        }
        (GameState::Level(level_data), GameAction::CashOut, PullEffect::OrbsReachZero) => todo!(),
        (GameState::Level(level_data), GameAction::EnterShop, PullEffect::HealthReachesZero) => {
            todo!()
        }
        (GameState::Level(level_data), GameAction::EnterShop, PullEffect::PointsLtMilestone) => {
            todo!()
        }
        (GameState::Level(level_data), GameAction::EnterShop, PullEffect::PointsGteMilestone) => {
            todo!()
        }
        (GameState::Level(level_data), GameAction::EnterShop, PullEffect::OrbsReachZero) => todo!(),
        (GameState::Level(level_data), GameAction::BuyOrb, PullEffect::HealthReachesZero) => {
            todo!()
        }
        (GameState::Level(level_data), GameAction::BuyOrb, PullEffect::PointsLtMilestone) => {
            todo!()
        }
        (GameState::Level(level_data), GameAction::BuyOrb, PullEffect::PointsGteMilestone) => {
            todo!()
        }
        (GameState::Level(level_data), GameAction::BuyOrb, PullEffect::OrbsReachZero) => todo!(),
        (GameState::Level(level_data), GameAction::NextLevel, PullEffect::HealthReachesZero) => {
            todo!()
        }
        (GameState::Level(level_data), GameAction::NextLevel, PullEffect::PointsLtMilestone) => {
            todo!()
        }
        (GameState::Level(level_data), GameAction::NextLevel, PullEffect::PointsGteMilestone) => {
            todo!()
        }
        (GameState::Level(level_data), GameAction::NextLevel, PullEffect::OrbsReachZero) => todo!(),
        (
            GameState::LevelComplete(level_complete_data),
            GameAction::PullOrb,
            PullEffect::HealthReachesZero,
        ) => todo!(),
        (
            GameState::LevelComplete(level_complete_data),
            GameAction::PullOrb,
            PullEffect::PointsLtMilestone,
        ) => todo!(),
        (
            GameState::LevelComplete(level_complete_data),
            GameAction::PullOrb,
            PullEffect::PointsGteMilestone,
        ) => todo!(),
        (
            GameState::LevelComplete(level_complete_data),
            GameAction::PullOrb,
            PullEffect::OrbsReachZero,
        ) => todo!(),
        (
            GameState::LevelComplete(level_complete_data),
            GameAction::CashOut,
            PullEffect::HealthReachesZero,
        ) => todo!(),
        (
            GameState::LevelComplete(level_complete_data),
            GameAction::CashOut,
            PullEffect::PointsLtMilestone,
        ) => todo!(),
        (
            GameState::LevelComplete(level_complete_data),
            GameAction::CashOut,
            PullEffect::PointsGteMilestone,
        ) => todo!(),
        (
            GameState::LevelComplete(level_complete_data),
            GameAction::CashOut,
            PullEffect::OrbsReachZero,
        ) => todo!(),
        (
            GameState::LevelComplete(level_complete_data),
            GameAction::EnterShop,
            PullEffect::HealthReachesZero,
        ) => todo!(),
        (
            GameState::LevelComplete(level_complete_data),
            GameAction::EnterShop,
            PullEffect::PointsLtMilestone,
        ) => todo!(),
        (
            GameState::LevelComplete(level_complete_data),
            GameAction::EnterShop,
            PullEffect::PointsGteMilestone,
        ) => todo!(),
        (
            GameState::LevelComplete(level_complete_data),
            GameAction::EnterShop,
            PullEffect::OrbsReachZero,
        ) => todo!(),
        (
            GameState::LevelComplete(level_complete_data),
            GameAction::BuyOrb,
            PullEffect::HealthReachesZero,
        ) => todo!(),
        (
            GameState::LevelComplete(level_complete_data),
            GameAction::BuyOrb,
            PullEffect::PointsLtMilestone,
        ) => todo!(),
        (
            GameState::LevelComplete(level_complete_data),
            GameAction::BuyOrb,
            PullEffect::PointsGteMilestone,
        ) => todo!(),
        (
            GameState::LevelComplete(level_complete_data),
            GameAction::BuyOrb,
            PullEffect::OrbsReachZero,
        ) => todo!(),
        (
            GameState::LevelComplete(level_complete_data),
            GameAction::NextLevel,
            PullEffect::HealthReachesZero,
        ) => todo!(),
        (
            GameState::LevelComplete(level_complete_data),
            GameAction::NextLevel,
            PullEffect::PointsLtMilestone,
        ) => todo!(),
        (
            GameState::LevelComplete(level_complete_data),
            GameAction::NextLevel,
            PullEffect::PointsGteMilestone,
        ) => todo!(),
        (
            GameState::LevelComplete(level_complete_data),
            GameAction::NextLevel,
            PullEffect::OrbsReachZero,
        ) => todo!(),
        (GameState::Shop(shop_data), GameAction::PullOrb, PullEffect::HealthReachesZero) => todo!(),
        (GameState::Shop(shop_data), GameAction::PullOrb, PullEffect::PointsLtMilestone) => todo!(),
        (GameState::Shop(shop_data), GameAction::PullOrb, PullEffect::PointsGteMilestone) => {
            todo!()
        }
        (GameState::Shop(shop_data), GameAction::PullOrb, PullEffect::OrbsReachZero) => todo!(),
        (GameState::Shop(shop_data), GameAction::CashOut, PullEffect::HealthReachesZero) => todo!(),
        (GameState::Shop(shop_data), GameAction::CashOut, PullEffect::PointsLtMilestone) => todo!(),
        (GameState::Shop(shop_data), GameAction::CashOut, PullEffect::PointsGteMilestone) => {
            todo!()
        }
        (GameState::Shop(shop_data), GameAction::CashOut, PullEffect::OrbsReachZero) => todo!(),
        (GameState::Shop(shop_data), GameAction::EnterShop, PullEffect::HealthReachesZero) => {
            todo!()
        }
        (GameState::Shop(shop_data), GameAction::EnterShop, PullEffect::PointsLtMilestone) => {
            todo!()
        }
        (GameState::Shop(shop_data), GameAction::EnterShop, PullEffect::PointsGteMilestone) => {
            todo!()
        }
        (GameState::Shop(shop_data), GameAction::EnterShop, PullEffect::OrbsReachZero) => todo!(),
        (GameState::Shop(shop_data), GameAction::BuyOrb, PullEffect::HealthReachesZero) => todo!(),
        (GameState::Shop(shop_data), GameAction::BuyOrb, PullEffect::PointsLtMilestone) => todo!(),
        (GameState::Shop(shop_data), GameAction::BuyOrb, PullEffect::PointsGteMilestone) => todo!(),
        (GameState::Shop(shop_data), GameAction::BuyOrb, PullEffect::OrbsReachZero) => todo!(),
        (GameState::Shop(shop_data), GameAction::NextLevel, PullEffect::HealthReachesZero) => {
            todo!()
        }
        (GameState::Shop(shop_data), GameAction::NextLevel, PullEffect::PointsLtMilestone) => {
            todo!()
        }
        (GameState::Shop(shop_data), GameAction::NextLevel, PullEffect::PointsGteMilestone) => {
            todo!()
        }
        (GameState::Shop(shop_data), GameAction::NextLevel, PullEffect::OrbsReachZero) => todo!(),
        (
            GameState::CashedOut(cashed_out_data),
            GameAction::PullOrb,
            PullEffect::HealthReachesZero,
        ) => todo!(),
        (
            GameState::CashedOut(cashed_out_data),
            GameAction::PullOrb,
            PullEffect::PointsLtMilestone,
        ) => todo!(),
        (
            GameState::CashedOut(cashed_out_data),
            GameAction::PullOrb,
            PullEffect::PointsGteMilestone,
        ) => todo!(),
        (GameState::CashedOut(cashed_out_data), GameAction::PullOrb, PullEffect::OrbsReachZero) => {
            todo!()
        }
        (
            GameState::CashedOut(cashed_out_data),
            GameAction::CashOut,
            PullEffect::HealthReachesZero,
        ) => todo!(),
        (
            GameState::CashedOut(cashed_out_data),
            GameAction::CashOut,
            PullEffect::PointsLtMilestone,
        ) => todo!(),
        (
            GameState::CashedOut(cashed_out_data),
            GameAction::CashOut,
            PullEffect::PointsGteMilestone,
        ) => todo!(),
        (GameState::CashedOut(cashed_out_data), GameAction::CashOut, PullEffect::OrbsReachZero) => {
            todo!()
        }
        (
            GameState::CashedOut(cashed_out_data),
            GameAction::EnterShop,
            PullEffect::HealthReachesZero,
        ) => todo!(),
        (
            GameState::CashedOut(cashed_out_data),
            GameAction::EnterShop,
            PullEffect::PointsLtMilestone,
        ) => todo!(),
        (
            GameState::CashedOut(cashed_out_data),
            GameAction::EnterShop,
            PullEffect::PointsGteMilestone,
        ) => todo!(),
        (
            GameState::CashedOut(cashed_out_data),
            GameAction::EnterShop,
            PullEffect::OrbsReachZero,
        ) => todo!(),
        (
            GameState::CashedOut(cashed_out_data),
            GameAction::BuyOrb,
            PullEffect::HealthReachesZero,
        ) => todo!(),
        (
            GameState::CashedOut(cashed_out_data),
            GameAction::BuyOrb,
            PullEffect::PointsLtMilestone,
        ) => todo!(),
        (
            GameState::CashedOut(cashed_out_data),
            GameAction::BuyOrb,
            PullEffect::PointsGteMilestone,
        ) => todo!(),
        (GameState::CashedOut(cashed_out_data), GameAction::BuyOrb, PullEffect::OrbsReachZero) => {
            todo!()
        }
        (
            GameState::CashedOut(cashed_out_data),
            GameAction::NextLevel,
            PullEffect::HealthReachesZero,
        ) => todo!(),
        (
            GameState::CashedOut(cashed_out_data),
            GameAction::NextLevel,
            PullEffect::PointsLtMilestone,
        ) => todo!(),
        (
            GameState::CashedOut(cashed_out_data),
            GameAction::NextLevel,
            PullEffect::PointsGteMilestone,
        ) => todo!(),
        (
            GameState::CashedOut(cashed_out_data),
            GameAction::NextLevel,
            PullEffect::OrbsReachZero,
        ) => todo!(),
        (
            GameState::GameOver(game_over_data),
            GameAction::PullOrb,
            PullEffect::HealthReachesZero,
        ) => todo!(),
        (
            GameState::GameOver(game_over_data),
            GameAction::PullOrb,
            PullEffect::PointsLtMilestone,
        ) => todo!(),
        (
            GameState::GameOver(game_over_data),
            GameAction::PullOrb,
            PullEffect::PointsGteMilestone,
        ) => todo!(),
        (GameState::GameOver(game_over_data), GameAction::PullOrb, PullEffect::OrbsReachZero) => {
            todo!()
        }
        (
            GameState::GameOver(game_over_data),
            GameAction::CashOut,
            PullEffect::HealthReachesZero,
        ) => todo!(),
        (
            GameState::GameOver(game_over_data),
            GameAction::CashOut,
            PullEffect::PointsLtMilestone,
        ) => todo!(),
        (
            GameState::GameOver(game_over_data),
            GameAction::CashOut,
            PullEffect::PointsGteMilestone,
        ) => todo!(),
        (GameState::GameOver(game_over_data), GameAction::CashOut, PullEffect::OrbsReachZero) => {
            todo!()
        }
        (
            GameState::GameOver(game_over_data),
            GameAction::EnterShop,
            PullEffect::HealthReachesZero,
        ) => todo!(),
        (
            GameState::GameOver(game_over_data),
            GameAction::EnterShop,
            PullEffect::PointsLtMilestone,
        ) => todo!(),
        (
            GameState::GameOver(game_over_data),
            GameAction::EnterShop,
            PullEffect::PointsGteMilestone,
        ) => todo!(),
        (GameState::GameOver(game_over_data), GameAction::EnterShop, PullEffect::OrbsReachZero) => {
            todo!()
        }
        (
            GameState::GameOver(game_over_data),
            GameAction::BuyOrb,
            PullEffect::HealthReachesZero,
        ) => todo!(),
        (
            GameState::GameOver(game_over_data),
            GameAction::BuyOrb,
            PullEffect::PointsLtMilestone,
        ) => todo!(),
        (
            GameState::GameOver(game_over_data),
            GameAction::BuyOrb,
            PullEffect::PointsGteMilestone,
        ) => todo!(),
        (GameState::GameOver(game_over_data), GameAction::BuyOrb, PullEffect::OrbsReachZero) => {
            todo!()
        }
        (
            GameState::GameOver(game_over_data),
            GameAction::NextLevel,
            PullEffect::HealthReachesZero,
        ) => todo!(),
        (
            GameState::GameOver(game_over_data),
            GameAction::NextLevel,
            PullEffect::PointsLtMilestone,
        ) => todo!(),
        (
            GameState::GameOver(game_over_data),
            GameAction::NextLevel,
            PullEffect::PointsGteMilestone,
        ) => todo!(),
        (GameState::GameOver(game_over_data), GameAction::NextLevel, PullEffect::OrbsReachZero) => {
            todo!()
        }
    }
}
