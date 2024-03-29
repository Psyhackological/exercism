"""
Triangle Type Determination Module.

This module provides functions to determine if a given set of sides can form a
triangle, and if so, whether the triangle is equilateral, isosceles, or scalene.

Optimizations include the use of mathematical properties and Python's set operations
to reduce computational overhead and improve readability.
"""


def is_triangle(sides):
    """Check if the given sides can form a triangle."""
    # Triangle Inequality Theorem: a + b > c, b + c > a, and a + c > b
    a, b, c = sorted(sides)
    return a > 0 and a + b >= c


def equilateral(sides):
    """Return True if the triangle is equilateral, False otherwise."""
    # All sides must be equal and must form a triangle
    return is_triangle(sides) and len(set(sides)) == 1


def isosceles(sides):
    """Return True if the triangle is isosceles, False otherwise."""
    # At least two sides are equal and must form a triangle
    return is_triangle(sides) and len(set(sides)) <= 2


def scalene(sides):
    """Return True if the triangle is scalene, False otherwise."""
    # No sides are equal and must form a triangle
    return is_triangle(sides) and len(set(sides)) == 3
