defmodule Cards do
  @moduledoc """
  Documentation for Cards Module

  Shuffle function is taken from "Etudes for Elixir", J. David Eisenberg:
  https://github.com/oreillymedia/etudes-for-elixir/blob/master/code/ch09-01/cards.ex
  """

  require Card

  @doc """
  Shuffle a list into random order using the Fisher-Yates method.
  From "Etudes for Elixir"
  """

  @spec shuffle([Card.t()]) :: [Card.t()]
  def shuffle(list) do
    :random.seed(:erlang.now())
    shuffle(list, [])
  end

  # From "Etudes for Elixir":
  #
  # The helper function takes a list to shuffle as its first
  # argument and the accumulated (shuffled) list as its second
  # argument.

  # When there are no more cards left to shuffle,
  # return the accumulated list.

  def shuffle([], acc) do
    acc
  end

  # From "Etudes for Elixir":
  #
  # This is easier to understand if you look at it as a
  # physical process. Split the deck at a random point.
  # Put the part above the "split point" aside (leading), and
  # take the first card (h) off the part below the split (t).
  # That first card goes onto a new pile ([h | acc]).
  # Now put together the part above the split and the
  # part below the split (leading ++ t) and go through
  # the process with the deck (which now has one less card).
  # This keeps going until you run out of cards to shuffle;
  # at that point, all the cards will have gotten to the
  # new pile, and that's your shuffled deck.

  def shuffle(list, acc) do
    {leading, [h | t]} = Enum.split(list, :random.uniform(Enum.count(list)) - 1)
    shuffle(leading ++ t, [h | acc])
  end

  @doc """
  Create a deck of 52 cards in the form [{Card, "Suit::Club", "Value::Three"},
  {Card, "Suit::Diamond", "Value::Ace"}...]
  """
  @spec make_deck() :: [Card.t()]
  def make_deck() do
    for suit <- [
          "Suit::Club",
          "Suit::Diamond",
          "Suit::Heart",
          "Suit::Spade"
        ] do
      for value <- [
            "Value::Two",
            "Value::Three",
            "Value::Four",
            "Value::Five",
            "Value::Six",
            "Value::Seven",
            "Value::Eight",
            "Value::Nine",
            "Value::Ten",
            "Value::Jack",
            "Value::Queen",
            "Value::King",
            "Value::Ace"
          ] do
        Card.record(suit: suit, value: value)
      end
    end
    |> List.flatten()
  end

  # Compare cards
  # First let's assign them values

  @doc """
  value(card) -> non_neg_integer

  ### Examples

  iex(3)> Cards.value({Card, "Suit::Spade", "Value::Ace"})
  14

  """

  @spec value(Card.t()) :: non_neg_integer
  def value(card) do
    case card do
      {_, _, value} ->
        case value do
          "Value::Two" -> 2
          "Value::Three" -> 3
          "Value::Four" -> 4
          "Value::Five" -> 5
          "Value::Six" -> 6
          "Value::Seven" -> 7
          "Value::Eight" -> 8
          "Value::Nine" -> 9
          "Value::Ten" -> 10
          "Value::Jack" -> 11
          "Value::Queen" -> 12
          "Value::King" -> 13
          "Value::Ace" -> 14
          _ -> 0
        end
    end
  end

  @doc """
  first_is_less(card1, card1)
  true if card1 < card2
  false otherwise

  ### Examples

  iex(7)> Cards.first_is_less({Card, "Suit::Spade", "Value::Two"}, {Card, "Suit::Heart", "Value::Queen"})
  false

  """

  @spec first_is_less(Card.t, Card.t) :: boolean
  def first_is_less(card1, card2) do
    value(card1) < value(card2)
  end

  @doc """
  cards_are_equal(card1, card2)
  true if card1 == card2
  false otherwise

  ### Examples

  iex(8)> Cards.cards_are_equal({Card, "Suit::Spade", "Value::Ace"}, {Card, "Suit::Heart", "Value::Queen"})
  false

  """

  @spec cards_are_equal(Card.t, Card.t) :: boolean
  def cards_are_equal(card1, card2) do
    value(card1) == value(card2)
  end
end
