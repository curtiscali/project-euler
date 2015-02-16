from time import clock

def digits(n):
    digitList = []

    while n > 0:
        digitList.append(n % 10)
        n //= 10

    return sorted(digitList)

def multiples(n):
    return set(
        (digits(x) for x in (n, 2*n, 3*n, 4*n, 5*n))
    )

def main():
    start = clock()
    n = 1
    while len(multiples(n)) > 1:
        n +=1
    
    print(n)
    print(clock() - start)

if __name__ == '__main__':
     main()