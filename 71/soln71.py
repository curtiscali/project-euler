#!/usr/bin/python3

from fractions import Fraction

def get_fractions_with_denom(denom):
    fractions = set()
    for i in range(1, denom):
        fractions.add(Fraction(i, denom))
    return fractions

super_set = set()

for i in range(1, 1000001):
    super_set |= get_fractions_with_denom(i)

print(sorted(super_set))
