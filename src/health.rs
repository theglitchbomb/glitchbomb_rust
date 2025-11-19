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

enum HealthExecution {
    MaxHP,
    SomeHP,
    ZeroHP,
}

fn stf_health(health: Health, action: &HealthAction, execution: &HealthExecution) -> Health {
    match (health, action, execution) {
        (Health::Max(_), HealthAction::TakeDamage, _) => Health::Damaged(DamagedData),
        (Health::Damaged(_), HealthAction::Heal, HealthExecution::MaxHP) => Health::Max(FullData),
        (Health::Damaged(_), HealthAction::Heal, HealthExecution::SomeHP) => {
            Health::Damaged(DamagedData)
        }
        (Health::Damaged(_), HealthAction::TakeDamage, HealthExecution::SomeHP) => {
            Health::Damaged(DamagedData)
        }
        (Health::Damaged(_), HealthAction::TakeDamage, HealthExecution::ZeroHP) => {
            Health::Zero(ZeroData)
        }
        (state, _, _) => state,
    }
}
