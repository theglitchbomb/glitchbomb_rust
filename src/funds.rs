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

enum SpendEffect {
    StillStacked,
    GoingBroke,
}

fn stf_funds(funds: Funds, action: &FundsAction, effect: &SpendEffect) -> Funds {
    match (funds, action, effect) {
        (Funds::Stacked(_), FundsAction::SpendMoney, SpendEffect::StillStacked) => {
            Funds::Stacked(StackedData)
        }
        (Funds::Stacked(_), FundsAction::SpendMoney, SpendEffect::GoingBroke) => {
            Funds::Broke(BrokeData)
        }
        (Funds::Broke(_), FundsAction::EarnMoney, _) => Funds::Stacked(StackedData),
        (state, _, _) => state,
    }
}
