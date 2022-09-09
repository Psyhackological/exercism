"""Resistor's color code indetifier"""


def color_code(color):
    """Returns resistor's color code.

    :param color: resistor color string
    :returns: index of the resistor color

    """
    colors_list = colors()

    return colors_list.index(color)


def colors():
    """Creates resistor's color list.

    :returns: resistor's color list

    """
    colors_list = [
        "black",
        "brown",
        "red",
        "orange",
        "yellow",
        "green",
        "blue",
        "violet",
        "grey",
        "white",
    ]

    return colors_list
