defmodule PlayerElixir do
  @moduledoc """
  Elixir version of Player process
  model: list of cards
  """
  use Exceptional
  require GameElixir.MsgResponseFromPlayer

  defmodule CommandAddCards do
    require Record
    @type t :: {__MODULE__, [Card.t()]}
    Record.defrecord(:record, __MODULE__, cards: [])
  end

  defmodule CommandRemoveCards do
    require Record
    @type t :: {__MODULE__, non_neg_integer}
    Record.defrecord(:record, __MODULE__, nr: 0)
  end

  defmodule Msg do
    @type t :: %__MODULE__{
            from: pid,
            command: CommandAddCards.t() | CommandRemoveCards.t()
          }
    defstruct from: 0, command: nil
  end

  defmodule ResponseCardsAdded do
    require Record
    @type t :: {__MODULE__, non_neg_integer}
    Record.defrecord(:record, __MODULE__, nr: 0)
  end

  defmodule ResponseCardsRemoved do
    require Record
    @type t :: {__MODULE__, [Card.t()]}
    Record.defrecord(:record, __MODULE__, cards: [])
  end

  defmodule ResponseUnableToRemoveCards do
    require Record
    @type t :: {__MODULE__, non_neg_integer}
    Record.defrecord(:record, __MODULE__, nr: 0)
  end

  defmodule ResponseError do
    require Record
    @type t :: {__MODULE__, bitstring}
    Record.defrecord(:record, __MODULE__, error: nil)
  end

  defmodule Cmd do
    @type t :: %__MODULE__{
            game: pid,
            response:
              ResponseCardsAdded.t()
              | ResponseCardsRemoved.t()
              | ResponseUnableToRemoveCards.t()
              | ResponseError.t()
          }
    defstruct game: 0, response: nil
  end

  @type model :: [Card.t()]

  @doc """

   ## Examples

   iex(1)> PlayerElixir.update([], :none)
   {[], %PlayerElixir.Cmd{game: nil, response: {PlayerElixir.ResponseError, "PlayerElixir - unknown message: :none"}}}

   iex(2)> PlayerElixir.update([{Card, "Suit::Heart", "Value::Two"}], %PlayerElixir.Msg{from: 1, command: {PlayerElixir.CommandAddCards, [{Card, "Suit::Heart", "Value::Ace"}]}})
   {[{Card, "Suit::Heart", "Value::Two"}, {Card, "Suit::Heart", "Value::Ace"}], %PlayerElixir.Cmd{game: 1, response: {PlayerElixir.ResponseCardsAdded, 1}}}

   iex(3)> PlayerElixir.update([{Card, "Suit::Heart", "Value::Two"}], %PlayerElixir.Msg{from: 1, command: {PlayerElixir.CommandRemoveCards, 10}})
   {[{Card, "Suit::Heart", "Value::Two"}], %PlayerElixir.Cmd{game: 1, response: {PlayerElixir.ResponseUnableToRemoveCards, 10}}}

   iex(4)> PlayerElixir.update([{Card, "Suit::Heart", "Value::Two"}], %PlayerElixir.Msg{from: 1, command: {PlayerElixir.CommandRemoveCards, 1}})
   {[], %PlayerElixir.Cmd{game: 1, response: {PlayerElixir.ResponseCardsRemoved, [{Card, "Suit::Heart", "Value::Two"}]}}}

  """
  @spec update(model, Msg.t()) :: {model, Cmd.t()}
  def update(model, msg) do
    case {model, msg} do
      {cards,
       %PlayerElixir.Msg{from: game, command: {PlayerElixir.CommandAddCards, cards_to_add}}} ->
        {cards ++ cards_to_add,
         %PlayerElixir.Cmd{
           game: game,
           response: {PlayerElixir.ResponseCardsAdded, Enum.count(cards_to_add)}
         }}

      {cards,
       %PlayerElixir.Msg{from: game, command: {PlayerElixir.CommandRemoveCards, nr_of_cards}}} ->
        case Enum.count(cards) >= nr_of_cards do
          true ->
            split_at = Enum.count(cards) - nr_of_cards
            {cards_left, cards_removed} = cards |> Enum.split(split_at)
            # {cards_removed, cards_left} = cards |> Enum.split(nr_of_cards)

            {cards_left,
             %PlayerElixir.Cmd{
               game: game,
               response: {PlayerElixir.ResponseCardsRemoved, cards_removed}
             }}

          false ->
            {cards,
             %PlayerElixir.Cmd{
               game: game,
               response: {PlayerElixir.ResponseUnableToRemoveCards, nr_of_cards}
             }}
        end

      {cards, msg} ->
        err = "PlayerElixir - unknown message: " <> inspect(msg)
        IO.puts(err)

        {cards,
         %PlayerElixir.Cmd{
           game: nil,
           response: {PlayerElixir.ResponseError, err}
         }}
    end
  end

  @spec execute_cmd(Cmd.t()) :: nil
  def execute_cmd(cmd) do
    case cmd do
      %PlayerElixir.Cmd{game: nil, response: {PlayerElixir.ResponseError, err}} ->
        IO.puts(err)

      %PlayerElixir.Cmd{game: game, response: response} ->
        msg = GameElixir.MsgResponseFromPlayer.record(player: self(), response: response)
        send(game, msg)
    end
  end

  @spec process(model) :: nil
  def process(model \\ []) do
    receive do
      msg ->
        {model, cmd} = update(model, msg)
        execute_cmd(cmd)
        process(model)
    end
  end
end
