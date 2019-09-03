defmodule GameRust do
  @moduledoc """
  Rust version of Game (arbiter) process

  More detailed description of types and functions can be found in GameElixir
  module.

  """

  use Exceptional

  # Pids are converted by term_to_binary(pid) and stored as list of bytes
  # On Rust side they are converted to binaries (Vec<u8>)
  defmodule Pids do
    @type t :: %__MODULE__{
            supervisor: [byte()],
            player1: [byte()],
            player2: [byte()]
          }
    defstruct supervisor: [0], player1: [1], player2: [2]
  end

  defmodule MsgResponseFromPlayer do
    require Record
    @type t :: {__MODULE__, [byte()], PlayerRust.Cmd.t()}
    Record.defrecord(:record, __MODULE__, player: [1], response: nil)
  end

  @type msg :: bitstring | MsgResponseFromPlayer.t()

  @type pile :: [Card.t()]

  defmodule ModelNone do
    require Record
    @type t :: {__MODULE__, Pids.t()}
    Record.defrecord(:record, __MODULE__, pids: nil)
  end

  defmodule ModelPlayers do
    require Record
    @type t :: {__MODULE__, Pids.t()}
    Record.defrecord(:record, __MODULE__, pids: nil)
  end

  defmodule ModelPlayersWithResponse do
    require Record
    @type t :: {__MODULE__, Pids.t(), [byte()], PlayerRust.Cmd.t()}
    Record.defrecord(:record, __MODULE__, pids: nil, player: [0], response: nil)
  end

  defmodule ModelBattle do
    require Record
    @type t :: {__MODULE__, Pids.t()}
    Record.defrecord(:record, __MODULE__, pids: nil)
  end

  defmodule ModelBattleWithResponse do
    require Record
    @type t :: {__MODULE__, Pids.t(), [byte()], PlayerRust.Cmd.t()}
    Record.defrecord(:record, __MODULE__, pids: nil, player: [1], response: nil)
  end

  defmodule ModelBattleWonByPlayer do
    require Record
    @type t :: {__MODULE__, Pids.t(), [byte()]}
    Record.defrecord(:record, __MODULE__, pids: nil, player: [1])
  end

  defmodule ModelWar do
    require Record
    @type t :: {__MODULE__, Pids.t(), GameRust.pile()}
    Record.defrecord(:record, __MODULE__, pids: nil, pile: nil)
  end

  defmodule ModelWarWithResponse do
    require Record
    @type t :: {__MODULE__, Pids.t(), GameRust.pile(), [byte()], PlayerRust.Cmd.t()}
    Record.defrecord(:record, __MODULE__, pids: nil, pile: nil, player: [1], resp: nil)
  end

  defmodule ModelWarWonByPlayer do
    require Record
    @type t :: {__MODULE__, Pids.t(), [byte()]}
    Record.defrecord(:record, __MODULE__, pids: nil, player: [1])
  end

  defmodule ModelPlayer1Won do
    require Record
    @type t :: {__MODULE__, Pids.t()}
    Record.defrecord(:record, __MODULE__, pids: nil)
  end

  defmodule ModelPlayer2Won do
    require Record
    @type t :: {__MODULE__, Pids.t()}
    Record.defrecord(:record, __MODULE__, pids: nil)
  end

  defmodule ModelTie do
    require Record
    @type t :: {__MODULE__, Pids.t()}
    Record.defrecord(:record, __MODULE__, pids: nil)
  end

  defmodule ModelError do
    require Record
    @type t :: {__MODULE__, Pids.t(), bitstring}
    Record.defrecord(:record, __MODULE__, pids: nil, error: nil)
  end

  @type model ::
          ModelNone.t()
          | ModelPlayers.t()
          | ModelPlayersWithResponse.t()
          | ModelBattle.t()
          | ModelBattleWithResponse.t()
          | ModelBattleWonByPlayer.t()
          | ModelWar.t()
          | ModelWarWithResponse.t()
          | ModelWarWonByPlayer.t()
          | ModelPlayer1Won.t()
          | ModelPlayer2Won.t()
          | ModelTie.t()
          | ModelError.t()

  defmodule SendCmd do
    @type t :: %__MODULE__{
            to: [byte()],
            cmd: PlayerRust.Msg.t()
          }
    defstruct to: [1], cmd: nil
  end

  defmodule SendCmds do
    require Record
    @type t :: {__MODULE__, [SendCmd.t()]}
    Record.defrecord(:record, __MODULE__, cmds: nil)
  end

  # Bitsting because of "Cmd::None"
  @type cmd :: bitstring | SendCmds.t()

  @spec update(model, msg) :: {model, cmd}
  def update(model, msg) do
    case safe(&Cardsnif.game_update/2).(model, msg) do
      {model, cmd} -> {model, cmd}
      %ErlangError{original: txt} -> {model, {:error, {txt, msg}}}
    end
  end

  def send_info(model, cmd, delta) do
    case {model, cmd} do
      {model, cmd} ->
        supervisor =
          case model do
            {_model, %GameRust.Pids{supervisor: supervisor, player1: _, player2: _}} ->
              supervisor

            {_model, %GameRust.Pids{supervisor: supervisor, player1: _, player2: _}, _} ->
              supervisor

            {_model, %GameRust.Pids{supervisor: supervisor, player1: _, player2: _}, _, _} ->
              supervisor

            {_model, %GameRust.Pids{supervisor: supervisor, player1: _, player2: _}, _, _, _} ->
              supervisor
          end

        send(Helper.list_to_pid(supervisor), {model, cmd, delta})
    end
  end

  @spec send_cmd(cmd) :: nil
  def send_cmd(cmd) do
    case cmd do
      %GameRust.SendCmd{cmd: cmd, to: pid} ->
        msg = %PlayerRust.Msg{from: Helper.pid_to_list(self()), command: cmd}
        send(Helper.list_to_pid(pid), msg)

      _ ->
        nil
    end
  end

  @spec send_cmds([cmd]) :: nil
  def send_cmds(list) do
    case list do
      [] ->
        nil

      [h | tail] ->
        send_cmd(h)
        send_cmds(tail)
    end
  end

  @spec execute(model, cmd, number) :: nil
  def execute(model, cmd, delta) do
    send_info(model, cmd, delta)

    case cmd do
      {GameRust.SendCmds, list_of_cmds} ->
        send_cmds(list_of_cmds)

      _ ->
        nil
    end
  end

  @spec process(model) :: nil
  def process(model) do
    receive do
      msg ->
        ts1 = System.monotonic_time(:microsecond)
        {model, cmd} = update(model, msg)
        ts2 = System.monotonic_time(:microsecond)
        delta = ts2 - ts1
        execute(model, cmd, delta)
        process(model)
    end
  end
end
