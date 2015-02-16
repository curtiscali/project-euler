def nCr(n, r):
    def go(x, y, acc):
        if y == 0 or x == y:
            return acc
        return go(x - 1, y - 1, (x / y) * acc)
    return go(n, r, 1)

def main():
    numGreater = 0
    for i in range(23, 101):
        for j in range(i):
            if nCr(i, j) > 1000000:
                numGreater += 1 
    print(numGreater)

if __name__ == '__main__':
    main()