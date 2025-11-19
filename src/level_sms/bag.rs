enum Bag {
    Full(FullData),
    SomeOrbs(SomeOrbsData),
    Empty(EmptyData),
}

struct FullData;
struct SomeOrbsData;
struct EmptyData;

enum BagAction {
    PullOrb,
}

enum PullEffect {
    SomeRemaining,
    NoOrbsRemaining,
}

fn stf_bag(bag: Bag, action: &BagAction, effect: &PullEffect) -> Bag {
    match (bag, action, effect) {
        (Bag::Full(_), BagAction::PullOrb, _) => Bag::SomeOrbs(SomeOrbsData),
        (Bag::SomeOrbs(_), BagAction::PullOrb, PullEffect::SomeRemaining) => {
            Bag::SomeOrbs(SomeOrbsData)
        }
        (Bag::SomeOrbs(_), BagAction::PullOrb, PullEffect::NoOrbsRemaining) => {
            Bag::Empty(EmptyData)
        }
        (state, _, _) => state,
    }
}
