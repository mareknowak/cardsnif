defmodule GameElixir do
  @moduledoc """
  Elixir version of Game (arbiter) process
  """

  use Exceptional

  # Pids is a struct that collects pids of supervisor and players. It allows
  # Game process to send and receive messages from them
  defmodule Pids do
    @type t :: %__MODULE__{
            supervisor: pid,
            player1: pid,
            player2: pid
          }
    defstruct supervisor: 0, player1: 1, player2: 2
  end

  # Player's command with its pid is a message for Game process
  defmodule MsgResponseFromPlayer do
    require Record
    @type t :: {__MODULE__, pid, PlayerElixir.Cmd.t()}
    Record.defrecord(:record, __MODULE__, player: 1, response: nil)
  end

  # msg received from Player is of MsgResponseFromPlayer.t type but it can be a
  # bitstring like "Msg::StartGame"
  @type msg :: bitstring | MsgResponseFromPlayer.t()

  # ModelWar also tracks pile of cards
  @type pile :: [Card.t()]

  # The Game starts with ModelNone model. It's only a pids container.
  defmodule ModelNone do
    require Record
    @type t :: {__MODULE__, Pids.t()}
    Record.defrecord(:record, __MODULE__, pids: nil)
  end

  # Game sends cards to Players and changes its model from ModelNone to
  # ModelPlayers to signal it
  defmodule ModelPlayers do
    require Record
    @type t :: {__MODULE__, Pids.t()}
    Record.defrecord(:record, __MODULE__, pids: nil)
  end

  # ModelPlayers receives a response from one player and changes to
  # ModelPlayersWithResponse
  defmodule ModelPlayersWithResponse do
    require Record
    @type t :: {__MODULE__, Pids.t(), pid, PlayerElixir.Cmd.t()}
    Record.defrecord(:record, __MODULE__, pids: nil, player: 0, response: nil)
  end

  # After receivnig confirmations from Players, Game sends commands to remove 1
  # card to each player. Game model is ModelBattle.
  defmodule ModelBattle do
    require Record
    @type t :: {__MODULE__, Pids.t()}
    Record.defrecord(:record, __MODULE__, pids: nil)
  end

  # ModelBattle changes to ModelBattleWithResponse after receiving a response
  # from one player.
  defmodule ModelBattleWithResponse do
    require Record
    @type t :: {__MODULE__, Pids.t(), pid, PlayerElixir.Cmd.t()}
    Record.defrecord(:record, __MODULE__, pids: nil, player: 1, response: nil)
  end

  # Game model is ModelBattleWithResponse. Game receives response from the
  # second player, judges the players, and changes to ModelBattleWonByPlayer or
  # ModelWar or ModelPlayer1Won or ModelPlayer2Won.
  defmodule ModelBattleWonByPlayer do
    require Record
    @type t :: {__MODULE__, Pids.t(), pid}
    Record.defrecord(:record, __MODULE__, pids: nil, player: 1)
  end

  # It was a tie and cards are moved to a pile. Model changes to ModelWar and
  # Game sends commands to Players to remove 2 cards.
  defmodule ModelWar do
    require Record
    @type t :: {__MODULE__, Pids.t(), GameElixir.pile()}
    Record.defrecord(:record, __MODULE__, pids: nil, pile: nil)
  end

  # ModelWar receives one response and changes to ModelWarWithResponse
  defmodule ModelWarWithResponse do
    require Record
    @type t :: {__MODULE__, Pids.t(), GameElixir.pile(), pid, PlayerElixir.Cmd.t()}
    Record.defrecord(:record, __MODULE__, pids: nil, pile: nil, player: 1, response: nil)
  end

  # War is over, cards are sent to the winner
  defmodule ModelWarWonByPlayer do
    require Record
    @type t :: {__MODULE__, Pids.t(), pid}
    Record.defrecord(:record, __MODULE__, pids: nil, player: 1)
  end

  # End of the Game
  defmodule ModelPlayer1Won do
    require Record
    @type t :: {__MODULE__, Pids.t()}
    Record.defrecord(:record, __MODULE__, pids: nil)
  end

  # End of the Game
  defmodule ModelPlayer2Won do
    require Record
    @type t :: {__MODULE__, Pids.t()}
    Record.defrecord(:record, __MODULE__, pids: nil)
  end

  # End of the Game
  defmodule ModelTie do
    require Record
    @type t :: {__MODULE__, Pids.t()}
    Record.defrecord(:record, __MODULE__, pids: nil)
  end

  # End of the Game
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

  # Commands send by Game to Players
  defmodule SendCmd do
    @type t :: %__MODULE__{
            to: pid,
            command: PlayerElixir.CommandAddCards.t() | PlayerElixir.CommandRemoveCards.t()
          }
    defstruct to: 1, command: nil
  end

  # List of commands to be sent
  defmodule SendCmds do
    require Record
    @type t :: {__MODULE__, [SendCmd.t()]}
    Record.defrecord(:record, __MODULE__, cmds: nil)
  end

  # Bitsting because of "Cmd::None"
  @type cmd :: bitstring | SendCmds.t()

  defmodule FightResultPlayerWon do
    require Record
    @type t :: {__MODULE__, pid, [Card.t()]}
    Record.defrecord(:record, __MODULE__, player: 1, cards: [])
  end

  defmodule FightResultTie do
    require Record
    @type t :: {__MODULE__, [Card.t()]}
    Record.defrecord(:record, __MODULE__, cards: [])
  end

  defmodule FightResultError do
    require Record
    @type t :: {__MODULE__, bitstring}
    Record.defrecord(:record, __MODULE__, error: nil)
  end

  @type fight_result :: FightResultPlayerWon.t() | FightResultTie.t() | FightResultError.t()

  @type maybe_pile :: [Card.t()] | nil


  @doc """
  Function fight_result
  This funcion is used as a helper function for judge_players

  Input: players pids their cards and nil or pile
  Out: fight_result

  Pile is used to distinguish between Battle and War.
  Battle: pile is nil because there is no pile
  War: there is a pile of cards
  """
  @spec fight_result(pid, [Card.t()], pid, [Card.t()], maybe_pile) :: fight_result
  def fight_result(player1, cards1, player2, cards2, pile) do
    require FightResultPlayerWon
    require FightResultTie
    require FightResultError

    has_pile =
      case pile do
        nil -> false
        _ -> true
      end

    correct_nr_of_cards =
      case {has_pile, Enum.count(cards1), Enum.count(cards2)} do
        {false, 1, 1} -> true
        {true, 2, 2} -> true
        _ -> false
      end

    case correct_nr_of_cards do
      false ->
        FightResultError.record(error: "Players must have right number of cards")

      true ->
        cards_to_send =
          case has_pile do
            false -> cards1 ++ cards2
            true -> pile ++ cards1 ++ cards2
          end

        get_cards_to_judge =
          case {cards1, cards2} do
            {[card1], [card2]} -> {card1, card2}
            {[_, card1], [_, card2]} -> {card1, card2}
            _ -> nil
          end

        case get_cards_to_judge do
          {card1, card2} ->
            case Cards.cards_are_equal(card1, card2) do
              true ->
                FightResultTie.record(cards: cards_to_send)

              false ->
                case Cards.first_is_less(card1, card2) do
                  true ->
                    FightResultPlayerWon.record(
                      player: player2,
                      cards: cards_to_send
                    )

                  false ->
                    FightResultPlayerWon.record(
                      player: player1,
                      cards: cards_to_send
                    )
                end
            end

          _ ->
            FightResultError.record(error: "Unable to get cards to judge")
        end
    end
  end

  @type player_response ::
          PlayerElixir.ResponseCardsAdded.t()
          | PlayerElixir.ResponseCardsRemoved.t()
          | PlayerElixir.ResponseUnableToRemoveCards.t()
          | PlayerElixir.ResponseError.t()

  @doc """
  match_players_with_responses

  When first response is received it is only carried in model(WithResponse)
  Responses are matched with players only after the second response arrives.

  """
  @spec match_players_with_responses(
          Pids.t(),
          pid,
          player_response,
          pid,
          player_response
        ) ::
          {:ok, {{pid, player_response}, {pid, player_response}}} | {:error, bitstring}
  def match_players_with_responses(pids, pid1, response1, pid2, response2) do
    players =
      case pids do
        %Pids{supervisor: _, player1: player1, player2: player2} ->
          {player1, player2}

        _ ->
          nil
      end

    case players == {pid1, pid2} do
      true ->
        {:ok, {{pid1, response1}, {pid2, response2}}}

      false ->
        case players == {pid2, pid1} do
          true ->
            {:ok, {{pid2, response2}, {pid1, response1}}}

          false ->
            {:error, "Unable to match players with responses"}
        end
    end
  end

  @doc """
  judge_players

  The second response arrives and input model is either ModelBattleWithResponse
  or ModelWarWithResponse

  This is a helper function used in update function to not repeat myself.

  Output model is:
  ModelBattleWonByPlayer (and send cards to the winner)
  ModelWarWonByPlayer (and send cards to the winner)
  ModelWar - let's play again
  ModelPlayer1Won (because Player2 is unable to remove cards)
  ModelPlayer2Won (because Player1 is unable to remove cards)
  ModelTie (both players don't have cards)
  ModelError


  """
  @spec judge_players(
          Pids.t(),
          pid(),
          player_response,
          pid(),
          player_response,
          maybe_pile
        ) :: {model, cmd}
  def judge_players(pids, pid1, response1, pid2, response2, pile) do
    require ModelBattleWonByPlayer
    require ModelError
    require ModelPlayer1Won
    require ModelPlayer2Won
    require ModelTie
    require ModelWar
    require ModelWarWonByPlayer

    require FightResultError
    require FightResultPlayerWon
    require FightResultTie

    require SendCmds

    require PlayerElixir.CommandAddCards
    require PlayerElixir.CommandRemoveCards
    require PlayerElixir.ResponseCardsRemoved
    require PlayerElixir.ResponseUnableToRemoveCards

    has_pile =
      case pile do
        nil -> false
        _ -> true
      end

    model_name =
      case has_pile do
        false -> "GameElixir.ModelBattleWithResponse"
        true -> "GameElixir.ModelWarWithResponse"
      end

    players_with_responses = match_players_with_responses(pids, pid1, response1, pid2, response2)

    case players_with_responses do
      {:error, error} ->
        ModelError.record(
          pids: pids,
          error: model_name <> error
        )

      {:ok, {{player1, player1_response}, {player2, player2_response}}} ->
        case {player1_response, player2_response} do
          {
            PlayerElixir.ResponseCardsRemoved.record(cards: player1_cards),
            PlayerElixir.ResponseCardsRemoved.record(cards: player2_cards)
          } ->
            fight_result = fight_result(player1, player1_cards, player2, player2_cards, pile)

            case fight_result do
              FightResultPlayerWon.record(
                player: player,
                cards: cards
              ) ->
                send_cards = [
                  %SendCmd{
                    to: player,
                    command: PlayerElixir.CommandAddCards.record(cards: cards)
                  }
                ]

                cmd = SendCmds.record(cmds: send_cards)

                case has_pile do
                  false ->
                    {
                      ModelBattleWonByPlayer.record(
                        pids: pids,
                        player: player
                      ),
                      cmd
                    }

                  true ->
                    {
                      ModelWarWonByPlayer.record(
                        pids: pids,
                        player: player
                      ),
                      cmd
                    }
                end

              FightResultTie.record(cards: pile) ->
                remove_cards = [
                  %SendCmd{
                    to: player1,
                    command: PlayerElixir.CommandRemoveCards.record(nr: 2)
                  },
                  %SendCmd{
                    to: player2,
                    command: PlayerElixir.CommandRemoveCards.record(nr: 2)
                  }
                ]

                {
                  ModelWar.record(pids: pids, pile: pile),
                  SendCmds.record(cmds: remove_cards)
                }

              FightResultError.record(error: error) ->
                error = model_name <> error

                {
                  ModelError.record(pids: pids, error: error),
                  "Cmd::None"
                }
            end

          {
            PlayerElixir.ResponseCardsRemoved.record(cards: _),
            PlayerElixir.ResponseUnableToRemoveCards.record(nr: _)
          } ->
            {
              ModelPlayer1Won.record(pids: pids),
              "Cmd::None"
            }

          {
            PlayerElixir.ResponseUnableToRemoveCards.record(nr: _),
            PlayerElixir.ResponseCardsRemoved.record(cards: _)
          } ->
            {
              ModelPlayer2Won.record(pids: pids),
              "Cmd::None"
            }

          {
            PlayerElixir.ResponseUnableToRemoveCards.record(nr: _),
            PlayerElixir.ResponseUnableToRemoveCards.record(nr: _)
          } ->
            {
              ModelTie.record(pids: pids),
              "Cmd::None"
            }

          {resp1, resp2} ->
            error =
              model_name <>
                " received wrong responses: " <> inspect(resp1) <> ", " <> inspect(resp2)

            {
              ModelError.record(pids: pids, error: error),
              "Cmd::None"
            }
        end
    end
  end

  @spec update(model, msg) :: {model, cmd}
  def update(model, msg) do
    require ModelNone
    require ModelPlayers
    require ModelPlayersWithResponse
    require ModelBattle
    require ModelBattleWithResponse
    require ModelBattleWonByPlayer
    require ModelWar
    require ModelWarWithResponse
    require ModelWarWonByPlayer
    require ModelError
    require ModelPlayer1Won
    require ModelPlayer2Won
    require ModelTie

    require SendCmds
    require MsgResponseFromPlayer

    require PlayerElixir.CommandAddCards
    require PlayerElixir.CommandRemoveCards
    require PlayerElixir.ResponseCardsAdded
    require PlayerElixir.ResponseCardsRemoved
    require PlayerElixir.ResponseUnableToRemoveCards

    case {model, msg} do
      # model: ModelNone, msg: Msg::StartGame -> model: ModelPlayers, cmd: AddCards
      # Start Game: change model to ModelPlayers and wait for response from players
      #
      # Game sends cards to Players and changes its model from ModelNone to
      # ModelPlayers to signal it
      {
        ModelNone.record(pids: %Pids{supervisor: supervisor, player1: player1, player2: player2}),
        "Msg::StartGame"
      } ->
        {cards1, cards2} =
          Cards.make_deck()
          |> Cards.shuffle()
          |> Enum.split(26)

        send_decks = [
          %SendCmd{
            to: player1,
            command: PlayerElixir.CommandAddCards.record(cards: cards1)
          },
          %SendCmd{
            to: player2,
            command: PlayerElixir.CommandAddCards.record(cards: cards2)
          }
        ]

        pids = %Pids{supervisor: supervisor, player1: player1, player2: player2}

        {
          ModelPlayers.record(pids: pids),
          SendCmds.record(cmds: send_decks)
        }

      # model: ModelPlayers, msg: ResponseCardsAdded -> model:
      # ModelPlayersWithResponse, cmd: Cmd::None
      #
      # Got response from one player so change model to
      # ModelPlayersWithResponse: wait for the second response and send nothing
      {
        ModelPlayers.record(
          pids: %Pids{supervisor: supervisor, player1: player1, player2: player2}
        ),
        MsgResponseFromPlayer.record(
          player: player,
          response: PlayerElixir.ResponseCardsAdded.record(nr: 26)
        )
      } ->
        {
          ModelPlayersWithResponse.record(
            pids: %Pids{supervisor: supervisor, player1: player1, player2: player2},
            player: player,
            response: PlayerElixir.ResponseCardsAdded.record(nr: 26)
          ),
          "Cmd::None"
        }

      # model: ModelPlayersWithResponse, msg: ResponseCardsAdded
      # -> model: ModelBattle, cmd: RemoveCards (request one card from each player)
      {
        ModelPlayersWithResponse.record(
          pids: %Pids{supervisor: supervisor, player1: player1, player2: player2},
          player: pid1,
          response: PlayerElixir.ResponseCardsAdded.record(nr: 26)
        ),
        MsgResponseFromPlayer.record(
          player: pid2,
          response: PlayerElixir.ResponseCardsAdded.record(nr: 26)
        )
      } ->
        # check pids
        correct_pids = {player1, player2} == {pid1, pid2} || {player1, player2} == {pid2, pid1}

        case correct_pids do
          false ->
            err = "GameElixir.ModelPlayersWithResponse received wrong pids"

            {
              ModelError.record(
                pids: %Pids{supervisor: supervisor, player1: player1, player2: player2},
                error: err
              ),
              "Cmd::None"
            }

          true ->
            remove_cards = [
              %SendCmd{
                to: player1,
                command: PlayerElixir.CommandRemoveCards.record(nr: 1)
              },
              %SendCmd{
                to: player2,
                command: PlayerElixir.CommandRemoveCards.record(nr: 1)
              }
            ]

            pids = %Pids{supervisor: supervisor, player1: player1, player2: player2}

            {
              ModelBattle.record(pids: pids),
              SendCmds.record(cmds: remove_cards)
            }
        end

      # model: ModelBattle, msg: ResponseCardsRemoved (from one player)
      # -> model: ModelBattleWithResponse, cmd: None (wait for the second response)
      {
        ModelBattle.record(
          pids: %Pids{supervisor: supervisor, player1: player1, player2: player2}
        ),
        MsgResponseFromPlayer.record(
          player: player,
          response: response
        )
      } ->
        {
          ModelBattleWithResponse.record(
            pids: %Pids{supervisor: supervisor, player1: player1, player2: player2},
            player: player,
            response: response
          ),
          "Cmd::None"
        }

      # model: ModelBattleWithResponse, msg: ResponseCardsRemoved (from the second player)
      # -> judge players
      #    if cards are not equal send them to the winner
      #    otherwise start War
      #
      # Game model is ModelBattleWithResponse. Game receives response from the
      # second player, judges the players, and changes to ModelBattleWonByPlayer or
      # ModelWar or ModelPlayer1Won or ModelPlayer2Won.
      {
        ModelBattleWithResponse.record(
          pids: pids,
          player: pid1,
          response: response1
        ),
        MsgResponseFromPlayer.record(
          player: pid2,
          response: response2
        )
      } ->
        judge_players(pids, pid1, response1, pid2, response2, nil)

      # ModelBattleWonByPlayer is one of the results of a battle. After
      # receiving confirmation from Player (ResponseCardsAdded) start another
      # battle.
      {
        ModelBattleWonByPlayer.record(
          pids: %Pids{supervisor: supervisor, player1: player1, player2: player2},
          player: player
        ),
        MsgResponseFromPlayer.record(
          player: pid,
          response: PlayerElixir.ResponseCardsAdded.record(nr: 2)
        )
      } ->
        case player == pid do
          true ->
            remove_cards = [
              %SendCmd{
                to: player1,
                command: PlayerElixir.CommandRemoveCards.record(nr: 1)
              },
              %SendCmd{
                to: player2,
                command: PlayerElixir.CommandRemoveCards.record(nr: 1)
              }
            ]

            {
              ModelBattle.record(
                pids: %Pids{supervisor: supervisor, player1: player1, player2: player2}
              ),
              SendCmds.record(cmds: remove_cards)
            }

          false ->
            {
              ModelError.record(
                pids: %Pids{supervisor: supervisor, player1: player1, player2: player2},
                error: "Model ModelBattleWonByPlayer received wrong pids."
              ),
              "Cmd::None"
            }
        end

      # ModelWar receives first response and waits for the second.
      {
        ModelWar.record(
          pids: %Pids{supervisor: supervisor, player1: player1, player2: player2},
          pile: pile
        ),
        MsgResponseFromPlayer.record(
          player: player,
          response: response
        )
      } ->
        {
          ModelWarWithResponse.record(
            pids: %Pids{supervisor: supervisor, player1: player1, player2: player2},
            pile: pile,
            player: player,
            response: response
          ),
          "Cmd::None"
        }

      # Second response arrived. Time to judge Players.
      #
      # Game model is ModelWarWithResponse. Game receives response from the
      # second player, judges the players, and changes to ModelWarWonByPlayer or
      # ModelBattle or ModelPlayer1Won or ModelPlayer2Won.
      {
        ModelWarWithResponse.record(
          pids: pids,
          pile: pile,
          player: pid1,
          response: response1
        ),
        MsgResponseFromPlayer.record(
          player: pid2,
          response: response2
        )
      } ->
        judge_players(pids, pid1, response1, pid2, response2, pile)

      {
        ModelWarWonByPlayer.record(
          pids: %Pids{supervisor: supervisor, player1: player1, player2: player2},
          player: player
        ),
        MsgResponseFromPlayer.record(
          player: player,
          response: PlayerElixir.ResponseCardsAdded.record(nr: _)
        )
      } ->
        remove_cards = [
          %SendCmd{
            to: player1,
            command: PlayerElixir.CommandRemoveCards.record(nr: 1)
          },
          %SendCmd{
            to: player2,
            command: PlayerElixir.CommandRemoveCards.record(nr: 1)
          }
        ]

        {
          ModelBattle.record(
            pids: %Pids{supervisor: supervisor, player1: player1, player2: player2}
          ),
          SendCmds.record(cmds: remove_cards)
        }

      # Unexpected message. Change Game model to ModelError.
      {
        {
          model,
          %Pids{supervisor: supervisor, player1: player1, player2: player2}
        },
        msg
      } ->
        err = inspect(model) <> " received wrong msg: " <> inspect(msg)

        {
          ModelError.record(
            pids: %Pids{supervisor: supervisor, player1: player1, player2: player2},
            error: err
          ),
          "Cmd::None"
        }
    end
  end

  def send_info(model, cmd, delta) do
    case {model, cmd} do
      {model, cmd} ->
        # get supervisor pid
        # for sure there is a smarter way to do it
        supervisor =
          case model do
            {_model, %Pids{supervisor: supervisor, player1: _, player2: _}} ->
              supervisor

            {_model, %Pids{supervisor: supervisor, player1: _, player2: _}, _} ->
              supervisor

            {_model, %Pids{supervisor: supervisor, player1: _, player2: _}, _, _} ->
              supervisor

            {_model, %Pids{supervisor: supervisor, player1: _, player2: _}, _, _, _} ->
              supervisor
          end

        send(supervisor, {model, cmd, delta})
    end
  end

  @spec send_cmd(cmd) :: nil
  def send_cmd(cmd) do
    case cmd do
      %SendCmd{command: cmd, to: pid} ->
        msg = %PlayerElixir.Msg{from: self(), command: cmd}
        send(pid, msg)

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
      {GameElixir.SendCmds, list_of_cmds} ->
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
