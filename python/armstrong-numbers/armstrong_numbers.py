"""returns if number is armstrong_sum or not"""


def is_armstrong_number(number):
    """
    armstrong_sum  - speaks for itself
    length         - number of digits in number
    """
    armstrong_sum = 0
    length = len(str(number))
    for digit in str(number):
        armstrong_sum += int(digit) ** length

    return armstrong_sum == number
