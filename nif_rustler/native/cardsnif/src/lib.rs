#[macro_use]
extern crate rustler;
extern crate rustler_codegen;
use rustler::types::tuple::make_tuple;
use rustler::{Encoder, Env, NifResult, Term};
// use rustler::{Encoder, Env, Error, NifResult, Term};
// use rustler::types::atom::Atom::from_term;

mod cards;
mod game;
mod player;

mod atoms {
    rustler_atoms! {
        atom card = "Elixir.Card";

        atom player_command_add_cards = "Elixir.PlayerRust.CommandAddCards";
        atom player_command_remove_cards = "Elixir.PlayerRust.CommandRemoveCards";

        atom player_response_cards_added = "Elixir.PlayerRust.ResponseCardsAdded";
        atom player_response_cards_removed = "Elixir.PlayerRust.ResponseCardsRemoved";
        atom player_response_error = "Elixir.PlayerRust.ResponseError";
        atom player_response_unable_to_remove_cards = "Elixir.PlayerRust.ResponseUnableToRemoveCards";

        atom game_msg_response_from_player = "Elixir.GameRust.MsgResponseFromPlayer";

        atom game_model_none = "Elixir.GameRust.ModelNone";
        atom game_model_players = "Elixir.GameRust.ModelPlayers";
        atom game_model_players_with_response = "Elixir.GameRust.ModelPlayersWithResponse";
        atom game_model_battle = "Elixir.GameRust.ModelBattle";
        atom game_model_battle_with_response = "Elixir.GameRust.ModelBattleWithResponse";
        atom game_model_battle_won_by_player = "Elixir.GameRust.ModelBattleWonByPlayer";
        atom game_model_war = "Elixir.GameRust.ModelWar";
        atom game_model_war_with_response = "Elixir.GameRust.ModelWarWithResponse";
        atom game_model_war_won_by_player = "Elixir.GameRust.ModelWarWonByPlayer";
        atom game_model_player1_won = "Elixir.GameRust.ModelPlayer1Won";
        atom game_model_player2_won = "Elixir.GameRust.ModelPlayer2Won";
        atom game_model_tie = "Elixir.GameRust.ModelTie";
        atom game_model_error = "Elixir.GameRust.ModelError";

        atom game_send_cmds = "Elixir.GameRust.SendCmds";

    }
}

rustler_export_nifs! {
    "Elixir.Cardsnif",
    [
        ("player_update", 2, player_update),
        ("game_update", 2, game_update),
    ],
    None
}

fn player_update<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let model: player::Model = args[0].decode()?;
    let msg: player::Msg = args[1].decode()?;
    let (new_model, cmd) = player::update(model, msg);
    let result = make_tuple(env, &vec![new_model.encode(env), cmd.encode(env)]);
    Ok(result)
}

fn game_update<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let model: game::Model = args[0].decode()?;
    let msg: game::Msg = args[1].decode()?;
    let (new_model, cmd) = game::update(model, msg);
    let result = make_tuple(env, &vec![new_model.encode(env), cmd.encode(env)]);
    Ok(result)
}

