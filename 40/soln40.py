#!/usr/bin/python3

from math import log10

def num_digits(n):    
    digit_count = 0
    while n > 0:
        digit_count += 1
        n //= 10
    return digit_count

def nth_digit(n, digit_num):
    curr_digit = num_digits(n)
    
    digit = n % 10
    n //= 10
    curr_digit -= 1

    while n > 0 and curr_digit > digit_num:
        digit = n % 10
        curr_digit -= 1
        n //= 10

    return digit


def champernowne_digit(digit_num):
    total_digits = 1
    current = 1
    while total_digits < digit_num:
        total_digits += num_digits(current)
        current += 1

    #calculate offset from desired digit number
    offset = total_digits - digit_num
    
    #go back the number of digits specified in offset
    return nth_digit(current, offset)


product = 1
for i in range(7):
    product *= champernowne_digit(10**i)

print(product)
