""" The Best Pling Plang and Plong module! """


def convert(number=0):
    """Converts number to Pling, Plang and Plong
    Arguments:
        number(int) - arguments to convert into Pling, Plang or Plong

    Returns:
        str - Pling, Plang and Plong or str(number)
    """
    result = ""

    # if divisible by 3 then it returns 0
    # 0 is False, so not False equals True
    if not number % 3:
        result += "Pling"

    if not number % 5:
        result += "Plang"

    if not number % 7:
        result += "Plong"

    # if empty string
    if not result:
        return str(number)

    return result
