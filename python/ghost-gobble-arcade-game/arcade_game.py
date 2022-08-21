"""Functions for implementing the rules of the classic arcade game Pac-Man."""


def eat_ghost(power_pellet_active, touching_ghost):
    """Verify that Pac-Man can eat a ghost if he is empowered by a power pellet.

    :param power_pellet_active: bool - does the player have an active power pellet?
    :param touching_ghost: bool - is the player touching a ghost?
    :return: bool - can the ghost be eaten?
    """
    # power_pellet AND touching_ghost
    # False, False - no power, no touching
    # False, True  - no power, touching
    # True,  False - power,    no touching
    # True,  True  - power,    touching
    # ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    # This is what we want (and)

    return power_pellet_active and touching_ghost


def score(touching_power_pellet, touching_dot):
    """Verify that Pac-Man has scored when a power pellet or dot has been eaten.

    :param touching_power_pellet: bool - is the player touching a power pellet?
    :param touching_dot: bool - is the player touching a dot?
    :return: bool - has the player scored or not?
    """
    # power_pellet_touch OR dot_touch
    # False, True  - no touching, touching
    # True,  False - touching,   no touching
    # True,  True  - touching,   touching
    # False, False - no touching, no touching
    # ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    # That is what we do NOT want (or)
    # Same as `not and`

    return touching_power_pellet or touching_dot


def lose(power_pellet_active, touching_ghost):
    """Trigger the game loop to end (GAME OVER) when Pac-Man touches a ghost without his power pellet.

    :param power_pellet_active: bool - does the player have an active power pellet?
    :param touching_ghost: bool - is the player touching a ghost?
    :return: bool - has the player lost the game?
    """

    # power_pellet AND touching_ghost
    # False, False - no power, no touching = still_playing            = False
    # True,  False - power,    no touching = still_playing            = False
    # True,  True  - power,    touching    = still_playing_and_eating = False
    # False, True  - no power, touching    = lose                     = True
    # ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    # That is what we want
    # Invert False, do it with and and done

    return not power_pellet_active and touching_ghost


def win(has_eaten_all_dots, power_pellet_active, touching_ghost):
    """Trigger the victory event when all dots have been eaten.

    :param has_eaten_all_dots: bool - has the player "eaten" all the dots?
    :param power_pellet_active: bool - does the player have an active power pellet?
    :param touching_ghost: bool - is the player touching a ghost?
    :return: bool - has the player won the game?
    """

    # Condition to win:
    # has_eaten_all_dots = True
    # and not lose

    return has_eaten_all_dots and not lose(power_pellet_active, touching_ghost)
