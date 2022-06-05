#[derive(PartialEq, Debug)]
pub struct Health(i32);

#[derive(PartialEq, Debug)]
pub struct Armor(i32);

#[derive(PartialEq, Debug)]
pub struct GameState {
    player_health: Health,
    player_armor: Armor,
}

#[cfg(test)]
mod tests {
    use crate::{Armor, GameState, Health};

    #[test]
    fn equal_gamestates_are_equal() {
        let game_state_1 = GameState {
            player_health: Health(1),
            player_armor: Armor(0),
        };
        let game_state_2 = GameState {
            player_health: Health(1),
            player_armor: Armor(0),
        };
        assert_eq!(game_state_1, game_state_2);
    }

    #[test]
    fn different_gamestates_are_not_equal() {
        let game_state_1 = GameState {
            player_health: Health(1),
            player_armor: Armor(0),
        };
        let game_state_2 = GameState {
            player_health: Health(1),
            player_armor: Armor(1),
        };
        assert_ne!(game_state_1, game_state_2);
    }
}
