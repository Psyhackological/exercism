""" The Bob module """


def response(hey_bob=""):
    """The Bob's Reponse function

    Arguments:
        hey_bob (str): say something to bob!

    Returns:
        str: Bob's going to think through what to say to you.
    """

    hey_bob = hey_bob.strip()

    if not hey_bob:
        return "Fine. Be that way!"

    if hey_bob.isupper() and "?" == hey_bob[-1]:
        return "Calm down, I know what I'm doing!"

    if "?" == hey_bob[-1]:
        return "Sure."

    if hey_bob.isupper():
        return "Whoa, chill out!"

    return "Whatever."
