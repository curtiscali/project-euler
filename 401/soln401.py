from math import sqrt
from functools import reduce
from time import clock

def factors(n):
    step = 2 if n%2 else 1
    return set(
        reduce(
            list.__add__, 
            ([i, n//i] for i in range(1, int(sqrt(n)) + 1, step) if n % i == 0)
        )
    )

def sigma2(n, cache={}):
    if n == 1:
        cache[1] = 1
        return 1

    all_factors = factors(n)

    if len(all_factors) == 2:
        #if the number is prime, there exists O(1) formula for sigma2
        result = (n*n) + 1
        cache[n] = result

        return result
    else:
        sigma = 0

        largest_cached = None
        for factor in all_factors:
            if factor in cache and factor % 2 == 0:
                largest_cached = factor
                break

        if largest_cached is not None:
            sigma += cache[largest_cached]

            for factor in all_factors:
                if largest_cached % factor != 0:
                    sigma += factor**2
        else:
            sigma = sum([x*x for x in all_factors])

        cache[n] = sigma
        return sigma


def SIGMA2(n, cache={}):
    if (n - 1) in cache:
        result = sigma2(n)
        cache[n] = result + cache[n - 1]
    else:
        result = sum([sigma2(n) for n in range(1, n+1)])
        cache[n] = result
    
    return cache[n]

def main():
    cache = {}
    for i in range(1,10**15):
        SIGMA2(i, cache)

    print("Result:", cache[10**15] % 10**9)

if __name__ == '__main__':
    main()