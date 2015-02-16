#!/usr/bin/python3

def f(n, k, prevs={1: 1}):
    result = 0
    if n in prevs:
        result = prevs[n]
    elif n-1 in prevs:
        result = (n**k) + prevs[n-1]
    else:
        for i in range(1, n+1):
            result += (i**k)

    prevs[n] = result
    return result

def S(n, k):
    return sum(f(i, k) for i in range(1, n + 1))

def main():
    print(S(10**12, 10000))
    pass


if __name__ == '__main__':
    main()
