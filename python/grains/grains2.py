""" The Best Grains module! """


def square(number):
    """Square function that returns 2 to the power of (number - 1)

    Args:
        number (int): 2 to be multiplied by the number - 1

    Returns:
        int: Returns  2 ** (number - 1)
    """
    if number < 1 or number > 64:
        raise ValueError("square must be between 1 and 64")

    # Given a 1 it is 2 ** 0 = 1
    # Given a 2 it is 2 ** 1 = 2
    # Given a 3 it is 2 ** 2 = 4
    # 4 / 2 = 2 / 2 = 1
    return 2 ** (number - 1)


def total():
    """Total grains on the checkboard with every square doubled than the previous one

    Returns:
        int: Returns the sum of the 64 squares with each doubled than previous one.
    """

    # Not including 65, step is 1 by default
    checkboard_numbers = range(1, 65)

    # map(function, iterable)
    checkboard_grains = map(square, checkboard_numbers)

    # Sums the checkboard_numbers but mapped through its square
    total_grains = sum(checkboard_grains)

    return total_grains
