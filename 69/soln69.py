#!/usr/bin/python3

from fractions import gcd

def phi(n, totients={2: 1, 3: 2, 6: 2}):
    if n in totients:
        return totients[n]
    else:
        phi = 1
        prev_rel_primes = []
        for i in range(2, n):
            if gcd(n, i) == 1:
                phi += 1
                prev_rel_primes.append(i)

        for p in prev_rel_primes:
            totients[n * p] = phi * totients[p]

        totients[n] = phi
        return phi

def main():
    max_n = -1
    max_ratio = -1.0
    for i in range(2, 1000001):
        phi_val = phi(i)
        if i / phi_val > max_ratio:
            max_ratio = i / phi_val
            max_n = i

    print(max_n)

if __name__ == '__main__':
    main()
