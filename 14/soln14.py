def chainLength(n, cache):
    original = n

    length = 1
    while n > 1:
        if n % 2 == 0:
            if (n // 2) in cache:
                length += cache[n // 2]
                break
            else:
                n //= 2
        else:
            if ((3*n) + 1) in cache:
                length += cache[(3*n) + 1]
                break
            else:
                n = (3*n) + 1
        length += 1

    cache[original] = length
    return length

if __name__ == '__main__':
    solutions = {1: 1}
    longestPath, longestPathNum = (1,1)

    for i in range(2, 1000001):
        length = chainLength(i, solutions)
        if length > longestPath:
            longestPath = length
            longestPathNum = i

    print(longestPathNum)