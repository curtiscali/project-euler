from math import sqrt

def is_prime(n):
    for i in range(2, int(sqrt(n)) + 2):
        if n % i == 0 and n != i:
            return False
    return True

def main():
    prime_count, value = 1, 2
    while prime_count < 10001:
        value += 1
        if is_prime(value):
            prime_count += 1
    print(value)

if __name__ == '__main__':
    main()
