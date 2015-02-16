#!/usr/bin/python3.3

def is_increasing(number):
    max_digit = number % 10
    number //= 10

    while number > 0:
        digit = number % 10
        if digit > max_digit:
            return False
        max_digit = digit
        number //= 10

    return True

def is_decreasing(number):
    min_digit = number % 10
    number //= 10

    while number > 0:
        digit = number % 10
        if digit < min_digit:
            return False
        min_digit = digit
        number //= 10

    return True

def is_bouncy(number):
    return not (is_increasing(number) or is_decreasing(number))


num_total = 21780
num_bouncy = int(0.9 * num_total)

while (num_bouncy / num_total) < 0.99:
    if is_bouncy(num_total):
        num_bouncy += 1
    num_total += 1

print(num_total)
