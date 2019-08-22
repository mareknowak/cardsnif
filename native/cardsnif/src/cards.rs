// #![allow(dead_code)]

extern crate rand;
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum Suit {
    #[serde(rename = "Suit::Club")]
    Club,
    #[serde(rename = "Suit::Diamond")]
    Diamond,
    #[serde(rename = "Suit::Heart")]
    Heart,
    #[serde(rename = "Suit::Spade")]
    Spade,
}

type Suits = [Suit; 4];

pub fn get_suits() -> Suits {
    [Suit::Club, Suit::Diamond, Suit::Heart, Suit::Spade]
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum Value {
    #[serde(rename = "Value::Two")]
    Two,
    #[serde(rename = "Value::Three")]
    Three,
    #[serde(rename = "Value::Four")]
    Four,
    #[serde(rename = "Value::Five")]
    Five,
    #[serde(rename = "Value::Six")]
    Six,
    #[serde(rename = "Value::Seven")]
    Seven,
    #[serde(rename = "Value::Eight")]
    Eight,
    #[serde(rename = "Value::Nine")]
    Nine,
    #[serde(rename = "Value::Ten")]
    Ten,
    #[serde(rename = "Value::Jack")]
    Jack,
    #[serde(rename = "Value::Queen")]
    Queen,
    #[serde(rename = "Value::King")]
    King,
    #[serde(rename = "Value::Ace")]
    Ace,
}

type Values = [Value; 13];

pub fn get_values() -> Values {
    [
        Value::Two,
        Value::Three,
        Value::Four,
        Value::Five,
        Value::Six,
        Value::Seven,
        Value::Eight,
        Value::Nine,
        Value::Ten,
        Value::Jack,
        Value::Queen,
        Value::King,
        Value::Ace,
    ]
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Elixir.Card")]
pub struct Card(pub Suit, pub Value);

impl Card {
    pub fn value(&self) -> u32 {
        match self.1 {
            Value::Two => 2,
            Value::Three => 3,
            Value::Four => 4,
            Value::Five => 5,
            Value::Six => 6,
            Value::Seven => 7,
            Value::Eight => 8,
            Value::Nine => 9,
            Value::Ten => 10,
            Value::Jack => 11,
            Value::Queen => 12,
            Value::King => 13,
            Value::Ace => 14,
        }
    }
}

pub fn first_is_less(first: &Card, second: &Card) -> bool {
    first.value() < second.value()
}

#[cfg(test)]
mod first_is_less {
    use super::*;

    #[test]
    fn two_less_than_ace() {
        let first = Card(Suit::Club, Value::Two);
        let second = Card(Suit::Heart, Value::Ace);
        assert_eq!(first_is_less(&first, &second), true);
    }

    #[test]
    fn jack_not_less_than_nine() {
        let first = Card(Suit::Diamond, Value::Jack);
        let second = Card(Suit::Spade, Value::Nine);
        assert_eq!(first_is_less(&first, &second), false);
    }
}

pub fn cards_are_equal(first: &Card, second: &Card) -> bool {
    first.value() == second.value()
}

#[cfg(test)]
mod cards_are_equal {
    use super::*;

    #[test]
    fn spade_ace_eq_diamond_ace() {
        let spade_ace = Card(Suit::Spade, Value::Ace);
        let diamond_ace = Card(Suit::Diamond, Value::Ace);
        assert_eq!(cards_are_equal(&spade_ace, &diamond_ace), true);
    }
}

fn make_deck() -> [Card; 52] {
    let suits = get_suits();
    let values = get_values();
    let mut deck: [Card; 52] = [Card(Suit::Spade, Value::Ace); 52];
    let mut i = 0;
    for suit in suits.iter() {
        for value in values.iter() {
            deck[i] = Card(*suit, *value);
            i += 1;
        }
    }
    deck
}

#[cfg(test)]
mod make_deck {
    use super::*;

    #[test]
    fn len_of_deck_is_52() {
        let deck = make_deck();
        assert_eq!(deck.len(), 52);
    }

    #[test]
    fn check_first_4_cards() {
        let deck = make_deck();
        let four_cards = [
            Card(Suit::Club, Value::Two),
            Card(Suit::Club, Value::Three),
            Card(Suit::Club, Value::Four),
            Card(Suit::Club, Value::Five),
        ];
        assert_eq!(deck[0..4], four_cards[..]);
    }
}

pub fn shuffled_deck() -> [Card; 52] {
    use cards::rand::prelude::*;
    let mut deck = make_deck();
    let len = deck.len();
    // println!("len: {}", len);
    for i in 0..(len - 2) {
        let ind = rand::thread_rng().gen_range(i + 1, len);
        // println!("i: {}, ind: {}",i , ind);
        deck.swap(i, ind);
    }
    deck
}

#[cfg(test)]
mod shuffled_deck {
    use super::*;
    use cards::rand::prelude::*;

    #[test]
    fn print_rand() {
        //let x: u8 = random();
        let x: u8 = rand::thread_rng().gen_range(52, 53);
        // let _shuffled_deck = shuffled_deck();
        // let mut _i = 1;
        // for card in _shuffled_deck.iter() {
        //     println!("{}: {:?}", _i, card);
        //     _i += 1;
        // }
        assert_eq!(x, 52);
    }
}
