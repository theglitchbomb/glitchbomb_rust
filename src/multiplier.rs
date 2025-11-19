enum Multiplier {
    Base(BaseData),
    Factor(FactorData),
}

struct BaseData;
struct FactorData;

enum MultiplierAction {
    AddMultiplier,
}

fn stf_multiplier(multiplier: Multiplier, action: &MultiplierAction) -> Multiplier {
    match (multiplier, action) {
        (Multiplier::Base(_), MultiplierAction::AddMultiplier) => Multiplier::Factor(FactorData),
        (Multiplier::Factor(_), MultiplierAction::AddMultiplier) => Multiplier::Factor(FactorData),
        (state, _) => state,
    }
}
