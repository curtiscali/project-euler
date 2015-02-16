#!/usr/bin/python3

def prime_sieve(limit):
    a = [True] * limit                          # Initialize the primality list
    a[0] = a[1] = False

    for (i, isprime) in enumerate(a):
        if isprime:
            for n in xrange(i*i, limit, i):     # Mark factors non-prime
                a[n] = False
    return a

def num_digits(n):
    digit_count = 0
    while n > 0:
        digit_count += 1
        n //= 10
    return digit_count

def is_pandigital(n):
    digits = []
    copy = n
    while copy > 0:
        digits.append(copy % 10)
        copy //= 10
    return len(set(digits)) == num_digits(n)


primes = prime_sieve(987654321)

largest_pandigital = 0
for i in range(100000000, len(primes)):
    if primes[i]:
        if is_pandigital(i) and i > largest_pandigital:
            largest_pandigital = i
