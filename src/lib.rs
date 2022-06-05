mod card;
mod unit;

use card::Card;
use unit::{Armor, Health};

#[derive(PartialEq, Debug)]
pub struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    pub fn new() -> Hand {
        Hand { cards: Vec::new() }
    }

    pub fn from(cards: &Vec<Card>) -> Hand {
        let mut sorted_cards = cards.clone();
        sorted_cards.sort();
        Hand {
            cards: sorted_cards,
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct GameState {
    player_health: Health,
    player_armor: Armor,
    hand: Hand,
}

#[cfg(test)]
mod tests {
    use crate::card::{Card, CardName};
    use crate::unit::{Armor, Energy, Health};
    use crate::{GameState, Hand};

    #[test]
    fn equal_gamestates_are_equal() {
        let game_state_1 = GameState {
            player_health: Health(1),
            player_armor: Armor(0),
            hand: Hand::new(),
        };
        let game_state_2 = GameState {
            player_health: Health(1),
            player_armor: Armor(0),
            hand: Hand::new(),
        };
        assert_eq!(game_state_1, game_state_2);
    }

    #[test]
    fn different_gamestates_are_not_equal() {
        let game_state_1 = GameState {
            player_health: Health(1),
            player_armor: Armor(0),
            hand: Hand::new(),
        };
        let game_state_2 = GameState {
            player_health: Health(1),
            player_armor: Armor(1),
            hand: Hand::new(),
        };
        assert_ne!(game_state_1, game_state_2);
    }

    #[test]
    fn empty_hands_are_equal() {
        let hand1 = Hand::new();
        let hand2 = Hand::new();
        assert_eq!(hand1, hand2);
    }

    #[test]
    fn hands_with_same_card_are_equal() {
        let strike = Card {
            name: CardName::Strike,
            cost: Energy::from(1),
        };
        let hand1 = Hand::from(&Vec::from([strike]));
        let hand2 = Hand::from(&Vec::from([strike]));
        assert_eq!(hand1, hand2);
    }

    #[test]
    fn hands_with_same_cards_in_diffent_order_are_equal() {
        let strike = Card {
            name: CardName::Strike,
            cost: Energy::from(1),
        };
        let defend = Card {
            name: CardName::Defend,
            cost: Energy::from(1),
        };
        let hand1 = Hand::from(&Vec::from([strike, defend]));
        let hand2 = Hand::from(&Vec::from([defend, strike]));
        assert_eq!(hand1, hand2);
    }

    #[test]
    fn hands_with_same_cards_in_same_order_are_equal() {
        let strike = Card {
            name: CardName::Strike,
            cost: Energy::from(1),
        };
        let defend = Card {
            name: CardName::Defend,
            cost: Energy::from(1),
        };
        let hand1 = Hand::from(&Vec::from([strike, defend]));
        let hand2 = Hand::from(&Vec::from([strike, defend]));
        assert_eq!(hand1, hand2);
    }
}
