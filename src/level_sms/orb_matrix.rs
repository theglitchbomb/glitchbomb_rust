use crate::invalid_states::InvalidState;

#[derive(Copy, Clone)]
enum OrbEffect {
    Health(u32),
    Points(u32),
    Bomb(u32),
    PointsPerAnyOrb(u32),
    PointsPerBombPulled(u32),
    Multiplier(u32),
    GlitchChips(u32),
    MoonRocks(u32),
    // PointRewind,
    // BombImmunity,
    // RiskFiveOrDie,
}

struct Internals {
    pullable: Vec<OrbEffect>,
    consumed: Vec<OrbEffect>,
}

enum OrbMatrix {
    Full(Internals),
    InProgress(Internals),
    Empty,
}

enum OrbMatrixAction {
    Pull,
}

fn stf_orb_matrix(
    orb_matrix: OrbMatrix,
    action: &OrbMatrixAction,
) -> Result<(OrbMatrix, OrbEffect), InvalidState> {
    match (orb_matrix, action) {
        (OrbMatrix::Full(mut internals), OrbMatrixAction::Pull) => match internals.pullable.len() {
            0 => Err(InvalidState::InvalidBag),
            _ => {
                let pulled_orb = internals.pullable.pop().unwrap();
                internals.consumed.push(pulled_orb);
                Ok((OrbMatrix::InProgress(internals), pulled_orb))
            }
        },
        (OrbMatrix::InProgress(mut internals), OrbMatrixAction::Pull) => {
            match internals.pullable.len() {
                0 => Err(InvalidState::InvalidBag),
                _ => {
                    let orb = internals.pullable.pop().unwrap();
                    internals.consumed.push(orb);
                    Ok((OrbMatrix::InProgress(internals), orb))
                }
            }
        }
        (_, _) => Err(InvalidState::InvalidBag),
    }
}

impl OrbMatrix {
    fn new() -> Self {
        let pullable = vec![
            OrbEffect::Health(0),
            OrbEffect::Points(0),
            OrbEffect::Bomb(0),
            OrbEffect::PointsPerAnyOrb(0),
            OrbEffect::PointsPerBombPulled(0),
            OrbEffect::Multiplier(0),
            OrbEffect::GlitchChips(0),
            OrbEffect::MoonRocks(0),
        ];
        let consumed = Vec::new();
        OrbMatrix::Full(Internals { pullable, consumed })
    }
}
