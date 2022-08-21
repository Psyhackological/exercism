""" ascii_lowercase is a constant of ascii letters"""
from string import ascii_lowercase


def is_pangram(sentence):
    """
    thanks for showing it up https://www.geeksforgeeks.org/python-set-check-string-panagram/
    is_it_really returns True or False
    it creates set(ascii_lowercase) and set(sentence.lower())
    then substracts one from another (set_ascii - set_sentence)
    if substraction is empty then is_pangram = True
    if not then is_panagram = False
    """

    is_it_really = (set(ascii_lowercase) - set(sentence.lower())) == set([])

    return is_it_really
