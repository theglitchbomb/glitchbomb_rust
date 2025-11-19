type MoonRocks = Option<MoonRocksData>;

struct MoonRocksData;

enum MoonRocksAction {
    Earn,
    Spend,
}

enum MoonRocksEffect {
    SomeMR,
    ZeroMR,
}

fn stf_moon_rocks(
    moon_rocks: MoonRocks,
    action: &MoonRocksAction,
    effect: &MoonRocksEffect,
) -> MoonRocks {
    match (moon_rocks, action, effect) {
        (None, MoonRocksAction::Earn, _) => Some(MoonRocksData),
        (Some(_), MoonRocksAction::Earn, _) => Some(MoonRocksData),
        (Some(_), MoonRocksAction::Spend, MoonRocksEffect::SomeMR) => Some(MoonRocksData),
        (Some(_), MoonRocksAction::Spend, MoonRocksEffect::ZeroMR) => None,
        (state, _, _) => state,
    }
}
