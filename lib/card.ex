defmodule Card do
  @moduledoc """
  Documentation for Card
  A card is a Erlang tuple (Elixir record) of form:
  {Card, "Suit::Club", "Value::Ace"}
  """

  require Record
  @type t :: {__MODULE__, bitstring, bitstring}
  Record.defrecord(:record, __MODULE__, suit: nil, value: nil)
end
