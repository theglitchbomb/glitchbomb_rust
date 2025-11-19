type GlitchChips = Option<GlitchChipsData>;

struct GlitchChipsData;

enum GlitchChipsAction {
    Earn,
    Spend,
}

enum SpendEffect {
    SomeGC,
    ZeroGC,
}

fn stf_glitch_chips(
    glitch_chips: GlitchChips,
    action: &GlitchChipsAction,
    effect: &SpendEffect,
) -> GlitchChips {
    match (glitch_chips, action, effect) {
        (None, GlitchChipsAction::Earn, _) => Some(GlitchChipsData),
        (Some(_), GlitchChipsAction::Spend, SpendEffect::SomeGC) => Some(GlitchChipsData),
        (Some(_), GlitchChipsAction::Spend, SpendEffect::ZeroGC) => None,
        (state, _, _) => state,
    }
}
