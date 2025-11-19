enum Points {
    Zero(ZeroData),
    MilestoneNotMet(MilestoneNotMetData),
    MilestoneMet(MilestoneMetData),
}

struct ZeroData;
struct MilestoneNotMetData;
struct MilestoneMetData;

enum PointsAction {
    Increment,
}

enum IncrementEffect {
    ZeroPoints,
    PointsLtMilestone,
    PointsGteMilestone,
}

fn stf_points(points: Points, action: &PointsAction, effect: &IncrementEffect) -> Points {
    match (points, action, effect) {
        (Points::Zero(_), PointsAction::Increment, IncrementEffect::PointsLtMilestone) => {
            Points::MilestoneNotMet(MilestoneNotMetData)
        }
        (Points::Zero(_), PointsAction::Increment, IncrementEffect::PointsGteMilestone) => {
            Points::MilestoneMet(MilestoneMetData)
        }
        (
            Points::MilestoneNotMet(_),
            PointsAction::Increment,
            IncrementEffect::PointsLtMilestone,
        ) => Points::MilestoneNotMet(MilestoneNotMetData),
        (
            Points::MilestoneNotMet(_),
            PointsAction::Increment,
            IncrementEffect::PointsGteMilestone,
        ) => Points::MilestoneMet(MilestoneMetData),
        (state, _, _) => state,
    }
}
