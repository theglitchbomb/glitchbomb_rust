enum Health {
    Max(FullData),
    Damaged(DamagedData),
    Zero(ZeroData),
}

struct FullData;
struct DamagedData;
struct ZeroData;

enum HealthAction {
    Heal,
    TakeDamage,
}

enum HealthEffect {
    MaxHP,
    SomeHP,
    ZeroHP,
}

fn stf_health(health: Health, action: &HealthAction, effect: &HealthEffect) -> Health {
    match (health, action, effect) {
        (Health::Max(_), HealthAction::TakeDamage, _) => Health::Damaged(DamagedData),
        (Health::Damaged(_), HealthAction::Heal, HealthEffect::MaxHP) => Health::Max(FullData),
        (Health::Damaged(_), HealthAction::Heal, HealthEffect::SomeHP) => {
            Health::Damaged(DamagedData)
        }
        (Health::Damaged(_), HealthAction::TakeDamage, HealthEffect::SomeHP) => {
            Health::Damaged(DamagedData)
        }
        (Health::Damaged(_), HealthAction::TakeDamage, HealthEffect::ZeroHP) => {
            Health::Zero(ZeroData)
        }
        (state, _, _) => state,
    }
}
