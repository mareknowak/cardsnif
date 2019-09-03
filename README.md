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
take a Player process as an example
([player_rust.ex](nif_rustler/lib/player_rust.ex)). It receives from Game
*messages* to add or remove cards. Player's *model* is simply a list of cards.
It's *update* function changes the model (adds or removes cards) and prepares
commands/responses (cards added, cards removed, unable to remove cards) that are
sent back to Game by Players's *execute* function.

## Installation

The repository contains three directiories with three different versions: elixir
(no NIFs, Elixir code only), nif_rustler (only rustler library, no serde) and
nif_serde_rustler (with serde_rustler library).

First clone or copy repository and cd into directory:

```bash
$ cd elixir
$ mix do deps.get, compile
```
or 

```bash
$ cd nif_rustler # or nif_serde_rustler
```

At the time of writing **rustler** compilation is not supported for Erlang 22
and we have to temporarily switch to Erlang 21.3 using for example
[kerl](https://github.com/kerl/kerl). After building Erlang 21.3 and installing
it in `~/kerl/21.3` we can compile our project and then switch back to Erlang
22:

```bash
$ . ~/kerl/21.3/activate
$ mix do deps.get, compile
$ kerl_deactivate
```

Game can be run as follows:

```bash
$ iex -S mix
$ Start.play() 
```

Execution time of subsequent Game updates is recorded in *replay.txt* in the
current directory.

## Conclusions

### rustler and serde_rustler libraries

[Rustler](https://github.com/rusterlium/rustler) library can be used to decode
Erlang terms to Rust types and encode them back to Erlang. One can find examples
of custom decoders and encoders in *nif_rustler* directory. 

Implementing decoders and encoder can be a bit tedious in *rustler*. On the
other side data conversion between Rust and Elixir is done flawlessly by
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

### Time of execution

*Serde_rustler* is a nice library but *rustler* is faster. On my machine
execution of the *rustler* version of a game *update* function takes on average
about 60 microseconds while *serde_rustler* takes about 160 microseconds.

How does it compare to Elixir version of the game? Simple *update* runs in about
5 microsecond so Rust NIF versions are slower. Only shuffling the deck of 52
cards is faster in Rust. *Caveat! First run of a Rust NIF function can take a
lot of time (like 20000 microseconds or more). Subsequent runs are much faster.*

If speed is the only concern then one should carefuly measure the speed of
execution of Elixir function before replacing it with NIF.
