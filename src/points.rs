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

enum IncrementExecution {
    ZeroPoints,
    PointsLtMilestone,
    PointsGteMilestone,
}

fn stf_points(points: Points, action: &PointsAction, execution: &IncrementExecution) -> Points {
    match (points, action, execution) {
        (Points::Zero(_), PointsAction::Increment, IncrementExecution::PointsLtMilestone) => {
            Points::MilestoneNotMet(MilestoneNotMetData)
        }
        (Points::Zero(_), PointsAction::Increment, IncrementExecution::PointsGteMilestone) => {
            Points::MilestoneMet(MilestoneMetData)
        }
        (
            Points::MilestoneNotMet(_),
            PointsAction::Increment,
            IncrementExecution::PointsLtMilestone,
        ) => Points::MilestoneNotMet(MilestoneNotMetData),
        (
            Points::MilestoneNotMet(_),
            PointsAction::Increment,
            IncrementExecution::PointsGteMilestone,
        ) => Points::MilestoneMet(MilestoneMetData),
        (state, _, _) => state,
    }
}
