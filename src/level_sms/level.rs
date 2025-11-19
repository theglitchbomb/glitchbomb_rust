enum Level {
    Base(BaseData),
    Some(SomeData),
    Max(MaxData),
}

struct BaseData;
struct SomeData;
struct MaxData;

enum LevelAction {
    LevelUp,
}

enum LevelUpEffect {
    StillSome,
    ReachedMax,
}

fn stf_level(level: Level, action: &LevelAction, effect: &LevelUpEffect) -> Level {
    match (level, action, effect) {
        (Level::Base(_), LevelAction::LevelUp, _) => Level::Some(SomeData),
        (Level::Some(_), LevelAction::LevelUp, LevelUpEffect::StillSome) => Level::Some(SomeData),
        (Level::Some(_), LevelAction::LevelUp, LevelUpEffect::ReachedMax) => Level::Max(MaxData),
        (state, _, _) => state,
    }
}
