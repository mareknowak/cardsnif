#![allow(dead_code)]

use rustler::types::tuple::{get_tuple, make_tuple};
use rustler::{Decoder, Encoder, Env, Error, NifResult, Term};
use rustler_codegen::{NifStruct};

use crate::cards::Card;

#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    AddCards(Vec<Card>), // add list of cards to the model
    RemoveCards(usize),  // remove nr of cards
}

impl<'a> Decoder<'a> for Command {
    fn decode(term: Term<'a>) -> NifResult<Self> {
        let terms = get_tuple(term)?;
        match terms.as_slice() {
            [command, arg] => {
                let env = term.get_env();
                if *command == ::atoms::player_command_add_cards().encode(env) {
                    Ok(Command::AddCards(arg.decode()?))
                } else if *command == ::atoms::player_command_remove_cards().encode(env) {
                    Ok(Command::RemoveCards(arg.decode()?))
                } else {
                    Err(Error::BadArg)
                }
            }
            _ => Err(Error::BadArg),
        }
    }
}

impl Encoder for Command {
    fn encode<'b>(&self, env: Env<'b>) -> Term<'b> {
        match self {
            Command::AddCards(cards) => {
                let add_cards = vec![
                    ::atoms::player_command_add_cards().encode(env),
                    cards.encode(env),
                ];
                make_tuple(env, &add_cards)
            }
            Command::RemoveCards(nr) => {
                let remove_cards = vec![
                    ::atoms::player_command_remove_cards().encode(env),
                    nr.encode(env),
                ];
                make_tuple(env, &remove_cards)
            }
        }
    }
}


/*
Msg to Player

Player's update function takes model, msg and returns new model and cmd to be
sent by Elixir
*/

#[derive(NifStruct)]
#[must_use] // Added to test Issue #152
#[module = "PlayerRust.Msg"]
#[derive(Debug, Clone, PartialEq)]
pub struct Msg {
    pub from: Vec<u8>,
    pub command: Command,
}

/*
Response from update function - Cmd to be executed by runtime
*/

#[derive(Debug, Clone, PartialEq)]
pub enum Response {
    CardsAdded(usize),
    CardsRemoved(Vec<Card>),
    UnableToRemoveCards(usize), // usize - nr of cards
    Error(String),
}

impl<'a> Decoder<'a> for Response {
    fn decode(term: Term<'a>) -> NifResult<Self> {
        let terms = get_tuple(term)?;
        match terms.as_slice() {
            [command, arg] => {
                let env = term.get_env();
                if *command == ::atoms::player_response_cards_added().encode(env) {
                    Ok(Response::CardsAdded(arg.decode()?))
                } else if *command == ::atoms::player_response_cards_removed().encode(env) {
                    Ok(Response::CardsRemoved(arg.decode()?))
                } else if *command == ::atoms::player_response_unable_to_remove_cards().encode(env) {
                    Ok(Response::UnableToRemoveCards(arg.decode()?))
                } else if *command == ::atoms::player_response_error().encode(env) {
                    Ok(Response::Error(arg.decode()?))
                } else {
                    Err(Error::BadArg)
                }
            }
            _ => Err(Error::BadArg),
        }
    }
}

impl Encoder for Response {
    fn encode<'b>(&self, env: Env<'b>) -> Term<'b> {
        match self {
            Response::CardsAdded(nr) => {
                let cards_added = vec![
                    ::atoms::player_response_cards_added().encode(env),
                    nr.encode(env),
                ];
                make_tuple(env, &cards_added)
            }
            Response::CardsRemoved(cards) => {
                let cards_removed = vec![
                    ::atoms::player_response_cards_removed().encode(env),
                    cards.encode(env),
                ];
                make_tuple(env, &cards_removed)
            }
            Response::UnableToRemoveCards(nr) => {
                let unable_to_remove_cards = vec![
                    ::atoms::player_response_unable_to_remove_cards().encode(env),
                    nr.encode(env),
                ];
                make_tuple(env, &unable_to_remove_cards)
            }
            Response::Error(err) => {
                let error = vec![
                    ::atoms::player_response_error().encode(env),
                    err.encode(env),
                ];
                make_tuple(env, &error)
            }
        }
    }
}


/*
Cmd will be sent to Elixir and executed
*/
#[derive(NifStruct)]
#[must_use] // Added to test Issue #152
#[module = "PlayerRust.Cmd"]
#[derive(Debug, Clone, PartialEq)]
pub struct Cmd {
    pub game: Vec<u8>,
    pub response: Response,
}

pub type Model = Vec<Card>;

pub fn update(model: Model, msg: Msg) -> (Model, Cmd) {
    match (model, msg) {
        (
            cards,
            Msg {
                from: sender,
                command: Command::AddCards(cards_to_add),
            },
        ) => {
            // let () = cards;
            let mut new_cards = cards.to_vec();
            let mut cards_received = cards_to_add.to_vec();
            new_cards.append(&mut cards_received);
            (
                new_cards,
                Cmd {
                    game: sender,
                    response: Response::CardsAdded(cards_to_add.len()),
                },
            )
        }
        (
            cards,
            Msg {
                from: sender,
                command: Command::RemoveCards(nr),
            },
        ) => {
            // let () = nr;
            let cards = cards.to_vec();
            let nr_cards = nr;
            // Ok to remove cards
            if cards.len() >= nr_cards {
                let mut cards_left = cards.to_vec();
                let length = cards_left.len();
                let cards_to_send = cards_left.split_off(length - nr_cards);
                //let () = cards_left;
                (
                    cards_left,
                    Cmd {
                        game: sender,
                        response: Response::CardsRemoved(cards_to_send.to_vec()),
                    },
                )
            } else {
                (
                    cards,
                    Cmd {
                        game: sender,
                        response: Response::UnableToRemoveCards(nr_cards),
                    },
                )
            }
        }
    }
}

#[cfg(test)]
mod update {
    use super::*;
    use crate::cards::Suit;
    use crate::cards::Value;

    #[test]
    fn add_card_to_empty_model() {
        let model = Vec::new();
        let card = Card(Suit::Heart, Value::Ace);
        let msg = Msg {
            from: vec![0],
            command: Command::AddCards(vec![card]),
        };
        let (updated_model, _cmd) = update(model, msg);
        assert_eq!(updated_model, vec![card]);
    }

    #[test]
    fn add_cards_to_model() {
        let model = vec![
            Card(Suit::Club, Value::Two),
            Card(Suit::Club, Value::Three),
            Card(Suit::Diamond, Value::Ace),
        ];
        let msg = Msg {
            from: vec![0],
            command: Command::AddCards(vec![
                Card(Suit::Heart, Value::Two),
                Card(Suit::Heart, Value::Three),
                Card(Suit::Spade, Value::Ace),
            ]),
        };
        let (updated_model, _cmd) = update(model, msg);
        assert_eq!(
            updated_model,
            vec![
                Card(Suit::Club, Value::Two),
                Card(Suit::Club, Value::Three),
                Card(Suit::Diamond, Value::Ace),
                Card(Suit::Heart, Value::Two),
                Card(Suit::Heart, Value::Three),
                Card(Suit::Spade, Value::Ace),
            ]
        );
    }

    #[test]
    fn remove_cards_success() {
        let model = vec![
            Card(Suit::Club, Value::Two),
            Card(Suit::Club, Value::Three),
            Card(Suit::Diamond, Value::Ace),
            Card(Suit::Heart, Value::Two),
            Card(Suit::Heart, Value::Three),
            Card(Suit::Spade, Value::Ace),
        ];
        let msg = Msg {
            from: vec![0],
            command: Command::RemoveCards(2),
        };
        let (updated_model, cmd) = update(model, msg);
        assert_eq!(
            (updated_model, cmd),
            (
                vec![
                    Card(Suit::Club, Value::Two),
                    Card(Suit::Club, Value::Three),
                    Card(Suit::Diamond, Value::Ace),
                    Card(Suit::Heart, Value::Two),
                ],
                Cmd {
                    game: vec![0],
                    response: Response::CardsRemoved(vec![
                        Card(Suit::Heart, Value::Three),
                        Card(Suit::Spade, Value::Ace),
                    ])
                }
            )
        );
    }

    #[test]
    fn remove_cards_failure() {
        let model = vec![
            Card(Suit::Club, Value::Two),
            Card(Suit::Club, Value::Three),
            Card(Suit::Diamond, Value::Ace),
            Card(Suit::Heart, Value::Two),
            Card(Suit::Heart, Value::Three),
            Card(Suit::Spade, Value::Ace),
        ];
        let msg = Msg {
            from: vec![0],
            command: Command::RemoveCards(12),
        };
        let (updated_model, cmd) = update(model, msg);
        assert_eq!(
            (updated_model, cmd),
            (
                vec![
                    Card(Suit::Club, Value::Two),
                    Card(Suit::Club, Value::Three),
                    Card(Suit::Diamond, Value::Ace),
                    Card(Suit::Heart, Value::Two),
                    Card(Suit::Heart, Value::Three),
                    Card(Suit::Spade, Value::Ace),
                ],
                Cmd {
                    game: vec![0],
                    response: Response::UnableToRemoveCards(12)
                }
            )
        );
    }

    #[test]
    fn remove_cards_from_empty() {
        let model = Vec::new();
        let msg = Msg {
            from: vec![0],
            command: Command::RemoveCards(2),
        };
        let (updated_model, cmd) = update(model, msg);
        assert_eq!(
            (updated_model, cmd),
            (
                vec![],
                Cmd {
                    game: vec![0],
                    response: Response::UnableToRemoveCards(2)
                }
            )
        );
    }
}
