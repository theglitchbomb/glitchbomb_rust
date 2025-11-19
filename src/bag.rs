enum Bag {
    Full(FullData),
    InProgress(InProgressData),
    Empty(EmptyData),
}

struct FullData;
struct InProgressData;
struct EmptyData;

enum BagAction {
    PullOrb,
}

enum PullEffect {
    OrbsRemaining,
    LastOrb,
}

fn stf_bag(bag: Bag, action: &BagAction, effect: &PullEffect) -> Bag {
    match (bag, action, effect) {
        (Bag::Full(_), BagAction::PullOrb, PullEffect::OrbsRemaining) => {
            Bag::InProgress(InProgressData)
        }
        (Bag::Full(_), BagAction::PullOrb, PullEffect::LastOrb) => Bag::Empty(EmptyData),
        (Bag::InProgress(_), BagAction::PullOrb, PullEffect::OrbsRemaining) => {
            Bag::InProgress(InProgressData)
        }
        (Bag::InProgress(_), BagAction::PullOrb, PullEffect::LastOrb) => Bag::Empty(EmptyData),
        (state, _, _) => state,
    }
}
