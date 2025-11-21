use crate::invalid_states::InvalidState;

enum GlitchChips {
    Zero,
    Some(u32),
}

enum GlitchChipsAction {
    Earn(u32),
    Spend(u32),
}

fn stf_glitch_chips(
    glitch_chips: GlitchChips,
    action: &GlitchChipsAction,
) -> Result<GlitchChips, InvalidState> {
    match (glitch_chips, action) {
        (GlitchChips::Zero, GlitchChipsAction::Earn(amount)) => match amount {
            0 => Err(InvalidState::InvalidGlitchChipsCannotEarnZero),
            n => Ok(GlitchChips::Some(*n)),
        },
        (GlitchChips::Some(current), GlitchChipsAction::Earn(amount)) => match amount {
            0 => Err(InvalidState::InvalidGlitchChipsCannotEarnZero),
            n => Ok(GlitchChips::Some(current + n)),
        },
        (GlitchChips::Some(current), GlitchChipsAction::Spend(amount)) => match amount {
            0 => Err(InvalidState::InvalidGlitchChipsCannotSpendZero),
            n if *n > current => Err(InvalidState::InvalidGlitchChips),
            n if current - n == 0 => Ok(GlitchChips::Zero),
            n => Ok(GlitchChips::Some(current - n)),
        },
        (_, _) => Err(InvalidState::InvalidGlitchChips),
    }
}

impl GlitchChips {
    fn new() -> Self {
        GlitchChips::Zero
    }
}
