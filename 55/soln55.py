def reverse(n):
    reverse = 0
    while n > 0:
        reverse *= 10
        reverse += n % 10
        n //= 10
    return reverse

def is_lychrel_number(n):
    for _ in range(50):
        number = n + reverse(n)
        if number == reverse(number):
            return False
        n = number
    return True 

def main():
    print(sum((1 for i in range(1, 10000) if is_lychrel_number(i))))

if __name__ == '__main__':
    main()