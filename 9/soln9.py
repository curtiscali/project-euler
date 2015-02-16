#!/usr/bin/python3

from math import sqrt

def get_factor_pairs(n):
    yield (1, n)
    for i in range(2, int(sqrt(n)) + 1):
        if n % i == 0:
            yield (i, n // i)

def generate_triple(factor_pair):
    m, n = factor_pair[0], factor_pair[1]
    return (abs(m**2 - n**2), 2 * m * n, m**2 + n**2)

def main():
    for i in range(100, 501, 2):
        for factor_pair in get_factor_pairs(i / 2):
            triple = generate_triple(factor_pair)
            a, b, c = triple[0], triple[1], triple[2]
            if a + b + c == 1000:
                print(a * b * c)
                return

if __name__ == '__main__':
    main()
