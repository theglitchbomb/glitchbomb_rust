use crate::invalid_states::InvalidState;

const BASE_LEVEL: u32 = 1;
const MAX_LEVEL: u32 = 7;

enum Level {
    Zero,
    Active(u32),
    Max,
}

impl Level {
    fn new() -> Self {
        Level::Zero
    }
}

enum LevelAction {
    LevelUp,
}

fn stf_level(level: Level, action: &LevelAction) -> Result<Level, InvalidState> {
    match (level, action) {
        (Level::Zero, LevelAction::LevelUp) => Ok(Level::Active(BASE_LEVEL)),
        (Level::Active(n), LevelAction::LevelUp) => match n + 1 {
            2..MAX_LEVEL => Ok(Level::Active(n + 1)),
            MAX_LEVEL => Ok(Level::Max),
            _ => Err(InvalidState::InvalidLevel),
        },
        (_, _) => Err(InvalidState::InvalidLevel),
    }
}
