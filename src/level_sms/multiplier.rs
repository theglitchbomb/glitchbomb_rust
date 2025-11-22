use crate::invalid_states::InvalidState;

const BASE_MULTIPLIER: u32 = 100;
const MULTIPLIER_INCREMENT: u32 = 50;

enum Multiplier {
    Base,
    Factor(u32),
}

enum MultiplierAction {
    Add(u32),
}

fn stf_multiplier(
    multiplier: Multiplier,
    action: &MultiplierAction,
) -> Result<Multiplier, InvalidState> {
    match (multiplier, action) {
        (Multiplier::Base, MultiplierAction::Add(amount)) => match amount {
            0 => Err(InvalidState::InvalidMultiplierCannotAddZero),
            n if n % MULTIPLIER_INCREMENT != 0 => {
                Err(InvalidState::InvalidMultiplierMustBeMultipleOf50)
            }
            n => Ok(Multiplier::Factor(BASE_MULTIPLIER + n)),
        },
        (Multiplier::Factor(current), MultiplierAction::Add(amount)) => match amount {
            0 => Err(InvalidState::InvalidMultiplierCannotAddZero),
            n if n % MULTIPLIER_INCREMENT != 0 => {
                Err(InvalidState::InvalidMultiplierMustBeMultipleOf50)
            }
            n => Ok(Multiplier::Factor(current + n)),
        },
        (_, _) => Err(InvalidState::InvalidMultiplier),
    }
}

impl Multiplier {
    fn new() -> Self {
        Multiplier::Base
    }
}
