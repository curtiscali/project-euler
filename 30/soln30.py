#!/usr/bin/python3.3

fifth_powers = [0, 1, 32, 243, 1024, 3125, 7776, 16807, 32768, 59049]

def fifth_power_sum(number):
    power_sum = 0
    while number > 0:
        power_sum += fifth_powers[number % 10]
        number //= 10
    return power_sum

print(
    sum((i for i in range(1000, 1000000) if fifth_power_sum(i) == i))
)
    
