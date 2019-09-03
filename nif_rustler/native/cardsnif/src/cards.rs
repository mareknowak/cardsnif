#![allow(dead_code)]

extern crate rand;
use rustler::types::tuple::{get_tuple, make_tuple};
use rustler::{Decoder, Encoder, Env, Error, NifResult, Term};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Suit {
    Club,
    Diamond,
    Heart,
    Spade,
}

impl<'a> Decoder<'a> for Suit {
    fn decode(term: Term<'a>) -> NifResult<Self> {
        let suit: &str = Decoder::decode(term)?;
        match suit {
            "Suit::Club" => Ok(Suit::Club),
            "Suit::Diamond" => Ok(Suit::Diamond),
            "Suit::Heart" => Ok(Suit::Heart),
            "Suit::Spade" => Ok(Suit::Spade),
            _ => Err(Error::BadArg),
        }
    }
}

impl Encoder for Suit {
    fn encode<'b>(&self, env: Env<'b>) -> Term<'b> {
        let suit: &str = match self {
            Suit::Club => "Suit::Club",
            Suit::Diamond => "Suit::Diamond",
            Suit::Heart => "Suit::Heart",
            Suit::Spade => "Suit::Spade",
        };
        suit.encode(env)
    }
}

type Suits = [Suit; 4];

pub fn get_suits() -> Suits {
    [Suit::Club, Suit::Diamond, Suit::Heart, Suit::Spade]
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Value {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl<'a> Decoder<'a> for Value {
    fn decode(term: Term<'a>) -> NifResult<Self> {
        let value: &str = Decoder::decode(term)?;
        match value {
            "Value::Two" => Ok(Value::Two),
            "Value::Three" => Ok(Value::Three),
            "Value::Four" => Ok(Value::Four),
            "Value::Five" => Ok(Value::Five),
            "Value::Six" => Ok(Value::Six),
            "Value::Seven" => Ok(Value::Seven),
            "Value::Eight" => Ok(Value::Eight),
            "Value::Nine" => Ok(Value::Nine),
            "Value::Ten" => Ok(Value::Ten),
            "Value::Jack" => Ok(Value::Jack),
            "Value::Queen" => Ok(Value::Queen),
            "Value::King" => Ok(Value::King),
            "Value::Ace" => Ok(Value::Ace),
            _ => Err(Error::BadArg),
        }
    }
}

impl Encoder for Value {
    fn encode<'b>(&self, env: Env<'b>) -> Term<'b> {
        let value: &str = match self {
            Value::Two => "Value::Two",
            Value::Three => "Value::Three",
            Value::Four => "Value::Four",
            Value::Five => "Value::Five",
            Value::Six => "Value::Six",
            Value::Seven => "Value::Seven",
            Value::Eight => "Value::Eight",
            Value::Nine => "Value::Nine",
            Value::Ten => "Value::Ten",
            Value::Jack => "Value::Jack",
            Value::Queen => "Value::Queen",
            Value::King => "Value::King",
            Value::Ace => "Value::Ace",
        };
        value.encode(env)
    }
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

// #[derive(NifRecord)]
// #[rustler(encode, decode)]
// #[must_use]
// #[tag = "record"]
#[derive(Debug, Copy, Clone, PartialEq)]
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

impl<'a> Decoder<'a> for Card {
    fn decode(term: Term<'a>) -> NifResult<Self> {
        let terms = get_tuple(term)?;
        match terms.as_slice() {
            [card, suit, value] => {
                let env = term.get_env();
                if *card == ::atoms::card().encode(env) {
                    Ok(Card(suit.decode()?, value.decode()?))
                } else {
                    Err(Error::BadArg)
                }
            }
            _ => Err(Error::BadArg),
        }
    }
}

impl Encoder for Card {
    fn encode<'b>(&self, env: Env<'b>) -> Term<'b> {
        let (suit, value) = match self {
            Card(suit, value) => (suit, value),
        };
        let card = vec![
            ::atoms::card().encode(env),
            suit.encode(env),
            value.encode(env),
        ];
        make_tuple(env, &card)
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
