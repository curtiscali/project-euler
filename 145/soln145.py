#!/usr/bin/python3

def reverse(n):
    rev = 0
    while n > 0:
        rev *= 10
        rev += n % 10
        n //= 10
    return rev

def all_digits_odd(n):
    while n > 0:
        digit = n % 10
        if digit % 2 == 0:
            return False
        n //= 10
    return True


existing_reversibles = set()
num_reversibles = 0
for i in range(1, 1000):
    if i % 10 != 0: 
        rev = reverse(i)
        rev_sum = i + rev
        if all_digits_odd(rev_sum):
            num_reversibles += 1

print(num_reversibles)
