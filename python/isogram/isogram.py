"""returns True or False from a string"""


def is_isogram(string):
    """
    :param string - string to check if is isogram
    :return bool - returns no character duplicates
    """
    # no spaces, no hyphens, everything lower-cased
    word_to_check = string.replace(" ", "").replace("-", "").lower()

    for character in word_to_check:
        # if more than one occurances of characters
        if word_to_check.count(character) > 1:
            return False

    return True
