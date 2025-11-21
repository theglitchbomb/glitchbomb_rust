use crate::invalid_states::InvalidState;

const MILESTONE: u32 = 12;

enum Points {
    Zero,
    MilestoneNotMet(u32),
    MilestoneMet(u32),
}

enum PointsAction {
    Add(u32),
}

fn stf_points(points: Points, action: &PointsAction) -> Result<Points, InvalidState> {
    match (points, action) {
        (Points::Zero, PointsAction::Add(amount)) => match amount {
            p if *p >= MILESTONE => Ok(Points::MilestoneMet(*p)),
            p if *p > 0 => Ok(Points::MilestoneNotMet(*p)),
            _ => Err(InvalidState::InvalidPoints),
        },
        (Points::MilestoneNotMet(points), PointsAction::Add(amount)) => match points + amount {
            p if p >= MILESTONE => Ok(Points::MilestoneMet(p)),
            p => Ok(Points::MilestoneNotMet(p)),
        },
        (_, _) => Err(InvalidState::InvalidPoints),
    }
}

impl Points {
    fn new() -> Self {
        Points::Zero
    }
}
