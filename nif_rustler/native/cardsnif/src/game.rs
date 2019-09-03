#![allow(dead_code)]

/*
Comment in GameElixir.ex can be helpful to understand logic of this module.
 */

use rustler::types::tuple::{get_tuple, make_tuple};
use rustler::{Decoder, Encoder, Env, Error, NifResult, Term};
use rustler_codegen::NifStruct;

use crate::player::Response as PlayerResp;

#[derive(NifStruct)]
#[must_use] // Added to test Issue #152
#[module = "GameRust.Pids"]
#[derive(Debug, Clone, PartialEq)]
pub struct Pids {
    pub supervisor: Vec<u8>,
    pub player1: Vec<u8>,
    pub player2: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Msg {
    StartGame,
    ResponseFromPlayer(Vec<u8>, PlayerResp), // player_id, response
}

impl<'a> Decoder<'a> for Msg {
    fn decode(term: Term<'a>) -> NifResult<Self> {
        if !term.is_tuple() {
            let start: &str = term.decode()?;
            if start == "Msg::StartGame" {
                Ok(Msg::StartGame)
            } else {
                Err(Error::BadArg)
            }
        } else {
            let terms = get_tuple(term)?;
            let env = term.get_env();
            match terms.as_slice() {
                [response, pid, player_response] => {
                    if *response == ::atoms::game_msg_response_from_player().encode(env) {
                        Ok(Msg::ResponseFromPlayer(
                            pid.decode()?,
                            player_response.decode()?,
                        ))
                    } else {
                        Err(Error::BadArg)
                    }
                }
                _ => Err(Error::BadArg),
            }
        }
    }
}

impl Encoder for Msg {
    fn encode<'b>(&self, env: Env<'b>) -> Term<'b> {
        match self {
            Msg::StartGame => {
                let start = vec!["Msg::StartGame".encode(env)];
                make_tuple(env, &start)
            }
            Msg::ResponseFromPlayer(pid, player_response) => {
                let response = vec![
                    ::atoms::game_msg_response_from_player().encode(env),
                    pid.encode(env),
                    player_response.encode(env),
                ];
                make_tuple(env, &response)
            }
        }
    }
}

use crate::cards::Card;

type Pile = Vec<Card>; // pile of cards

#[derive(Debug, Clone, PartialEq)]
pub enum Model {
    None(Pids),
    Players(Pids),
    PlayersWithResponse(Pids, Vec<u8>, PlayerResp),
    Battle(Pids),
    BattleWithResponse(Pids, Vec<u8>, PlayerResp),
    BattleWonByPlayer(Pids, Vec<u8>),
    War(Pids, Pile),
    WarWithResponse(Pids, Pile, Vec<u8>, PlayerResp),
    WarWonByPlayer(Pids, Vec<u8>),
    Player1Won(Pids),
    Player2Won(Pids),
    Tie(Pids),
    Error(Pids, String),
}

impl<'a> Decoder<'a> for Model {
    fn decode(term: Term<'a>) -> NifResult<Self> {
        let terms = get_tuple(term)?;
        let env = term.get_env();
        match terms.as_slice() {
            [command, arg] => {
                if *command == ::atoms::game_model_none().encode(env) {
                    Ok(Model::None(arg.decode()?))
                } else if *command == ::atoms::game_model_players().encode(env) {
                    Ok(Model::Players(arg.decode()?))
                } else if *command == ::atoms::game_model_battle().encode(env) {
                    Ok(Model::Battle(arg.decode()?))
                } else if *command == ::atoms::game_model_player1_won().encode(env) {
                    Ok(Model::Player1Won(arg.decode()?))
                } else if *command == ::atoms::game_model_player2_won().encode(env) {
                    Ok(Model::Player2Won(arg.decode()?))
                } else if *command == ::atoms::game_model_tie().encode(env) {
                    Ok(Model::Tie(arg.decode()?))
                } else {
                    Err(Error::BadArg)
                }
            }
            [command, arg1, arg2] => {
                if *command == ::atoms::game_model_battle_won_by_player().encode(env) {
                    Ok(Model::BattleWonByPlayer(arg1.decode()?, arg2.decode()?))
                } else if *command == ::atoms::game_model_war().encode(env) {
                    Ok(Model::War(arg1.decode()?, arg2.decode()?))
                } else if *command == ::atoms::game_model_war_won_by_player().encode(env) {
                    Ok(Model::WarWonByPlayer(arg1.decode()?, arg2.decode()?))
                } else if *command == ::atoms::game_model_error().encode(env) {
                    Ok(Model::Error(arg1.decode()?, arg2.decode()?))
                } else {
                    Err(Error::BadArg)
                }
            }
            [command, arg1, arg2, arg3] => {
                if *command == ::atoms::game_model_players_with_response().encode(env) {
                    Ok(Model::PlayersWithResponse(
                        arg1.decode()?,
                        arg2.decode()?,
                        arg3.decode()?,
                    ))
                } else if *command == ::atoms::game_model_battle_with_response().encode(env) {
                    Ok(Model::BattleWithResponse(
                        arg1.decode()?,
                        arg2.decode()?,
                        arg3.decode()?,
                    ))
                } else {
                    Err(Error::BadArg)
                }
            }
            [command, arg1, arg2, arg3, arg4] => {
                if *command == ::atoms::game_model_war_with_response().encode(env) {
                    Ok(Model::WarWithResponse(
                        arg1.decode()?,
                        arg2.decode()?,
                        arg3.decode()?,
                        arg4.decode()?,
                    ))
                } else {
                    Err(Error::BadArg)
                }
            }
            _ => Err(Error::BadArg),
        }
    }
}

impl Encoder for Model {
    fn encode<'b>(&self, env: Env<'b>) -> Term<'b> {
        match self {
            Model::None(pids) => {
                let none = vec![::atoms::game_model_none().encode(env), pids.encode(env)];
                make_tuple(env, &none)
            }
            Model::Players(pids) => {
                let players = vec![::atoms::game_model_players().encode(env), pids.encode(env)];
                make_tuple(env, &players)
            }
            Model::PlayersWithResponse(pids, pid, response) => {
                let resp = vec![
                    ::atoms::game_model_players_with_response().encode(env),
                    pids.encode(env),
                    pid.encode(env),
                    response.encode(env),
                ];
                make_tuple(env, &resp)
            }
            Model::Battle(pids) => {
                let resp = vec![::atoms::game_model_battle().encode(env), pids.encode(env)];
                make_tuple(env, &resp)
            }
            Model::BattleWithResponse(pids, pid, response) => {
                let resp = vec![
                    ::atoms::game_model_battle_with_response().encode(env),
                    pids.encode(env),
                    pid.encode(env),
                    response.encode(env),
                ];
                make_tuple(env, &resp)
            }
            Model::BattleWonByPlayer(pids, player) => {
                let resp = vec![
                    ::atoms::game_model_battle_won_by_player().encode(env),
                    pids.encode(env),
                    player.encode(env),
                ];
                make_tuple(env, &resp)
            }
            Model::War(pids, pile) => {
                let resp = vec![
                    ::atoms::game_model_war().encode(env),
                    pids.encode(env),
                    pile.encode(env),
                ];
                make_tuple(env, &resp)
            }
            Model::WarWithResponse(pids, pile, pid, response) => {
                let resp = vec![
                    ::atoms::game_model_war_with_response().encode(env),
                    pids.encode(env),
                    pile.encode(env),
                    pid.encode(env),
                    response.encode(env),
                ];
                make_tuple(env, &resp)
            }
            Model::WarWonByPlayer(pids, player) => {
                let resp = vec![
                    ::atoms::game_model_war_won_by_player().encode(env),
                    pids.encode(env),
                    player.encode(env),
                ];
                make_tuple(env, &resp)
            }
            Model::Player1Won(pids) => {
                let resp = vec![
                    ::atoms::game_model_player1_won().encode(env),
                    pids.encode(env),
                ];
                make_tuple(env, &resp)
            }
            Model::Player2Won(pids) => {
                let resp = vec![
                    ::atoms::game_model_player2_won().encode(env),
                    pids.encode(env),
                ];
                make_tuple(env, &resp)
            }
            Model::Tie(pids) => {
                let resp = vec![::atoms::game_model_tie().encode(env), pids.encode(env)];
                make_tuple(env, &resp)
            }
            Model::Error(pids, err) => {
                let resp = vec![
                    ::atoms::game_model_error().encode(env),
                    pids.encode(env),
                    err.encode(env),
                ];
                make_tuple(env, &resp)
            }
        }
    }
}

use crate::player::Command as PlayerCmd;

#[derive(NifStruct)]
#[must_use] // Added to test Issue #152
#[module = "GameRust.SendCmd"]
#[derive(Debug, Clone, PartialEq)]
pub struct SendCmd {
    pub to: Vec<u8>,
    pub cmd: PlayerCmd,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Cmd {
    // command to send from elixir
    None,
    SendCmds(Vec<SendCmd>),
}

impl<'a> Decoder<'a> for Cmd {
    fn decode(term: Term<'a>) -> NifResult<Self> {
        let terms = get_tuple(term)?;
        let env = term.get_env();
        match terms.as_slice() {
            [none] => {
                let none: &str = none.decode()?;
                if none == "Cmd::None" {
                    Ok(Cmd::None)
                } else {
                    Err(Error::BadArg)
                }
            }
            [send_cmds, cmds] => {
                if *send_cmds == ::atoms::game_send_cmds().encode(env) {
                    Ok(Cmd::SendCmds(cmds.decode()?))
                } else {
                    Err(Error::BadArg)
                }
            }
            _ => Err(Error::BadArg),
        }
    }
}

impl Encoder for Cmd {
    fn encode<'b>(&self, env: Env<'b>) -> Term<'b> {
        match self {
            Cmd::None => {
                let none = vec!["Cmd::None".encode(env)];
                make_tuple(env, &none)
            }
            Cmd::SendCmds(cmds) => {
                let commands = vec![::atoms::game_send_cmds().encode(env), cmds.encode(env)];
                make_tuple(env, &commands)
            }
        }
    }
}

fn cards_to_send(
    player1_cards: &Vec<Card>,
    player2_cards: &Vec<Card>,
    // player1_cards: &[Card],
    // player2_cards: &[Card],
    pile: Option<&Vec<Card>>,
) -> Vec<Card> {
    let mut cards_to_send = Vec::new();
    match pile {
        None => {
            cards_to_send.append(&mut player1_cards.clone());
            cards_to_send.append(&mut player2_cards.clone());
        }
        Some(pile) => {
            cards_to_send.append(&mut pile.clone());
            cards_to_send.append(&mut player1_cards.clone());
            cards_to_send.append(&mut player2_cards.clone());
        }
    };
    cards_to_send
}

#[cfg(test)]
mod cards_to_send {
    use super::*;

    use crate::cards::Suit;
    use crate::cards::Value;

    #[test]
    fn without_pile() {
        let player1_cards = vec![Card(Suit::Club, Value::Two)];
        let player2_cards = vec![
            Card(Suit::Club, Value::Three),
            Card(Suit::Heart, Value::Ace),
        ];
        let cards_to_send = cards_to_send(&player1_cards, &player2_cards, None);
        assert_eq!(
            cards_to_send,
            vec![
                Card(Suit::Club, Value::Two),
                Card(Suit::Club, Value::Three),
                Card(Suit::Heart, Value::Ace),
            ]
        );
    }

    #[test]
    fn with_pile() {
        let player1_cards = vec![Card(Suit::Club, Value::Two)];
        let player2_cards = vec![
            Card(Suit::Club, Value::Three),
            Card(Suit::Heart, Value::Ace),
        ];
        let pile = vec![
            Card(Suit::Diamond, Value::King),
            Card(Suit::Heart, Value::Ten),
        ];
        let cards_to_send = cards_to_send(&player1_cards, &player2_cards, Some(&pile));
        assert_eq!(
            cards_to_send,
            vec![
                Card(Suit::Diamond, Value::King),
                Card(Suit::Heart, Value::Ten),
                Card(Suit::Club, Value::Two),
                Card(Suit::Club, Value::Three),
                Card(Suit::Heart, Value::Ace),
            ]
        );
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum FightResult {
    PlayerWon(Vec<u8>, Vec<Card>), // player_id, cards_to_send
    Tie(Vec<Card>),
}

fn fight_result(
    player1: &Vec<u8>,
    player1_cards: &Vec<Card>,
    player2: &Vec<u8>,
    player2_cards: &Vec<Card>,
    pile: Option<&Vec<Card>>,
) -> Result<FightResult, String> {
    use crate::cards::cards_are_equal;
    use crate::cards::first_is_less;

    let correct_nr_of_cards = match (pile, player1_cards.len(), player2_cards.len()) {
        (None, 1, 1) => true,
        (Some(_), 2, 2) => true,
        (_, _, _) => false,
    };

    if !correct_nr_of_cards {
        Err("players must have right number of cards".to_string())
    } else {
        let cards_to_send = cards_to_send(player1_cards, player2_cards, pile);
        let mut player1_cards = player1_cards.clone();
        let mut player2_cards = player2_cards.clone();
        match (player1_cards.pop(), player2_cards.pop()) {
            (Some(card1), Some(card2)) => {
                if cards_are_equal(&card1, &card2) {
                    Ok(FightResult::Tie(cards_to_send))
                } else if first_is_less(&card1, &card2) {
                    Ok(FightResult::PlayerWon(player2.to_vec(), cards_to_send))
                } else {
                    Ok(FightResult::PlayerWon(player1.to_vec(), cards_to_send))
                }
            }
            (_, _) => Err("unable to compare cards".to_string()),
        }
    }
}

#[cfg(test)]
mod fight_result {
    use super::*;

    use crate::cards::Suit;
    use crate::cards::Value;

    //use crate::player::Msg::RemoveCards;
    #[test]
    fn err_wrong_nr_of_cars_without_pile() {
        let player1 = vec![1];
        let player2 = vec![2];
        let player1_c = vec![Card(Suit::Club, Value::Two), Card(Suit::Heart, Value::Ace)];
        let player2_c = vec![
            Card(Suit::Club, Value::Three),
            Card(Suit::Heart, Value::Ace),
        ];
        let fight_result = fight_result(&player1, &player1_c, &player2, &player2_c, None);
        assert_eq!(
            fight_result,
            Err("players must have right number of cards".to_string())
        );
    }

    #[test]
    fn player1_won_with_pile() {
        let player1 = vec![1];
        let player2 = vec![2];
        let player1_c = vec![Card(Suit::Club, Value::Two), Card(Suit::Heart, Value::Ace)];
        let player2_c = vec![
            Card(Suit::Club, Value::Three),
            Card(Suit::Heart, Value::Ten),
        ];
        let pile = vec![Card(Suit::Club, Value::Three)];
        let fight_result = fight_result(&player1, &player1_c, &player2, &player2_c, Some(&pile));
        assert_eq!(
            fight_result,
            Ok(FightResult::PlayerWon(
                vec![1],
                [
                    Card(Suit::Club, Value::Three),
                    Card(Suit::Club, Value::Two),
                    Card(Suit::Heart, Value::Ace),
                    Card(Suit::Club, Value::Three),
                    Card(Suit::Heart, Value::Ten),
                ]
                .to_vec()
            ))
        );
    }

    #[test]
    fn tie() {
        let player1 = vec![1];
        let player2 = vec![2];
        let player1_c = vec![Card(Suit::Club, Value::Two)];
        let player2_c = vec![Card(Suit::Heart, Value::Two)];
        let fight_result = fight_result(&player1, &player1_c, &player2, &player2_c, None);
        assert_eq!(
            fight_result,
            Ok(FightResult::Tie(
                [Card(Suit::Club, Value::Two), Card(Suit::Heart, Value::Two)].to_vec()
            ))
        );
    }
}

fn match_players_with_responses(
    pids: Pids,
    pid1: Vec<u8>,
    response1: PlayerResp,
    pid2: Vec<u8>,
    response2: PlayerResp,
) -> Result<((Vec<u8>, PlayerResp), (Vec<u8>, PlayerResp)), String> {
    let Pids {
        supervisor: _,
        player1,
        player2,
    } = pids;
    // match response with player
    if (&player1, &player2) == (&pid1, &pid2) {
        Ok(((player1, response1), (player2, response2)))
    } else if (&player1, &player2) == (&pid2, &pid1) {
        Ok(((player1, response2), (player2, response1)))
    } else {
        let error = format!(
            "unable to match players: {:?}, {:?} with pids: {:?}, {:?}",
            player1, player2, pid1, pid2
        );
        Err(error)
    }
}

#[cfg(test)]
mod match_players_with_responses {
    use super::*;

    use crate::player::Response::CardsAdded;
    use crate::player::Response::UnableToRemoveCards;

    #[test]
    fn matched_responses() {
        let pids = Pids {
            supervisor: vec![0],
            player1: vec![1],
            player2: vec![2],
        };
        let resp1 = UnableToRemoveCards(2);
        let resp2 = CardsAdded(3);
        let match_players = match_players_with_responses(pids, vec![1], resp1, vec![2], resp2);
        assert_eq!(
            match_players,
            Ok(((vec![1], UnableToRemoveCards(2)), (vec![2], CardsAdded(3))))
        );
    }

    #[test]
    fn wrong_pids() {
        let pids = Pids {
            supervisor: vec![0],
            player1: vec![1],
            player2: vec![2],
        };
        let resp1 = UnableToRemoveCards(2);
        let resp2 = CardsAdded(3);
        let match_players = match_players_with_responses(pids, vec![1], resp1, vec![1], resp2);
        assert_eq!(
            match_players,
            Err("unable to match players: [1], [2] with pids: [1], [1]".to_string())
        );
    }
}

fn judge_players(
    pids: Pids,
    pid1: Vec<u8>,
    response1: PlayerResp,
    pid2: Vec<u8>,
    response2: PlayerResp,
    pile: Option<&Vec<Card>>,
) -> (Model, Cmd) {
    use crate::player::Command::*;
    use crate::player::Response::CardsRemoved;
    use crate::player::Response::UnableToRemoveCards;

    let model = match pile {
        None => "Model::BattleWithRespose",
        Some(_) => "Model::WarWithResponse",
    };

    let players_with_responses =
        match_players_with_responses(pids.clone(), pid1, response1, pid2, response2);
    match players_with_responses {
        Err(details) => {
            let error = model.to_owned() + ": " + &details;
            (Model::Error(pids, error), Cmd::None)
        }
        Ok(((player1, player1_response), (player2, player2_response))) => {
            match (player1_response, player2_response) {
                (CardsRemoved(player1_cards), CardsRemoved(player2_cards)) => {
                    let fight_result =
                        fight_result(&player1, &player1_cards, &player2, &player2_cards, pile);
                    match fight_result {
                        Ok(FightResult::PlayerWon(player, cards)) => {
                            let cmd = vec![SendCmd {
                                to: player.clone(),
                                cmd: AddCards(cards),
                            }];
                            match pile {
                                None => {
                                    (Model::BattleWonByPlayer(pids, player), Cmd::SendCmds(cmd))
                                }
                                Some(_) => {
                                    (Model::WarWonByPlayer(pids, player), Cmd::SendCmds(cmd))
                                }
                            }
                        }
                        Ok(FightResult::Tie(pile)) => {
                            let remove_cards = vec![
                                SendCmd {
                                    to: player1,
                                    cmd: RemoveCards(2),
                                },
                                SendCmd {
                                    to: player2,
                                    cmd: RemoveCards(2),
                                },
                            ];
                            (Model::War(pids, pile), Cmd::SendCmds(remove_cards))
                        }
                        Err(error) => {
                            let error = model.to_owned() + ": " + &error;
                            (Model::Error(pids, error), Cmd::None)
                        }
                    }
                }

                (CardsRemoved(_), UnableToRemoveCards(_)) => (Model::Player1Won(pids), Cmd::None),
                (UnableToRemoveCards(_), CardsRemoved(_)) => (Model::Player2Won(pids), Cmd::None),
                (UnableToRemoveCards(_), UnableToRemoveCards(_)) => (Model::Tie(pids), Cmd::None),
                (resp1, resp2) => {
                    let error = format!(
                        "{} received wrong responses: {:?}, {:?}",
                        model, resp1, resp2
                    );
                    (Model::Error(pids, error), Cmd::None)
                }
            }
        }
    }
}

#[cfg(test)]
mod judge_players {
    use super::*;

    use crate::player::Response::CardsAdded;
    use crate::player::Response::CardsRemoved;
    use crate::player::Response::UnableToRemoveCards;

    #[test]
    fn wrong_pids_no_pile() {
        let pids = Pids {
            supervisor: vec![0],
            player1: vec![1],
            player2: vec![2],
        };
        let resp1 = UnableToRemoveCards(2);
        let resp2 = CardsAdded(3);
        let judge_players = judge_players(pids.clone(), vec![1], resp1, vec![1], resp2, None);
        assert_eq!(
            judge_players,
            (
                Model::Error(pids,
                    "Model::BattleWithRespose: unable to match players: [1], [2] with pids: [1], [1]"
                        .to_string()
                ),
                Cmd::None
            )
        );
    }

    #[test]
    fn wrong_pids_with_pile() {
        let pids = Pids {
            supervisor: vec![0],
            player1: vec![1],
            player2: vec![2],
        };
        let resp1 = UnableToRemoveCards(2);
        let resp2 = CardsAdded(3);
        let pile = vec![];
        let judge_players =
            judge_players(pids.clone(), vec![1], resp1, vec![1], resp2, Some(&pile));
        assert_eq!(
            judge_players,
            (
                Model::Error(
                    pids,
                    "Model::WarWithResponse: unable to match players: [1], [2] with pids: [1], [1]"
                        .to_string()
                ),
                Cmd::None
            )
        );
    }

    #[test]
    fn wrong_response() {
        let pids = Pids {
            supervisor: vec![0],
            player1: vec![1],
            player2: vec![2],
        };
        let resp1 = UnableToRemoveCards(2);
        let resp2 = CardsAdded(3);
        let judge_players = judge_players(pids.clone(), vec![1], resp1, vec![2], resp2, None);
        assert_eq!(
            judge_players,
            (Model::Error(pids, "Model::BattleWithRespose received wrong responses: UnableToRemoveCards(2), CardsAdded(3)".to_string()), Cmd::None)
        );
    }

    #[test]
    fn wrong_nr_of_cards_removed() {
        use crate::cards::Suit;
        use crate::cards::Value;

        let resp1 = CardsRemoved(vec![
            Card(Suit::Club, Value::Two),
            Card(Suit::Club, Value::Three),
            Card(Suit::Diamond, Value::Ace),
        ]);
        let resp2 = CardsRemoved(vec![
            Card(Suit::Heart, Value::Two),
            Card(Suit::Heart, Value::Three),
            Card(Suit::Spade, Value::Ace),
        ]);
        let (model, cmd) = judge_players(
            Pids {
                supervisor: vec![0],
                player1: vec![1],
                player2: vec![2],
            },
            vec![1],
            resp1,
            vec![2],
            resp2,
            None,
        );
        assert_eq!(
            (model, cmd),
            (
                Model::Error(
                    Pids {
                        supervisor: vec![0],
                        player1: vec![1],
                        player2: vec![2],
                    },
                    "Model::BattleWithRespose: players must have right number of cards".to_string()
                ),
                Cmd::None
            )
        );
    }

    #[test]
    fn battle_won_by_player_without_pile() {
        use crate::cards::Suit;
        use crate::cards::Suit::Club;
        use crate::cards::Suit::Heart;
        use crate::cards::Value;
        use crate::cards::Value::Three;
        use crate::cards::Value::Two;
        use crate::game::Cmd::SendCmds;
        use crate::player::Command::AddCards;

        let resp1 = CardsRemoved(vec![Card(Suit::Club, Value::Two)]);
        let resp2 = CardsRemoved(vec![Card(Suit::Heart, Value::Three)]);
        let (model, cmd) = judge_players(
            Pids {
                supervisor: vec![0],
                player1: vec![1],
                player2: vec![2],
            },
            vec![1],
            resp1,
            vec![2],
            resp2,
            None,
        );
        assert_eq!(
            (model, cmd),
            (
                Model::BattleWonByPlayer(
                    Pids {
                        supervisor: vec![0],
                        player1: vec![1],
                        player2: vec![2]
                    },
                    vec![2]
                ),
                SendCmds(
                    [SendCmd {
                        to: vec![2],
                        cmd: AddCards([Card(Club, Two), Card(Heart, Three)].to_vec())
                    }]
                    .to_vec()
                )
            )
        );
    }

    #[test]
    fn tie_without_pile() {
        use crate::cards::Suit;
        use crate::cards::Suit::Club;
        use crate::cards::Suit::Heart;
        use crate::cards::Value;
        use crate::cards::Value::Two;
        use crate::game::Cmd::SendCmds;
        use crate::player::Command::RemoveCards;

        let resp1 = CardsRemoved(vec![Card(Suit::Club, Value::Two)]);
        let resp2 = CardsRemoved(vec![Card(Suit::Heart, Value::Two)]);

        let (model, cmd) = judge_players(
            Pids {
                supervisor: vec![0],
                player1: vec![1],
                player2: vec![2],
            },
            vec![1],
            resp1,
            vec![2],
            resp2,
            None,
        );
        assert_eq!(
            (model, cmd),
            (
                Model::War(
                    Pids {
                        supervisor: vec![0],
                        player1: vec![1],
                        player2: vec![2]
                    },
                    vec![Card(Club, Two), Card(Heart, Two)]
                ),
                SendCmds(vec![
                    SendCmd {
                        to: vec![1],
                        cmd: RemoveCards(2)
                    },
                    SendCmd {
                        to: vec![2],
                        cmd: RemoveCards(2)
                    }
                ]),
            )
        );
    }

    #[test]
    fn tie_with_pile() {
        use crate::cards::Suit;
        use crate::cards::Suit::*;
        use crate::cards::Value;
        use crate::cards::Value::*;
        use crate::game::Cmd::SendCmds;
        use crate::player::Command::RemoveCards;

        let resp1 = CardsRemoved(vec![
            Card(Suit::Club, Value::Two),
            Card(Suit::Heart, Value::Ace),
        ]);
        let resp2 = CardsRemoved(vec![
            Card(Suit::Heart, Value::Ten),
            Card(Suit::Spade, Value::Ace),
        ]);
        let pile = vec![Card(Suit::Club, Value::Ten), Card(Suit::Heart, Value::Ace)];

        let (model, cmd) = judge_players(
            Pids {
                supervisor: vec![0],
                player1: vec![1],
                player2: vec![2],
            },
            vec![1],
            resp1,
            vec![2],
            resp2,
            Some(&pile),
        );

        assert_eq!(
            (model, cmd),
            (
                Model::War(
                    Pids {
                        supervisor: vec![0],
                        player1: vec![1],
                        player2: vec![2]
                    },
                    vec![
                        Card(Club, Ten),
                        Card(Heart, Ace),
                        Card(Club, Two),
                        Card(Heart, Ace),
                        Card(Heart, Ten),
                        Card(Spade, Ace)
                    ]
                ),
                SendCmds(vec![
                    SendCmd {
                        to: vec![1],
                        cmd: RemoveCards(2)
                    },
                    SendCmd {
                        to: vec![2],
                        cmd: RemoveCards(2)
                    }
                ])
            ),
        );
    }

    #[test]
    fn player_won() {
        use crate::cards::Suit;
        use crate::cards::Value;

        let resp1 = UnableToRemoveCards(2);
        let resp2 = CardsRemoved(vec![
            Card(Suit::Heart, Value::Ten),
            Card(Suit::Spade, Value::Ace),
        ]);
        let pile = vec![Card(Suit::Club, Value::Ten), Card(Suit::Heart, Value::Ace)];

        let (model, cmd) = judge_players(
            Pids {
                supervisor: vec![0],
                player1: vec![1],
                player2: vec![2],
            },
            vec![1],
            resp1,
            vec![2],
            resp2,
            Some(&pile),
        );

        assert_eq!(
            (model, cmd),
            (
                Model::Player2Won(Pids {
                    supervisor: vec![0],
                    player1: vec![1],
                    player2: vec![2],
                }),
                Cmd::None
            ),
        );
    }

}

pub fn update(model: Model, msg: Msg) -> (Model, Cmd) {
    use crate::cards::shuffled_deck;

    use crate::player::Command::*;
    use crate::player::Response::CardsAdded;

    match (model, msg) {
        (Model::None(pids), Msg::StartGame) => {
            let Pids {
                supervisor: _,
                player1,
                player2,
            } = pids.clone();
            let deck = shuffled_deck();
            let (cards1, cards2) = deck.split_at(26);
            let send_decks = vec![
                SendCmd {
                    to: player1,
                    cmd: AddCards(cards1.to_vec()),
                },
                SendCmd {
                    to: player2,
                    cmd: AddCards(cards2.to_vec()),
                },
            ];
            (Model::Players(pids), Cmd::SendCmds(send_decks))
        }
        (Model::None(pids), msg) => {
            let error = format!("Model::None got msg: {:?}", msg);
            (Model::Error(pids, error), Cmd::None)
        }
        (Model::Players(pids), Msg::ResponseFromPlayer(pid, CardsAdded(26))) => (
            Model::PlayersWithResponse(pids, pid, CardsAdded(26)),
            Cmd::None,
        ),
        (Model::Players(pids), msg) => {
            let error = format!("Model::Players got msg: {:?}", msg);
            (Model::Error(pids, error), Cmd::None)
        }
        (
            Model::PlayersWithResponse(pids, pid1, CardsAdded(26)),
            Msg::ResponseFromPlayer(pid2, CardsAdded(26)),
        ) => {
            let Pids {
                supervisor: _,
                player1,
                player2,
            } = pids.clone();
            // received correct pids
            let game_initialized =
                (&player1, &player2) == (&pid1, &pid2) || (&player1, &player2) == (&pid2, &pid1);
            match game_initialized {
                false => (
                    Model::Error(
                        pids,
                        "Model::PlayersWithResponse: received wrong pids".to_string(),
                    ),
                    Cmd::None,
                ),
                true => {
                    let remove_cards = vec![
                        SendCmd {
                            to: player1,
                            cmd: RemoveCards(1),
                        },
                        SendCmd {
                            to: player2,
                            cmd: RemoveCards(1),
                        },
                    ];
                    (Model::Battle(pids), Cmd::SendCmds(remove_cards))
                }
            }
        }
        (Model::PlayersWithResponse(pids, _, _), msg) => {
            let error = format!("Model::PlayersWithResponse got msg: {:?}", msg);
            (Model::Error(pids, error), Cmd::None)
        }
        // Game is initialized. Start the battle.
        (Model::Battle(pids), Msg::ResponseFromPlayer(player, response)) => {
            (Model::BattleWithResponse(pids, player, response), Cmd::None)
        }
        (Model::Battle(pids), msg) => {
            let error = format!("Model::Battle got msg: {:?}", msg);
            (Model::Error(pids, error), Cmd::None)
        }
        (
            Model::BattleWithResponse(pids, pid1, response1),
            Msg::ResponseFromPlayer(pid2, response2),
        ) => judge_players(pids, pid1, response1, pid2, response2, None),
        (Model::BattleWithResponse(pids, _, _), msg) => {
            let error = format!("Model::BattleWithResponse received wrong msg: {:?}", msg);
            (Model::Error(pids, error), Cmd::None)
        }
        (Model::BattleWonByPlayer(pids, player), Msg::ResponseFromPlayer(pid, CardsAdded(2))) => {
            if player == pid {
                let Pids {
                    supervisor: _,
                    player1,
                    player2,
                } = pids.clone();
                let remove_cards = vec![
                    SendCmd {
                        to: player1,
                        cmd: RemoveCards(1),
                    },
                    SendCmd {
                        to: player2,
                        cmd: RemoveCards(1),
                    },
                ];
                (Model::Battle(pids), Cmd::SendCmds(remove_cards))
            } else {
                (
                    Model::Error(
                        pids,
                        "Model::BattleWonByPlayer received wrong pid".to_string(),
                    ),
                    Cmd::None,
                )
            }
        }
        (Model::BattleWonByPlayer(pids, _), msg) => {
            let error = format!("Model::BattleWonByPlayer received wrong msg: {:?}", msg);
            (Model::Error(pids, error), Cmd::None)
        }
        (Model::War(pids, pile), Msg::ResponseFromPlayer(player, response)) => (
            Model::WarWithResponse(pids, pile, player, response),
            Cmd::None,
        ),
        (Model::War(pids, _), msg) => {
            let error = format!("Model::War received wrong msg: {:?}", msg);
            (Model::Error(pids, error), Cmd::None)
        }
        (
            Model::WarWithResponse(pids, pile, pid1, response1),
            Msg::ResponseFromPlayer(pid2, response2),
        ) => judge_players(pids, pid1, response1, pid2, response2, Some(&pile)),
        (Model::WarWithResponse(pids, _, _, _), msg) => {
            let error = format!("Model::WarWithResponse received wrong msg: {:?}", msg);
            (Model::Error(pids, error), Cmd::None)
        }
        (Model::WarWonByPlayer(pids, player), Msg::ResponseFromPlayer(pid, CardsAdded(_))) => {
            if player == pid {
                let Pids {
                    supervisor: _,
                    player1,
                    player2,
                } = pids.clone();
                let remove_cards = vec![
                    SendCmd {
                        to: player1,
                        cmd: RemoveCards(1),
                    },
                    SendCmd {
                        to: player2,
                        cmd: RemoveCards(1),
                    },
                ];
                (Model::Battle(pids), Cmd::SendCmds(remove_cards))
            } else {
                (
                    Model::Error(pids, "Model::WarWonByPlayer received wrong pid".to_string()),
                    Cmd::None,
                )
            }
        }
        (Model::WarWonByPlayer(pids, _), msg) => {
            let error = format!("Model::WarWonByPlayer received wrong msg: {:?}", msg);
            (Model::Error(pids, error), Cmd::None)
        }
        (Model::Player1Won(pids), msg) => {
            let error = format!("Model::Player1Won received wrong msg: {:?}", msg);
            (Model::Error(pids, error), Cmd::None)
        }
        (Model::Player2Won(pids), msg) => {
            let error = format!("Model::Player2Won received wrong msg: {:?}", msg);
            (Model::Error(pids, error), Cmd::None)
        }
        (Model::Tie(pids), msg) => {
            let error = format!("Model::Tie received wrong msg: {:?}", msg);
            (Model::Error(pids, error), Cmd::None)
        }
        (Model::Error(pids, error), _) => (Model::Error(pids, error), Cmd::None),
    }
}

#[cfg(test)]
mod game_update {
    use super::*;
    use crate::player::Command::AddCards;
    use crate::player::Response::CardsAdded;

    #[test]
    fn update_none_with_wrong_msg() {
        let pids = Pids {
            supervisor: vec![0],
            player1: vec![1],
            player2: vec![2],
        };
        let model = Model::None(pids);
        let msg = Msg::ResponseFromPlayer(vec![1], CardsAdded(23));
        let (updated_model, _cmd) = update(model, msg);
        assert_eq!(
            updated_model,
            Model::Error(
                Pids {
                    supervisor: vec![0],
                    player1: vec![1],
                    player2: vec![2],
                },
                "Model::None got msg: ResponseFromPlayer([1], CardsAdded(23))".to_string()
            )
        );
    }

    #[test]
    fn update_players_with_cards_added_26() {
        let pids = Pids {
            supervisor: vec![0],
            player1: vec![1],
            player2: vec![2],
        };
        let model = Model::Players(pids);
        let msg = Msg::ResponseFromPlayer(vec![1], CardsAdded(26));
        let (updated_model, _cmd) = update(model, msg);
        assert_eq!(
            updated_model,
            Model::PlayersWithResponse(
                Pids {
                    supervisor: vec![0],
                    player1: vec![1],
                    player2: vec![2]
                },
                vec![1],
                CardsAdded(26)
            ),
        );
    }

    #[test]
    fn update_none_with_msg_pids() {
        let pids = Pids {
            supervisor: vec![0],
            player1: vec![1],
            player2: vec![2],
        };
        let model = Model::None(pids);
        let msg = Msg::StartGame;
        let (updated_model, cmd) = update(model, msg);
        let ((to1, cards1), (to2, cards2)) = match cmd {
            Cmd::SendCmds(commands) => {
                let mut cmds = commands;
                let cmd2 = cmds.pop();
                let cmd1 = cmds.pop();
                match (cmd1, cmd2) {
                    (
                        Some(SendCmd {
                            to: one,
                            cmd: AddCards(c1),
                        }),
                        Some(SendCmd {
                            to: two,
                            cmd: AddCards(c2),
                        }),
                    ) => ((one, c1), (two, c2)),
                    (_, _) => ((vec![0], vec![]), (vec![0], vec![])),
                }
            }
            _ => ((vec![0], vec![]), (vec![0], vec![])),
        };
        println!();
        println!("Player{:?} cards:", to1);
        for card in cards1.iter() {
            println!("{:?}", card);
        }
        println!();
        println!("Player{:?} cards:", to2);
        for card in cards2.iter() {
            println!("{:?}", card);
        }
        println!("");
        let pids = Pids {
            supervisor: vec![0],
            player1: vec![1],
            player2: vec![2],
        };
        assert_eq!(updated_model, Model::Players(pids));
    }

    #[test]
    fn update_players_with_response_with_cards_added_26() {
        let pids = Pids {
            supervisor: vec![0],
            player1: vec![1],
            player2: vec![2],
        };
        let model = Model::PlayersWithResponse(pids, vec![1], CardsAdded(26));
        let msg = Msg::ResponseFromPlayer(vec![2], CardsAdded(26));
        let (updated_model, _cmd) = update(model, msg);
        assert_eq!(
            updated_model,
            Model::Battle(Pids {
                supervisor: vec![0],
                player1: vec![1],
                player2: vec![2]
            }),
        );
    }

    #[test]
    fn update_players_with_response_with_wrong_pid() {
        let pids = Pids {
            supervisor: vec![0],
            player1: vec![1],
            player2: vec![2],
        };
        let model = Model::PlayersWithResponse(pids.clone(), vec![1], CardsAdded(26));
        let msg = Msg::ResponseFromPlayer(vec![1], CardsAdded(26));
        let (updated_model, _cmd) = update(model, msg);
        assert_eq!(
            updated_model,
            Model::Error(
                pids,
                "Model::PlayersWithResponse: received wrong pids".to_string()
            ),
        );
    }
}
