"""Functions for creating, transforming, and adding prefixes to strings."""


def add_prefix_un(word):
    """Take the given word and add the 'un' prefix.

    :param word: str - containing the root word.
    :return: str - of root word prepended with 'un'.
    """
    return f"un{word}"


def make_word_groups(vocab_words):
    """Transform a list containing a prefix and words into a string with the prefix followed by
    the words with prefix prepended.

    :param vocab_words: list - of vocabulary words with prefix in first index.
    :return: str - of prefix followed by vocabulary words with
            prefix applied.

    This function takes a `vocab_words` list and returns a string
    with the prefix and the words with prefix applied, separated
     by ' :: '.

    For example: list('en', 'close', 'joy', 'lighten'),
    produces the following string: 'en :: enclose :: enjoy :: enlighten'.
    """
    prefix = vocab_words[0]

    # vocab_words[1:] is a list slice without prefix
    words_with_prefix = [prefix + word for word in vocab_words[1:]]

    # prefix inthe beggining is missing, so let's add it back again
    words_with_prefix.insert(0, prefix)

    # this adds in-between " :: " and creates a string from list
    return " :: ".join(words_with_prefix)


def remove_suffix_ness(word):
    """Remove the suffix from the word while keeping spelling in mind.

    :param word: str - of word to remove suffix from.
    :return: str - of word with suffix removed & spelling adjusted.

    For example: "heaviness" becomes "heavy", but "sadness" becomes "sad".
    """
    # we do not want "ness" anyway
    word = word.replace("ness", "")

    # if word ends with
    if "i" == word[-1]:
        return f"{word[:-1]}y"

    # if not return word
    return word


def adjective_to_verb(sentence, index):
    """Change the adjective within the sentence to a verb.

    :param sentence: str - that uses the word in sentence.
    :param index: int - index of the word to remove and transform.
    :return: str - word that changes the extracted adjective to a verb.

    For example, ("It got dark as the sun set", 2) becomes "darken".
    """
    suffix = "en"

    # split string to list by space
    sentence_to_list = sentence.split(" ")
    adjective = sentence_to_list[index]

    if index == -1 and adjective.endswith("."):
        # return without a dot
        return f"{adjective[:-1]}{suffix}"

    # no dot deteted so return this
    return f"{adjective}{suffix}"
