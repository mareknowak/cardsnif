defmodule Helper do
  def pid_to_list(pid) do
    pid
    |> :erlang.term_to_binary()
    |> :erlang.binary_to_list()
  end

  def list_to_pid(list) do
    list
    |> :erlang.list_to_binary()
    |> :erlang.binary_to_term()
  end
end
