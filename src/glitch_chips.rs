type GlitchChips = Option<GlitchChipsData>;

struct GlitchChipsData;

enum GlitchChipsAction {
    Earn,
    Spend,
}

enum SpendExecution {
    SomeGC,
    ZeroGC,
}

fn stf_glitch_chips(
    glitch_chips: GlitchChips,
    action: &GlitchChipsAction,
    execution: &SpendExecution,
) -> GlitchChips {
    match (glitch_chips, action, execution) {
        (None, GlitchChipsAction::Earn, _) => Some(GlitchChipsData),
        (Some(_), GlitchChipsAction::Spend, SpendExecution::SomeGC) => Some(GlitchChipsData),
        (Some(_), GlitchChipsAction::Spend, SpendExecution::ZeroGC) => None,
        (state, _, _) => state,
    }
}
