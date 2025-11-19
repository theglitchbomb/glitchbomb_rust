enum Funds {
    Stacked(StackedData),
    Broke(BrokeData),
}

struct StackedData;
struct BrokeData;

enum FundsAction {
    SpendMoney,
    EarnMoney,
}

enum SpendExecution {
    StillStacked,
    GoingBroke,
}

fn stf_funds(funds: Funds, action: &FundsAction, execution: &SpendExecution) -> Funds {
    match (funds, action, execution) {
        (Funds::Stacked(_), FundsAction::SpendMoney, SpendExecution::StillStacked) => {
            Funds::Stacked(StackedData)
        }
        (Funds::Stacked(_), FundsAction::SpendMoney, SpendExecution::GoingBroke) => {
            Funds::Broke(BrokeData)
        }
        (Funds::Broke(_), FundsAction::EarnMoney, _) => Funds::Stacked(StackedData),
        (state, _, _) => state,
    }
}
