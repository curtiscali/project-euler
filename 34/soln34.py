#!/usr/bin/python3.3

factorial_values = [1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880]

def digit_factorial_sum(number):
    factorial_sum = 0
    while number > 0:
        factorial_sum += factorial_values[number % 10]
        number //= 10
    return factorial_sum

print(
    sum((i for i in range(10, 2540161) if digit_factorial_sum(i) == i))
)
