enum Player {
    Idle(IdleData),
    HasPack(PackOwnedData),
    Playing(PlayingData),
}

struct IdleData;
struct PackOwnedData;
struct PlayingData;

enum PlayerAction {
    BuyPack,
    OpenPack,
    CompletePack,
}

fn stf_player(player: Player, action: &PlayerAction) -> Player {
    match (player, action) {
        (Player::Idle(_), PlayerAction::BuyPack) => Player::HasPack(PackOwnedData),
        (Player::HasPack(_), PlayerAction::OpenPack) => Player::Playing(PlayingData),
        (Player::Playing(_), PlayerAction::CompletePack) => Player::Idle(IdleData),
        (state, _) => state,
    }
}
