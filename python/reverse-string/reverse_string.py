""" Reverse String Module """


def reverse(text):
    """Reverse a string

    Param:
        text (str): the text to reverse

    Returns:
        str: the same string but backwards
    """

    # ::-1 simply means: start from last character to the first character
    # ::1 would mean:    start from first character to the last character
    return text[::-1]
