"""Functions to help play and score a game of blackjack.

How to play blackjack:    https://bicyclecards.com/how-to-play/blackjack/
"Standard" playing cards: https://en.wikipedia.org/wiki/Standard_52-card_deck
"""


def value_of_card(card):
    """Determine the scoring value of a card.

    :param card: str - given card.
    :return: int - value of a given card.  See below for values.

    1.  'J', 'Q', or 'K' (otherwise known as "face cards") = 10
    2.  'A' (ace card) = 1
    3.  '2' - '10' = numerical value.
    """
    match card:
        case "J" | "Q" | "K" | "10":
            return 10
        case "A":
            return 1
        case _:
            return int(card)


def higher_card(card_one, card_two):
    """Determine which card has a higher value in the hand.

    :param card_one, card_two: str - cards dealt in hand.  See below for values.
    :return: str or tuple - resulting Tuple contains both cards if they are of equal value.

    1.  'J', 'Q', or 'K' (otherwise known as "face cards") = 10
    2.  'A' (ace card) = 1
    3.  '2' - '10' = numerical value.
    """
    if value_of_card(card_one) == value_of_card(card_two):
        return (card_one, card_two)

    return card_one if value_of_card(card_one) > value_of_card(card_two) else card_two


def value_of_ace(card_one, card_two):
    """Calculate the most advantageous value for the ace card.

    :param card_one, card_two: str - card dealt. See below for values.
    :return: int - either 1 or 11 value of the upcoming ace card.

    1.  'J', 'Q', or 'K' (otherwise known as "face cards") = 10
    2.  'A' (ace card) = 11 (if already in hand)
    3.  '2' - '10' = numerical value.
    """
    card_one_value = 11 if card_one == "A" else value_of_card(card_one)
    card_two_value = 11 if card_two == "A" else value_of_card(card_two)

    # if over 10 then for example 11 + 11 would overflow
    # 21 - 11 = 10
    # 10 < 11

    # I was worried about case when ('A', 'A') which sums to 22,
    # however 22 >= 11 and everything works
    return 1 if (card_one_value + card_two_value) >= 11 else 11


def is_blackjack(card_one, card_two):
    """Determine if the hand is a 'natural' or 'blackjack'.

    :param card_one, card_two: str - card dealt. See below for values.
    :return: bool - is the hand is a blackjack (two cards worth 21).

    1.  'J', 'Q', or 'K' (otherwise known as "face cards") = 10
    2.  'A' (ace card) = 11 (if already in hand)
    3.  '2' - '10' = numerical value.
    """
    # Possible Blackjack Hands:
    # ('A', '10') ('10', 'A')
    # ('A', 'J')  ('J', 'A')
    # ('A', 'Q') ('Q', 'A')
    # ('A', 'K') ('K', 'A')

    if card_one == "A":
        match card_two:
            case "10" | "J" | "Q" | "K":
                return True

    if card_two == "A":
        match card_one:
            case "10" | "J" | "Q" | "K":
                return True

    return False


def can_split_pairs(card_one, card_two):
    """Determine if a player can split their hand into two hands.

    :param card_one, card_two: str - cards dealt.
    :return: bool - can the hand be split into two pairs? (i.e. cards are of the same value).
    """

    return value_of_card(card_one) == value_of_card(card_two)


def can_double_down(card_one, card_two):
    """Determine if a blackjack player can place a double down bet.

    :param card_one, card_two: str - first and second cards in hand.
    :return: bool - can the hand can be doubled down? (i.e. totals 9, 10 or 11 points).
    """

    # Possible Double Down Combinations with Ace:
    # (A, 9) (A, 10) (A, J) (A, Q) (A, K)
    # (9, A) (10, A) (J, A) (Q, A) (K, A)
    # So either 9 or 10 value on the other card
    # (A, A) should return sum of 12

    # Thanks to: https://exercism.org/tracks/python/exercises/black-jack/solutions/wenngle
    return 9 <= sum(value_of_card(card) for card in (card_one, card_two)) <= 11
