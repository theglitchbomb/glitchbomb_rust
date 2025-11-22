use crate::invalid_states::InvalidState;

enum MoonRocks {
    Zero,
    Some(u32),
}

enum MoonRocksAction {
    Earn(u32),
    Spend(u32),
}

fn stf_moon_rocks(
    moon_rocks: MoonRocks,
    action: &MoonRocksAction,
) -> Result<MoonRocks, InvalidState> {
    match (moon_rocks, action) {
        (MoonRocks::Zero, MoonRocksAction::Earn(amount)) => match amount {
            0 => Err(InvalidState::InvalidMoonRocksCannotEarnZero),
            n => Ok(MoonRocks::Some(*n)),
        },
        (MoonRocks::Some(current), MoonRocksAction::Earn(amount)) => match amount {
            0 => Err(InvalidState::InvalidMoonRocksCannotEarnZero),
            n => Ok(MoonRocks::Some(current + n)),
        },
        (MoonRocks::Some(current), MoonRocksAction::Spend(amount)) => match amount {
            0 => Err(InvalidState::InvalidMoonRocksCannotSpendZero),
            n if *n > current => Err(InvalidState::InvalidMoonRocks),
            n if current - n == 0 => Ok(MoonRocks::Zero),
            n => Ok(MoonRocks::Some(current - n)),
        },
        (_, _) => Err(InvalidState::InvalidMoonRocks),
    }
}

impl MoonRocks {
    fn new() -> Self {
        MoonRocks::Zero
    }
}
