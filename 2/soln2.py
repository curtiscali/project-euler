#!/usr/bin/env python3

def main():
    fib1, fib2, fib_sum = 0, 1, 0

    while fib2 <= 4000000:
        new_fib = fib1 + fib2
        fib1 = fib2
        fib2 = new_fib

        if new_fib % 2 == 0:
            fib_sum += new_fib

    print(fib_sum)

if __name__ == '__main__':
    main()