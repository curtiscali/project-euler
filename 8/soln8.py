def read_number(fileName):
    number = ""
    for line in open(fileName):
        number += line.strip()
    return number

def generate_fivedigit_combinations(number):
    combos = []
    for i in range(0, len(number) - 5):
        combos.append(int(number[i:i+5]))
    return combos

def digit_product(n):
    product = 1
    while n > 0:
        product *= (n % 10)
        n //= 10
    return product

def main():
    combos = generate_fivedigit_combinations(read_number("number.txt"))
    products = [digit_product(n) for n in combos]
    print(max(products))

if __name__ == '__main__':
    main()