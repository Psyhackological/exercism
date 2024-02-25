"""
Triangle Type Determination Module.

This module provides functions to determine if a given set of sides can form a
triangle, and if so, whether the triangle is equilateral, isosceles, or scalene.
"""


def is_triangle(sides):
    """Check if the given sides can form a triangle."""
    # Triangle Inequality Theorem: a + b > c, b + c > a, and a + c > b
    return all(side > 0 for side in sides) and \
        (sides[0] + sides[1] >= sides[2]) and \
        (sides[1] + sides[2] >= sides[0]) and \
        (sides[0] + sides[2] >= sides[1])


def equilateral(sides):
    """Return True if the triangle is equilateral, False otherwise."""
    # All sides must be equal and must form a triangle
    return is_triangle(sides) and sides[0] == sides[1] == sides[2]


def isosceles(sides):
    """Return True if the triangle is isosceles, False otherwise."""
    # At least two sides are equal and must form a triangle
    return is_triangle(sides) and \
        (sides[0] == sides[1] or sides[1] == sides[2] or sides[0] == sides[2])


def scalene(sides):
    """Return True if the triangle is scalene, False otherwise."""
    # No sides are equal and must form a triangle
    return is_triangle(sides) and \
        sides[0] != sides[1] and sides[1] != sides[2] and sides[0] != sides[2]
