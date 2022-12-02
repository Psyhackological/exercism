"""Anagram module

Has only one function - find_anagrams
"""


def find_anagrams(word: str, candidates: list[str]) -> list[str]:
    """Find anagrams function

    Args:
        word (str): an anagram word to compare to each element of the candidates
        candidates (list): candidates list to be checked for an anagram

    Returns:
        list: the list of anagrams from candidates
    """
    word = word.lower()
    word_len = len(word)

    anagrams_list = []
    for candidate in candidates:
        before_lower = candidate
        candidate = candidate.lower()

        if word != candidate and word_len == len(candidate):
            counter = 0
            for letter in candidate:
                if word.count(letter) == candidate.count(letter):
                    counter += 1
                else:
                    break

            if counter == word_len:
                anagrams_list.append(before_lower)

    return anagrams_list
