#[macro_use] extern crate rustler;
use rustler::{Env, NifResult, Term};
#[macro_use] extern crate serde;
#[macro_use] extern crate serde_rustler;
use serde_rustler::{from_term, to_term};

mod cards;
mod player;
mod game;

rustler_export_nifs! {
    "Elixir.Cardsnif",
    [
        ("player_update", 2, player_update),
        ("game_update", 2, game_update),
    ],
    None
}

fn player_update<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let model: player::Model = from_term(args[0])?;
    let msg: player::Msg = from_term(args[1])?;
    to_term(env, player::update(model, msg)).map_err(|err| err.into())
}

fn game_update<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let model: game::Model = from_term(args[0])?;
    let msg: game::Msg = from_term(args[1])?;
    to_term(env, game::update(model, msg)).map_err(|err| err.into())
}
