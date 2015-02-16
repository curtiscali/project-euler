#!/usr/bin/python3.3

numbers = filter(lambda x: x > 10 and x % 10 != 0, range(1, 101))

def digital_sum(number):
    digit_sum = 0
    while number > 0:
        digit_sum += number % 10
        number //= 10
    return digit_sum


max_digit_sum = 0
for i in numbers:
    for j in range(2, 101):
        digit_sum = digital_sum(i**j)
        if digit_sum > max_digit_sum:
            max_digit_sum = digit_sum

print(max_digit_sum)
