"""Anagram module

Has only one function - find_anagrams
"""

# https://the-algorithms.com/algorithm/anagrams?lang=python
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
    word_sorted = "".join(sorted(word))

    anagrams_list = []
    for candidate in candidates:
        if word != candidate.lower() and word_len == len(candidate):
            # sorted returns a list, so I convert it back to str
            candidate_sorted = "".join(sorted(candidate.lower()))
            if word_sorted == candidate_sorted:
                anagrams_list.append(candidate)

    return anagrams_list
