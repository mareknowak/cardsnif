defmodule GameElixirTest do
  use ExUnit.Case
  import GameElixir

  test "fight_result: incorrect nr of cards" do
    player1 = 1
    cards1 = [{Card, "Suit::Heart", "Value::Ten"}]
    player2 = 2
    cards2 = [{Card, "Suit::Heart", "Value::Ace"}, {Card, "Suit::Spade", "Value::Queen"}]
    pile = nil

    assert fight_result(player1, cards1, player2, cards2, pile) ==
             {GameElixir.FightResultError, "Players must have right number of cards"}
  end

  test "fight_result: without a pile" do
    player1 = 1
    cards1 = [{Card, "Suit::Heart", "Value::Ten"}]
    player2 = 2
    cards2 = [{Card, "Suit::Heart", "Value::Ace"}]
    pile = nil

    assert fight_result(player1, cards1, player2, cards2, pile) ==
             {
               GameElixir.FightResultPlayerWon,
               player2,
               [{Card, "Suit::Heart", "Value::Ten"}, {Card, "Suit::Heart", "Value::Ace"}]
             }
  end

  test "fight_result: with an empty pile" do
    player1 = 1
    cards1 = [{Card, "Suit::Heart", "Value::Ten"}, {Card, "Suit::Spade", "Value::Queen"}]
    player2 = 2
    cards2 = [{Card, "Suit::Heart", "Value::Nine"}, {Card, "Suit::Heart", "Value::Ace"}]
    pile = []

    assert fight_result(player1, cards1, player2, cards2, pile) ==
             {
               GameElixir.FightResultPlayerWon,
               player2,
               [
                 {Card, "Suit::Heart", "Value::Ten"},
                 {Card, "Suit::Spade", "Value::Queen"},
                 {Card, "Suit::Heart", "Value::Nine"},
                 {Card, "Suit::Heart", "Value::Ace"}
               ]
             }
  end

  test "fight_result: with non empty pile" do
    player1 = 1
    cards1 = [{Card, "Suit::Heart", "Value::Ten"}, {Card, "Suit::Spade", "Value::Queen"}]
    player2 = 2
    cards2 = [{Card, "Suit::Heart", "Value::Nine"}, {Card, "Suit::Heart", "Value::Ace"}]
    pile = [{Card, "Suit::Heart", "Value::Jack"}]

    assert fight_result(player1, cards1, player2, cards2, pile) ==
             {
               GameElixir.FightResultPlayerWon,
               player2,
               [
                 {Card, "Suit::Heart", "Value::Jack"},
                 {Card, "Suit::Heart", "Value::Ten"},
                 {Card, "Suit::Spade", "Value::Queen"},
                 {Card, "Suit::Heart", "Value::Nine"},
                 {Card, "Suit::Heart", "Value::Ace"}
               ]
             }
  end

  test "fight_result: tie with an empty pile" do
    player1 = 1
    cards1 = [{Card, "Suit::Heart", "Value::Ten"}, {Card, "Suit::Spade", "Value::Ace"}]
    player2 = 2
    cards2 = [{Card, "Suit::Heart", "Value::Nine"}, {Card, "Suit::Heart", "Value::Ace"}]
    pile = []

    assert fight_result(player1, cards1, player2, cards2, pile) ==
             {
               GameElixir.FightResultTie,
               [
                 {Card, "Suit::Heart", "Value::Ten"},
                 {Card, "Suit::Spade", "Value::Ace"},
                 {Card, "Suit::Heart", "Value::Nine"},
                 {Card, "Suit::Heart", "Value::Ace"}
               ]
             }
  end

  test "match players with responses" do
    pids = %GameElixir.Pids{supervisor: 0, player1: 1, player2: 2}

    assert match_players_with_responses(pids, 2, "dudek2", 1, "dudek1") ==
             {:ok, {{1, "dudek1"}, {2, "dudek2"}}}
  end

  test "update (model: ModelNone, msg: \"Msg::StartGame\") -> {model: Players, cmd: AddCards}" do
    model = {GameElixir.ModelNone, %GameElixir.Pids{supervisor: 0, player1: 1, player2: 2}}
    msg = "Msg::StartGame"

    result =
      case update(model, msg) do
        {{GameElixir.ModelPlayers, %GameElixir.Pids{supervisor: 0, player1: 1, player2: 2}},
         {GameElixir.SendCmds,
          [
            %GameElixir.SendCmd{
              to: 1,
              command: {PlayerElixir.CommandAddCards, _}
            },
            %GameElixir.SendCmd{
              to: 2,
              command: {PlayerElixir.CommandAddCards, _}
            }
          ]}} ->
          true

        _ ->
          false
      end

    assert result
  end

  test "update (model: ModelPlayers, msg: ResponseFromPlayer) -> {model: ModelPlayersWithResponse, cmd: None}" do
    model = {GameElixir.ModelPlayers, %GameElixir.Pids{supervisor: 0, player1: 1, player2: 2}}
    msg = {GameElixir.MsgResponseFromPlayer, 2, {PlayerElixir.ResponseCardsAdded, 26}}

    assert update(model, msg) ==
             {{GameElixir.ModelPlayersWithResponse,
               %GameElixir.Pids{supervisor: 0, player1: 1, player2: 2}, 2,
               {PlayerElixir.ResponseCardsAdded, 26}}, "Cmd::None"}
  end

  test "update (model: ModelPlayersWithResponse, msg: ResponseFromPlayer) -> {model: ModelBattle, cmd: CommandRemoveCards}" do
    model =
      {GameElixir.ModelPlayersWithResponse,
       %GameElixir.Pids{supervisor: 0, player1: 1, player2: 2}, 1,
       {PlayerElixir.ResponseCardsAdded, 26}}

    msg = {GameElixir.MsgResponseFromPlayer, 2, {PlayerElixir.ResponseCardsAdded, 26}}

    assert update(model, msg) ==
             {
               {GameElixir.ModelBattle, %GameElixir.Pids{supervisor: 0, player1: 1, player2: 2}},
               {GameElixir.SendCmds,
                [
                  %GameElixir.SendCmd{
                    to: 1,
                    command: {PlayerElixir.CommandRemoveCards, 1}
                  },
                  %GameElixir.SendCmd{
                    to: 2,
                    command: {PlayerElixir.CommandRemoveCards, 1}
                  }
                ]}
             }
  end

  test "update (model: ModelBattle, msg: ResponseFromPlayer) -> {model: ModelBattleWithResponse, cmd: None}" do
    model = {GameElixir.ModelBattle, %GameElixir.Pids{supervisor: 0, player1: 1, player2: 2}}

    msg =
      {GameElixir.MsgResponseFromPlayer, 1,
       {PlayerElixir.ResponseCardsRemoved, [{Card, "Suit::Hearts", "Value::Ten"}]}}

    assert update(model, msg) ==
             {
               {GameElixir.ModelBattleWithResponse,
                %GameElixir.Pids{supervisor: 0, player1: 1, player2: 2}, 1,
                {PlayerElixir.ResponseCardsRemoved, [{Card, "Suit::Hearts", "Value::Ten"}]}},
               "Cmd::None"
             }
  end
end
