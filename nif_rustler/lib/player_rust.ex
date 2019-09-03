defmodule PlayerRust do
  @moduledoc """
  Rust version of Player process
  model: list of cards
  """
  use Exceptional
  require GameRust.MsgResponseFromPlayer

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
            from: [byte()],
            command: CommandAddCards.t() | CommandRemoveCards.t()
          }
    defstruct from: [0], command: nil
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
            game: [byte()],
            response:
              ResponseCardsAdded.t()
              | ResponseCardsRemoved.t()
              | ResponseUnableToRemoveCards.t()
              | ResponseError.t()
          }
    defstruct game: [0], response: nil
  end

  @type model :: [Card.t()]

  @spec update(model, Msg.t()) :: {model, Cmd.t()}
  def update(model, msg) do
    require ResponseError

    case safe(&Cardsnif.player_update/2).(model, msg) do
      {model, cmd} -> {model, cmd}
      %ErlangError{original: txt} -> {model, {:error, {txt, msg}}}
    end
  end

  @spec execute_cmd(Cmd.t()) :: nil
  def execute_cmd(cmd) do
    case cmd do
      %PlayerRust.Cmd{game: game, response: response} ->
        game = Helper.list_to_pid(game)
        player = Helper.pid_to_list(self())
        msg = GameRust.MsgResponseFromPlayer.record(player: player, response: response)
        send(game, msg)

      {:error, {erlang_error, _msg}} ->
        IO.puts(erlang_error)
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
