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

enum PullExecution {
    OrbsRemaining,
    LastOrb,
}

fn stf_bag(bag: Bag, action: &BagAction, execution: &PullExecution) -> Bag {
    match (bag, action, execution) {
        (Bag::Full(_), BagAction::PullOrb, PullExecution::OrbsRemaining) => {
            Bag::InProgress(InProgressData)
        }
        (Bag::Full(_), BagAction::PullOrb, PullExecution::LastOrb) => Bag::Empty(EmptyData),
        (Bag::InProgress(_), BagAction::PullOrb, PullExecution::OrbsRemaining) => {
            Bag::InProgress(InProgressData)
        }
        (Bag::InProgress(_), BagAction::PullOrb, PullExecution::LastOrb) => Bag::Empty(EmptyData),
        (state, _, _) => state,
    }
}
