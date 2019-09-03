defmodule Start do
  @moduledoc """
  Application starting point.

  Play_rust and play_elixir functions construct game (arbiter) and players
  processes and then send message \"Msg::StartGame\" to Game process.

  Plays are recorded in files: replay_rust.txt or replay_elixir.txt
  """

  require GameRust.ModelNone

  @spec process_rust(pid) :: nil
  defp process_rust(file) do
    receive do
      {{GameRust.ModelPlayer1Won, _}, _, _} ->
        IO.puts(file, "")
        IO.puts(file, "The End: Player 1 Won!")

      {{GameRust.ModelPlayer2Won, _}, _, _} ->
        IO.puts(file, "")
        IO.puts(file, "The End: Player 2 Won!")

      {{GameRust.ModelTie, _}, _, _} ->
        IO.puts(file, "")
        IO.puts(file, "The End: Tie!")

      {{GameRust.ModelError, err}, _, _} ->
        IO.puts(file, "")
        IO.puts(file, "Error occured: #{err}")

      {game_model, cmd, delta} ->
        IO.inspect(file, game_model, label: "Game model")
        IO.inspect(file, cmd, label: "Game cmd")
        IO.inspect(file, delta, label: "Execution of update in microseconds")
        IO.puts(file, "")
        process_rust(file)

      msg ->
        IO.puts(file, "")
        IO.inspect(file, msg, label: "Unknown msg")
    end
  end

  @spec play() :: nil
  def play() do
    case File.open("replay.txt", [:write]) do
      {:ok, file} ->
        IO.puts(file, "NIF Rustler Version")
        IO.puts(file, "")
        player1 = spawn(PlayerRust, :process, [])
        player2 = spawn(PlayerRust, :process, [])

        pids = %GameRust.Pids{
          supervisor: Helper.pid_to_list(self()),
          player1: Helper.pid_to_list(player1),
          player2: Helper.pid_to_list(player2)
        }

        game_model = GameRust.ModelNone.record(pids: pids)
        game = spawn(GameRust, :process, [game_model])

        send(game, "Msg::StartGame")

        process_rust(file)
        File.close(file)
        :erlang.exit(player1, :kill)
        :erlang.exit(player2, :kill)
        :erlang.exit(game, :kill)

      {:error, _} ->
        IO.puts("Unable to write to file replay_rust.txt")
    end

    nil
  end

end
