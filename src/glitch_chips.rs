type GlitchChips = Option<GlitchChipsData>;

struct GlitchChipsData;

enum GlitchChipsAction {
    Earn,
    Spend,
}

enum GlitchChipsEffect {
    SomeGC,
    ZeroGC,
}

fn stf_glitch_chips(
    glitch_chips: GlitchChips,
    action: &GlitchChipsAction,
    effect: &GlitchChipsEffect,
) -> GlitchChips {
    match (glitch_chips, action, effect) {
        (None, GlitchChipsAction::Earn, _) => Some(GlitchChipsData),
        (Some(_), GlitchChipsAction::Earn, _) => Some(GlitchChipsData),
        (Some(_), GlitchChipsAction::Spend, GlitchChipsEffect::SomeGC) => Some(GlitchChipsData),
        (Some(_), GlitchChipsAction::Spend, GlitchChipsEffect::ZeroGC) => None,
        (state, _, _) => state,
    }
}
