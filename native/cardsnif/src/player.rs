use serde::{Deserialize, Serialize};

use crate::cards::Card;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Command {
    #[serde(rename = "Elixir.PlayerRust.CommandAddCards")]
    AddCards(Vec<Card>), // add list of cards to the model
    #[serde(rename = "Elixir.PlayerRust.CommandRemoveCards")]
    RemoveCards(usize), // remove nr of cards
}

/*
Msg to Player

Player's update function takes model, msg and returns new model and cmd to be
sent by Elixir
*/

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Elixir.PlayerRust.Msg")]
pub struct Msg {
    pub from: Vec<u8>,
    pub command: Command,
}

/*
Response from update function - Cmd to be executed by runtime
*/

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Response {
    #[serde(rename = "Elixir.PlayerRust.ResponseCardsAdded")]
    CardsAdded(usize),
    #[serde(rename = "Elixir.PlayerRust.ResponseCardsRemoved")]
    CardsRemoved(Vec<Card>),
    #[serde(rename = "Elixir.PlayerRust.ResponseUnableToRemoveCards")]
    UnableToRemoveCards(usize), // usize - nr of cards
    #[serde(rename = "Elixir.PlayerRust.ResponseError")]
    Error(String),
}

/*
Cmd will be sent to Elixir and executed
*/
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Elixir.PlayerRust.Cmd")]
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
                ]);
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
        let model =             vec![
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
