"""Resistor Color Duo module
    only 1 method - value that returns a str (merged 2 resistor color values)"""


def value(colors):
    """Returns merged Resistor Color Values (up to 2)

    Args:
        colors (list): The list of colors (more than 2 elements are ignored)

    Returns:
        str: Merged 2 Resistor Color Values according to r_color_vales
    """

    r_color_vales = [
        "black",  # 0
        "brown",  # 1
        "red",    # 2
        "orange", # 3
        "yellow", # 4
        "green",  # 5
        "blue",   # 6
        "violet", # 7
        "grey",   # 8
        "white",  # 9
    ]

    color_1, color_2 = colors[0].lower(), colors[1].lower()

    if color_1 == "black" and color_2 in r_color_vales:
        return r_color_vales.index(color_2)

    if color_1 in r_color_vales and color_2 in r_color_vales:
        color_12_values = (
            f"{(r_color_vales.index(color_1))}{r_color_vales.index(color_2)}"
        )
        return int(color_12_values)

    return "??"
