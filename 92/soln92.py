def digit_square_sum(n):
    digit_sum = 0
    while n > 0:
        digit_sum += (n % 10) * (n % 10)
        n //= 10
    return digit_sum

def digit_chain(n, cache={}):
    copy = n
    while copy != 89 and copy != 1:
        copy = digit_square_sum(copy)

        if copy in cache:
            if cache[copy]:
                cache[n] = True
                return cache[n]
            else:
                cache[n] = False
                return cache[n]


    cache[n] = copy == 89
    return cache[n]

def main():
    num_qualified = 0
    for i in range(1, 10000001):
        if digit_chain(i):
            num_qualified += 1
    print(num_qualified)

if __name__ == '__main__':
    main()