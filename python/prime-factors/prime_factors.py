""" Prime Factors module

This module has only 1 function - factors
"""

# https://the-algorithms.com/algorithm/prime-factors?lang=python
# Probably the most efficient solution, but harder (for me) to understand.
def factors(value: int) -> list[int]:
    """Factors function

    Args:
        value (int): value to search for factors.

    Returns:
        list: the list of factors.
    """
    factors_list = []
    factor = 1
    while value >= 2:
        factor += 1
        if value % factor == 0:
            factors_list.append(factor)
            value //= factor
            factor = 1

    return factors_list
