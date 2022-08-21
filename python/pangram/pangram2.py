""" ascii_lowercase is a constant of ascii letters"""
from string import ascii_lowercase


def count_letters(sentence):
    """
    creates a dictionary with ascii letters
    and then counts letters in lowercase sentence with count()
    """
    ascii_lowercase_dict = {}

    for ascii_char in ascii_lowercase:
        ascii_lowercase_dict[ascii_char] = sentence.lower().count(ascii_char)

    return ascii_lowercase_dict


def is_pangram(sentence):
    """
    given a sentence it returns boolean value
    if 0 is not present in counted letters
    then True
    otherwise False
    """
    ascii_lowercase_dict = count_letters(sentence)

    return 0 not in ascii_lowercase_dict.values()
