""" The Best Grains module! """


def square(number):
    """Square function that returns 2 to the power of (number - 1)

    Args:
        number (int): 2 to be multiplied by the number - 1

    Returns:
        int: Returns  2 ** (number - 1)
    """
    if not 1 <= number <= 64:
        raise ValueError("square must be between 1 and 64")

    return 1 << (number - 1)


def total():
    """Total grains on the checkboard with every square doubled than the previous one

    Returns:
        int: Returns the sum of the 64 squares with each doubled than previous one.
    """

    # z << n
    # z - numbers of zeroes to push from right to left
    # n - normal binary number that is going to be the same
    # but with pushed zeroes from right to left
    # example:
    # 1 << 2
    # decimal = binary
    # 2 = 10
    # pushing 1 zero from right to left
    # 100 = 10+0
    # 100 to decimal equals 4

    # 1 << 0 = 1    2^0
    # 1 << 1 = 2    2^1
    # 1 << 2 = 4    2^2

    return (1 << 64) - 1
