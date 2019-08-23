# CardsNif

This repository contains code for the card game of
[War](https://en.wikipedia.org/wiki/War_(card_game)) written in Elixir. Some
functions are implemented as Rust NIFs (Native Implemented Functions) with help
of [rustler](https://github.com/rusterlium/rustler) library. Data conversion
between Elixir and Rust is done by
[serde_rustler](https://github.com/sunny-g/serde_rustler) Rust crate.

## Motivation

I miss static typing in Elixir and in this project I want to explore the idea of
moving as much application logic as possible to Rust NIFs. It can be done as
follows. Elixir process `receive` function can be split into pure `update`
function and "impure" `execute` function. The first one changes the state of a
process (`model`) and prepares command (`cmd`) to be executed/sent by Elixir
runtime. The second executes/sends prepared command:

```elixir
def process(model) do
  receive do
    msg ->
      {model, cmd} = update(model, msg)
      execute(cmd)
      process(model)
  end
end
```

This separation allows us to implement `update` in statically typed Rust and
treat Elixir as kind of postman that sends prepared commands.

In our card game we have processes for Game (arbiter) and two Players. Let us
take a Player process as an example ([player_rust.ex](lib/player_rust.ex)). It
receives from Game *messages* to add or remove cards. Player's *model* is simply
a list of cards. It's *update* function changes the model (adds or removes
cards) and prepares commands/responses (cards added, cards removed, unable to
remove cards) that are sent back to Game by Players's *execute* function.

## Installation

First clone or copy repository and cd into it:

```bash
$ cd cardsnif
```

At the time of writing **rustler** compilation is not supported for Erlang 22
and we have to temporary switch to Erlang 21.3 using for example
[kerl](https://github.com/kerl/kerl). After building Erlang 21.3 and installing
it in `~/kerl/21.3` we can compile our project and then switch back to Erlang
22:

```bash
$ . ~/kerl/21.3/activate
$ mix do deps.get, compile
$ kerl_deactivate
```

Game can be run in two ways, with Rust NIFs or with Elixir only functions,
as follows:

```bash
$ iex -S mix
$ Start.play_rust() # or Start.play_elixir()
```

Execution time of subsequent Game updates is recorded in *replay_rust.txt* or
*replay_elixir.txt* in the current directory.

## Conclusions

### Rust and Elixir interplay easily

Data conversion between Rust and Elixir is done flawlessly by
[serde_rustler](https://github.com/sunny-g/serde_rustler) crate. One has to only
put some annotations in types definitions. For example:

```rust
// Rust side:

// Rust enum is mapped to Elixir records
pub enum Command {
    #[serde(rename = "Elixir.PlayerRust.CommandAddCards")]
    AddCards(Vec<Card>), // add list of cards to the model
    #[serde(rename = "Elixir.PlayerRust.CommandRemoveCards")]
    RemoveCards(usize), // remove nr of cards
}

// Rust struct is mapped to Elixir struct
#[serde(rename = "Elixir.PlayerRust.Msg")]
pub struct Msg {
    pub from: Vec<u8>,
    pub command: Command,
}
```

```elixir
# Elixir side:

defmodule PlayerRust do

  ## Rust enum
  defmodule CommandAddCards do
    require Record
    @type t :: {__MODULE__, [Card.t()]}
    Record.defrecord(:record, __MODULE__, cards: [])
  end

  ## Rust enum
  defmodule CommandRemoveCards do
    require Record
    @type t :: {__MODULE__, non_neg_integer}
    Record.defrecord(:record, __MODULE__, nr: 0)
  end
  
  ## Rust struct counterpart
  defmodule Msg do
    @type t :: %__MODULE__{
            from: [byte()],
            command: CommandAddCards.t() | CommandRemoveCards.t()
          }
    defstruct from: [0], command: nil
  end
end
```

### Rust NIF version runs slower

Serialization and deserialization is relatively easy but is not free. The
simplest Elixir's update function takes 1-4 microseconds but its Rust's
counterpart needs from 100 to 400 microseconds so Elixir's version of the whole
game runs faster than Rust's one! Only shuffling the deck of 52 cards is faster
in Rust. *Caveat! First run of a Rust NIF function takes a lot of time (like
20000 microseconds or more). Subsequent runs are much faster.*

If speed is the only concern then one should consider substituting Elixir
function with Rust NIF only if its execution time is greater then 500
microseconds.
