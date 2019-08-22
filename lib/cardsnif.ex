defmodule Cardsnif do
  use Rustler, otp_app: :cardsnif

  defp err() do
    :erlang.nif_error(:nif_not_loaded)
  end

  @moduledoc """
  Documentation for Cardsnif.
  Elixir stubs for NIFs
  Rust NIF functions are defined in native/cardsnif/src/lib.rs
  """

  # When your NIF is loaded, it will override this function.
  def player_update(_, _), do: err()
  def game_update(_, _), do: err()

end
