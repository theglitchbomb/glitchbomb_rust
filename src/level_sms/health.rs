use crate::invalid_states::InvalidState;

const MAX_HEALTH: u32 = 100;

enum Health {
    Zero,
    Damaged(u32),
    Max,
}

enum HealthAction {
    Heal(u32),
    TakeDamage(u32),
}

fn stf_health(health: Health, action: &HealthAction) -> Result<Health, InvalidState> {
    match (health, action) {
        (Health::Max, HealthAction::TakeDamage(dmg)) => match dmg {
            0 => Err(InvalidState::InvalidHealth),
            n if *n >= MAX_HEALTH => Ok(Health::Zero),
            n => Ok(Health::Damaged(MAX_HEALTH - n)),
        },
        (Health::Damaged(hp), HealthAction::Heal(heal)) => match heal {
            0 => Err(InvalidState::InvalidHealth),
            n if hp + n >= MAX_HEALTH => Ok(Health::Max),
            n => Ok(Health::Damaged(hp + n)),
        },
        (Health::Damaged(hp), HealthAction::TakeDamage(dmg)) => match dmg {
            0 => Err(InvalidState::InvalidHealth),
            n if *n >= hp => Ok(Health::Zero),
            n => Ok(Health::Damaged(hp - n)),
        },
        (_, _) => Err(InvalidState::InvalidHealth),
    }
}

impl Health {
    fn new() -> Self {
        Health::Max
    }
}
