"""returns is a year a leap year or not""" ""


def leap_year(year):
    """
    divisible by 4

    not divisible by 100
    or divisible by 400

    otherwise False
    """
    return year % 4 == 0 and year % 100 != 0 or year % 400 == 0
